pub(super) const SQL: &str = r#"
create table if not exists keystore_items (
    id text primary key,
    secret_kind text not null default 'mnemonic',
    ciphertext text not null,
    nonce text not null,
    salt text not null,
    kdf_name text not null,
    kdf_params_json text not null,
    cipher_name text not null,
    version integer not null,
    created_at text not null,
    updated_at text not null
);

create table if not exists wallets (
    id text primary key,
    label text not null,
    keystore_id text not null unique,
    hidden integer not null default 0,
    created_at text not null,
    updated_at text not null,
    foreign key (keystore_id) references keystore_items(id)
);

create table if not exists accounts (
    id text primary key,
    wallet_id text not null,
    chain text not null,
    address text not null,
    derivation_path text not null,
    account_index integer not null,
    created_at text not null,
    unique(id, wallet_id),
    unique(wallet_id, chain, account_index),
    foreign key (wallet_id) references wallets(id)
);
"#;
