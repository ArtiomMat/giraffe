use super::*;

enum Tree {
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

pub struct TreeEvaluator(Tree);

impl TreeEvaluator {
    pub fn new() -> Self {
        Self(Tree::Empty)
    }

    fn evaluate_tree(tree: &Tree) -> f32 {
        match tree {
            Tree::Empty => 0.0,
            Tree::Number(num) => *num,
            Tree::Add(a, b) => Self::evaluate_tree(a) + Self::evaluate_tree(b),
            Tree::Subtract(a, b) => Self::evaluate_tree(a) - Self::evaluate_tree(b),
            Tree::Multiply(a, b) => Self::evaluate_tree(a) * Self::evaluate_tree(b),
            Tree::Divide(a, b) => Self::evaluate_tree(a) / Self::evaluate_tree(b),
            Tree::Power(a, b) => Self::evaluate_tree(a).powf(Self::evaluate_tree(b)),
            Tree::Square(x) => {
                let x_evaluated = Self::evaluate_tree(x);
                x_evaluated * x_evaluated
            }
            Tree::Cube(x) => {
                let x_evaluated = Self::evaluate_tree(x);
                x_evaluated * x_evaluated * x_evaluated
            }
        }
    }

    /// The lower the number is the lower the order of the operation, as in, it
    /// should be performed last.
    fn get_order_of_operation(o: Operation) -> usize {
        match o {
            Operation::Add => 0,
            Operation::Subtract => 0,
            Operation::Multiply => 1,
            Operation::Divide => 1,
            Operation::Power => 2,
            Operation::Root => 2,
        }
    }

    /// If there are operations in the tokens vector, find the last-est order
    /// operation in that vector.
    ///
    /// The tuple returned is made of the last order operation, and its token
    /// index.
    ///
    /// If some index is returned, it is **guaranteed** to be that of an
    /// `ExpressionToken::Operation`.
    fn find_last_order_operation(tokens: &Vec<ExpressionToken>) -> Option<(Operation, usize)> {
        let mut best: Option<(Operation, usize)> = None;

        // Used to ignore stuff within brackets, as we rely on external logic to
        // expand them the moment the tokens vector is entirely within brackets.
        // E.g. '(', '123', '+', ')'
        // This is done to give absolute order of operations to the brackets.
        let mut bracket_depth: usize = 0;

        for i in 0..tokens.len() {
            match tokens[i] {
                ExpressionToken::Operation(op) => {
                    // Ignore as long as inside of brackets
                    if bracket_depth != 0 {
                        continue;
                    }

                    if let Some((best_op, _)) = best {
                        if Self::get_order_of_operation(best_op) >= Self::get_order_of_operation(op)
                        {
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

        best
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
                Operation::Add => {
                    return Tree::Add(left_detokenized, right_detokenized);
                }
                Operation::Subtract => {
                    return Tree::Subtract(left_detokenized, right_detokenized);
                }
                Operation::Multiply => {
                    return Tree::Multiply(left_detokenized, right_detokenized);
                }
                Operation::Divide => {
                    return Tree::Divide(left_detokenized, right_detokenized);
                }
                Operation::Power => {
                    return Tree::Power(left_detokenized, right_detokenized);
                }
                Operation::Root => todo!("Didn't implement Root support yet"),
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
}

impl ExpressionEvaluator for TreeEvaluator {
    fn load_tokens(&mut self, tokens_arr: &[ExpressionToken]) {
        let tokens = tokens_arr.to_vec();

        self.0 = Self::detokenize(tokens)
    }

    fn evaluate(&mut self, variables: &[(&str, f32)]) -> f32 {
        let _ = variables;
        Self::evaluate_tree(&self.0)
    }
}
