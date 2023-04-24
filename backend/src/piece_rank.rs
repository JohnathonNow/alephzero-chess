#[derive(Debug, Clone)]
pub enum Directive {
    One{location: Box<Location>, piece: String},
}
pub enum Location {
    Or(Box<Location>, Box<Location>),
    And(Box<Location>, Box<Location>),
    Not(Box<Location>),
    Expr(Box<Expression>),
}
pub enum Expression {
    Eq(Box<Variable>, Box<String>),
    Gt(Box<Variable>, Box<String>),
    Lt(Box<Variable>, Box<String>),
    Ne(Box<Variable>, Box<String>),
}

pub enum Variable {
    X,
    Y,
}

