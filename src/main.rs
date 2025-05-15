use design_project_m11::{
    readwrite::{HDF5Reader, HDF5Writer},
    Chain, EmptyReader, EmptyWriter, IdentityFilter, filters::DoubleFilter,
};

fn main() {
    let chain = Chain::new(
        Box::new(HDF5Reader::<164>::new("pipeline0002_mflc1_00000.hdf5")),
        vec![Box::new(DoubleFilter {})],
        Box::new(HDF5Writer::new("test.hdf5")),
    );
    chain.run();
}
