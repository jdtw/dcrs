mod op;
mod registers;
mod val;

use crate::error::Error;
use registers::Registers;
use val::Val;

pub struct Stack {
    stack: Vec<Val>,
    output: op::Radix,
    reg: Registers,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::new(),
            output: op::Radix::Dec,
            reg: Registers::new(),
        }
    }

    pub fn op(&mut self, op: op::Op) -> Result<bool, Error> {
        use op::Op::*;
        match op {
            Stack(op) => self.stack_op(op)?,
            Reg(op) => self.reg_op(op)?,
            Print(op) => self.print_op(op)?,
            Calc(op) => self.calc_op(op)?,
            Cast(op) => self.cast_op(op)?,
            Quit => return Ok(true),
        }
        Ok(false)
    }

    pub fn top(&self) -> Option<&Val> {
        self.stack.last()
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
            Add => self.map2_push(|v1, v2| Ok(v2 + v1)),
            Sub => self.map2_push(|v1, v2| Ok(v2 - v1)),
            Mul => self.map2_push(|v1, v2| Ok(v2 * v1)),
            Div => self.map2_push(|v1, v2| {
                if !v1.is_zero() {
                    Ok(v2 / v1)
                } else {
                    Err(Error::DivideByZero)
                }
            }),
            Mod => self.map2_push(|v1, v2| {
                if !v1.is_zero() {
                    Ok(v2 % v1)
                } else {
                    Err(Error::DivideByZero)
                }
            }),
            Pow => self.map2_push(|v1, v2| Ok(v2.pow(v1))),
            And => self.map2_push(|v1, v2| Ok(v2 & v1)),
            Or => self.map2_push(|v1, v2| Ok(v2 | v1)),
            Xor => self.map2_push(|v1, v2| Ok(v2 ^ v1)),
            Not => self.map_push(|v| Ok(!v)),
            Shl => self.map2_push(|v1, v2| Ok(v2 << v1)),
            Shr => self.map2_push(|v1, v2| Ok(v2 >> v1)),
        }
    }

    fn print_op(&mut self, op: op::PrintOp) -> Result<(), Error> {
        use op::PrintOp::*;
        match op {
            Print => {
                if let Some(last) = self.top() {
                    self.print(last);
                }
            }
            Dump => {
                for v in self.stack.iter() {
                    self.print(v);
                }
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

    fn reg_op(&mut self, op: op::RegOp) -> Result<(), Error> {
        use op::RegOp::*;
        let id = self.pop()?;
        match op {
            Push => {
                let val = self.pop()?;
                self.reg.push(id, val);
            }
            Get => {
                let val = self.reg.val(&id)?;
                self.stack.push(val);
            }
            Pop => {
                let val = self.reg.pop(&id)?;
                self.stack.push(val);
            }
            Dump => {
                if let Some(it) = self.reg.iter(&id) {
                    for v in it {
                        self.print(v);
                    }
                }
            }
        }
        Ok(())
    }

    fn cast_op(&mut self, op: op::CastOp) -> Result<(), Error> {
        use op::CastOp::*;
        use Val::*;
        match op {
            U => self.map_push(|v| Ok(U64(v.into()))),
            I => self.map_push(|v| Ok(I64(v.into()))),
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

    fn map_push(&mut self, f: impl Fn(Val) -> Result<Val, Error>) -> Result<(), Error> {
        let val = self.pop().and_then(f)?;
        self.stack.push(val);
        Ok(())
    }

    fn map2_push(&mut self, f: impl Fn(Val, Val) -> Result<Val, Error>) -> Result<(), Error> {
        let val = self.pop2().and_then(|(v1, v2)| f(v1, v2))?;
        self.stack.push(val);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use op::CalcOp::{self, *};
    use op::StackOp::*;
    use Val::*;

    fn calc(op: CalcOp, v1: Val, v2: Val) -> Val {
        let mut s = Stack::new();
        s.stack_op(Push(v1)).unwrap();
        s.stack_op(Push(v2)).unwrap();
        s.calc_op(op).unwrap();
        s.top().cloned().unwrap()
    }

    #[test]
    fn test_addition() {
        assert_eq!(calc(Add, U64(0), U64(0)), U64(0));
        assert_eq!(calc(Add, U64(1), U64(1)), U64(2));
        assert_eq!(calc(Add, U64(1), U64(1)), U64(2));
        assert_eq!(calc(Add, U64(1000), U64(1234)), U64(2234));
        assert_eq!(calc(Add, U64(1), I64(-1)), I64(0));
        assert_eq!(calc(Add, I64(1), I64(-1)), I64(0));
        assert_eq!(calc(Add, U64(1), I64(100)), I64(101));
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(calc(Sub, U64(0), U64(0)), U64(0));
        assert_eq!(
            calc(Sub, U64(u64::max_value()), U64(u64::max_value())),
            U64(0)
        );
        assert_eq!(
            calc(Sub, I64(i64::max_value()), I64(i64::max_value())),
            I64(0)
        );
        assert_eq!(calc(Sub, U64(0), I64(100)), I64(-100));
        assert_eq!(calc(Sub, I64(0), U64(100)), I64(-100));
        assert_eq!(calc(Sub, I64(100), I64(50)), I64(50));
        assert_eq!(calc(Sub, U64(100), U64(50)), U64(50));
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(calc(Mul, U64(0), U64(0)), U64(0));
        assert_eq!(calc(Mul, U64(1), U64(100)), U64(100));
        assert_eq!(calc(Mul, U64(2), U64(100)), U64(200));
        assert_eq!(calc(Mul, I64(2), U64(100)), I64(200));
        assert_eq!(calc(Mul, U64(2), I64(100)), I64(200));
        assert_eq!(calc(Mul, U64(2), I64(-100)), I64(-200));
    }

    #[test]
    fn test_division() {
        assert_eq!(calc(Div, U64(0), U64(10)), U64(0));
        assert_eq!(
            calc(Div, U64(u64::max_value()), U64(u64::max_value())),
            U64(1)
        );
        assert_eq!(
            calc(Div, I64(i64::max_value()), I64(i64::max_value())),
            I64(1)
        );
        assert_eq!(calc(Div, U64(4), U64(2)), U64(2));
        assert_eq!(calc(Div, U64(5), I64(2)), I64(2));
        assert_eq!(calc(Div, I64(2), I64(1)), I64(2));
        assert_eq!(calc(Div, I64(3), I64(2)), I64(1));
        assert_eq!(calc(Div, I64(-3), I64(2)), I64(-1));
    }

    #[test]
    fn test_mod() {
        assert_eq!(calc(Mod, U64(12345), U64(2)), U64(1));
        assert_eq!(calc(Mod, U64(12346), U64(2)), U64(0));
        assert_eq!(calc(Mod, I64(12345), U64(2)), I64(1));
        assert_eq!(calc(Mod, U64(12346), I64(2)), I64(0));
        assert_eq!(
            calc(Mod, U64(u64::max_value()), U64(u64::max_value())),
            U64(0)
        );
        assert_eq!(
            calc(Mod, I64(i64::max_value()), I64(i64::max_value())),
            I64(0)
        );
        assert_eq!(
            calc(Mod, I64(i64::min_value()), I64(i64::min_value())),
            I64(0)
        );
    }

    #[test]
    fn test_pow() {
        assert_eq!(calc(Pow, U64(1), U64(0)), U64(1));
        assert_eq!(calc(Pow, I64(1), I64(0)), I64(1));
        assert_eq!(calc(Pow, I64(1), I64(i64::max_value())), I64(1));
        assert_eq!(calc(Pow, I64(1), U64(u64::max_value())), I64(1));
        assert_eq!(calc(Pow, U64(2), U64(10)), U64(1024));
        assert_eq!(calc(Pow, I64(2), U64(10)), I64(1024));
        assert_eq!(calc(Pow, I64(2), I64(10)), I64(1024));
        assert_eq!(calc(Pow, U64(2), U64(32)), U64(u32::max_value() as u64 + 1));
    }

    #[test]
    fn test_and() {
        assert_eq!(calc(And, U64(0b0), U64(0b0)), U64(0b0));
        assert_eq!(calc(And, U64(0b1), U64(0b0)), U64(0b0));
        assert_eq!(calc(And, U64(0b0), U64(0b1)), U64(0b0));
        assert_eq!(calc(And, U64(0b1), U64(0b1)), U64(0b1));
        assert_eq!(calc(And, U64(0b11110000), U64(0b00001111)), U64(0b0));
        assert_eq!(calc(And, U64(0b11110000), U64(0b11110000)), U64(0b11110000));
        assert_eq!(calc(And, U64(0b11110000), U64(0b11111111)), U64(0b11110000));
        assert_eq!(calc(And, U64(0b11110000), U64(0b00000000)), U64(0b00000000));
    }

    #[test]
    fn test_or() {
        assert_eq!(calc(Or, U64(0b0), U64(0b0)), U64(0b0));
        assert_eq!(calc(Or, U64(0b1), U64(0b0)), U64(0b1));
        assert_eq!(calc(Or, U64(0b0), U64(0b1)), U64(0b1));
        assert_eq!(calc(Or, U64(0b1), U64(0b1)), U64(0b1));
        assert_eq!(calc(Or, U64(0b11110000), U64(0b00001111)), U64(0b11111111));
        assert_eq!(calc(Or, U64(0b11110000), U64(0b11110000)), U64(0b11110000));
        assert_eq!(calc(Or, U64(0b11110000), U64(0b11111111)), U64(0b11111111));
        assert_eq!(calc(Or, U64(0b11110000), U64(0b00000000)), U64(0b11110000));
    }

    #[test]
    fn test_xor() {
        assert_eq!(calc(Xor, U64(0b0), U64(0b0)), U64(0b0));
        assert_eq!(calc(Xor, U64(0b1), U64(0b0)), U64(0b1));
        assert_eq!(calc(Xor, U64(0b0), U64(0b1)), U64(0b1));
        assert_eq!(calc(Xor, U64(0b1), U64(0b1)), U64(0b0));
        assert_eq!(calc(Xor, U64(0b11110000), U64(0b00001111)), U64(0b11111111));
        assert_eq!(calc(Xor, U64(0b11110000), U64(0b11110000)), U64(0b00000000));
        assert_eq!(calc(Xor, U64(0b11110000), U64(0b11111111)), U64(0b00001111));
        assert_eq!(calc(Xor, U64(0b11110000), U64(0b00000000)), U64(0b11110000));
    }

    #[test]
    fn test_shl() {
        assert_eq!(calc(Shl, U64(1), U64(1)), U64(1 << 1));
        assert_eq!(calc(Shl, U64(1), U64(10)), U64(1 << 10));
        assert_eq!(calc(Shl, U64(100), U64(1)), U64(100 << 1));
        assert_eq!(calc(Shl, U64(12), U64(12)), U64(12 << 12));
    }

    #[test]
    fn test_shr() {
        assert_eq!(calc(Shr, U64(10 << 10), U64(10)), U64(10));
        assert_eq!(calc(Shr, U64(1 << 1), U64(1)), U64(1));
        assert_eq!(calc(Shr, U64(100 << 3), U64(3)), U64(100));
        assert_eq!(calc(Shr, U64(1), U64(0)), U64(1));
    }

    #[test]
    fn test_not() {
        let not = |v| {
            let mut s = Stack::new();
            s.stack_op(Push(v)).unwrap();
            s.calc_op(Not).unwrap();
            s.top().cloned().unwrap()
        };
        let mask = u64::max_value() << 8;
        assert_eq!(not(U64(0b00001111 | mask)), U64(0b11110000));
        assert_eq!(not(U64(0b11110000 | mask)), U64(0b00001111));
        assert_eq!(not(U64(0b11111111)), U64(0b00000000 | mask));
        assert_eq!(not(U64(0b00000000)), U64(0b11111111 | mask));
    }

    #[test]
    fn test_cast() {
        use op::CastOp::*;
        let mut s = Stack::new();
        s.stack_op(Push(U64(u64::max_value()))).unwrap();
        s.cast_op(I).unwrap();
        assert_eq!(s.top(), Some(&I64(-1)));
        s.cast_op(U).unwrap();
        assert_eq!(s.top(), Some(&U64(u64::max_value())));
    }

    #[test]
    fn test_calc() {
        let mut s = Stack::new();
        for op in "100 0x10 r r 0b1 + - 13 1 d + r s 13 l ^ 13 L / p f i d n".split_whitespace() {
            s.op(op.parse().unwrap()).unwrap();
        }
        assert_eq!(s.top(), Some(&I64(((100u64 - (16 + 1)).pow(2) / 2) as i64)));
    }
}
