use hdf5::Dataset;
use hdf5::File;
use ndarray::RawData;

use crate::chunk::Chunk;
use hdf5::dataset;
use ndarray::s;
use ndarray::ArrayBase;
use ndarray::Data;
use ndarray::Dim;
use ndarray::IxDynImpl;
use ndarray::OwnedRepr;
use ndarray::Shape;
use ndarray::SliceInfo;
use ndarray::SliceInfoElem;
use std::fmt::Debug;
use std::{any::type_name, thread, time::Duration};

/// Reads [Chunk]s from for example a file.
pub trait ChunkReader: Send {
    /// Read a [Chunk]
    fn read_chunk(&mut self) -> Option<Chunk>;
}

/// Reader that reads N empty Chunks.
pub struct EmptyReader<const N: usize> {
    n: usize,
}

impl<const N: usize> ChunkReader for EmptyReader<N> {
    fn read_chunk(&mut self) -> Option<Chunk> {
        if self.n == N {
            None
        } else {
            println!("reading chunk...");
            thread::sleep(Duration::from_millis(500));
            self.n = self.n + 1;
            Some(Chunk::new(self.n))
        }
    }
}

impl<const N: usize> EmptyReader<N> {
    pub fn new() -> Self {
        EmptyReader { n: 0 }
    }
}

pub struct HDF5Reader<const N: usize> {
    pos: usize,
    file: File,
}

impl<const N: usize> ChunkReader for HDF5Reader<N> {
    fn read_chunk(&mut self) -> Option<Chunk> {
        let mut chunk = Chunk::new(self.pos);
        // println!("{:?}", &self.file.datasets().unwrap());
        // let been_here: bool = false;

        for dataset in self.file.datasets().unwrap() {
            // need better name for the variables.
            let (chunk_size, sensor_size, third_dim) = opt_chunk_size(&dataset, 6);
            //N`s ARE CHANGED WITH CHUNK SIZE
            // const M: usize = chunk_size;
            if dataset.name() == "/circdistance" {
                continue;
            }

            if dataset.shape()[0] < self.pos {
                return None;
            }

            // TODO :: dont skip this
            if dataset.shape()[0] < self.pos + N {
                continue;
            }

            println!("{:?}", &dataset.name());
            // if dataset.name() != "/mflc1" {
            // remeber to change this
            //-------------------------------------------------------------------------------
            let dim: usize = dataset.shape().len();
            // println!("{}", dim);
            if dim == 1 {
                // insert the shape
                chunk.insert_shape(dataset.name(), dataset.shape().to_vec());

                // insert the data
                chunk.insert_double(
                        dataset.name(),
                        dataset.read_slice::<f64,SliceInfo<[SliceInfoElem;1],Dim<[usize;1]>,Dim<[usize;1]>>,Dim<[usize;1]>>
                    (s![self.pos  .. self.pos + N])
                    .unwrap().into_raw_vec()
                    );
            } else if dim == 2 {
                // insert shape
                chunk.insert_shape(dataset.name(), dataset.shape().to_vec());

                // insert data

                chunk.insert_double(
                        dataset.name(),
                        dataset.read_slice::<f64,SliceInfo<[SliceInfoElem;2],Dim<[usize;2]>,Dim<[usize;2]>>,Dim<[usize;2]>>
                            (s![self.pos  .. self.pos + N, 0..sensor_size])
                                .unwrap().into_raw_vec()
                    );
            } else if dim == 3 {
                // insert shape
                chunk.insert_shape(dataset.name(), dataset.shape().to_vec());

                // insert data

                chunk.insert_double(
                        dataset.name(),
                        dataset.read_slice::<f64,SliceInfo<[SliceInfoElem;3],Dim<[usize;3]>,Dim<[usize;3]>>,Dim<[usize;3]>>
                            (s![self.pos ..self.pos + N, 0..sensor_size,  0..third_dim])
                                .unwrap().into_raw_vec()
                    );
                // }
            } else {
                // println!("broke here");
                break;
            }
        }
        // if (been_here) {
        //     return None;
        // }
        self.pos += N;
        Some(chunk)
    }
}

impl<const N: usize> HDF5Reader<N> {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        Self { pos: 0, file: file }
    }
}

