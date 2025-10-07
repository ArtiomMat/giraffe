use std::collections::LinkedList;

use evaluator::tree_evaluator::TreeEvaluator;

use crate::evaluator::{ExpressionEvaluator, ExpressionToken};

mod evaluator;

fn main() {
    let mut eval = TreeEvaluator::new();
    // 1.67 + 3.66 / 2 / 3.66 * 2 + 2 - 3 = 1.67
    let tokens = [
        ExpressionToken::Number(1.67),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(3.66),
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(3.66),
        ExpressionToken::Operation(evaluator::Operation::Multiply),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Subtract),
        ExpressionToken::Number(3.0),
    ];
    
    eval.load_tokens(&tokens);
    println!("{}", eval.evaluate(&[]));
}
