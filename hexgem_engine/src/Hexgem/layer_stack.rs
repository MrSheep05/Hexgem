use std::slice::Iter;

use super::layer::Layer;

pub struct LayarStack<T: Layer> {
    overlayers: Vec<T>,
    layers: Vec<T>,
}

impl<T: Layer> LayarStack<T> {
    pub fn create() -> Self {
        Self {
            overlayers: vec![],
            layers: vec![],
        }
    }
    pub fn push_layer(&mut self, layer: T) {
        self.layers.push(layer);
    }
    pub fn pop_layer(&mut self) -> Option<T> {
        self.layers.pop()
    }
    pub fn push_overlay(&mut self, layer: T) {
        self.overlayers.push(layer);
    }
    pub fn pop_overlay(&mut self) -> Option<T> {
        self.overlayers.pop()
    }

    pub fn layers(&self) -> Vec<&T> {
        let mut vector = Vec::from_iter(&self.overlayers);
        vector.extend(self.layers.iter());
        return vector;
    }

    pub fn layers_rev(&self) -> Vec<&T> {
        let mut vector = Vec::from_iter(self.overlayers.iter().rev().collect::<Vec<&T>>());
        vector.extend(self.layers.iter().rev());
        return vector;
    }
}