//return (chunk_size,sensor_size ,third_dim)
fn opt_chunk_size(dataset: &Dataset, n: usize) -> (usize, usize, usize) {
    let dim: usize = dataset.shape().len();
    let row_size = *dataset.shape().get(0).unwrap();
    let mut primes: Vec<i64> = factors(row_size as i64);
    primes.reverse();
    let mut chunk_size = 1;

    //this number 0..X determines the chunk size
    for i in 0..n {
        chunk_size = chunk_size * primes.get(i).unwrap();
    }
    if dim == 1 {
        return (chunk_size as usize, 0, 0);
    } else if dim == 2 {
        let sensor_size = *dataset.shape().get(1).unwrap();
        return (chunk_size as usize, sensor_size, 0);
    } else if dim == 3 {
        let sensor_size = *dataset.shape().get(1).unwrap();
        let third_dim = *dataset.shape().get(2).unwrap();
        return (chunk_size as usize, sensor_size, third_dim);
    } else {
        return (0, 0, 0);
    }
}

//https://exercism.org/tracks/rust/exercises/prime-factors/solutions/chenhowa
pub fn factors(mut n: i64) -> Vec<i64> {
    let mut out = vec![];
    for i in 2..(n + 1) {
        while n % i == 0 {
            out.push(i);
            n /= i;
        }
        if n == 1 {
            break;
        }
    }
    out
}

/// Writes [Chunk]s to for example a file.
pub trait ChunkWriter: Send {
    /// Write a [Chunk]
    fn write_chunk(&mut self, chunk: Chunk);
}

pub struct EmptyWriter {}

impl ChunkWriter for EmptyWriter {
    fn write_chunk(&mut self, _chunk: Chunk) {
        println!("writing chunk...");
        thread::sleep(Duration::from_millis(500));
    }
}

pub struct HDF5Writer {
    file: File,
}

impl ChunkWriter for HDF5Writer {
    fn write_chunk(&mut self, chunk: Chunk) {
        // println!("{}", chunk.queue);
        // println!("double data {}", chunk.double_data.len());
        // println!("double data {}", chunk.double_data.len());
        for (var, data) in chunk.double_data {
            // println!("{:?}", data);
            // println!("{var}");
            // println!("{:?}", self.file.dataset(&var));
            // println!("{:?}", self.file.dataset(&var));
            // get the shape of the chunk
            let shape = chunk.shape.get(&var).unwrap();
            let dim = shape.len();
            // if dim > 1 {
            //     println!("{:?}", shape[1]);
            //     println!("{:?}", data.len());
            // };
            // println!("{:?}", &dim);

            if dim == 1 {
                let dataset = self.file.dataset(&var).unwrap_or_else(|_| {
                    self.file
                        .new_dataset::<f64>()
                        .shape((167936, None))
                        .create(&*var)
                        .unwrap()
                });
                dataset.resize(chunk.queue + data.len());
                dataset
                    .write_slice(&data, chunk.queue..chunk.queue + data.len())
                    .unwrap();
            } else if dim == 2 {
                // change data back to 2d array
                let data: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>> =
                    ArrayBase::from_shape_vec((data.len() / shape[1], shape[1]), data)
                        .expect("no result");
                // assert!(data.);
                let dataset = self.file.dataset(&var).unwrap_or_else(|_| {
                    self.file
                        .new_dataset::<f64>()
                        .shape(((167936, None), shape[1]))
                        .create(&*var)
                        .unwrap()
                });
                dataset.resize((chunk.queue + (data.len() / shape[1]), shape[1]));
                dataset
                    .write_slice(
                        &data,
                        (
                            chunk.queue..(chunk.queue + (data.len()) / shape[1]),
                            0..shape[1],
                        ),
                    )
                    .unwrap();
            } else if dim == 3 {
                // change data back to 2d array
                let data: ArrayBase<OwnedRepr<f64>, Dim<[usize; 3]>> = ArrayBase::from_shape_vec(
                    (data.len() / (shape[1] + shape[2]), shape[1], shape[2]),
                    data,
                )
                .expect("no result");
                let dataset = self.file.dataset(&var).unwrap_or_else(|_| {
                    self.file
                        .new_dataset::<f64>()
                        .shape(((167936, None), shape[1], shape[2]))
                        .create(&*var)
                        .unwrap()
                });
                dataset.resize((
                    chunk.queue + data.len() / (shape[1] + shape[2]),
                    shape[1],
                    shape[2],
                ));
                dataset
                    .write_slice(
                        &data,
                        (
                            chunk.queue..chunk.queue + data.len() / (shape[1] + shape[2]),
                            0..shape[1],
                            0..shape[2],
                        ),
                    )
                    .unwrap();
            } else {
                panic!("Unsupported dimension")
            }
        }
    }
}

impl HDF5Writer {
    pub fn new(filename: &str) -> Self {
        let file = File::create(filename).unwrap();
        HDF5Writer { file }
    }
}
