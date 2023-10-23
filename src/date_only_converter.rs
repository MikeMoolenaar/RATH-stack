pub mod date_only_to_timestamp {
    use chrono::{NaiveDate, NaiveDateTime};
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: String = NaiveDateTime::from_timestamp_opt(*date, 0)
            .unwrap()
            .format(FORMAT)
            .to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let ts = dt.and_hms_opt(0, 0, 0).unwrap().timestamp();
        Ok(ts)
    }
}
