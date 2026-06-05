pub(super) const SQL: &str = r#"
create table if not exists chain_settings (
    chain text primary key,
    network_name text not null default '',
    chain_id text,
    enabled integer not null,
    default_rpc_url text not null,
    user_rpc_url text,
    explorer_url text,
    native_symbol text not null,
    native_decimals integer not null,
    updated_at text not null
);

create table if not exists indexer_settings (
    id text primary key,
    chain text not null,
    provider text not null,
    endpoint text not null,
    encrypted_api_key text,
    enabled integer not null,
    last_sync_at text,
    updated_at text not null
);

create table if not exists network_privacy_settings (
    id integer primary key check (id = 1),
    proxy_enabled integer not null,
    proxy_mode text not null,
    proxy_url text,
    updated_at text not null
);
"#;
