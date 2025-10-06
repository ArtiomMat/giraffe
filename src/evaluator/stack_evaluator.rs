
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
