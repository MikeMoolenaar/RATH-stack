pub mod filters {
    use chrono::NaiveDateTime;

    pub fn date_string(timestamp: &i64) -> ::askama::Result<String> {
        let date_formatted: String = NaiveDateTime::from_timestamp_opt(*timestamp, 0)
            .unwrap()
            .format("%d-%m-%Y")
            .to_string();
        Ok(date_formatted)
    }
}
