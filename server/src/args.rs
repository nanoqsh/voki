use clap::Parser;
use std::{fmt, str::FromStr};

struct Address(String);

impl Default for Address {
    fn default() -> Self {
        Self("0.0.0.0:4567".into())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.into()))
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Server local address
    #[clap(default_value_t)]
    address: Address,
}

impl Args {
    pub fn address(&self) -> String {
        self.address.0.clone()
    }
}
