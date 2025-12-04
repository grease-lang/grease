// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Identifier(Token),
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
    ModuleAccess {
        module: Token,
        member: Token,
    },
    Grouping(Box<Expression>),
    Array(Vec<Expression>),
    Dictionary(Vec<(Expression, Expression)>),
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    NewInstance {
        class: Box<Expression>,
        arguments: Vec<Expression>,
    },
    PropertyAccess {
        object: Box<Expression>,
        property: Token,
    },
    MethodCall {
        object: Box<Expression>,
        method: Token,
        arguments: Vec<Expression>,
    },
    SuperCall {
        method: Option<Token>,
        arguments: Vec<Expression>,
    },
    RustInline {
        code: String,
    },
    AsmInline {
        code: String,
    },
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
    Use {
        module: String,
        alias: Option<String>,
    },
    ClassDeclaration {
        name: Token,
        superclass: Option<Token>,
        methods: Vec<Statement>,
    },
    Try {
        try_block: Vec<Statement>,
        catch_block: Vec<Statement>,
    },
    Throw {
        value: Option<Expression>,
    },
    RustInline {
        code: String,
    },
    AsmInline {
        code: String,
    },
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}