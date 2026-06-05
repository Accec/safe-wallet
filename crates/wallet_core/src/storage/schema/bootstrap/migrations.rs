pub(super) const SQL: &str = r#"
create table if not exists schema_migrations (
    version integer primary key,
    applied_at text not null
);
"#;
