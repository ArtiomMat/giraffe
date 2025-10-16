use std::io;
use super::CoreBackend;
use image::codecs::png::{CompressionType, PngDecoder, PngEncoder};

pub struct PngBackend<W>
where
    W: io::Write,
{
    width: usize,
    height: usize,
    encoder: PngEncoder<W>,
}

impl<W> PngBackend<W>
where
    W: io::Write,
{
    pub fn new(width: usize, height: usize, w: W) -> Self {
        Self {
            width,
            height,
            encoder: PngEncoder::new(w)
        }
    }
}

impl<W> CoreBackend for PngBackend<W>
where 
    W: io::Write
{
    fn load_bitmap(&mut self, pixels: &[super::Color]) {
        
    }

    fn flush(&mut self) {
        todo!()
    }
}

