use std::str::FromStr;

pub fn read<T: FromStr>(value: &str) -> Result<T, String>
where
    <T as FromStr>::Err: std::fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|e| format!("Failed to read {value}: {e}"))
}
