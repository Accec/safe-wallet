pub(super) const SQL: &str = r#"
create unique index if not exists tokens_chain_contract_unique
    on tokens(chain, contract_address)
    where contract_address is not null;

create unique index if not exists tokens_native_chain_unique
    on tokens(chain)
    where contract_address is null and kind = 'native';
"#;
