use near_openapi_client::types;

#[test]
fn test_global_contract_identifier_view_account_id() {
    let json = r#"{"account_id":"mt_receiver_global.sandbox"}"#;

    let result: Result<types::GlobalContractIdentifierView, _> = serde_json::from_str(json);

    assert!(
        result.is_ok(),
        "Failed to deserialize GlobalContractIdentifierView from account_id: {:?}",
        result.err()
    );
}

#[test]
fn test_global_contract_identifier_view_hash() {
    let json = r#"{"hash":"11111111111111111111111111111111"}"#;

    let result: Result<types::GlobalContractIdentifierView, _> = serde_json::from_str(json);

    assert!(
        result.is_ok(),
        "Failed to deserialize GlobalContractIdentifierView from hash: {:?}",
        result.err()
    );
}

#[test]
fn test_deterministic_state_init_action_view() {
    let json = r#"{"DeterministicStateInit":{"code":{"account_id":"mt_receiver_global.sandbox"},"data":{},"deposit":"0"}}"#;

    let result: Result<types::ActionView, _> = serde_json::from_str(json);

    assert!(
        result.is_ok(),
        "Failed to deserialize DeterministicStateInit ActionView: {:?}",
        result.err()
    );
}

#[test]
fn test_deterministic_state_init_action_view_with_hash() {
    let json = r#"{"DeterministicStateInit":{"code":{"hash":"AdFXNfTP8JHKhbekQ1XxSetG7qCciL9CnQaCzm373U6R"},"data":{"":"AQ=="},"deposit":"0"}}"#;

    let result: Result<types::ActionView, _> = serde_json::from_str(json);

    assert!(
        result.is_ok(),
        "Failed to deserialize DeterministicStateInit ActionView with hash: {:?}",
        result.err()
    );
}
