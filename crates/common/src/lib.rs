/// Normalze name
/// See PEP 503 and 508
pub fn normalize_name(name: &str) -> String {
    let regex = regex::Regex::new(r"[-_.]+").expect("a valid regex expression");
    regex.replace_all(&name.to_lowercase(), "-").to_string()
}

#[cfg(test)]
mod tests {
    use super::normalize_name;

    #[test]
    fn normalizes_case_and_separators() {
        let cases = [
            ("Hello-World_test.file", "hello-world-test-file"),
            ("one--two__three..four", "one-two-three-four"),
        ];

        for (input, expected) in cases {
            assert_eq!(normalize_name(input), expected);
        }
    }

    #[test]
    fn does_not_normalize_undocumented_separators() {
        assert_eq!(normalize_name("Hello World"), "hello world");
    }
}
