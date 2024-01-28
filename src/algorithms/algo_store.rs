use std::fmt::{Display, Formatter};

use crate::{ceaser_cipher, utils};

#[derive(Debug)]
pub struct Algo<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub execute: fn(),
}

impl Display for Algo<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.name)
    }
}

impl Clone for Algo<'_> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            description: self.description,
            execute: self.execute,
        }
    }
}

pub const ALGORITHMS: [Algo;2] = [
    Algo {
        name: "Ceaser Cipher",
        description: "a substitution cipher where each letter in the plaintext is shifted a fixed number of positions down or up the alphabet.",
        execute: ceaser_cipher::execute
    },
    Algo {
        name: "Demo Algo",
        description: "demo algo",
        execute: utils::on_interrupt
    },
];
