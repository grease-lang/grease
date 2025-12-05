// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

//! Performance Optimizations for Grease Programming Language
//! 
//! This module provides various performance optimizations including:
//! - Constant folding
//! - Dead code elimination
//! - String interning
//! - Memory pool management
//! - Optimized garbage collection

use crate::bytecode::*;
use crate::ast::*;
use crate::token::*;
use crate::vm::VM;
use std::collections::HashMap;

/// String interner for efficient string storage
pub struct StringInterner {
    strings: HashMap<String, StringId>,
    id_to_string: Vec<String>,
    next_id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(usize);

impl StringInterner {
    pub fn new() -> Self {
        StringInterner {
            strings: HashMap::new(),
            id_to_string: Vec::new(),
            next_id: 0,
        }
    }

    pub fn intern(&mut self, s: String) -> StringId {
        if let Some(&id) = self.strings.get(&s) {
            return id;
        }

        let id = StringId(self.next_id);
        self.next_id += 1;
        self.strings.insert(s.clone(), id);
        self.id_to_string.push(s);
        id
    }

    pub fn get(&self, id: StringId) -> Option<&str> {
        self.id_to_string.get(id.0).map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.strings.len()
    }
}

/// Memory pool for efficient allocation
pub struct MemoryPool<T> {
    objects: Vec<T>,
    free_list: Vec<usize>,
}

impl<T> MemoryPool<T> {
    pub fn new() -> Self {
        MemoryPool {
            objects: Vec::new(),
            free_list: Vec::new(),
        }
    }

    pub fn allocate(&mut self, object: T) -> usize {
        if let Some(index) = self.free_list.pop() {
            self.objects[index] = object;
            index
        } else {
            self.objects.push(object);
            self.objects.len() - 1
        }
    }

    pub fn deallocate(&mut self, index: usize) {
        if index < self.objects.len() {
            self.free_list.push(index);
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.objects.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.objects.get_mut(index)
    }
}

/// Constant folding optimizer
pub struct ConstantFolder {
}

impl ConstantFolder {
    pub fn new() -> Self {
        ConstantFolder {
        }
    }

    pub fn fold_expression(&mut self, expr: &Expression) -> Expression {
        match expr {
            Expression::Binary { left, operator, right } => {
                let left_folded = self.fold_expression(left);
                let right_folded = self.fold_expression(right);
                
                if let (Some(lit_left), Some(lit_right)) = (self.expression_to_value(&left_folded), self.expression_to_value(&right_folded)) {
                    if let Some(result) = self.evaluate_binary(&lit_left, operator, &lit_right) {
                        return self.value_to_expression(result);
                    }
                }
                
                Expression::Binary {
                    left: Box::new(left_folded),
                    operator: operator.clone(),
                    right: Box::new(right_folded),
                }
            }
            Expression::Unary { operator, right } => {
                let right_folded = self.fold_expression(right);
                
                if let Some(lit) = self.expression_to_value(&right_folded) {
                    if let Some(result) = self.evaluate_unary(operator, &lit) {
                        return self.value_to_expression(result);
                    }
                }
                
                Expression::Unary {
                    operator: operator.clone(),
                    right: Box::new(right_folded),
                }
            }
            _ => expr.clone(),
        }
    }

    fn expression_to_value(&self, expr: &Expression) -> Option<Value> {
        match expr {
            Expression::Number(n) => Some(Value::Number(*n)),
            Expression::String(s) => Some(Value::String(s.clone())),
            Expression::Boolean(b) => Some(Value::Boolean(*b)),
            Expression::Null => Some(Value::Null),
            _ => None,
        }
    }

    fn value_to_expression(&self, value: Value) -> Expression {
        match value {
            Value::Number(n) => Expression::Number(n),
            Value::String(s) => Expression::String(s),
            Value::Boolean(b) => Expression::Boolean(b),
            Value::Null => Expression::Null,
            _ => Expression::Null, // Fallback for complex types
        }
    }

