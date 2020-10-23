extern crate imgui;

use std::collections::HashSet;

use crate::model::node::PinIndex;
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
    source: PinIndex,
    destination: PinIndex,
}

impl Patch {
    pub fn new(source: &PinIndex, destination: &PinIndex) -> Self {
        Self {
            source: source.clone(),
            destination: destination.clone(),
        }
    }

    pub fn source(&self) -> &PinIndex {
        &self.source
    }

    pub fn destination(&self) -> &PinIndex {
        &self.destination
    }
}
