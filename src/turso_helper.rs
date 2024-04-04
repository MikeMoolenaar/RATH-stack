use libsql::{de, params::IntoParams};

/**
 * Query all rows and convert to Vec<T>
 */
pub async fn fetch_all<T: serde::de::DeserializeOwned>(
    conn: &libsql::Connection,
    sql: &str,
    params: impl IntoParams,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut result = conn.query(sql, params).await?;

    let mut results = Vec::<T>::new();
    while let Ok(Some(row)) = result.next().await {
        results.push(de::from_row::<T>(&row)?);
    }
    return Ok(results);
}

/**
 * Query one row or None and convert to T
 */
pub async fn fetch_optional<T: serde::de::DeserializeOwned>(
    conn: &libsql::Connection,
    sql: &str,
    params: impl IntoParams,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let mut result = conn.query(sql, params).await?;
    if let Ok(Some(row)) = result.next().await {
        return Ok(Some(de::from_row::<T>(&row)?));
    }
    return Ok(None);
}

/**
 * Get result of count query, sql str must return single column,
 * example: `SELECT COUNT(*) FROM table`
 */
pub async fn count(
    conn: &libsql::Connection,
    sql: &str,
    params: impl IntoParams,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut result = conn.query(sql, params).await?;
    if let Ok(Some(row)) = result.next().await {
        return Ok(row.get(0).unwrap_or(0));
    }
    return Ok(0);
}
