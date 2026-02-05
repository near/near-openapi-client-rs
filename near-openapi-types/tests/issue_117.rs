//! Test for issue #117: RpcTransactionResponse should deserialize NONE finality responses

use near_openapi_types::{RpcTransactionResponse, TxExecutionStatus};

#[test]
fn test_deserialize_none_finality() {
    // The problematic JSON that was failing before the fix
    let json = r#"{"final_execution_status":"NONE"}"#;
    let result: RpcTransactionResponse = serde_json::from_str(json).expect("Should deserialize");

    // subtype_0 is the variant with just final_execution_status
    assert!(result.subtype_0.is_some());
    assert_eq!(
        result.subtype_0.unwrap().final_execution_status,
        TxExecutionStatus::None
    );
    // Other subtypes should be None
    assert!(result.subtype_1.is_none());
    assert!(result.subtype_2.is_none());
}

#[test]
fn test_deserialize_included_finality() {
    // INCLUDED finality - still might not have execution outcome
    let json = r#"{"final_execution_status":"INCLUDED"}"#;
    let result: RpcTransactionResponse = serde_json::from_str(json).expect("Should deserialize");

    assert!(result.subtype_0.is_some());
    assert_eq!(
        result.subtype_0.unwrap().final_execution_status,
        TxExecutionStatus::Included
    );
}
