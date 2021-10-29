use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct MemAddress {
    lifetime: Lifetime,
    datatype: DataType,
    idx: usize,
}

impl MemAddress {
    pub fn new_builtin_callable(idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Constant,
            datatype: DataType::Callable,
            idx,
        }
    }

    pub fn new_constant(datatype: DataType, idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Constant,
            datatype,
            idx,
        }
    }

    pub fn new_temp(datatype: DataType, idx: usize) -> MemAddress {
        MemAddress {
            lifetime: Lifetime::Temporal,
            datatype,
            idx,
        }
    }
}

impl From<&MemAddress> for usize {
    fn from(address: &MemAddress) -> Self {
        usize::from(&address.lifetime) + usize::from(&address.datatype) + address.idx
    }
}

impl TryFrom<usize> for MemAddress {
    type Error = ();

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        let life = Lifetime::try_from(num)?;
        let data = DataType::try_from(num)?;
        let idx = num & 0xFFFFFF;
        Ok(MemAddress {
            lifetime: life,
            datatype: data,
            idx,
        })
    }
}

impl Display for MemAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", usize::from(self))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Lifetime {
    Constant,
    Variable,
    Temporal,
}

impl From<&Lifetime> for usize {
    fn from(life: &Lifetime) -> Self {
        let bits = 28;
        match life {
            Lifetime::Constant => (2 << bits),
            Lifetime::Variable => 2 * (2 << bits),
            Lifetime::Temporal => 3 * (2 << bits),
        }
    }
}

impl TryFrom<usize> for Lifetime {
    type Error = ();

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        let life_bits = (num >> 28) & 0xF;
        match life_bits {
            1 => Ok(Lifetime::Constant),
            2 => Ok(Lifetime::Variable),
            3 => Ok(Lifetime::Temporal),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Number,
    String,
    List,
    Vector,
    Set,
    Map,
    Callable,
    Nil,
    Unknown,
}

impl From<&DataType> for usize {
    fn from(data: &DataType) -> Self {
        let bits = 24;
        match data {
            DataType::Number => (2 << bits),
            DataType::String => 2 * (2 << bits),
            DataType::List => 3 * (2 << bits),
            DataType::Vector => 4 * (2 << bits),
            DataType::Set => 5 * (2 << bits),
            DataType::Map => 6 * (2 << bits),
            DataType::Callable => 7 * (2 << bits),
            DataType::Nil => 8 * (2 << bits),
            DataType::Unknown => 9 * (2 << bits),
        }
    }
}

impl TryFrom<usize> for DataType {
    type Error = ();

    fn try_from(num: usize) -> Result<Self, Self::Error> {
        let data_bits = (num >> 24) & 0xF;
        match data_bits {
            1 => Ok(DataType::Number),
            2 => Ok(DataType::String),
            3 => Ok(DataType::List),
            4 => Ok(DataType::Vector),
            5 => Ok(DataType::Set),
            6 => Ok(DataType::Map),
            7 => Ok(DataType::Callable),
            8 => Ok(DataType::Nil),
            9 => Ok(DataType::Unknown),
            _ => Err(()),
        }
    }
}
