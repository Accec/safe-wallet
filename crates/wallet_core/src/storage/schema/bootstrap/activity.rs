pub(super) const SQL: &str = r#"
create table if not exists activities (
    id text primary key,
    wallet_id text not null,
    account_id text not null,
    chain text not null,
    provider_id text,
    tx_hash text not null,
    kind text not null,
    status text not null,
    from_address text,
    to_address text,
    contract_address text,
    asset_symbol text,
    amount text,
    fee text,
    block_height integer,
    happened_at text,
    decoded_summary text,
    safe_provider_ref text,
    unique(chain, tx_hash, kind, account_id),
    foreign key (wallet_id) references wallets(id),
    foreign key (account_id, wallet_id) references accounts(id, wallet_id)
);

create table if not exists transactions (
    id text primary key,
    wallet_id text not null,
    account_id text not null,
    chain text not null,
    asset_id text not null,
    to_address text not null,
    amount text not null,
    fee_estimate text,
    status text not null,
    tx_hash text,
    created_at text not null,
    updated_at text not null,
    foreign key (wallet_id) references wallets(id),
    foreign key (account_id, wallet_id) references accounts(id, wallet_id),
    foreign key (asset_id) references tokens(id)
);
"#;
