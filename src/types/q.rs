pub const POW10: [u64; 20] = [
    1,
    10,
    100,
    1000,
    10000,
    100000,
    1000000,
    10000000,
    100000000,
    1000000000,
    10000000000,
    100000000000,
    1000000000000,
    10000000000000,
    100000000000000,
    1000000000000000,
    10000000000000000,
    100000000000000000,
    1000000000000000000,
    10000000000000000000,
];

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Q64<const DECIMALS: u32 = 8>(pub u64);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct IQ64<const DECIMALS: u32 = 8>(pub i64);

impl<const DECIMALS: u32> Q64<DECIMALS> {
    pub const SCALE: u64 = POW10[DECIMALS as usize];
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::SCALE);

    #[inline(always)]
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn to_raw(self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub const fn from_integer(val: u64) -> Self {
        Self(val * Self::SCALE)
    }

    #[inline(always)]
    pub fn parse_ascii(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::ZERO;
        }

        let dot_idx = bytes.iter().position(|&b| b == b'.').unwrap_or(bytes.len());

        let mut int_part = 0u64;
        for &b in &bytes[..dot_idx] {
            if !b.is_ascii_digit() {
                return Self::ZERO;
            }
            int_part = int_part.wrapping_mul(10).wrapping_add((b - b'0') as u64);
        }

        let mut frac_part = 0u64;
        let mut frac_len = 0u32;

        if dot_idx < bytes.len() {
            for &b in &bytes[dot_idx + 1..] {
                if frac_len >= DECIMALS {
                    break;
                }
                if !b.is_ascii_digit() {
                    return Self::ZERO;
                }
                frac_part = frac_part.wrapping_mul(10).wrapping_add((b - b'0') as u64);
                frac_len += 1;
            }
        }

        let pad = DECIMALS.saturating_sub(frac_len) as usize;
        let scale_pad = POW10.get(pad).copied().unwrap_or(1);
        frac_part = frac_part.wrapping_mul(scale_pad);

        Self(int_part.wrapping_mul(Self::SCALE).wrapping_add(frac_part))
    }

    #[inline(always)]
    pub const fn as_iq64(self) -> IQ64<DECIMALS> {
        IQ64(self.0 as i64)
    }

    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        Self((((self.0 as u128) * (rhs.0 as u128)) / (Self::SCALE as u128)) as u64)
    }

    #[inline(always)]
    pub const fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self::ZERO;
        }
        Self((((self.0 as u128) * (Self::SCALE as u128)) / (rhs.0 as u128)) as u64)
    }
}

impl<const DECIMALS: u32> IQ64<DECIMALS> {
    pub const SCALE: i64 = POW10[DECIMALS as usize] as i64;
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(Self::SCALE);

    #[inline(always)]
    pub const fn from_raw(raw: i64) -> Self {
        Self(raw)
    }

    #[inline(always)]
    pub const fn to_raw(self) -> i64 {
        self.0
    }

    #[inline(always)]
    pub const fn from_integer(val: i64) -> Self {
        Self(val * Self::SCALE)
    }

    #[inline(always)]
    pub fn parse_ascii(bytes: &[u8]) -> Self {
        if bytes.is_empty() {
            return Self::ZERO;
        }

        let (is_neg, rest) = match bytes[0] {
            b'-' => (true, &bytes[1..]),
            b'+' => (false, &bytes[1..]),
            _ => (false, bytes),
        };

        let q = Q64::<DECIMALS>::parse_ascii(rest).0 as i64;
        Self(if is_neg { -q } else { q })
    }

    #[inline(always)]
    pub const fn mul(self, rhs: Self) -> Self {
        Self((((self.0 as i128) * (rhs.0 as i128)) / (Self::SCALE as i128)) as i64)
    }

    #[inline(always)]
    pub const fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            return Self::ZERO;
        }
        Self((((self.0 as i128) * (Self::SCALE as i128)) / (rhs.0 as i128)) as i64)
    }
}

impl<const DECIMALS: u32> core::ops::Add for Q64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<const DECIMALS: u32> core::ops::Sub for Q64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<const DECIMALS: u32> core::ops::Add for IQ64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl<const DECIMALS: u32> core::ops::Sub for IQ64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl<const DECIMALS: u32> core::ops::Neg for IQ64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl<const DECIMALS: u32> core::ops::AddAssign for Q64<DECIMALS> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl<const DECIMALS: u32> core::ops::SubAssign for Q64<DECIMALS> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl<const DECIMALS: u32> core::ops::Mul for Q64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl<const DECIMALS: u32> core::ops::MulAssign for Q64<DECIMALS> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl<const DECIMALS: u32> core::ops::Div for Q64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(rhs)
    }
}

impl<const DECIMALS: u32> core::ops::DivAssign for Q64<DECIMALS> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl<const DECIMALS: u32> core::ops::AddAssign for IQ64<DECIMALS> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
    }
}

impl<const DECIMALS: u32> core::ops::SubAssign for IQ64<DECIMALS> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_sub(rhs.0);
    }
}

impl<const DECIMALS: u32> core::ops::Mul for IQ64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

impl<const DECIMALS: u32> core::ops::MulAssign for IQ64<DECIMALS> {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl<const DECIMALS: u32> core::ops::Div for IQ64<DECIMALS> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.div(rhs)
    }
}

impl<const DECIMALS: u32> core::ops::DivAssign for IQ64<DECIMALS> {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}
