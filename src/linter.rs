// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LintError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Linter {
    errors: Vec<LintError>,
    variables: HashMap<String, VariableInfo>,
    scope_depth: usize,
}

#[derive(Debug, Clone)]
struct VariableInfo {
    declared_at: (usize, usize),
    used: bool,
    scope_depth: usize,
}

impl Linter {
    pub fn new() -> Self {
        Linter {
            errors: Vec::new(),
            variables: HashMap::new(),
            scope_depth: 0,
        }
    }

    pub fn lint(&mut self, program: &Program) -> Vec<LintError> {
        self.errors.clear();
        self.variables.clear();
        self.scope_depth = 0;

        self.lint_program(program);

        // Check for unused variables
        for (name, info) in &self.variables {
            if !info.used {
                self.errors.push(LintError {
                    message: format!("Unused variable '{}'", name),
                    line: info.declared_at.0,
                    column: info.declared_at.1,
                });
            }
        }

        self.errors.clone()
    }

    fn lint_program(&mut self, program: &Program) {
        for statement in &program.statements {
            self.lint_statement(statement);
        }
    }

    fn lint_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::VariableDeclaration { name, type_annotation: _, initializer } => {
                // Mark variable as declared
                let var_name = match &name.token_type {
                    crate::token::TokenType::Identifier(s) => s.clone(),
                    _ => return, // Should not happen for variable declarations
                };
                let info = VariableInfo {
                    declared_at: (name.line, name.column),
                    used: false,
                    scope_depth: self.scope_depth,
                };
                self.variables.insert(var_name, info);

                if let Some(init) = initializer {
                    self.lint_expression(init);
                }
            }
            Statement::FunctionDeclaration { name: _, parameters, return_type: _, body } => {
                self.scope_depth += 1;

                // Add parameters as variables in function scope
                for (param, _) in parameters {
                    let param_name = match &param.token_type {
                        crate::token::TokenType::Identifier(s) => s.clone(),
                        _ => continue, // Should not happen for parameters
                    };
                    let info = VariableInfo {
                        declared_at: (param.line, param.column),
                        used: false,
                        scope_depth: self.scope_depth,
                    };
                    self.variables.insert(param_name, info);
                }

                for stmt in body {
                    self.lint_statement(stmt);
                }

                // Remove variables from this scope
                self.variables.retain(|_, info| info.scope_depth < self.scope_depth);
                self.scope_depth -= 1;
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.lint_expression(condition);
                self.scope_depth += 1;
                for stmt in then_branch {
                    self.lint_statement(stmt);
                }
                self.scope_depth -= 1;

                if let Some(else_stmts) = else_branch {
                    self.scope_depth += 1;
                    for stmt in else_stmts {
                        self.lint_statement(stmt);
                    }
                    self.scope_depth -= 1;
                }
            }
            Statement::While { condition, body } => {
                self.lint_expression(condition);
                self.scope_depth += 1;
                for stmt in body {
                    self.lint_statement(stmt);
                }
                self.scope_depth -= 1;
            }
            Statement::For { variable, iterable, body } => {
                self.lint_expression(iterable);

                self.scope_depth += 1;
                // Add loop variable
                let var_name = match &variable.token_type {
                    crate::token::TokenType::Identifier(s) => s.clone(),
                    _ => return, // Should not happen for for loop variables
                };
                let info = VariableInfo {
                    declared_at: (variable.line, variable.column),
                    used: false,
                    scope_depth: self.scope_depth,
                };
                self.variables.insert(var_name, info);

                for stmt in body {
                    self.lint_statement(stmt);
                }
                self.scope_depth -= 1;
            }
            Statement::Block(statements) => {
                self.scope_depth += 1;
                for stmt in statements {
                    self.lint_statement(stmt);
                }
                self.scope_depth -= 1;
            }
            Statement::Expression(expr) => {
                self.lint_expression(expr);
            }
            Statement::Return { value } => {
                if let Some(val) = value {
                    self.lint_expression(val);
                }
            }
            Statement::Use { module: _, alias: _ } => {
                // Imports are handled elsewhere
            }
            Statement::ClassDeclaration { name, superclass: _, methods } => {
                // Lint class name as variable
                let class_name = match &name.token_type {
                    crate::token::TokenType::Identifier(s) => s.clone(),
                    _ => return,
                };
                let info = VariableInfo {
                    declared_at: (name.line, name.column),
                    used: false,
                    scope_depth: self.scope_depth,
                };
                self.variables.insert(class_name, info);

                 // Lint methods
                 for method in methods {
                     self.lint_statement(method);
                 }
             }
             Statement::Try { try_block, catch_block } => {
                 self.scope_depth += 1;
                 for stmt in try_block {
                     self.lint_statement(stmt);
                 }
                 self.scope_depth -= 1;

                 self.scope_depth += 1;
                 for stmt in catch_block {
                     self.lint_statement(stmt);
                 }
                 self.scope_depth -= 1;
             }
        }
    }

    fn lint_expression(&mut self, expression: &Expression) {
        match expression {
            Expression::Identifier(token) => {
                // Mark variable as used
                if let crate::token::TokenType::Identifier(ref name) = token.token_type {
                    if let Some(info) = self.variables.get_mut(name) {
                        info.used = true;
                    }
                }
            }
            Expression::Binary { left, operator: _, right } => {
                self.lint_expression(left);
                self.lint_expression(right);
            }
            Expression::Unary { operator: _, right } => {
                self.lint_expression(right);
            }
            Expression::Assignment { name, value } => {
                self.lint_expression(value);
                // Mark variable as used (assignment counts as usage)
                if let crate::token::TokenType::Identifier(ref var_name) = name.token_type {
                    if let Some(info) = self.variables.get_mut(var_name) {
                        info.used = true;
                    }
                }
            }
            Expression::Call { callee, arguments } => {
                self.lint_expression(callee);
                for arg in arguments {
                    self.lint_expression(arg);
                }
            }
            Expression::ModuleAccess { module, member: _ } => {
                // Mark module access as usage if it's a variable
                if let crate::token::TokenType::Identifier(ref module_name) = module.token_type {
                    if let Some(info) = self.variables.get_mut(module_name) {
                        info.used = true;
                    }
                }
                // Member access doesn't count as variable usage for linting
            }
            Expression::Grouping(expr) => {
                self.lint_expression(expr);
            }
            Expression::Index { array, index } => {
                self.lint_expression(array);
                self.lint_expression(index);
            }
            Expression::NewInstance { class, arguments } => {
                self.lint_expression(class);
                for arg in arguments {
                    self.lint_expression(arg);
                }
            }
            Expression::PropertyAccess { object, property: _ } => {
                self.lint_expression(object);
            }
            Expression::MethodCall { object, method: _, arguments } => {
                self.lint_expression(object);
                for arg in arguments {
                    self.lint_expression(arg);
                }
            }
            Expression::SuperCall { method: _, arguments } => {
                for arg in arguments {
                    self.lint_expression(arg);
                }
            }
            Expression::Number(_) | Expression::String(_) | Expression::Boolean(_) | Expression::Null | Expression::Array(_) => {
                // Literals don't need linting
            }
        }
    }
}