use evaluator::{ExpressionEvaluator, ExpressionToken, TreeEvaluator};

mod evaluator;

fn main() {
    let mut eval = TreeEvaluator::new();
    
    // 1.67 + 3.66 / 2 / 3.66 * 2 + 2 - 3
    // = 1.67
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

    // 3 * (1.67 + 3.66) / 2 / 3.66 * 2 + 2 - 3
    // = 5.7377043
    let tokens = [
        ExpressionToken::Number(3.0),
        ExpressionToken::Operation(evaluator::Operation::Multiply),
        ExpressionToken::OpenBracket,
        ExpressionToken::Number(1.67),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(3.66),
        ExpressionToken::CloseBracket,
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Divide),
        ExpressionToken::Number(3.66),
        ExpressionToken::Operation(evaluator::Operation::Multiply),
        ExpressionToken::OpenBracket,
        ExpressionToken::Number(2.0),
        ExpressionToken::Operation(evaluator::Operation::Add),
        ExpressionToken::Number(2.0),
        ExpressionToken::CloseBracket,
        ExpressionToken::Operation(evaluator::Operation::Subtract),
        ExpressionToken::Number(3.0),
    ];
    
    eval.load_tokens(&tokens);
    println!("{}", eval.evaluate(&[]));
}
