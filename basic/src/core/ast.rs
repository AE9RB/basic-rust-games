pub enum Ident {
    Plain(String),
}

pub enum Statement {
    Data(Vec<Expr>),
    Def(Ident, Vec<Ident>),
    Dim(Ident, usize),
    Let(Ident, Box<Expr>),
}

pub enum Expr {
    Single(f32),
    Double(f64),
    Integer(i32),
    String(String),
    Function(Ident, Vec<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Equality(Box<Expr>, Box<Expr>),
    Exponential(Box<Expr>, Box<Expr>),
}
