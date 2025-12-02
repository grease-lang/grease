use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Assignment {
        name: Token,
        value: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Grouping(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration {
        name: Token,
        type_annotation: Option<String>,
        initializer: Option<Expression>,
    },
    FunctionDeclaration {
        name: Token,
        parameters: Vec<(Token, Option<String>)>,
        return_type: Option<String>,
        body: Vec<Statement>,
    },
    Return {
        value: Option<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        variable: Token,
        iterable: Expression,
        body: Vec<Statement>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}