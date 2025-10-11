//! The parser phase translates 

use crate::tokenizer::ExpressionToken;

pub enum Tree {
    Empty,
    Number(f32),
    Add(Box<Tree>, Box<Tree>),
    Subtract(Box<Tree>, Box<Tree>),
    Multiply(Box<Tree>, Box<Tree>),
    Divide(Box<Tree>, Box<Tree>),
    Power(Box<Tree>, Box<Tree>),
    Square(Box<Tree>),
    Cube(Box<Tree>),
}

impl Tree {
    /// The lower the number is the lower the order of the operation, as in, it
    /// should be performed last.
    fn get_order_of_operation(op: &ExpressionToken) -> Option<usize> {
        match op {
            ExpressionToken::Plus => Some(0),
            ExpressionToken::Minus => Some(0),
            ExpressionToken::Star => Some(1),
            ExpressionToken::Slash => Some(1),
            ExpressionToken::Caret => Some(2),
            _ => None,
        }
    }

    /// If there are operations in the tokens vector, find the last-est order
    /// operation in that vector.
    ///
    /// The tuple returned is made of the last order operation, and its token
    /// index.
    ///
    /// If some index is returned, it is **guaranteed** to be that of some kind of operation.
    fn find_last_order_operation(
        tokens: &Vec<ExpressionToken>,
    ) -> Option<(ExpressionToken, usize)> {
        let mut best: Option<(&ExpressionToken, usize)> = None;

        // Used to ignore stuff within brackets, as we rely on external logic to
        // expand them the moment the tokens vector is entirely within brackets.
        // E.g. '(', '123', '+', ')'
        // This is done to give absolute order of operations to the brackets.
        let mut bracket_depth: usize = 0;

        for i in 0..tokens.len() {
            match &tokens[i] {
                op @ (ExpressionToken::Plus
                | ExpressionToken::Caret
                | ExpressionToken::Slash
                | ExpressionToken::Star
                | ExpressionToken::Minus) => {
                    // Ignore as long as inside of brackets
                    if bracket_depth != 0 {
                        continue;
                    }

                    if let Some((ref best_op, _)) = best {
                        let best_op_order = Self::get_order_of_operation(best_op).unwrap();
                        let op_order = Self::get_order_of_operation(op).unwrap();
                        if best_op_order >= op_order {
                            best = Some((op, i));
                        }
                    } else {
                        best = Some((op, i));
                    }
                }
                ExpressionToken::OpenBracket => {
                    bracket_depth += 1;
                }
                ExpressionToken::CloseBracket => {
                    bracket_depth = bracket_depth.checked_sub(1).unwrap_or_default();
                }
                _ => {}
            }
        }

        best.map(|(op, index)| (op.clone(), index))
    }

    fn detokenize(mut tokens: Vec<ExpressionToken>) -> Tree {
        if tokens[0] == ExpressionToken::OpenBracket
            && tokens[tokens.len() - 1] == ExpressionToken::CloseBracket
        {
            _ = tokens.remove(0);
            _ = tokens.pop();
        }

        // If there is an operation found it means that we split it at that
        if let Some((op, op_i)) = Self::find_last_order_operation(&tokens) {
            let right_tokens = tokens.split_off(op_i + 1);

            // We then pop the last token, because it's the operation itself.
            // We only want the rest of the expression.
            _ = tokens.pop().expect("Expected there to be an operation!");

            let left_detokenized = Box::new(Self::detokenize(tokens));
            let right_detokenized = Box::new(Self::detokenize(right_tokens));

            match op {
                ExpressionToken::Plus => {
                    return Tree::Add(left_detokenized, right_detokenized);
                }
                ExpressionToken::Minus => {
                    return Tree::Subtract(left_detokenized, right_detokenized);
                }
                ExpressionToken::Star => {
                    return Tree::Multiply(left_detokenized, right_detokenized);
                }
                ExpressionToken::Slash => {
                    return Tree::Divide(left_detokenized, right_detokenized);
                }
                ExpressionToken::Caret => {
                    return Tree::Power(left_detokenized, right_detokenized);
                }
                _ => unreachable!(
                    "An invalid operation token was returned, either forgot to match it or invalid."
                ),
            }
        } else if tokens.len() == 1 {
            match tokens[0] {
                ExpressionToken::Number(x) => {
                    return Tree::Number(x);
                }
                _ => todo!("Not supported yet."),
            }
        } else {
            panic!("The tokens were more than 1 in length, but they shoudln't.");
        }
    }

    pub fn load_tokens(&mut self, tokens: Vec<ExpressionToken>) {
        *self = Self::detokenize(tokens);
    }

}

