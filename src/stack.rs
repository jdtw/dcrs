mod op;
mod val;

use crate::error::Error;
use val::Val;

pub struct Stack {
    stack: Vec<Val>,
    output: op::Radix,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            output: op::Radix::Dec,
        }
    }

    pub fn op(&mut self, op: op::Op) -> Result<bool, Error> {
        use op::Op::*;
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
        use op::Radix::*;
        match self.output {
            Bin => println!("{:b}", val),
            Dec => println!("{}", val),
            Hex => println!("{:x}", val),
        }
    }

    fn calc_op(&mut self, op: op::CalcOp) -> Result<(), Error> {
        use op::CalcOp::*;
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

    fn print_op(&mut self, op: op::PrintOp) -> Result<(), Error> {
        use op::PrintOp::*;
        match op {
            Print => {
                if !self.stack.is_empty() {
                    self.print(&self.stack[self.stack.len() - 1]);
                }
            }
            Dump(grow) => {
                match grow {
                    op::Grow::Down => {
                        for v in self.stack.iter() {
                            self.print(v)
                        }
                    }
                    op::Grow::Up => {
                        for v in self.stack.iter().rev() {
                            self.print(v)
                        }
                    }
                };
            }
            Output => {
                let radix: Option<op::Radix> = self.pop()?.into();
                self.output = radix.ok_or(Error::BadRadix)?;
            }
        }
        Ok(())
    }

    fn stack_op(&mut self, op: op::StackOp) -> Result<(), Error> {
        use op::StackOp::*;
        match op {
            Push(val) => self.stack.push(val),
            Pop => {
                let val = self.pop()?;
                self.print(&val);
            }
            Dup => {
                let val = self.pop()?;
                self.stack.push(val.clone());
                self.stack.push(val);
            }
            Clear => self.stack.clear(),
            Rev => {
                let (v1, v2) = self.pop2()?;
                self.stack.push(v1);
                self.stack.push(v2);
            }
        }
        Ok(())
    }

    fn cast_op(&mut self, op: op::CastOp) -> Result<(), Error> {
        use op::CastOp::*;
        use Val::*;
        match op {
            U => self.map_push(|v| U64(v.into())),
            I => self.map_push(|v| I64(v.into())),
        }
    }

    fn pop(&mut self) -> Result<Val, Error> {
        self.stack.pop().ok_or(Error::EndOfStack)
    }

    fn pop2(&mut self) -> Result<(Val, Val), Error> {
        if self.stack.len() < 2 {
            Err(Error::EndOfStack)
        } else {
            Ok((self.stack.pop().unwrap(), self.stack.pop().unwrap()))
        }
    }

    fn map_push(&mut self, f: impl Fn(Val) -> Val) -> Result<(), Error> {
        let val = self.pop().map(f)?;
        self.stack.push(val);
        Ok(())
    }

    fn map2_push(&mut self, f: impl Fn(Val, Val) -> Val) -> Result<(), Error> {
        let val = self.pop2().map(|(v1, v2)| f(v1, v2))?;
        self.stack.push(val);
        Ok(())
    }
}
