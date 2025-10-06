use std::collections::LinkedList;

use evaluator::tree_evaluator::TreeEvaluator;

use crate::evaluator::{ExpressionEvaluator, ExpressionToken};

mod evaluator;

fn main() {
    let mut eval = TreeEvaluator::new();
    // 1 + 3 / 2 / 2 + 2
    let tokens = [
        ExpressionToken::Number(1.0),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(3.0),
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(2.0),
    ];
    
    eval.load_tokens(&tokens);
    println!("{}", eval.evaluate(&[]));
}
