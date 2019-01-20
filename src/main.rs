mod error;
mod val;

use crate::error::Error;
use crate::val::Val;
use std::convert::From;
use std::io;
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let mut stack = Stack::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        for word in line.split_whitespace() {
            match word.parse().and_then(|op| stack.op(op)) {
                Ok(quit) => {
                    if quit {
                        return Ok(());
                    }
                }
                Err(error) => eprintln!("{}", error),
            }
        }
    }
}

#[derive(Debug)]
enum CastOp {
    U,
    I,
    F,
}

#[derive(Debug)]
enum CalcOp {
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
enum StackOp {
    Push(Val),
    Pop,
    Dup,
    Clear,
    Rev,
}

#[derive(Debug)]
enum PrintOp {
    Dump,
    Print,
    Output,
}

#[derive(Debug)]
enum Op {
    Stack(StackOp),
    Print(PrintOp),
    Calc(CalcOp),
    Cast(CastOp),
    Quit,
}

#[derive(Debug)]
enum Radix {
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

struct Stack {
    stack: Vec<Val>,
    output: Radix,
}

impl Stack {
    fn new() -> Self {
        Stack {
            stack: Vec::new(),
            output: Radix::Dec,
        }
    }

    fn op(&mut self, op: Op) -> Result<bool, Error> {
        use Op::*;
        match op {
            Stack(op) => self.stack_op(op)?,
            Print(op) => self.print_op(op)?,
            Calc(op) => self.calc_op(op)?,
            Cast(op) => self.cast_op(op)?,
            Quit => return Ok(true),
        }
        Ok(false)
    }

    fn print(&self, val: &Val) {
        match self.output {
            Radix::Bin => println!("0b{:0>64b}", val),
            Radix::Dec => println!("{}", val),
            Radix::Hex => println!("0x{:0>16x}", val),
        }
    }

    fn calc_op(&mut self, op: CalcOp) -> Result<(), Error> {
        use CalcOp::*;
        match op {
            Add => self.map2_push(|v1, v2| v2 + v1),
            Sub => self.map2_push(|v1, v2| v2 - v1),
            Mul => self.map2_push(|v1, v2| v2 * v1),
            Div => self.map2_push(|v1, v2| v2 / v1),
            Mod => self.map2_push(|v1, v2| v2 % v1),
            Pow => self.map2_push(|v1, v2| v2.pow(v1)),
            And => self.map2_push(|v1, v2| v2 & v1),
            Or => self.map2_push(|v1, v2| v2 | v1),
            Xor => self.map2_push(|v1, v2| v2 ^ v1),
            Not => self.map_push(|v| !v),
            Shl => self.map2_push(|v1, v2| v2 << v1),
            Shr => self.map2_push(|v1, v2| v2 >> v1),
        }
    }

    fn print_op(&mut self, op: PrintOp) -> Result<(), Error> {
        use PrintOp::*;
        match op {
            Print => {
                if self.stack.len() > 0 {
                    self.print(&self.stack[self.stack.len() - 1]);
                }
            }
            Dump => {
                for val in self.stack.iter().rev() {
                    self.print(val);
                }
            }
            Output => {
                let radix: Option<Radix> = self.pop()?.into();
                self.output = radix.ok_or(Error::BadRadix)?;
            }
        }
        Ok(())
    }

    fn stack_op(&mut self, op: StackOp) -> Result<(), Error> {
        use StackOp::*;
        match op {
            Push(val) => self.push(val),
            Pop => {
                if let Some(val) = self.stack.pop() {
                    self.print(&val);
                }
            }
            Dup => {
                if let Some(val) = self.stack.pop() {
                    self.stack.push(val.clone());
                    self.stack.push(val);
                }
            }
            Clear => self.stack.clear(),
            Rev => {
                let len = self.stack.len();
                if len < 2 {
                    self.stack.swap(len - 1, len - 2);
                }
            }
        }
        Ok(())
    }

    fn cast_op(&mut self, op: CastOp) -> Result<(), Error> {
        use CastOp::*;
        use Val::*;
        match op {
            U => self.map_push(|v| U64(v.into())),
            I => self.map_push(|v| I64(v.into())),
            F => self.map_push(|v| F64(v.into())),
        }
    }

    fn pop(&mut self) -> Result<Val, Error> {
        if self.stack.len() < 1 {
            Err(Error::EndOfStack)
        } else {
            Ok(self.stack.pop().unwrap())
        }
    }

    fn pop2(&mut self) -> Result<(Val, Val), Error> {
        if self.stack.len() < 2 {
            Err(Error::EndOfStack)
        } else {
            Ok((self.stack.pop().unwrap(), self.stack.pop().unwrap()))
        }
    }

    fn push(&mut self, val: Val) {
        self.stack.push(val)
    }

    fn map_push(&mut self, f: impl Fn(Val) -> Val) -> Result<(), Error> {
        let val = self.pop().map(|v| f(v))?;
        Ok(self.push(val))
    }

    fn map2_push(&mut self, f: impl Fn(Val, Val) -> Val) -> Result<(), Error> {
        let val = self.pop2().map(|(v1, v2)| f(v1, v2))?;
        Ok(self.push(val))
    }
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
            "l" => Op::Print(PrintOp::Dump),
            "o" => Op::Print(PrintOp::Output),

            // Stack operations
            "n" => Op::Stack(StackOp::Pop),
            "d" => Op::Stack(StackOp::Dup),
            "c" => Op::Stack(StackOp::Clear),
            "r" => Op::Stack(StackOp::Rev),

            // Casting operations
            "u" => Op::Cast(CastOp::U),
            "i" => Op::Cast(CastOp::I),
            "f" => Op::Cast(CastOp::F),

            "q" => Op::Quit,

            // If none of the above, parse this as a Val and push to the stack.
            _ => Op::Stack(StackOp::Push(s.parse()?)),
        })
    }
}
