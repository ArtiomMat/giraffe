use evaluators::{Evaluator, TreeEvaluator};
use lexer::Token;

mod evaluators;
mod parser;
mod lexer;

fn main() {
    let mut tree = parser::Tree::Empty;
    let mut eval = TreeEvaluator::new();

    // 1.67 + 3.66 / 2 / 3.66 * 2 + 2 - 3
    // = 1.67
    let tokens = [
        Token::Number(1.67),
        Token::Plus,
        Token::Number(3.66),
        Token::Slash,
        Token::Number(2.0),
        Token::Slash,
        Token::Number(3.66),
        Token::Star,
        Token::Number(2.0),
        Token::Plus,
        Token::Number(2.0),
        Token::Minus,
        Token::Number(3.0),
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
        Token::Number(3.0),
        Token::Star,
        Token::OpenBracket,
        Token::Number(1.67),
        Token::Plus,
        Token::Number(3.66),
        Token::CloseBracket,
        Token::Slash,
        Token::Number(2.0),
        Token::Slash,
        Token::Number(3.66),
        Token::Star,
        Token::OpenBracket,
        Token::Number(2.0),
        Token::Plus,
        Token::Number(2.0),
        Token::CloseBracket,
        Token::Minus,
        Token::Number(3.0),
    ];

    let mut tree = parser::Tree::Empty;
    tree.load_tokens(tokens.to_vec());
    eval.load_tree(tree);
    println!(
        "3 * (1.67 + 3.66) / 2 / 3.66 * 2 + 2 - 3\n{}\n",
        eval.evaluate(&[])
    );
}
