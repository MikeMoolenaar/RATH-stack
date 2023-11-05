pub mod filters {
    use chrono::NaiveDateTime;

    pub fn date_string(timestamp: String) -> String {
        let timestamp = timestamp.parse::<i64>().expect("timestamp to be a avalid integer");
        let date_formatted: String = NaiveDateTime::from_timestamp_opt(timestamp, 0)
            .unwrap()
            .format("%d-%m-%Y")
            .to_string();
        return date_formatted;
    }
}
