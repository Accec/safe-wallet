pub(super) const SQL: &str = r#"
create table if not exists tokens (
    id text primary key,
    chain text not null,
    contract_address text,
    kind text not null,
    symbol text not null,
    name text not null,
    decimals integer not null,
    source text not null,
    visible integer not null,
    updated_at text not null
);

create table if not exists asset_balances (
    id text primary key,
    wallet_id text not null,
    account_id text not null,
    asset_id text not null,
    balance text not null,
    block_height integer,
    source text not null,
    refreshed_at text not null,
    unique(wallet_id, account_id, asset_id),
    foreign key (wallet_id) references wallets(id),
    foreign key (account_id, wallet_id) references accounts(id, wallet_id),
    foreign key (asset_id) references tokens(id)
);
"#;
