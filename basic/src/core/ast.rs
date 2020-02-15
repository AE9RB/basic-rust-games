#[derive(Debug, PartialEq)]
pub enum Ast {
    Statement(Option<u32>), // Line number 0 to 65529, none is immediate
    Expression,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Line(u32), // 0 to 65529
    Data(Vec<Expression>),
    Def(Ident, Vec<Ident>),
    Dim(Ident, Vec<i32>),
    Let(Ident, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Single(f32),
    Double(f64),
    Integer(i32),
    String(String),
    Function(Ident, Vec<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Equality(Box<Expression>, Box<Expression>),
    Exponential(Box<Expression>, Box<Expression>),
}

#[derive(Debug, PartialEq)]
pub enum Ident {
    Plain(String),
}
