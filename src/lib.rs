#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "alloc")]
extern crate alloc;

pub use crate::alias::{MilliTimeStamp, TimeStamp};
use core::ops::{Add, AddAssign, Deref, DerefMut, Sub, SubAssign};

/// Strongly typed seconds and milliseconds with ordinary `u64` arithmetic.
pub mod alias;

/// Packed BCD clock time (`0xHHMM`), from `0x0000` to `0x2359`.
///
/// Raw construction and mutable access are unchecked for compatibility. Call
/// [`Self::is_valid`] before arithmetic when accepting untrusted values.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct BcdTimeStamp {
    /// Raw packed BCD value. Writing this field does not validate the time.
    pub timestamp: u16,
}

/// Mask for the packed BCD minute byte.
pub const MINUTE_MASK: u16 = 0xFF;
/// Compatibility spelling of [`MINUTE_MASK`].
pub const MINUET_MASK: u16 = MINUTE_MASK;

impl BcdTimeStamp {
    /// Creates midnight (`00:00`).
    pub const fn new() -> BcdTimeStamp {
        BcdTimeStamp { timestamp: 0 }
    }

    /// Advances one minute, wrapping at midnight. Requires a valid BCD time.
    pub fn increment(&mut self) {
        let mut hour_carry = false;

        if self.minute() == 0x59 {
            hour_carry = true;
            self.timestamp &= 0xFF00;
        } else if self.timestamp & 0xF == 9 {
            self.timestamp += 0x7;
        } else {
            self.timestamp += 1;
        }

        if hour_carry {
            if self.hour() == 0x23 {
                self.timestamp = 0x0000;
            } else if self.timestamp & 0x0F00 == 0x0900 {
                self.timestamp += 0x0700;
            } else {
                self.timestamp += 0x0100;
            }
        }
    }

    /// Returns the BCD-encoded hour byte, e.g. `0x23`, not decimal `23`.
    #[inline(always)]
    pub fn hour(self) -> u16 {
        self.timestamp >> 8
    }

    /// Returns the BCD-encoded minute byte, e.g. `0x59`, not decimal `59`.
    #[inline(always)]
    pub fn minute(self) -> u16 {
        self.timestamp & MINUET_MASK
    }
}

impl From<u16> for BcdTimeStamp {
    fn from(value: u16) -> BcdTimeStamp {
        BcdTimeStamp { timestamp: value }
    }
}

impl Default for BcdTimeStamp {
    fn default() -> BcdTimeStamp {
        BcdTimeStamp::new()
    }
}

impl Deref for BcdTimeStamp {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.timestamp
    }
}

impl DerefMut for BcdTimeStamp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timestamp
    }
}

/// Converts seconds to a clock time modulo 24 hours, discarding seconds.
/// Unix timestamps yield UTC; no timezone or calendar adjustment is performed.
impl From<TimeStamp> for BcdTimeStamp {
    fn from(value: TimeStamp) -> Self {
        let h = value.0 / 3600 % 24;
        let m = value.0 / 60 % 60;

        let bcd = (h / 10) << 12 | (h % 10) << 8 | (m / 10) << 4 | (m % 10);

        Self::from(bcd as u16)
    }
}

impl BcdTimeStamp {
    /// Checks all BCD digits and the `00:00..=23:59` clock range.
    pub const fn is_valid(self) -> bool {
        let h = self.timestamp >> 8;
        let m = self.timestamp & MINUTE_MASK;
        h <= 0x23 && h & 0xF <= 9 && m <= 0x59 && m & 0xF <= 9
    }

    /// Constructs a clock time from validated packed BCD, or returns `None`.
    pub const fn from_bcd(value: u16) -> Option<Self> {
        let time = Self { timestamp: value };
        if time.is_valid() { Some(time) } else { None }
    }

    fn minutes_since_midnight(self) -> u16 {
        let hour = (self.hour() >> 4) * 10 + (self.hour() & 0xF);
        let minute = (self.minute() >> 4) * 10 + (self.minute() & 0xF);
        hour * 60 + minute
    }

    fn from_minutes(minutes: u16) -> Self {
        let hour = minutes / 60;
        let minute = minutes % 60;
        Self::from((hour / 10) << 12 | (hour % 10) << 8 | (minute / 10) << 4 | (minute % 10))
    }
}

/// Adds minutes to a valid BCD time, wrapping at midnight.
impl Add<u16> for BcdTimeStamp {
    type Output = Self;
    fn add(self, minutes: u16) -> Self {
        Self::from_minutes((self.minutes_since_midnight() + minutes % 1440) % 1440)
    }
}

/// Subtracts minutes from a valid BCD time, wrapping at midnight.
impl Sub<u16> for BcdTimeStamp {
    type Output = Self;
    fn sub(self, minutes: u16) -> Self {
        Self::from_minutes((self.minutes_since_midnight() + 1440 - minutes % 1440) % 1440)
    }
}

impl AddAssign<u16> for BcdTimeStamp {
    fn add_assign(&mut self, minutes: u16) {
        *self = *self + minutes;
    }
}

impl SubAssign<u16> for BcdTimeStamp {
    fn sub_assign(&mut self, minutes: u16) {
        *self = *self - minutes;
    }
}

#[cfg(test)]
mod tests {
    use super::BcdTimeStamp;

    #[test]
    fn minute_encoding_round_trips_for_entire_day() {
        for minute in 0..1440 {
            let time = BcdTimeStamp::from_minutes(minute);
            assert!(time.is_valid());
            assert_eq!(time.minutes_since_midnight(), minute);
        }
    }
}
