#[allow(dead_code)]
fn descending_order(x: u64) -> u64 {
    let mut result = x
        .to_string()
        .split("")
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    result.sort_by(|a, b: &u64| b.cmp(a));

    return result
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
}

#[test]
fn returns_expected() {
    assert_eq!(descending_order(0), 0);
    assert_eq!(descending_order(1), 1);
    assert_eq!(descending_order(15), 51);
    assert_eq!(descending_order(1021), 2110);
    assert_eq!(descending_order(123456789), 987654321);
    assert_eq!(descending_order(145263), 654321);
    assert_eq!(descending_order(1254859723), 9875543221);
}
