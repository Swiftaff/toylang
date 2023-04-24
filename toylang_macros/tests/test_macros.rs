#[cfg(test)]
mod tests {

    use toylang_macros::make_answer;

    #[test]
    fn test_macro() {
        assert_eq!(String::from(make_answer!("testy")), "test".to_string());
    }
}
