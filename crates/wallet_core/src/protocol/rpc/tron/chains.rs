use crate::models::ChainId;

pub(super) fn supports_chain(chain: ChainId) -> bool {
    chain == ChainId::Tron
}
