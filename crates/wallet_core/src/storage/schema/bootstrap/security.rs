pub(super) const SQL: &str = r#"
create table if not exists app_security (
    id integer primary key check (id = 1),
    password_salt text not null,
    password_verifier text not null,
    kdf_name text not null,
    kdf_params_json text not null,
    duress_salt text,
    duress_verifier text,
    duress_kdf_name text,
    duress_kdf_params_json text,
    duress_security_version integer,
    biometric_enabled integer not null default 0,
    security_version integer not null,
    created_at text not null,
    updated_at text not null
);
"#;
