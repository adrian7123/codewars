#[cfg(test)]
mod sample_tests {
    macro_rules! compare {
        ( $got : expr, $expected : expr ) => {
            if $got != $expected {
                panic!("Got: {}\nExpected: {}\n", $got, $expected);
            }
        };
    }

    fn rgb(r: i32, g: i32, b: i32) -> String {
        format!(
            "{:02X}{:02X}{:02X}",
            r.clamp(0, 255),
            g.clamp(0, 255),
            b.clamp(0, 255)
        )
    }

    #[test]
    fn tests() {
        compare!(rgb(-20, 275, 125), "00FF7D");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(255, 255, 255), "FFFFFF");
    }
}
