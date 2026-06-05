pub(super) const SQL: &str = r#"
create table if not exists ui_preferences (
    key text primary key,
    value_json text not null,
    updated_at text not null
);
"#;
