use crate::Tape;

#[derive(Copy, Clone)]
struct CharType(u8);

impl CharType {
    const IS_SIMPLE_WS: Self = Self(0b0100);
    const FLAGS_LEN: u32 = 1; // number of flag bits

    #[inline]
    const fn bits(self) -> u8 {
        self.0
    }

    #[inline]
    const fn with_len(self, len: u8) -> u8 {
        self.0 | (len << Self::FLAGS_LEN)
    }
}

/// One byte for every possible `u8` value.
const CHAR_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    table[b' ' as usize] = CharType::IS_SIMPLE_WS.with_len(1);
    table[b'\t' as usize] = CharType::IS_SIMPLE_WS.with_len(4);
    table[b'\r' as usize] = CharType::IS_SIMPLE_WS.with_len(1);
    table
};

pub trait CharExt {
    /// Returns true if this is a whitespace character that is commonly typed.
    /// Includes space, tab, newline, and carriage return.
    #[must_use]
    #[allow(clippy::wrong_self_convention)] // pass primitive by value
    fn is_simple_ws(self) -> bool;

    /// Returns the length of the given whitespace character,
    /// where a tab counts as 4 spaces and all other simple whitespace count as 1.
    ///
    /// All other characters return a length of 0.
    #[must_use]
    fn simple_ws_len(self) -> u8;
}

impl CharExt for u8 {
    #[inline]
    fn is_simple_ws(self) -> bool {
        (CHAR_TABLE[self as usize] & CharType::IS_SIMPLE_WS.bits()) != 0
    }

    #[inline]
    fn simple_ws_len(self) -> u8 {
        // shift the stored length value back down
        CHAR_TABLE[self as usize] >> CharType::FLAGS_LEN
    }
}

pub trait SliceExt<'a, T> {
    /// Returns a subslice with leading and trailing flanking white space removed.
    fn trim_simple_ws(self) -> Self;
}

impl<'a> SliceExt<'a, u8> for &'a [u8] {
    fn trim_simple_ws(mut self) -> Self {
        while let [first, rest @ ..] = self {
            // peel off front
            if first.is_simple_ws() {
                self = rest;
            } else {
                break;
            }
        }
        while let [rest @ .., last] = self {
            // peel off back
            if last.is_simple_ws() {
                self = rest;
            } else {
                break;
            }
        }
        self
    }
}

pub trait ToTape<'a, T> {
    /// An ergonomic alternative to `Tape::new()`.
    fn to_tape(self) -> Tape<'a, T>;
}

impl<'a, T> ToTape<'a, T> for &'a [T] {
    fn to_tape(self) -> Tape<'a, T> {
        Tape::new(self)
    }
}
