use std::{
    fmt::{self, Display, Formatter},
    hash::Hash,
};

#[derive(Debug, Clone, Copy)]
pub struct MemAddress {
    lifetime: Lifetime,
    idx: usize,
}

impl MemAddress {
    pub fn new_const(idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Constant,
            idx,
        }
    }

    pub fn new_var(idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Variable,
            idx,
        }
    }

    pub fn new_arg(idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Argument,
            idx,
        }
    }

    pub fn new_temp(idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Temporal,
            idx,
        }
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

impl From<&MemAddress> for usize {
    fn from(address: &MemAddress) -> Self {
        usize::from(&address.lifetime) + address.idx
    }
}

impl TryFrom<usize> for MemAddress {
    type Error = ();

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        let lifetime = Lifetime::try_from(num)?;
        let idx = num & 0xFFFFFF;
        Ok(MemAddress { lifetime, idx })
    }
}

impl Display for MemAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", usize::from(self))
    }
}

impl PartialEq for MemAddress {
    fn eq(&self, other: &Self) -> bool {
        usize::from(self) == usize::from(other)
    }
}
impl Eq for MemAddress {}

impl Hash for MemAddress {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        usize::from(self).hash(state)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lifetime {
    Constant,
    Variable,
    Argument,
    Temporal,
}

const LIFETIME_SHIFT: usize = 28;
const LIFETIME_BITS: usize = 4;
const LIFETIME_MASK: usize = (1 << LIFETIME_BITS) - 1;

impl From<&Lifetime> for usize {
    fn from(life: &Lifetime) -> Self {
        let base = 1 << LIFETIME_SHIFT;
        match life {
            Lifetime::Constant => base,
            Lifetime::Variable => 2 * base,
            Lifetime::Argument => 3 * base,
            Lifetime::Temporal => 4 * base,
        }
    }
}

impl TryFrom<usize> for Lifetime {
    type Error = ();

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        match (num >> LIFETIME_SHIFT) & LIFETIME_MASK {
            1 => Ok(Lifetime::Constant),
            2 => Ok(Lifetime::Variable),
            3 => Ok(Lifetime::Argument),
            4 => Ok(Lifetime::Temporal),
            _ => Err(()),
        }
    }
}
