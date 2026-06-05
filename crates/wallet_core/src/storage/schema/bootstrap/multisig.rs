pub(super) const SQL: &str = r#"
create table if not exists multisig_accounts (
    id text primary key,
    label text not null,
    chain text not null,
    kind text not null,
    address text not null,
    threshold integer not null,
    permission_id integer,
    created_at text not null,
    updated_at text not null,
    unique(chain, address, kind)
);

create table if not exists multisig_owners (
    id text primary key,
    multisig_account_id text not null,
    address text not null,
    weight integer not null,
    created_at text not null,
    unique(multisig_account_id, address),
    foreign key (multisig_account_id) references multisig_accounts(id)
);

create table if not exists multisig_proposals (
    id text primary key,
    multisig_account_id text not null,
    chain text not null,
    to_address text not null,
    asset_symbol text not null,
    amount text not null,
    payload_json text not null,
    status text not null,
    threshold integer not null,
    signature_weight integer not null,
    created_at text not null,
    updated_at text not null,
    foreign key (multisig_account_id) references multisig_accounts(id)
);

create table if not exists multisig_signatures (
    id text primary key,
    proposal_id text not null,
    owner_address text not null,
    signature text not null,
    weight integer not null,
    created_at text not null,
    unique(proposal_id, owner_address),
    foreign key (proposal_id) references multisig_proposals(id)
);
"#;
