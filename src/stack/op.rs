use crate::error::Error;
use crate::stack::val::Val;
use std::convert::From;
use std::str::FromStr;

#[derive(Debug)]
pub enum Op {
    Stack(StackOp),
    Reg(RegOp),
    Print(PrintOp),
    Calc(CalcOp),
    Cast(CastOp),
    Quit,
}

impl FromStr for Op {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            // Calculator operations
            "+" => Op::Calc(CalcOp::Add),
            "-" => Op::Calc(CalcOp::Sub),
            "*" => Op::Calc(CalcOp::Mul),
            "/" => Op::Calc(CalcOp::Div),
            "%" => Op::Calc(CalcOp::Mod),
            "^" => Op::Calc(CalcOp::Pow),
            "&" => Op::Calc(CalcOp::And),
            "|" => Op::Calc(CalcOp::Or),
            "!" => Op::Calc(CalcOp::Not),
            "x" => Op::Calc(CalcOp::Xor),
            "<" => Op::Calc(CalcOp::Shl),
            ">" => Op::Calc(CalcOp::Shr),

            // Printing operations
            "p" => Op::Print(PrintOp::Print),
            "f" => Op::Print(PrintOp::Dump),
            "o" => Op::Print(PrintOp::Output),

            // Stack operations
            "n" => Op::Stack(StackOp::Pop),
            "d" => Op::Stack(StackOp::Dup),
            "c" => Op::Stack(StackOp::Clear),
            "r" => Op::Stack(StackOp::Rev),

            // Register operations
            "s" => Op::Reg(RegOp::Push),
            "l" => Op::Reg(RegOp::Get),
            "L" => Op::Reg(RegOp::Pop),
            "F" => Op::Reg(RegOp::Dump),

            // Casting operations
            "u" => Op::Cast(CastOp::U),
            "i" => Op::Cast(CastOp::I),

            "q" => Op::Quit,

            // If none of the above, parse this as a Val and push to the stack.
            _ => Op::Stack(StackOp::Push(s.parse()?)),
        })
    }
}

#[derive(Debug)]
pub enum CalcOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Xor,
    Not,
    Shl,
    Shr,
}

#[derive(Debug)]
pub enum RegOp {
    Push,
    Get,
    Pop,
    Dump,
}

#[derive(Debug)]
pub enum StackOp {
    Push(Val),
    Pop,
    Dup,
    Clear,
    Rev,
}

#[derive(Debug)]
pub enum PrintOp {
    Dump,
    Print,
    Output,
}

#[derive(Debug)]
pub enum CastOp {
    U,
    I,
}

#[derive(Debug)]
pub enum Radix {
    Bin,
    Dec,
    Hex,
}

impl From<Val> for Option<Radix> {
    fn from(val: Val) -> Option<Radix> {
        match u32::from(val) {
            2 => Some(Radix::Bin),
            10 => Some(Radix::Dec),
            16 => Some(Radix::Hex),
            _ => None,
        }
    }
}
