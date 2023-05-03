/*! Only contains get_formatted_argname_argtype_pairs
 */

/// Formats three vecs into a single string usually for inserting into the arguments of a function
///
/// # Example
/// ```
/// let argnames = vec!["arg1".to_string(), "arg2".to_string()];
/// let argtypes = vec!["i64".to_string(), "String".to_string()];
/// let argmodifiers = vec!["&".to_string(), "".to_string()];
/// let output = toylang::formatting::get_formatted_argname_argtype_pairs(&argnames, &argtypes, &argmodifiers);
/// // output = "arg1: &i64, arg2: String"
/// // which can be used in formatting the string of a function call, e.g.
/// println!("test_function({});", output);
/// // test_function(arg1: &i64, arg2: String);
/// ```
pub fn get_formatted_argname_argtype_pairs(
    argnames: &Vec<String>,
    argtypes: &Vec<String>,
    argmodifiers: &Vec<String>,
) -> String {
    let mut args = "".to_string();
    for a in 0..argnames.len() {
        let comma = if a + 1 == argnames.len() {
            "".to_string()
        } else {
            ", ".to_string()
        };
        args = format!(
            "{}{}: {}{}{}",
            args, argnames[a], argmodifiers[a], argtypes[a], comma
        );
    }
    args
}

pub fn get_formatted_argnames(argnames: &Vec<String>) -> String {
    let mut args = "".to_string();
    for a in 0..argnames.len() {
        let comma = if a + 1 == argnames.len() {
            "".to_string()
        } else {
            ", ".to_string()
        };
        args = format!("{}{}{}", args, argnames[a], comma);
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
                vec!["".to_string()],
                vec!["arg1: i64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string()],
                vec!["i64".to_string(), "f64".to_string()],
                vec!["".to_string(), "".to_string()],
                vec!["arg1: i64, arg2: f64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
                vec!["i64".to_string(), "f64".to_string(), "String".to_string()],
                vec!["".to_string(), "".to_string(), "".to_string()],
                vec!["arg1: i64, arg2: f64, arg3: String".to_string()],
            ],
            vec![
                vec!["arg1".to_string()],
                vec!["i64".to_string()],
                vec!["&".to_string()],
                vec!["arg1: &i64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string()],
                vec!["i64".to_string(), "f64".to_string()],
                vec!["&".to_string(), "&mut ".to_string()],
                vec!["arg1: &i64, arg2: &mut f64".to_string()],
            ],
            vec![
                vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()],
                vec!["i64".to_string(), "f64".to_string(), "String".to_string()],
                vec!["&".to_string(), "&mut ".to_string(), "mut ".to_string()],
                vec!["arg1: &i64, arg2: &mut f64, arg3: mut String".to_string()],
            ],
        ];
        for test in test_case_passes {
            let argnames = &test[0];
            let argtypes = &test[1];
            let argmodifiers = &test[2];
            let output = &test[3][0];
            assert_eq!(
                &get_formatted_argname_argtype_pairs(&argnames, &argtypes, &argmodifiers),
                output
            );
        }
    }
}
