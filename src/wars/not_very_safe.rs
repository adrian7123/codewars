#[cfg(test)]
mod tests {
    fn alphanumeric(password: &str) -> bool {
        password.to_string().is_al
    }

    fn do_test(s: &str, expected: bool) {
        let actual = alphanumeric(s);
        assert_eq!(
            actual, expected,
            "\nInput: {s:?}\nYour result (left) did not match the expected output (right)"
        )
    }

    #[test]
    fn sample_tests() {
        do_test("hello world_", false);
        do_test("PassW0rd", true);
        do_test("     ", false);
        do_test("&)))(((", false);
        do_test("MN5Iqrs?L{j]:kfqC5(Dk]C@M8", false);
    }
}