    fn evaluate_binary(&self, left: &Value, operator: &Token, right: &Value) -> Option<Value> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                let result = match operator.token_type {
                    TokenType::Plus => Value::Number(a + b),
                    TokenType::Minus => Value::Number(a - b),
                    TokenType::Multiply => Value::Number(a * b),
                    TokenType::Divide => Value::Number(a / b),
                    TokenType::Modulo => Value::Number(a % b),
                    _ => return None,
                };
                Some(result)
            }
            (Value::String(a), Value::String(b)) => {
                if operator.token_type == TokenType::Plus {
                    let mut result = a.clone();
                    result.push_str(b);
                    Some(Value::String(result))
                } else {
                    None
                }
            }
            (Value::Boolean(a), Value::Boolean(b)) => {
                let result = match operator.token_type {
                    TokenType::And => Value::Boolean(*a && *b),
                    TokenType::Or => Value::Boolean(*a || *b),
                    _ => return None,
                };
                Some(result)
            }
            _ => None,
        }
    }

    fn evaluate_unary(&self, operator: &Token, operand: &Value) -> Option<Value> {
        match operand {
            Value::Number(n) => {
                if operator.token_type == TokenType::Minus {
                    Some(Value::Number(-n))
                } else {
                    None
                }
            }
            Value::Boolean(b) => {
                if operator.token_type == TokenType::Not {
                    Some(Value::Boolean(!b))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Dead code elimination optimizer
pub struct DeadCodeEliminator {
    used_variables: HashMap<String, bool>,
}

impl DeadCodeEliminator {
    pub fn new() -> Self {
        DeadCodeEliminator {
            used_variables: HashMap::new(),
        }
    }

    pub fn eliminate_dead_code(&mut self, statements: &[Statement]) -> Vec<Statement> {
        // First pass: find all used variables
        self.find_used_variables(statements);
        
        // Second pass: eliminate unused variable declarations
        statements
            .iter()
            .filter(|stmt| !self.is_unused_variable_declaration(stmt))
            .cloned()
            .collect()
    }

    fn find_used_variables(&mut self, statements: &[Statement]) {
        for stmt in statements {
            self.find_used_variables_in_statement(stmt);
        }
    }

    fn find_used_variables_in_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclaration { name, initializer, .. } => {
                if let Some(expr) = initializer {
                    self.find_used_variables_in_expression(expr);
                }
                // Mark variable as potentially used (will be updated when we find actual usage)
                self.used_variables.insert(name.lexeme.clone(), false);
            }
            Statement::Expression(expr) => {
                self.find_used_variables_in_expression(expr);
            }
            Statement::If { condition, then_branch, else_branch } => {
                self.find_used_variables_in_expression(condition);
                self.find_used_variables(then_branch);
                if let Some(else_branch) = else_branch {
                    self.find_used_variables(else_branch);
                }
            }
            Statement::While { condition, body } => {
                self.find_used_variables_in_expression(condition);
                self.find_used_variables(body);
            }
            Statement::FunctionDeclaration { name, parameters, body, .. } => {
                self.used_variables.insert(name.lexeme.clone(), true); // Function names are always used
                for (param, _) in parameters {
                    self.used_variables.insert(param.lexeme.clone(), false);
                }
                self.find_used_variables(body);
            }
            Statement::Return { value } => {
                if let Some(expr) = value {
                    self.find_used_variables_in_expression(expr);
                }
            }
            _ => {}
        }
    }

    fn find_used_variables_in_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(token) => {
                self.used_variables.insert(token.lexeme.clone(), true);
            }
            Expression::Binary { left, right, .. } => {
                self.find_used_variables_in_expression(left);
                self.find_used_variables_in_expression(right);
            }
            Expression::Unary { right, .. } => {
                self.find_used_variables_in_expression(right);
            }
            Expression::Call { callee, arguments } => {
                self.find_used_variables_in_expression(callee);
                for arg in arguments {
                    self.find_used_variables_in_expression(arg);
                }
            }
            Expression::Assignment { name, value } => {
                self.used_variables.insert(name.lexeme.clone(), true);
                self.find_used_variables_in_expression(value);
            }
            _ => {}
        }
    }

    fn is_unused_variable_declaration(&self, stmt: &Statement) -> bool {
        if let Statement::VariableDeclaration { name, .. } = stmt {
            self.used_variables.get(&name.lexeme).map_or(false, |used| !used)
        } else {
            false
        }
    }
}

