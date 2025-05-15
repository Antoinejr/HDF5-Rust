use std::collections::HashMap;

/// Stores data for a certain stretch of pipe.
pub struct Chunk {
    pub(crate) double_data: HashMap<String, Vec<f64>>,
    pub(crate) queue: usize,
    pub(crate) shape: HashMap<String, Vec<usize>>,
    //start: f64,
    //end: f64,
}

impl Chunk {
    /// Produce an empty chunk without data
    pub(crate) fn new(queue: usize) -> Chunk {
        Chunk {
            queue: queue,
            double_data: HashMap::<String, Vec<f64>>::new(),
            shape: HashMap::<String, Vec<usize>>::new(),
        }
    }

    pub fn insert_double(&mut self, name: impl Into<String>, data: Vec<f64>) {
        self.double_data.insert(name.into(), data);
    }

    //keep the shape of the read data.
    pub fn insert_shape(&mut self, name: impl Into<String>, shape: Vec<usize>) {
        self.shape.insert(name.into(), shape);
    }
}
