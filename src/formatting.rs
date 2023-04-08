pub fn get_formatted_argname_argtype_pairs(
    argnames: &Vec<String>,
    argtypes: &Vec<String>,
) -> String {
    let mut args = "".to_string();
    for a in 0..argnames.len() {
        let comma = if a + 1 == argnames.len() {
            "".to_string()
        } else {
            ", ".to_string()
        };
        args = format!("{}{}: {}{}", args, argnames[a], argtypes[a], comma);
    }
    args
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_formatted_argname_argtype_pairs() {
        let test_case_passes: Vec<Vec<Vec<String>>> = vec![
            vec![
                vec!["arg1".to_string()],
                vec!["i64".to_string()],
                vec!["arg1: i64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string()],
                vec!["i64".to_string(), "f64".to_string()],
                vec!["arg1: i64, arg2: f64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
                vec!["i64".to_string(), "f64".to_string(), "String".to_string()],
                vec!["arg1: i64, arg2: f64, arg3: String".to_string()],
            ],
        ];
        for test in test_case_passes {
            let argnames = &test[0];
            let argtypes = &test[1];
            let output = &test[2][0];
            assert_eq!(
                &get_formatted_argname_argtype_pairs(&argnames, &argtypes),
                output
            );
        }
    }
}
