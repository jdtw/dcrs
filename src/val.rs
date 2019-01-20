use crate::error::Error;
use std::convert::From;
use std::fmt;
use std::ops;
use std::str::FromStr;
use Val::*;

#[derive(Debug, Clone)]
pub enum Val {
    U64(u64),
    I64(i64),
    F64(f64),
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => write!(f, "{}u64", u),
            I64(i) => write!(f, "{}i64", i),
            F64(fl) => write!(f, "{}f64", fl),
        }
    }
}

impl fmt::LowerHex for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => f.pad(&format!("{:x}", u)),
            I64(i) => f.pad(&format!("{:x}", i)),
            F64(fl) => write!(f, "{}f64", fl),
        }
    }
}

impl fmt::Binary for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            U64(u) => f.pad(&format!("{:b}", u)),
            I64(i) => f.pad(&format!("{:b}", i)),
            F64(fl) => write!(f, "{}f64", fl),
        }
    }
}

impl Val {
    pub fn pow(self, other: Val) -> Val {
        match self {
            U64(u) => U64(u.pow(other.into())),
            I64(i) => I64(i.pow(other.into())),
            F64(f) => F64(f.powi(other.into())),
        }
    }
}

impl From<Val> for i32 {
    fn from(v: Val) -> i32 {
        match v {
            U64(u) => u as i32,
            I64(i) => i as i32,
            F64(f) => f as i32,
        }
    }
}

impl From<Val> for u32 {
    fn from(v: Val) -> u32 {
        match v {
            U64(u) => u as u32,
            I64(i) => i as u32,
            F64(f) => f as u32,
        }
    }
}

impl From<Val> for u64 {
    fn from(v: Val) -> u64 {
        match v {
            U64(u) => u,
            I64(i) => i as u64,
            F64(f) => f as u64,
        }
    }
}

impl From<Val> for i64 {
    fn from(v: Val) -> i64 {
        match v {
            U64(u) => u as i64,
            I64(i) => i,
            F64(f) => f as i64,
        }
    }
}

impl From<Val> for f64 {
    fn from(v: Val) -> f64 {
        match v {
            U64(u) => u as f64,
            I64(i) => i as f64,
            F64(f) => f,
        }
    }
}

impl ops::Add<Val> for u64 {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self + u),
            I64(i) => I64(self as i64 + i),
            F64(f) => F64(self as f64 + f),
        }
    }
}

impl ops::Add<Val> for i64 {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self + u as i64),
            I64(i) => I64(self + i),
            F64(f) => F64(self as f64 + f),
        }
    }
}

impl ops::Add<Val> for f64 {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => F64(self + u as f64),
            I64(i) => F64(self + i as f64),
            F64(f) => F64(self + f),
        }
    }
}

impl ops::Add for Val {
    type Output = Val;
    fn add(self, rhs: Val) -> Val {
        match self {
            U64(u) => u + rhs,
            I64(i) => i + rhs,
            F64(f) => f + rhs,
        }
    }
}

impl ops::Sub<Val> for u64 {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self - u),
            I64(i) => I64(self as i64 - i),
            F64(f) => F64(self as f64 - f),
        }
    }
}

impl ops::Sub<Val> for i64 {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self - u as i64),
            I64(i) => I64(self - i),
            F64(f) => F64(self as f64 - f),
        }
    }
}

impl ops::Sub<Val> for f64 {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => F64(self - u as f64),
            I64(i) => F64(self - i as f64),
            F64(f) => F64(self - f),
        }
    }
}

impl ops::Sub for Val {
    type Output = Val;
    fn sub(self, rhs: Val) -> Val {
        match self {
            U64(u) => u - rhs,
            I64(i) => i - rhs,
            F64(f) => f - rhs,
        }
    }
}

impl ops::Mul<Val> for u64 {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self * u),
            I64(i) => I64(self as i64 * i),
            F64(f) => F64(self as f64 * f),
        }
    }
}

impl ops::Mul<Val> for i64 {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self * u as i64),
            I64(i) => I64(self * i),
            F64(f) => F64(self as f64 * f),
        }
    }
}

impl ops::Mul<Val> for f64 {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => F64(self * u as f64),
            I64(i) => F64(self * i as f64),
            F64(f) => F64(self * f),
        }
    }
}

impl ops::Mul for Val {
    type Output = Val;
    fn mul(self, rhs: Val) -> Val {
        match self {
            U64(u) => u * rhs,
            I64(i) => i * rhs,
            F64(f) => f * rhs,
        }
    }
}

impl ops::Div<Val> for u64 {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self / u),
            I64(i) => I64(self as i64 / i),
            F64(f) => F64(self as f64 / f),
        }
    }
}

impl ops::Div<Val> for i64 {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self / u as i64),
            I64(i) => I64(self / i),
            F64(f) => F64(self as f64 / f),
        }
    }
}

impl ops::Div<Val> for f64 {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => F64(self / u as f64),
            I64(i) => F64(self / i as f64),
            F64(f) => F64(self / f),
        }
    }
}

impl ops::Div for Val {
    type Output = Val;
    fn div(self, rhs: Val) -> Val {
        match self {
            U64(u) => u / rhs,
            I64(i) => i / rhs,
            F64(f) => f / rhs,
        }
    }
}

