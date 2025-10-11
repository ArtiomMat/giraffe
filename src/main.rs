use evaluator::{Evaluator, TreeEvaluator};
use tokenizer::ExpressionToken;

mod evaluator;
mod parser;
mod tokenizer;

fn main() {
    let mut tree = parser::Tree::Empty;
    let mut eval = TreeEvaluator::new();

    // 1.67 + 3.66 / 2 / 3.66 * 2 + 2 - 3
    // = 1.67
    let tokens = [
        ExpressionToken::Number(1.67),
        ExpressionToken::Plus,
        ExpressionToken::Number(3.66),
        ExpressionToken::Slash,
        ExpressionToken::Number(2.0),
        ExpressionToken::Slash,
        ExpressionToken::Number(3.66),
        ExpressionToken::Star,
        ExpressionToken::Number(2.0),
        ExpressionToken::Plus,
        ExpressionToken::Number(2.0),
        ExpressionToken::Minus,
        ExpressionToken::Number(3.0),
    ];

    tree.load_tokens(tokens.to_vec());
    eval.load_tree(tree);
    println!(
        "\n1.67 + 3.66 / 2 / 3.66 * 2 + 2 - 3\n{}\n",
        eval.evaluate(&[])
    );

    // 3 * (1.67 + 3.66) / 2 / 3.66 * 2 + 2 - 3
    // = 5.7377043
    let tokens = [
        ExpressionToken::Number(3.0),
        ExpressionToken::Star,
        ExpressionToken::OpenBracket,
        ExpressionToken::Number(1.67),
        ExpressionToken::Plus,
        ExpressionToken::Number(3.66),
        ExpressionToken::CloseBracket,
        ExpressionToken::Slash,
        ExpressionToken::Number(2.0),
        ExpressionToken::Slash,
        ExpressionToken::Number(3.66),
        ExpressionToken::Star,
        ExpressionToken::OpenBracket,
        ExpressionToken::Number(2.0),
        ExpressionToken::Plus,
        ExpressionToken::Number(2.0),
        ExpressionToken::CloseBracket,
        ExpressionToken::Minus,
        ExpressionToken::Number(3.0),
    ];

    let mut tree = parser::Tree::Empty;
    tree.load_tokens(tokens.to_vec());
    eval.load_tree(tree);
    println!(
        "3 * (1.67 + 3.66) / 2 / 3.66 * 2 + 2 - 3\n{}\n",
        eval.evaluate(&[])
    );
}
