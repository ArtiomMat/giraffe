use super::*;

use crate::parser::Tree;

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
}

impl Evaluator for TreeEvaluator {
    fn load_tree(&mut self, tree: Tree) {
        self.0 = tree;
    }

    fn evaluate(&mut self, variables: &[(&str, f32)]) -> f32 {
        let _ = variables;
        Self::evaluate_tree(&self.0)
    }
}
