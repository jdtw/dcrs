use crate::error::Error;
use std::convert::From;
use std::fmt;
use std::ops;
use std::str::FromStr;
use termion::style;
use Val::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Val {
    U64(u64),
    I64(i64),
}

impl Val {
    pub fn is_zero(&self) -> bool {
        match self {
            Val::U64(u) => *u == 0u64,
            Val::I64(i) => *i == 0i64,
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => write!(f, "{}{: >20}{}u64", style::Bold, u, style::Reset),
            I64(i) => write!(f, "{}{: >20}{}i64", style::Bold, i, style::Reset),
        }
    }
}

impl fmt::LowerHex for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => f.pad(&format!("0x{}{:0>16x}{}u64", style::Bold, u, style::Reset)),
            I64(i) => f.pad(&format!("0x{}{:0>16x}{}i64", style::Bold, i, style::Reset)),
        }
    }
}

impl fmt::Binary for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => f.pad(&format!("0b{}{:0>64b}{}u64", style::Bold, u, style::Reset)),
            I64(i) => f.pad(&format!("0b{}{:0>64b}{}i64", style::Bold, i, style::Reset)),
        }
    }
}

impl Val {
    pub fn pow(self, other: Val) -> Val {
        match self {
            U64(u) => U64(u.pow(other.into())),
            I64(i) => I64(i.pow(other.into())),
        }
    }
}

impl From<Val> for i32 {
    fn from(v: Val) -> i32 {
        match v {
            U64(u) => u as i32,
            I64(i) => i as i32,
        }
    }
}

impl From<Val> for u32 {
    fn from(v: Val) -> u32 {
        match v {
            U64(u) => u as u32,
            I64(i) => i as u32,
        }
    }
}

impl From<Val> for u64 {
    fn from(v: Val) -> u64 {
        match v {
            U64(u) => u,
            I64(i) => i as u64,
        }
    }
}

impl From<Val> for i64 {
    fn from(v: Val) -> i64 {
        match v {
            U64(u) => u as i64,
            I64(i) => i,
        }
    }
}

impl ops::Add<Val> for u64 {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self + u),
            I64(i) => I64(self as i64 + i),
        }
    }
}

impl ops::Add<Val> for i64 {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self + u as i64),
            I64(i) => I64(self + i),
        }
    }
}

impl ops::Add for Val {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match self {
            U64(u) => u + rhs,
            I64(i) => i + rhs,
        }
    }
}

impl ops::Sub<Val> for u64 {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self - u),
            I64(i) => I64(self as i64 - i),
        }
    }
}

impl ops::Sub<Val> for i64 {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self - u as i64),
            I64(i) => I64(self - i),
        }
    }
}

impl ops::Sub for Val {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match self {
            U64(u) => u - rhs,
            I64(i) => i - rhs,
        }
    }
}

impl ops::Mul<Val> for u64 {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self * u),
            I64(i) => I64(self as i64 * i),
        }
    }
}

impl ops::Mul<Val> for i64 {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self * u as i64),
            I64(i) => I64(self * i),
        }
    }
}

impl ops::Mul for Val {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match self {
            U64(u) => u * rhs,
            I64(i) => i * rhs,
        }
    }
}

impl ops::Div<Val> for u64 {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self / u),
            I64(i) => I64(self as i64 / i),
        }
    }
}

impl ops::Div<Val> for i64 {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self / u as i64),
            I64(i) => I64(self / i),
        }
    }
}

impl ops::Div for Val {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match self {
            U64(u) => u / rhs,
            I64(i) => i / rhs,
        }
    }
}

impl ops::Rem<Val> for u64 {
    type Output = Val;
    fn rem(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self % u),
            I64(i) => I64(self as i64 % i),
        }
    }
}

impl ops::Rem<Val> for i64 {
    type Output = Val;
    fn rem(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self % u as i64),
            I64(i) => I64(self % i),
        }
    }
}

impl ops::Rem for Val {
    type Output = Val;
    fn rem(self, rhs: Val) -> Val {
        match self {
            U64(u) => u % rhs,
            I64(i) => i % rhs,
        }
    }
}

impl ops::BitAnd<Val> for u64 {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self & u),
            I64(i) => I64(self as i64 & i),
        }
    }
}

impl ops::BitAnd<Val> for i64 {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self & u as i64),
            I64(i) => I64(self & i),
        }
    }
}

impl ops::BitAnd for Val {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match self {
            U64(u) => u & rhs,
            I64(i) => i & rhs,
        }
    }
}

impl ops::BitOr<Val> for u64 {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self | u),
            I64(i) => I64(self as i64 | i),
        }
    }
}

impl ops::BitOr<Val> for i64 {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self | u as i64),
            I64(i) => I64(self | i),
        }
    }
}

impl ops::BitOr for Val {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match self {
            U64(u) => u | rhs,
            I64(i) => i | rhs,
        }
    }
}

impl ops::BitXor<Val> for u64 {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self ^ u),
            I64(i) => I64(self as i64 ^ i),
        }
    }
}

impl ops::BitXor<Val> for i64 {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self ^ u as i64),
            I64(i) => I64(self ^ i),
        }
    }
}

impl ops::BitXor for Val {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match self {
            U64(u) => u ^ rhs,
            I64(i) => i ^ rhs,
        }
    }
}

impl ops::Not for Val {
    type Output = Val;
    fn not(self) -> Val {
        match self {
            U64(u) => U64(!u),
            I64(i) => I64(!i),
        }
    }
}

impl ops::Shl<Val> for u64 {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self << u),
            I64(i) => I64((self as i64) << i),
        }
    }
}

impl ops::Shl<Val> for i64 {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self << u as i64),
            I64(i) => I64(self << i),
        }
    }
}
impl ops::Shl for Val {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match self {
            U64(u) => u << rhs,
            I64(i) => i << rhs,
        }
    }
}

impl ops::Shr<Val> for u64 {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self >> u),
            I64(i) => I64((self as i64) >> i),
        }
    }
}

impl ops::Shr<Val> for i64 {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self >> u as i64),
            I64(i) => I64(self >> i),
        }
    }
}

impl ops::Shr for Val {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match self {
            U64(u) => u >> rhs,
            I64(i) => i >> rhs,
        }
    }
}

impl FromStr for Val {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix("0b") {
            if let Ok(u) = u64::from_str_radix(s, 2) {
                return Ok(U64(u));
            }
        } else if let Some(s) = s.strip_prefix("-0b") {
            if let Ok(i) = i64::from_str_radix(s, 2) {
                return Ok(I64(-i));
            }
        } else if let Some(s) = s.strip_prefix("0x") {
            if let Ok(u) = u64::from_str_radix(s, 16) {
                return Ok(U64(u));
            }
        } else if let Some(s) = s.strip_prefix("-0x") {
            if let Ok(i) = i64::from_str_radix(s, 16) {
                return Ok(I64(-i));
            }
        } else if let Ok(u) = s.parse::<u64>() {
            return Ok(U64(u));
        } else if let Ok(i) = s.parse::<i64>() {
            return Ok(I64(i));
        }
        Err(Error::InvalidInput(s.into()))
    }
}
