mod simplifier;
mod tokenizer;

#[cfg(test)]
mod tests {
    use crate::simplifier::Simplifier;
    use crate::tokenizer::Tokenizer;

    #[test]
    fn simplify_test() {
        println!(
            "{:?}",
            Simplifier::simplify(Tokenizer::tokenize(
                r#"
                Add(
                    Num(1),
                    Mul(
                        Num(2),
                        Num(3)
                    )
                )"#
            ))
        );
    }
}
