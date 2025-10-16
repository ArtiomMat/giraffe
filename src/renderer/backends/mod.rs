//! A rendering backend exposes simple functions for the renderer to draw them on.

use std::ops::Range;

pub mod png;
pub mod gpu;

#[derive(Clone, Copy, PartialEq)]
pub struct Point([f32; 2]);

#[derive(Clone, PartialEq)]
pub struct Line(Range<Point>);

/// RGBA format
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color([u8; 4]);
pub struct Label {
    pub text: String,
}

pub enum FgStyle {
    Dotted,
    Dashed,
    Continuous,
}

pub struct PutProperties {
    pub fg_color: Color,
    pub fg_style: Option<FgStyle>,
    pub fg_thickness: f32,
    // Not always used
    pub bg_color: Color,
}

/// Exposes the absolute fundamental functions and nothing fancy.
/// This is used by the `PngBackend`, `JpegBackend`, `XcbBackend`, etc.
/// And is also supported by all the graphical backends.
pub trait CoreBackend {
    /// If the pixels are of invalid size or format, there is not requirement
    /// of the implemenation to check that. **However**, it must maintain 
    /// safety and protect against overflows or crashes, that's crucial.
    fn load_bitmap(&mut self, pixels: &[Color]);
    fn flush(&mut self);
}

/// Exposes more complex operations that actually aid in drawing graphs.
/// This is used by `GpuBackend` and `SoftwareBackend`.
pub trait GraphicalBackend: CoreBackend {
    fn set_put_properties(&mut self, properties: &PutProperties);
    fn put_fill(&mut self, color: Color);
    fn put_points(&mut self, points: &[Point]);
    fn put_lines(&mut self, lines: &[Line]);
    fn put_labels(&mut self, text: &str, point: Point);
}
