use super::layer::Layer;

type LayerList = Vec<Box<dyn Layer>>;
type LayerIterator<'a> = Vec<&'a mut Box<dyn Layer>>;

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
        // layer.on_attach();
        self.layers.push(Box::new(layer));
    }

    pub fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        self.layers.pop()
        // if let Some(layer) = self.layers.pop() {
        //     layer.on_detach();
        //     Some(layer)
        // } else {
        //     None
        // }
    }
    pub fn push_overlay<T>(&mut self, layer: T)
    where
        T: Layer + 'static,
    {
        // layer.on_attach();
        self.overlayers.push(Box::new(layer));
    }
    pub fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        self.overlayers.pop()
        // if let Some(layer) = self.overlayers.pop() {
        //     layer.on_detach();
        //     Some(layer)
        // } else {
        //     None
        // }
    }

    pub fn layers(&mut self) -> LayerIterator {
        let mut vector = Vec::from_iter(self.overlayers.iter_mut());
        vector.extend(self.layers.iter_mut());
        return vector;
    }

    pub fn layers_rev(&mut self) -> LayerIterator {
        let mut vector =
            Vec::from_iter(self.overlayers.iter_mut().rev().collect::<LayerIterator>());
        vector.extend(self.layers.iter_mut().rev());
        return vector;
    }
}
