use core::ops::{Add, AddAssign, Sub, SubAssign};

macro_rules! timestamp {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        pub struct $name(#[doc = "The raw value in this timestamp’s unit."] pub u64);

        impl $name {
            /// Creates a zero timestamp.
            pub const fn new() -> Self {
                Self(0)
            }
        }

        impl From<u64> for $name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<$name> for u64 {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl Add<u64> for $name {
            type Output = Self;
            fn add(self, rhs: u64) -> Self {
                Self(self.0 + rhs)
            }
        }

        impl Sub<u64> for $name {
            type Output = Self;
            fn sub(self, rhs: u64) -> Self {
                Self(self.0 - rhs)
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self {
                self + rhs.0
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self {
                self - rhs.0
            }
        }

        impl AddAssign<u64> for $name {
            fn add_assign(&mut self, rhs: u64) {
                *self = *self + rhs;
            }
        }

        impl SubAssign<u64> for $name {
            fn sub_assign(&mut self, rhs: u64) {
                *self = *self - rhs;
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, rhs: Self) {
                *self += rhs.0;
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, rhs: Self) {
                *self -= rhs.0;
            }
        }
    };
}

timestamp!(
    TimeStamp,
    "Timestamp in seconds. Arithmetic follows u64 overflow behavior."
);
timestamp!(
    MilliTimeStamp,
    "Timestamp in milliseconds. Arithmetic follows u64 overflow behavior."
);
