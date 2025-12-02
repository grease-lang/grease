// Copyright 2025 Nicholas Girga <nickgirga@gmail.com>
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, Clone)]
pub enum OpCode {
    // Constants
    Constant,
    Null,
    True,
    False,
    
    // Variables
    GetGlobal,
    SetGlobal,
    GetLocal,
    SetLocal,
    
    // Control flow
    Jump,
    JumpIfFalse,
    JumpIfTrue,
    Loop,
    
    // Functions
    Call,
    Return,
    
    // Operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Negate,
    Array,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    // Logical
    Not,
    And,
    Or,
    
    // Stack
    Pop,

    // Modules
    Import,
    GetModule,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Function(Function),
    NativeFunction(NativeFunction),
    Array(Vec<Value>),
}

#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub function: fn(&mut crate::vm::VM, Vec<Value>) -> Result<Value, String>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==", name);
        
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:4} ", self.lines[offset]);
        }
        
        let instruction = self.code[offset];
        match OpCode::from_byte(instruction) {
            Some(op) => match op {
                OpCode::Constant => self.constant_instruction("CONSTANT", offset),
                OpCode::Null => self.simple_instruction("NULL", offset),
                OpCode::True => self.simple_instruction("TRUE", offset),
                OpCode::False => self.simple_instruction("FALSE", offset),
                OpCode::GetGlobal => self.constant_instruction("GET_GLOBAL", offset),
                OpCode::SetGlobal => self.constant_instruction("SET_GLOBAL", offset),
                OpCode::GetLocal => self.byte_instruction("GET_LOCAL", offset),
                OpCode::SetLocal => self.byte_instruction("SET_LOCAL", offset),
                OpCode::Jump => self.jump_instruction("JUMP", 1, offset),
                OpCode::JumpIfFalse => self.jump_instruction("JUMP_IF_FALSE", 1, offset),
                OpCode::JumpIfTrue => self.jump_instruction("JUMP_IF_TRUE", 1, offset),
                OpCode::Loop => self.jump_instruction("LOOP", -1, offset),
                OpCode::Call => self.byte_instruction("CALL", offset),
                OpCode::Return => self.simple_instruction("RETURN", offset),
                OpCode::Add => self.simple_instruction("ADD", offset),
                OpCode::Subtract => self.simple_instruction("SUBTRACT", offset),
                OpCode::Multiply => self.simple_instruction("MULTIPLY", offset),
                OpCode::Divide => self.simple_instruction("DIVIDE", offset),
                OpCode::Modulo => self.simple_instruction("MODULO", offset),
                OpCode::Negate => self.simple_instruction("NEGATE", offset),
                OpCode::Array => self.byte_instruction("ARRAY", offset),
                OpCode::Equal => self.simple_instruction("EQUAL", offset),
                OpCode::NotEqual => self.simple_instruction("NOT_EQUAL", offset),
                OpCode::Less => self.simple_instruction("LESS", offset),
                OpCode::LessEqual => self.simple_instruction("LESS_EQUAL", offset),
                OpCode::Greater => self.simple_instruction("GREATER", offset),
                OpCode::GreaterEqual => self.simple_instruction("GREATER_EQUAL", offset),
                OpCode::Not => self.simple_instruction("NOT", offset),
                OpCode::And => self.simple_instruction("AND", offset),
                OpCode::Or => self.simple_instruction("OR", offset),
                OpCode::Pop => self.simple_instruction("POP", offset),
                OpCode::Import => self.constant_instruction("IMPORT", offset),
                OpCode::GetModule => self.constant_instruction("GET_MODULE", offset),
            },
            None => {
                println!("Unknown opcode {}", instruction);
                offset + 1
            }
        }
    }

    fn simple_instruction(&self, name: &str, offset: usize) -> usize {
        println!("{}", name);
        offset + 1
    }

    fn constant_instruction(&self, name: &str, offset: usize) -> usize {
        let constant = self.code[offset + 1];
        print!("{:16} {:4} '", name, constant);
        println!("{:?}'", self.constants[constant as usize]);
        offset + 2
    }

    fn byte_instruction(&self, name: &str, offset: usize) -> usize {
        let slot = self.code[offset + 1];
        println!("{:16} {:4}", name, slot);
        offset + 2
    }

    fn jump_instruction(&self, name: &str, sign: i16, offset: usize) -> usize {
        let jump = ((self.code[offset + 1] as u16) << 8) | (self.code[offset + 2] as u16);
        println!("{:16} {:4} -> {}", name, offset, (offset as i16) + sign * (jump as i16));
        offset + 3
    }
}

impl OpCode {
    pub fn to_byte(self) -> u8 {
        match self {
            OpCode::Constant => 0,
            OpCode::Null => 1,
            OpCode::True => 2,
            OpCode::False => 3,
            OpCode::GetGlobal => 4,
            OpCode::SetGlobal => 5,
            OpCode::GetLocal => 6,
            OpCode::SetLocal => 7,
            OpCode::Jump => 8,
            OpCode::JumpIfFalse => 9,
            OpCode::JumpIfTrue => 10,
            OpCode::Loop => 11,
            OpCode::Call => 12,
            OpCode::Return => 13,
            OpCode::Add => 14,
            OpCode::Subtract => 15,
            OpCode::Multiply => 16,
            OpCode::Divide => 17,
            OpCode::Modulo => 18,
            OpCode::Negate => 19,
            OpCode::Array => 20,
            OpCode::Equal => 21,
            OpCode::NotEqual => 22,
            OpCode::Less => 23,
            OpCode::LessEqual => 24,
            OpCode::Greater => 25,
            OpCode::GreaterEqual => 26,
            OpCode::Not => 27,
            OpCode::And => 28,
            OpCode::Or => 29,
            OpCode::Pop => 30,
            OpCode::Import => 31,
            OpCode::GetModule => 32,
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(OpCode::Constant),
            1 => Some(OpCode::Null),
            2 => Some(OpCode::True),
            3 => Some(OpCode::False),
            4 => Some(OpCode::GetGlobal),
            5 => Some(OpCode::SetGlobal),
            6 => Some(OpCode::GetLocal),
            7 => Some(OpCode::SetLocal),
            8 => Some(OpCode::Jump),
            9 => Some(OpCode::JumpIfFalse),
            10 => Some(OpCode::JumpIfTrue),
            11 => Some(OpCode::Loop),
            12 => Some(OpCode::Call),
            13 => Some(OpCode::Return),
            14 => Some(OpCode::Add),
            15 => Some(OpCode::Subtract),
            16 => Some(OpCode::Multiply),
            17 => Some(OpCode::Divide),
            18 => Some(OpCode::Modulo),
            19 => Some(OpCode::Negate),
            20 => Some(OpCode::Array),
            21 => Some(OpCode::Equal),
            22 => Some(OpCode::NotEqual),
            23 => Some(OpCode::Less),
            24 => Some(OpCode::LessEqual),
            25 => Some(OpCode::Greater),
            26 => Some(OpCode::GreaterEqual),
            27 => Some(OpCode::Not),
            28 => Some(OpCode::And),
            29 => Some(OpCode::Or),
            30 => Some(OpCode::Pop),
            31 => Some(OpCode::Import),
            32 => Some(OpCode::GetModule),
            _ => None,
        }
    }
}