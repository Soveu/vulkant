use core::fmt;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32);

impl Version {
    pub const fn new(variant: u8, major: u8, minor: u16, patch: u16) -> Self {
        assert!(patch < (1 << 12));
        assert!(minor < (1 << 10));
        assert!(major < (1 << 7));
        assert!(variant < (1 << 3));

        return Self(0
            | ((variant as u32) << 29)
            | ((major as u32) << 22)
            | ((minor as u32) << 12)
            | ((patch as u32) << 0));
    }

    pub const fn variant(self) -> u8 {
        ((self.0 >> 29) & 0b111) as u8
    }

    pub const fn major(self) -> u8 {
        ((self.0 >> 22) & 0b1111111) as u8
    }

    pub const fn minor(self) -> u16 {
        ((self.0 >> 12) & 0b1111111111) as u16
    }

    pub const fn patch(self) -> u16 {
        ((self.0 >> 0) & 0xFFF) as u16
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}",
            self.variant(),
            self.major(),
            self.minor(),
            self.patch(),
        )
    }
}
