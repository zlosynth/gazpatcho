extern crate imgui;

use std::collections::HashSet;

use crate::model::node::PinAddress;
use crate::model::Model;

impl Model {
    pub fn add_patch(&mut self, patch: Patch) {
        self.patches.insert(patch);
    }

    pub fn patches(&self) -> &HashSet<Patch> {
        &self.patches
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Patch {
    source: PinAddress,
    destination: PinAddress,
}

impl Patch {
    pub fn new(source: PinAddress, destination: PinAddress) -> Self {
        Self {
            source,
            destination,
        }
    }

    pub fn source(&self) -> &PinAddress {
        &self.source
    }

    pub fn destination(&self) -> &PinAddress {
        &self.destination
    }
}
