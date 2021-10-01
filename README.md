# d7050e_lab6

In this final lab we will put everything together into a complete frontend for a subset of Rust.

## Learning outcomes

In this (final) lab, we will introduce references. At a minimum your interpreter `vm.rs` should correctly handle references, local scopes and passing of arguments as references. To that end you may assume all variables (and references) to be mutable. In case of ill-formed programs, your interpreter may return with an error (or even panic).

The type checker `type_check` should at a minimum reject programs with ill formed types (without regard to life-times, mutability and aliasing).

Optionally (towards higher grades) you can:

- Life-time/scoping analysis, to ensure that all references goes to current (or outer) scope(s). (Easy.)

- Mutability analysis, ensuring that mutations are only allowed for mutable data (even through references). (Moderately complex.)
  
- Aliasing analysis, ensuring that illegal borrows are rejected by implementing a borrow checker (`bc.rs`). (Hard.)
  
You will also learn how a command line interface (cli) can be easily added to your application, allowing your compiler to be run directly from your terminal. (You can install your compiler `cargo install --path .`, and run it as `rnr --help`. You can change the name of the application in the `Cargo.toml` file.

---

## Workflow

Start by back-porting your code from previous labs. It is encouraged to follow the AST given, and parse according to Rust syntax (then the given tests and examples will guide you towards a working implementation). If stepping away from Rust syntax, you need to clearly motivate your choices and provide tests accordingly.

Keep your EBNF, type rules, SOS, and CHANGELOG up to date with the status of your development.

---

## Crate structure

The files are structures as follows:

Data structures:

- `ast`, the internal representation of the parse tree, also used for semantic analysis and natural interpretation. (Notice, a realistic compiler typically use a large number of internal representations, AST -> HIR -> MIR -> ..., but we keep it simple here.)
  
- `ast_traits`, functionality to display the AST in readable form.
  
- `parse`, the parser.

API:s:

- `error`, the definition of the error type.

- `common`, common API for processing the AST.

- `env`, a generic stacked environment for interpretation and semantic analysis.
  
Analysis:

- `type_check`, the type checker.

- `bc`, the optional borrow checker.
  
Interpretation:

- `vm`, an AST level interpreter for the natural semantics.

CLI:

- `main`, provides a simple command line interface.
  
Documentation:

- `README`, this file.
  
- `ebnf`, the EBNF grammar for RNR.
  
- `type_rules`, formalization of type rules for RNR.
  
- `sos`, formalization of semantics for RNR.
  
- `CHANGELOG`, tracking of project status.

---

## Some remarks

`&`, `*` and `mut` occur as unary operators in expressions. The recursive descent parser will render UnOp(op, expr), where expr is the complete expression (without regard to precedence). You may optionally take this into regard in your precedence climber.

Currently the CLI (`main`) supports only type checking and interpretation. You may optionally add precedence climbing and borrow checking of programs.

You can use this lab as the outset for your home exam. When you complete the mandatory parts (with corresponding tests passed and documentation updated) you will also pass the course.

For higher grades, document the set of added features in the CHANGELOG.

## Licence

Let knowledge be free! Free to use for any purpose.
