pub use tree_evaluator::*;

pub mod tree_evaluator;

/// The operations are ordered from last to operate to first to operate
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum ExpressionToken {
    Number(f32),
    Variable(String),
    Operation(Operation),
}

pub trait ExpressionEvaluator {
    fn load_tokens(&mut self, tokens: &[ExpressionToken]);
    fn evaluate(&mut self, variables: &[(&str, f32)]) -> f32;
}
