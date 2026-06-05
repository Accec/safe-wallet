use chrono::{DateTime, Utc};
use uuid::Uuid;

pub(super) fn parse_uuid_sql(value: &str, index: usize) -> rusqlite::Result<Uuid> {
    Uuid::parse_str(value).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Text,
            Box::new(error),
        )
    })
}

pub(super) fn parse_datetime_sql(value: &str, index: usize) -> rusqlite::Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .map_err(|error| {
            rusqlite::Error::FromSqlConversionFailure(
                index,
                rusqlite::types::Type::Text,
                Box::new(error),
            )
        })
        .map(|datetime| datetime.with_timezone(&Utc))
}
