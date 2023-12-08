use std::str::FromStr;

pub fn read<T: FromStr>(value: &str) -> Result<T, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|e| format!("Failed to read {value}: {e}"))
}


pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (0, b) => b,
        (a, 0) => a,
        (a, b) if a < b => gcd(a, b.rem_euclid(a)),
        (a, b) if b < a => gcd(b, a.rem_euclid(b)),
        _ => panic!("never finished gcd"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lcm_test() {
        assert_eq!(15, lcm(3, 5))
    }

    #[test]
    fn gcd_test() {
        assert_eq!(6, gcd(48, 18))
    }
}