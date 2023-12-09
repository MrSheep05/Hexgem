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
