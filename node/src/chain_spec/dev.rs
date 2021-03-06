use crate::chain_spec::{
    authority_keys_from_seed, get_account_id_from_seed, ChainSpec, GenesisConfigBuilder,
};
use sc_service::ChainType;
use vln_commons::runtime::AccountId;
use vln_runtime::WASM_BINARY;

pub fn chain_spec() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "WASM binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || {
            GenesisConfigBuilder {
                initial_authorities: &[authority_keys_from_seed("Alice")],
                sudo_key: get_account_id_from_seed::<AccountId>("Alice"),
                wasm_binary,
            }
            .build()
        },
        vec![],
        None,
        None,
        None,
        None,
    ))
}
