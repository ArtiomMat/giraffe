//! The stack evaluator uses the tree the parser makes and takes it
//! 1 step forward forward, translating it into a set of "actions".
//!
//! The resulting evaluator's behaviour resembles that of a stack
//! reliant program, pushing, operating on, and popping numbers.

enum Action {
    Push(f32),

    // The simple algebraic instructions pop the `.0` elements
    // and operate on them, pushing the result back.
    Add(usize),
    Subtract(usize),
    Multiply(usize),
    Divide(usize),
}

struct StackEvaluator {
    actions: Vec<Action>,
}

impl ExpressionEvaluator for StackEvaluator {
    fn load_tokens(tokens_arr: &[ExpressionToken]) {
        let mut tokens = LinkedList::from_iter(tokens_arr.iter());

        for token in tokens {}
    }

    fn evaluate(variables: &[(&str, f32)]) -> f32 {
        todo!()
    }
}