impl ops::Rem for Val {
    type Output = Val;
    fn rem(self, rhs: Val) -> Val {
        match self {
            U64(u) => u / rhs,
            I64(i) => i / rhs,
            F64(f) => f / rhs,
        }
    }
}

impl ops::BitAnd<Val> for u64 {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self & u),
            I64(i) => I64(self as i64 & i),
            // TODO(jdtw): Warn
            F64(f) => I64(self as i64 & f as i64),
        }
    }
}

impl ops::BitAnd<Val> for i64 {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self & u as i64),
            I64(i) => I64(self & i),
            // TODO(jdtw): Warn
            F64(f) => I64(self & f as i64),
        }
    }
}

impl ops::BitAnd for Val {
    type Output = Val;
    fn bitand(self, rhs: Val) -> Val {
        match self {
            U64(u) => u & rhs,
            I64(i) => i & rhs,
            // TODO(jdtw): Warn
            F64(f) => f as i64 & rhs,
        }
    }
}

impl ops::BitOr<Val> for u64 {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self | u),
            I64(i) => I64(self as i64 | i),
            // TODO(jdtw): Warn
            F64(f) => I64(self as i64 | f as i64),
        }
    }
}

impl ops::BitOr<Val> for i64 {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self | u as i64),
            I64(i) => I64(self | i),
            // TODO(jdtw): Warn
            F64(f) => I64(self | f as i64),
        }
    }
}

impl ops::BitOr for Val {
    type Output = Val;
    fn bitor(self, rhs: Val) -> Val {
        match self {
            U64(u) => u | rhs,
            I64(i) => i | rhs,
            // TODO(jdtw): Warn
            F64(f) => f as i64 | rhs,
        }
    }
}

impl ops::BitXor<Val> for u64 {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self ^ u),
            I64(i) => I64(self as i64 ^ i),
            // TODO(jdtw): Warn
            F64(f) => I64(self as i64 ^ f as i64),
        }
    }
}

impl ops::BitXor<Val> for i64 {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self ^ u as i64),
            I64(i) => I64(self ^ i),
            // TODO(jdtw): Warn
            F64(f) => I64(self ^ f as i64),
        }
    }
}

impl ops::BitXor for Val {
    type Output = Val;
    fn bitxor(self, rhs: Val) -> Val {
        match self {
            U64(u) => u ^ rhs,
            I64(i) => i ^ rhs,
            // TODO(jdtw): Warn
            F64(f) => f as i64 ^ rhs,
        }
    }
}

impl ops::Not for Val {
    type Output = Val;
    fn not(self) -> Val {
        match self {
            U64(u) => U64(!u),
            I64(i) => I64(!i),
            // TODO(jdtw): Warn
            F64(f) => I64(!(f as i64)),
        }
    }
}

impl ops::Shl<Val> for u64 {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self << u),
            I64(i) => I64((self as i64) << i),
            // TODO(jdtw): Warn
            F64(f) => I64((self as i64) << f as i64),
        }
    }
}

impl ops::Shl<Val> for i64 {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self << u as i64),
            I64(i) => I64(self << i),
            // TODO(jdtw): Warn
            F64(f) => I64(self << f as i64),
        }
    }
}
impl ops::Shl for Val {
    type Output = Val;
    fn shl(self, rhs: Val) -> Val {
        match self {
            U64(u) => u << rhs,
            I64(i) => i << rhs,
            F64(f) => (f as i64) << rhs,
        }
    }
}

impl ops::Shr<Val> for u64 {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => U64(self >> u),
            I64(i) => I64((self as i64) >> i),
            // TODO(jdtw): Warn
            F64(f) => I64((self as i64) >> f as i64),
        }
    }
}

impl ops::Shr<Val> for i64 {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match rhs {
            U64(u) => I64(self >> u as i64),
            I64(i) => I64(self >> i),
            // TODO(jdtw): Warn
            F64(f) => I64(self >> f as i64),
        }
    }
}

impl ops::Shr for Val {
    type Output = Val;
    fn shr(self, rhs: Val) -> Val {
        match self {
            U64(u) => u >> rhs,
            I64(i) => i >> rhs,
            F64(f) => (f as i64) >> rhs,
        }
    }
}

impl FromStr for Val {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("0b") {
            if let Ok(u) = u64::from_str_radix(&s[2..], 2) {
                return Ok(U64(u));
            }
        } else if s.starts_with("-0b") {
            if let Ok(i) = i64::from_str_radix(&s[3..], 2) {
                return Ok(I64(-i));
            }
        } else if s.starts_with("0x") {
            if let Ok(u) = u64::from_str_radix(&s[2..], 16) {
                return Ok(U64(u));
            }
        } else if s.starts_with("-0x") {
            if let Ok(i) = i64::from_str_radix(&s[3..], 16) {
                return Ok(I64(-i));
            }
        } else if let Ok(u) = s.parse::<u64>() {
            return Ok(U64(u));
        } else if let Ok(i) = s.parse::<i64>() {
            return Ok(I64(i));
        } else if let Ok(f) = s.parse::<f64>() {
            return Ok(F64(f));
        }
        Err(Error::InvalidInput(s.into()))
    }
}
