
use super::Expr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Bool(bool),
    Int(i32),
    Unit,
    String(String),
    Array(Vec<Box<Literal>>),
}
impl std::ops::Add for Literal {
    type Output = Literal;
    fn add(self, other: Literal) -> Self::Output {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => Literal::from(value + other),
                t => {
                    panic!(
                        "std::ops::Add is not implemented for {:?},{:?}",
                        "Literal::int", t
                    );
                }
            },
            t => {
                panic!(
                    "std::ops::Add is not implemented for {:?},{:?}",
                    "Literal::int", t
                );
            }
        }
    }
}

impl std::ops::Sub for Literal {
    type Output = Literal;
    fn sub(self, other: Literal) -> Self::Output {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => Literal::from(value - other),
                t => {
                    panic!(
                        "std::ops::Sub is not implemented for {:?},{:?}",
                        "Literal::int", t
                    );
                }
            },
            t => {
                panic!(
                    "std::ops::Sub is not implemented for {:?},{:?}",
                    "Literal::int", t
                );
            }
        }
    }
}

impl std::ops::Mul for Literal {
    type Output = Literal;
    fn mul(self, other: Literal) -> Self::Output {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => Literal::from(value * other),
                t => {
                    panic!(
                        "std::ops::Mul is not implemented for {:?},{:?}",
                        "Literal::int", t
                    );
                }
            },
            t => {
                panic!(
                    "std::ops::Mul is not implemented for {:?},{:?}",
                    "Literal::int", t
                );
            }
        }
    }
}

impl std::ops::Div for Literal {
    type Output = Literal;
    fn div(self, other: Literal) -> Self::Output {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => Literal::from(value / other),
                t => {
                    panic!(
                        "std::ops::Div is not implemented for {:?},{:?}",
                        "Literal::int", t
                    );
                }
            },
            t => {
                panic!(
                    "std::ops::Div is not implemented for {:?},{:?}",
                    "Literal::int", t
                );
            }
        }
    }
}

impl Literal {
    pub fn and(self, other: Literal) -> Literal {
        match self {
            Literal::Bool(value) => match other {
                Literal::Bool(other) => Literal::from(value && other),
                t => {
                    panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
                }
            },
            t => {
                panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
            }
        }
    }
    pub fn or(self, other: Literal) -> Literal {
        match self {
            Literal::Bool(value) => match other {
                Literal::Bool(other) => Literal::from(value || other),
                t => {
                    panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
                }
            },
            t => {
                panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
            }
        }
    }
    pub fn le(self, other: Literal) -> Literal {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => Literal::from(value > other),
                t => {
                    panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
                }
            },
            t => {
                panic!("And is not implemented for {:?},{:?}", "Literal::bool", t);
            }
        }
    }
}
impl std::cmp::PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self > other {
            return Some(std::cmp::Ordering::Greater);
        }
        if self < other {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
    fn lt(&self, other: &Self) -> bool {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => value < other,
                t => {
                    panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
                }
            },
            t => {
                panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
            }
        }
    }
    fn gt(&self, other: &Self) -> bool {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => value <= other,
                t => {
                    panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
                }
            },
            t => {
                panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
            }
        }
    }
    fn le(&self, other: &Self) -> bool {
        match self {
            Literal::Int(value) => match other {
                Literal::Int(other) => value <= other,
                t => {
                    panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
                }
            },
            t => {
                panic!("Cannot compare {:?} and {:?}", "Literal::int", t);
            }
        }
    }
    fn ge(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(value), Literal::Int(other)) => value >= other,
            (t1, t2) => {
                panic!("Cannot compare {:?} and {:?}", t1, t2);
            }
        }
    }
}

impl From<Literal> for bool {
    fn from(val: Literal) -> Self {
        match val {
            Literal::Bool(value) => value,
            value => panic!("Cannot treat {:?} as a boolean", value),
        }
    }
}
impl From<bool> for Literal {
    fn from(i: bool) -> Self {
        Literal::Bool(i)
    }
}

impl From<i32> for Literal {
    fn from(i: i32) -> Self {
        Literal::Int(i)
    }
}

impl From<Expr> for Literal {
    fn from(e: Expr) -> Self {
        match e {
            Expr::Lit(l) => l,
            _ => unreachable!(),
        }
    }
}
