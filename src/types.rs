use std::ops::{Neg, Not};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum LBool {
    True,
    False,
    Undef
}

impl Neg for LBool {
    type Output = LBool;
    fn neg(self) -> Self::Output {
        match self {
            LBool::True => LBool::False,
            LBool::False => LBool::True,
            LBool::Undef => LBool::Undef
        }
    }
}

impl Not for LBool {
    type Output = LBool;
    fn not(self) -> Self::Output {
        self.neg()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Literal (u32);

impl Literal {
    pub fn new(variable: u32, sign: bool) -> Literal {
        Literal((variable << 1) | (sign as u32))
    }
    pub fn variable(&self) -> usize {
        (self.0 >> 1) as usize
    }
    pub fn sign(&self) ->bool {
        (self.0 & 1) == 1
    }
}

impl Neg for Literal {
    type Output = Literal;
    fn neg(self) -> Self::Output {
        Literal::new(self.variable() as u32, !self.sign())
    }
}

impl Not for Literal {
    type Output = Literal;
    fn not(self) -> Self::Output {
        self.neg()
    }
}