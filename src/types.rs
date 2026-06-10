use std::ops::Neg;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Literal (u32);

impl Literal {
    pub fn new(variable: u32, sign: bool) -> Literal {
        Literal((variable << 1) | (sign as u32))
    }
}