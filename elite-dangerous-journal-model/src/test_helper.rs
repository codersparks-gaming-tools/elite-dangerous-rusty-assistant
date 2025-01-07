
#[cfg(test)]
pub(crate) mod serde_helpers {
    use chrono::NaiveDateTime;

    pub fn create_timestamp(timestamp_str: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%dT%H:%M:%SZ").unwrap()
    }

}