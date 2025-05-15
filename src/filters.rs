pub use crate::chain::Container;
use crate::chunk;

/// Trait [Filters](Filter) have to implement.
pub trait Filter: Send {
    /// Function to configure the filter.
    fn config(&self, config: &mut FilterConfig);

    /// Function run for every [Chunk] that is being processed.
    fn run<'a>(&self, container: &mut Container<'a>);
}

/// Struct used to store configuration for [Filters](Filter).
/// for example the amount of chunks they will need.
pub struct FilterConfig {
    pre_chunks: usize,
    pos_chunks: usize,
}

/// A [Filter] that does nothing.
pub struct IdentityFilter {}

impl Filter for IdentityFilter {
    fn config(&self, _config: &mut FilterConfig) {
        // No configuration needed
    }

    fn run(&self, _container: &mut Container) {
        // Do nothing
        println!("processing chunk...")
    }
}

pub struct DoubleFilter {}

impl Filter for DoubleFilter {
    fn config(&self, _config: &mut FilterConfig) {}
    fn run<'a>(&self, container: &mut Container<'a>) {
        let mut c = container.get_chunk_mut();
        for (k, chunk) in &mut c.double_data {
            for data in chunk {
                *data = *data + *data;

                // println!(d)
            }
        }
    }
}
