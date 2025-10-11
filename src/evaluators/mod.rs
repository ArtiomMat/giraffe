pub use tree_evaluator::*;

pub mod tree_evaluator;

use crate::{parser, tokenizer::Token};

pub trait Evaluator {
    fn load_tree(&mut self, tree: parser::Tree);
    fn evaluate(&mut self, variables: &[(&str, f32)]) -> f32;
}
