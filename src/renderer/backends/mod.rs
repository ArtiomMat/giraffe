//! A rendering backend exposes simple functions for the renderer to draw them on.

use std::ops::Range;

pub struct Point(f32, f32);

pub trait Backend {
    fn put_lines(lines: &[Range<Point>]);
    /// If the pixels are of invalid size or format, there is not requirement
    /// of the implemenation to check that. **However**, it must maintain 
    /// safety and protect against overflows or crashes, that's crucial.
    fn put_pixels(pixels: &[u8]);
    fn flush();
}
