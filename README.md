# Giraffe

A graphing utility.

# FIXME

- [ ] If there is a `(` token and `)` in start and end respectively, no matter what, it removes them.
This can cause stuff like `(1+1)+(1+1)` to effectively become `1+1)+(1+1` which would destroy parsing.

# TODO

- [x] Split the evaluator into a parser + evaluator. The parser would have the tree, the evaluators 
will just have different methods of evaluation for trees.
- [ ] Add optimization step to parser or evaluator.
- [ ] make it a CLI.
- [ ] Add tokenizer.
- [ ] Add more operations.
    - [ ] Add roots.
    - [ ] Add derivation.
    - [ ] Add integration.
- [ ] Add variable support.
- [ ] Add function support.
- [ ] Add actual graphing support, make it generic, into png, jpeg, live window, etc.
- [ ] Figure out how to add actual userul analysis tooling.
