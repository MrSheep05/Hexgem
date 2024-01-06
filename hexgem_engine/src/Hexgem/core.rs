pub trait ToAny: 'static {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct Position<T> {
    pub x: T,
    pub y: T,
}

pub struct Size<T> {
    pub width: T,
    pub height: T,
}
#[macro_export]
macro_rules! toAnyImpl {
    ($struct:ident) => {
        impl crate::Hexgem::core::ToAny for $struct {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

pub const fn bit(i: u8) -> u32 {
    return 1 << i;
}

#[macro_export]
macro_rules! bitOperations {
    ($str:ident) => {
        use std::ops::{BitAnd, BitOr};

        impl BitOr for $str {
            type Output = $str;

            fn bitor(self, rhs: Self) -> Self {
                Self(rhs.0 | self.0)
            }
        }

        impl BitAnd for $str {
            type Output = $str;

            fn bitand(self, rhs: Self) -> Self {
                Self(rhs.0 & self.0)
            }
        }
    };
}