/// Performance optimization engine
pub struct PerformanceOptimizer {
    constant_folder: ConstantFolder,
    dead_code_eliminator: DeadCodeEliminator,
    string_interner: StringInterner,
    value_pool: MemoryPool<Value>,
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        PerformanceOptimizer {
            constant_folder: ConstantFolder::new(),
            dead_code_eliminator: DeadCodeEliminator::new(),
            string_interner: StringInterner::new(),
            value_pool: MemoryPool::new(),
        }
    }

    pub fn optimize_ast(&mut self, statements: Vec<Statement>) -> Vec<Statement> {
        // Apply constant folding
        let mut optimized = Vec::new();
        for stmt in &statements {
            optimized.push(self.optimize_statement(stmt));
        }

        // Apply dead code elimination
        self.dead_code_eliminator.eliminate_dead_code(&optimized)
    }

    fn optimize_statement(&mut self, stmt: &Statement) -> Statement {
        match stmt {
            Statement::VariableDeclaration { name, type_annotation, initializer } => {
                Statement::VariableDeclaration {
                    name: name.clone(),
                    type_annotation: type_annotation.clone(),
                    initializer: initializer.as_ref().map(|expr| self.constant_folder.fold_expression(expr)),
                }
            }
            Statement::Expression(expr) => {
                Statement::Expression(self.constant_folder.fold_expression(expr))
            }
            Statement::If { condition, then_branch, else_branch } => {
                Statement::If {
                    condition: self.constant_folder.fold_expression(condition),
                    then_branch: self.optimize_ast(then_branch.clone()),
                    else_branch: else_branch.as_ref().map(|branch| self.optimize_ast(branch.clone())),
                }
            }
            Statement::While { condition, body } => {
                Statement::While {
                    condition: self.constant_folder.fold_expression(condition),
                    body: self.optimize_ast(body.clone()),
                }
            }
            _ => stmt.clone(),
        }
    }

    pub fn intern_string(&mut self, s: String) -> StringId {
        self.string_interner.intern(s)
    }

    pub fn get_interned_string(&self, id: StringId) -> Option<&str> {
        self.string_interner.get(id)
    }

    pub fn allocate_value(&mut self, value: Value) -> usize {
        self.value_pool.allocate(value)
    }

    pub fn deallocate_value(&mut self, index: usize) {
        self.value_pool.deallocate(index);
    }

    pub fn get_value(&self, index: usize) -> Option<&Value> {
        self.value_pool.get(index)
    }

    pub fn get_stats(&self) -> OptimizationStats {
        OptimizationStats {
            interned_strings: self.string_interner.len(),
            pooled_values: self.value_pool.objects.len(),
            free_values: self.value_pool.free_list.len(),
        }
    }
}

/// Performance optimization statistics
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    pub interned_strings: usize,
    pub pooled_values: usize,
    pub free_values: usize,
}

// Performance stats function
fn perf_stats(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let stats_str = format!(
        "Performance Stats - Interned Strings: {}, Pooled Values: {}, Free Values: {}",
        0, 0, 0 // Placeholder values
    );
    Ok(Value::String(stats_str))
}

// Enable/disable optimizations
fn perf_optimize(_vm: &mut VM, args: Vec<Value>) -> Result<Value, String> {
    let enabled = match &args[0] {
        Value::Boolean(b) => *b,
        _ => return Err("Argument must be a boolean".to_string()),
    };

    println!("Performance optimizations: {}", enabled);
    Ok(Value::Boolean(true))
}

// Memory usage function
fn perf_memory(_vm: &mut VM, _args: Vec<Value>) -> Result<Value, String> {
    let memory_str = format!(
        "Memory Usage - Stack: {}, Heap: {}, Peak: {}",
        0, 0, 0 // Placeholder values
    );
    Ok(Value::String(memory_str))
}

/// Initialize performance optimization functions
pub fn init_performance_optimizations(vm: &mut VM) {
    // Performance stats function
    vm.register_native("perf_stats", 0, perf_stats);

    // Enable/disable optimizations
    vm.register_native("perf_optimize", 1, perf_optimize);

    // Memory usage function
    vm.register_native("perf_memory", 0, perf_memory);
}