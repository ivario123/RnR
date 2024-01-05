# Borrow checking and linearization

The borrow checker works based on 2 simple data structures, 
both are hashmaps, one hashmap that maps $`1->many`$ [`borrows`](./src/borrow_checker/env.rs#13) and one hashmap that
maps $`1->1`$ [`borrowers`](./src/borrow_checker/env.rs#14).
`borrows` tracks how many, and from where a specific value is borrowed so for this example

```rust
let a = 2;
let b = &a;
let c = &a;
```

borrowers would contain 1 single key, with a corresponding vector of borrowers. While the borrowers map tracks who is borrowing what,
so for the example above it would contain 2 keys,

```bash
 "b" -> "a",
 "c" -> "a",
```

This means that borrows, ideally, takes $`O(n)`$ space, and $`O(1)`$ time, as it merely inserts/deletes keys from a hashmap.
However, an issue arises with this implementation when there are multiple identifiers with the same name, say 

```rust
let a = 2;
let b = &a;
let a = 2;
let c = &a;
*b;
```

Would invalidate $b$, but this does not need to be the case as no other new access will ever be made to a, the b reference will always be sound.
For this case to be valid one needs to "linearize" the program, this can be done, quite simply by a few counters.
This implementation tracks, the number of scopes ever declared, the scope depth and the re assign counter in that scope.
And then simply reformats the identifier to make each identifier unique, using the following format

```rust
>{scope_depth}#{scope_counter}!{reassign_counter}_{original_identifier}
```

This format ensures that no 2 identifiers are the same and therefore the previously mentioned example now passes the borrow checker. The corresponding code now looks like

```rust
let >1#1!0_a = 2;
let >1#1!0_b = &>1#1!0_a;
let >1#1!1_a = 2;
let >1#1!0_c = &>1#1!1_a;
```

Which is a lot harder to read.

## Borrowing intermediate values

The system I have implemented does not support borrowing values that are not identifiers. This means that code such as

```rust
let a = &{2+3};
```

is invalid. To borrow intermediate values, the system uses introduces new identifier in a separate pass by recursively introducing new identifiers for every right hand side of unary operators until the expression can be determined by a single unary operator on a single identifier.
The above code would be represented as

```rust
let #1_unop = {2+3};
let a = &#1_unop;
```

which now works with the above described paradigm.

## End Of Life

When a variable goes out of scope, that variable is `finalized`. This means two things, if the variable is unused it raises an error ( this is stricter than rust, but in my opinion leads to cleaner code. ). And finally it destroys all references to that variable. This means that code like

```rust
let mut a = 2;
let b = &mut a;
let a = 1;
a;
*b = 1;
let mut b = &a;
{
    let a = 2;
    b = &a;
};
*b; // Error here
```

will be rejected. When the system destroys a reference it removes the variables that borrow it from the `borrowers` hashmap, this means that checking if a variable is valid when dereferencing it is as simple as checking if the key exists in the `borrowers` hashmap.
