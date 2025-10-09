pub use tree_evaluator::*;

pub mod tree_evaluator;

use crate::tokenizer::ExpressionToken;

pub trait ExpressionEvaluator {
    fn load_tokens(&mut self, tokens: &[ExpressionToken]);
    fn evaluate(&mut self, variables: &[(&str, f32)]) -> f32;
}
