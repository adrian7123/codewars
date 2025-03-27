#![allow(dead_code)]
fn ips_between(start: &str, end: &str) -> u32 {
    let startVec = start
        .split('.')
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let endVec = end
        .split('.')
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut result = 0;

    for i in 0..4 {
        result += (endVec[i] - startVec[i]) - 256;
    }

    result
}

#[test]
fn basic() {
    assert_eq!(ips_between("10.0.0.0", "10.0.0.50"), 50);
    assert_eq!(ips_between("20.0.0.10", "20.0.1.0"), 246);
}
