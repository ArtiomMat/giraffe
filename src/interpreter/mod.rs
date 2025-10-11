//! All about turning anything from `1+1` to
//! `f(x, z) = { x < 2: x * z; z - x } + g(z!)^2` into internal representations
//! that can be evaluated and actually plotted on a graph.

pub mod evaluators;
pub mod parser;
pub mod lexer;
