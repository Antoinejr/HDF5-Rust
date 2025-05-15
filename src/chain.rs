use crate::chunk::Chunk;
use crate::filters::Filter;
use crate::readwrite::{ChunkReader, ChunkWriter};
use std::{
    collections::VecDeque,
    mem,
    sync::mpsc,
    thread::{self},
    time::Duration,
};

#[derive(Debug)]
struct Buffer<T> {
    size: usize,
    buffer: VecDeque<T>,
}

impl<T> Buffer<T> {
    fn new(size: usize) -> Self {
        Self {
            size,
            buffer: VecDeque::new(),
        }
    }

    fn shift(&mut self, item: Option<T>) -> Option<T> {
        if let Some(item) = item {
            self.buffer.push_front(item);
            if self.buffer.len() > self.size {
                self.buffer.pop_back()
            } else {
                None
            }
        } else {
            None
        }
    }

    fn drain(&mut self) -> Option<T> {
        self.buffer.pop_back()
    }
}

/// A struct that acts like a window for a [Filter] to see into the buffers of a thread.
pub struct Container<'a> {
    pre_buffer: &'a Buffer<Chunk>,
    current: &'a mut Option<Chunk>,
    post_buffer: &'a Buffer<Chunk>,
}

impl Container<'_> {
    // this takes a &mut reference meaning that it only borrows self and not consume
    // container.get_chunk_mut() <- will borrow container
    // container.get_chunk_mut() <- will work if borrow is finished
    pub fn get_chunk_mut(&mut self) -> &mut Chunk {
        self.current.as_mut().unwrap()
    }
}

/// Collects a [ChunkReader], [Filters](Filter), and [ChunkWriter] into a runable package.
pub struct Chain {
    reader: Box<dyn ChunkReader>,
    filters: Vec<Box<dyn Filter>>,
    writer: Box<dyn ChunkWriter>,
}

impl Chain {
    /// Create a new chain.
    pub fn new(
        reader: Box<dyn ChunkReader>,
        filters: Vec<Box<dyn Filter>>,
        writer: Box<dyn ChunkWriter>,
    ) -> Chain {
        Chain {
            reader,
            filters,
            writer,
        }
    }

    /// Run the Chain.
    // TODO: this method is a bit long and complex
    pub fn run(mut self) {
        let (sx, mut rx) = mpsc::channel::<Chunk>();
        // Create a thread for the reader
        let reader_handle = thread::spawn(move || {
            while let Some(chunk) = self.reader.read_chunk() {
                sx.send(chunk).expect("reciever to live longer than sender");
                //println!("send chunk!");
            }
        });
        // Create a thread for each filter
        for filter in self.filters {
            let (sx, mut rx2) = mpsc::channel::<Chunk>();
            mem::swap(&mut rx, &mut rx2);
            thread::spawn(move || {
                // See the design document to see how it is implemented
                // https://drive.google.com/file/d/1Wj7agRHp8A1HtRGF7zzeF89S7ihTm9s4/view?usp=sharing
                let mut pre_buffer: Buffer<Chunk> = Buffer::new(1);
                let mut post_buffer: Buffer<Chunk> = Buffer::new(1);
                let mut current: Option<Chunk> = None;
                while let Ok(chunk) = rx2.recv() {
                    let mut chunk = pre_buffer.shift(Some(chunk));
                    (chunk, current) = (current, chunk);
                    chunk = post_buffer.shift(chunk);
                    if let Some(_) = current {
                        let mut container = Container {
                            pre_buffer: &pre_buffer,
                            current: &mut current,
                            post_buffer: &post_buffer,
                        };
                        filter.run(&mut container);
                    }
                    if let Some(chunk) = chunk {
                        sx.send(chunk).expect("reciever to live longer than sender");
                    }
                }
                while let Some(chunk) = pre_buffer.drain() {
                    let mut chunk = Some(chunk);
                    (chunk, current) = (current, chunk);
                    chunk = post_buffer.shift(chunk);
                    let mut container = Container {
                        pre_buffer: &pre_buffer,
                        current: &mut current,
                        post_buffer: &post_buffer,
                    };

                    if let Some(chunk) = chunk {
                        sx.send(chunk).expect("reciever to live longer than sender");
                    }
                }
                if let Some(chunk) = post_buffer.shift(current) {
                    sx.send(chunk).expect("reciever to live longer than sender");
                }
                while let Some(chunk) = post_buffer.drain() {
                    sx.send(chunk).expect("reciever to live longer than sender");
                }
            });
        }
        // Create a thread for the writer
        let handle = thread::spawn(move || {
            while let Ok(chunk) = rx.recv() {
                //println!("recieved chunk!");
                self.writer.write_chunk(chunk);
                thread::sleep(Duration::from_millis(0));
            }
        });
        handle.join().expect("oef");
    }
}
