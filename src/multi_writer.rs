use std::{
    cell::RefCell,
    io::{BufRead, BufReader, Read},
    rc::Rc,
};

use digest::DynDigest;

use super::error::Result;

pub struct MultiWriter<'a> {
    writers: &'a [Rc<RefCell<dyn DynDigest>>],
}

impl<'a> MultiWriter<'a> {
    pub fn new(writers: &'a [Rc<RefCell<dyn DynDigest>>]) -> Self {
        Self { writers }
    }

    pub fn write<R>(&mut self, reader: &mut R) -> Result<usize>
    where
        R: Read,
    {
        let mut bytes_read = 0;
        let mut reader = BufReader::new(reader);

        loop {
            let buffer;

            let length = {
                buffer = reader.fill_buf()?;
                buffer.len()
            };

            if buffer.is_empty() {
                break;
            }

            for writer in self.writers {
                writer.borrow_mut().update(buffer);
            }

            reader.consume(length);
            bytes_read += length;
        }

        Ok(bytes_read)
    }
}
