use chrono::DateTime;

pub fn date_string(timestamp: String) -> String {
    let timestamp = timestamp.parse::<i64>().expect("timestamp to be a avalid integer");
    let date_formatted: String = DateTime::from_timestamp(timestamp, 0)
        .unwrap()
        .format("%d-%m-%Y")
        .to_string();
    return date_formatted;
}
