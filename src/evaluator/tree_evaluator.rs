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

    fn find_last_order_operation(tokens: &Vec<ExpressionToken>) -> Option<usize> {
        let mut best_i: Option<usize> = None;

        for i in 0..tokens.len() {
            match tokens[i] {
                ExpressionToken::Operation(operation) => {
                    if let Some(some_best_i) = best_i {
                        let best_operation =
                            if let ExpressionToken::Operation(operation) = &tokens[some_best_i] {
                                operation
                            } else {
                                unreachable!()
                            };

                        if Self::get_order_of_operation(*best_operation)
                            >= Self::get_order_of_operation(operation)
                        {
                            best_i = Some(i);
                        }
                    } else {
                        best_i = Some(i);
                    }
                }
                _ => {}
            }
        }

        best_i
    }

    fn detokenize(mut tokens: Vec<ExpressionToken>) -> Tree {
        // If there is an operation found it means that we split it at that
        if let Some(operation_i) = Self::find_last_order_operation(&tokens) {
            let right_tokens = tokens.split_off(operation_i + 1);
            let operation = tokens.pop().expect("Expected there to be an operation!");
            let left_detokenized = Box::new(Self::detokenize(tokens));
            let right_detokenized = Box::new(Self::detokenize(right_tokens));
            match operation {
                ExpressionToken::Operation(operation) => match operation {
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
                },
                _ => {
                    unreachable!(
                        "The token returned by find_last_order_operation, if any, MUST be an operation"
                    );
                }
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
