use super::layer::Layer;

type LayerList = Vec<Box<dyn Layer>>;
type LayerIterator<'a> = Vec<&'a Box<dyn Layer>>;

pub struct LayerStack {
    overlayers: LayerList,
    layers: LayerList,
}

impl LayerStack {
    pub fn create() -> Self {
        Self {
            overlayers: vec![],
            layers: vec![],
        }
    }
    pub fn push_layer<T>(&mut self, layer: T)
    where
        T: Layer + 'static,
    {
        self.layers.push(Box::new(layer));
    }
    pub fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        self.layers.pop()
    }
    pub fn push_overlay<T>(&mut self, layer: T)
    where
        T: Layer + 'static,
    {
        self.overlayers.push(Box::new(layer));
    }
    pub fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        self.overlayers.pop()
    }

    pub fn layers(&self) -> LayerIterator {
        let mut vector = Vec::from_iter(&self.overlayers);
        vector.extend(self.layers.iter());
        return vector;
    }

    pub fn layers_rev(&self) -> LayerIterator {
        let mut vector = Vec::from_iter(self.overlayers.iter().rev().collect::<LayerIterator>());
        vector.extend(self.layers.iter().rev());
        return vector;
    }
}
