//! This crate provides types for the Near OpenAPI specification.
//!
//! Used in [near-openapi-client](https://docs.rs/near-openapi-client/latest/near_openapi_client/)
pub mod error;
mod util;
pub use near_account_id::AccountId;
pub use near_gas::NearGas;
pub use near_token::NearToken;
pub use util::CryptoHash;
///Access key provides limited access to an account. Each access key
/// belongs to some account and is identified by a unique (within the
/// account) public key. One account may have large number of
/// access keys. Access keys allow to act on behalf of the account by
/// restricting transactions that can be issued.
///`account_id,public_key` is a key in the state
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Access key provides limited access to an account. Each
/// access key belongs to some account and\nis identified by a unique
/// (within the account) public key. One account may have large number
/// of\naccess keys. Access keys allow to act on behalf of the account by
/// restricting transactions\nthat can be issued.\n`account_id,public_key`
/// is a key in the state",
///  "type": "object",
///  "required": [
///    "nonce",
///    "permission"
///  ],
///  "properties": {
///    "nonce": {
///      "description": "Nonce for this access key, used for tx nonce generation. When access key is created, nonce\nis set to `(block_height - 1) * 1e6` to avoid tx hash collision on access key re-creation.\nSee <https://github.com/near/nearcore/issues/3779> for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "permission": {
///      "description": "Defines permissions for this access key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKeyPermission"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKey {
    ///Nonce for this access key, used for tx nonce generation. When access
    /// key is created, nonce is set to `(block_height - 1) * 1e6`
    /// to avoid tx hash collision on access key re-creation. See <https://github.com/near/nearcore/issues/3779> for more details.
    pub nonce: u64,
    ///Defines permissions for this access key.
    pub permission: AccessKeyPermission,
}
///Describes the cost of creating an access key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the cost of creating an access key.",
///  "type": "object",
///  "required": [
///    "full_access_cost",
///    "function_call_cost",
///    "function_call_cost_per_byte"
///  ],
///  "properties": {
///    "full_access_cost": {
///      "description": "Base cost of creating a full access access-key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "function_call_cost": {
///      "description": "Base cost of creating an access-key restricted to
/// specific functions.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "function_call_cost_per_byte": {
///      "description": "Cost per byte of method_names of creating a
/// restricted access-key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyCreationConfigView {
    ///Base cost of creating a full access access-key.
    pub full_access_cost: Fee,
    ///Base cost of creating an access-key restricted to specific
    /// functions.
    pub function_call_cost: Fee,
    ///Cost per byte of method_names of creating a restricted access-key.
    pub function_call_cost_per_byte: Fee,
}
///Describes information about an access key including the public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes information about an access key including the
/// public key.",
///  "type": "object",
///  "required": [
///    "access_key",
///    "public_key"
///  ],
///  "properties": {
///    "access_key": {
///      "$ref": "#/components/schemas/AccessKeyView"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyInfoView {
    pub access_key: AccessKeyView,
    pub public_key: PublicKey,
}
///Lists access keys
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Lists access keys",
///  "type": "object",
///  "required": [
///    "keys"
///  ],
///  "properties": {
///    "keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccessKeyInfoView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyList {
    pub keys: ::std::vec::Vec<AccessKeyInfoView>,
}
///Defines permissions for AccessKey
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Defines permissions for AccessKey",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "$ref": "#/components/schemas/FunctionCallPermission"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Grants full access to the account.\nNOTE: It's used
/// to replace account-level public keys.",
///      "type": "string",
///      "enum": [
///        "FullAccess"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum AccessKeyPermission {
    FunctionCall(FunctionCallPermission),
    ///Grants full access to the account.
    ///NOTE: It's used to replace account-level public keys.
    FullAccess,
}
impl ::std::convert::From<FunctionCallPermission> for AccessKeyPermission {
    fn from(value: FunctionCallPermission) -> Self {
        Self::FunctionCall(value)
    }
}
///Describes the permission scope for an access key. Whether it is a
/// function call or a full access key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the permission scope for an access key.
/// Whether it is a function call or a full access key.",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "FullAccess"
///      ]
///    },
///    {
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "type": "object",
///          "required": [
///            "method_names",
///            "receiver_id"
///          ],
///          "properties": {
///            "allowance": {
///              "anyOf": [
///                {
///                  "$ref": "#/components/schemas/NearToken"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "method_names": {
///              "type": "array",
///              "items": {
///                "type": "string"
///              }
///            },
///            "receiver_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum AccessKeyPermissionView {
    FullAccess,
    FunctionCall {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        allowance: ::std::option::Option<NearToken>,
        method_names: ::std::vec::Vec<::std::string::String>,
        receiver_id: ::std::string::String,
    },
}
///Describes access key permission scope and nonce.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes access key permission scope and nonce.",
///  "type": "object",
///  "required": [
///    "nonce",
///    "permission"
///  ],
///  "properties": {
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "permission": {
///      "$ref": "#/components/schemas/AccessKeyPermissionView"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccessKeyView {
    pub nonce: u64,
    pub permission: AccessKeyPermissionView,
}
///`AccountChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "account_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccountChangesByBlockIdChangesType {
    #[serde(rename = "account_changes")]
    AccountChanges,
}
impl ::std::fmt::Display for AccountChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AccountChanges => f.write_str("account_changes"),
        }
    }
}
impl ::std::str::FromStr for AccountChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account_changes" => Ok(Self::AccountChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccountChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccountChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AccountChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "account_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccountChangesByFinalityChangesType {
    #[serde(rename = "account_changes")]
    AccountChanges,
}
impl ::std::fmt::Display for AccountChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AccountChanges => f.write_str("account_changes"),
        }
    }
}
impl ::std::str::FromStr for AccountChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account_changes" => Ok(Self::AccountChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccountChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccountChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AccountChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "account_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AccountChangesBySyncCheckpointChangesType {
    #[serde(rename = "account_changes")]
    AccountChanges,
}
impl ::std::fmt::Display for AccountChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AccountChanges => f.write_str("account_changes"),
        }
    }
}
impl ::std::str::FromStr for AccountChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account_changes" => Ok(Self::AccountChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccountChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccountChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccountChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The structure describes configuration for creation of new accounts.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The structure describes configuration for creation of
/// new accounts.",
///  "type": "object",
///  "properties": {
///    "min_allowed_top_level_account_length": {
///      "description": "The minimum length of the top-level account ID that
/// is allowed to be created by any account.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "registrar_account_id": {
///      "description": "The account ID of the account registrar. This account ID allowed to create top-level\naccounts of any valid length.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountCreationConfigView {
    ///The minimum length of the top-level account ID that is allowed to be
    /// created by any account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_allowed_top_level_account_length: ::std::option::Option<u8>,
    ///The account ID of the account registrar. This account ID allowed to
    /// create top-level accounts of any valid length.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub registrar_account_id: ::std::option::Option<AccountId>,
}
impl ::std::default::Default for AccountCreationConfigView {
    fn default() -> Self {
        Self {
            min_allowed_top_level_account_length: Default::default(),
            registrar_account_id: Default::default(),
        }
    }
}
///AccountData is a piece of global state that a validator
///signs and broadcasts to the network.
///
///It is essentially the data that a validator wants to share with the
/// network. All the nodes in the network are collecting the account
/// data broadcasted by the validators.
///Since the number of the validators is bounded and their
///identity is known (and the maximal size of allowed AccountData is
/// bounded) the global state that is distributed in the form of
/// AccountData is bounded as well.
///Find more information in the docs [here](https://github.com/near/nearcore/blob/560f7fc8f4b3106e0d5d46050688610b1f104ac6/chain/client/src/client.rs#L2232)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "AccountData is a piece of global state that a validator\nsigns and broadcasts to the network.\n\nIt is essentially the data that a validator wants to share with the network.\nAll the nodes in the network are collecting the account data\nbroadcasted by the validators.\nSince the number of the validators is bounded and their\nidentity is known (and the maximal size of allowed AccountData is bounded)\nthe global state that is distributed in the form of AccountData is bounded\nas well.\nFind more information in the docs [here](https://github.com/near/nearcore/blob/560f7fc8f4b3106e0d5d46050688610b1f104ac6/chain/client/src/client.rs#L2232)",
///  "type": "object",
///  "required": [
///    "account_key",
///    "peer_id",
///    "proxies",
///    "timestamp"
///  ],
///  "properties": {
///    "account_key": {
///      "description": "Account key of the validator signing this
/// AccountData.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    },
///    "peer_id": {
///      "description": "ID of the node that handles the account key (aka
/// validator key).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    },
///    "proxies": {
///      "description": "Proxy nodes that are directly connected to the
/// validator node\n(this list may include the validator node
/// itself).\nTIER1 nodes should connect to one of the proxies to sent
/// TIER1\nmessages to the validator.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/Tier1ProxyView"
///      }
///    },
///    "timestamp": {
///      "description": "UTC timestamp of when the AccountData has been
/// signed.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountDataView {
    ///Account key of the validator signing this AccountData.
    pub account_key: PublicKey,
    ///ID of the node that handles the account key (aka validator key).
    pub peer_id: PublicKey,
    ///Proxy nodes that are directly connected to the validator node
    ///(this list may include the validator node itself).
    ///TIER1 nodes should connect to one of the proxies to sent TIER1
    ///messages to the validator.
    pub proxies: ::std::vec::Vec<Tier1ProxyView>,
    ///UTC timestamp of when the AccountData has been signed.
    pub timestamp: ::std::string::String,
}
///`AccountIdValidityRulesVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "integer",
///  "format": "uint8",
///  "maximum": 255.0,
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AccountIdValidityRulesVersion(pub u8);
impl ::std::ops::Deref for AccountIdValidityRulesVersion {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}
impl ::std::convert::From<AccountIdValidityRulesVersion> for u8 {
    fn from(value: AccountIdValidityRulesVersion) -> Self {
        value.0
    }
}
impl ::std::convert::From<u8> for AccountIdValidityRulesVersion {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for AccountIdValidityRulesVersion {
    type Err = <u8 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for AccountIdValidityRulesVersion {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for AccountIdValidityRulesVersion {
    type Error = <u8 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for AccountIdValidityRulesVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Account info for validators
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Account info for validators",
///  "type": "object",
///  "required": [
///    "account_id",
///    "amount",
///    "public_key"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "amount": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountInfo {
    pub account_id: AccountId,
    pub amount: NearToken,
    pub public_key: PublicKey,
}
///A view of the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A view of the account",
///  "type": "object",
///  "required": [
///    "amount",
///    "code_hash",
///    "locked",
///    "storage_usage"
///  ],
///  "properties": {
///    "amount": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "code_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "global_contract_account_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "global_contract_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "locked": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "storage_paid_at": {
///      "description": "TODO(2271): deprecated.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "storage_usage": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountView {
    pub amount: NearToken,
    pub code_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_hash: ::std::option::Option<CryptoHash>,
    pub locked: NearToken,
    ///TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: u64,
    pub storage_usage: u64,
}
///Account ID with its public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Account ID with its public key.",
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AccountWithPublicKey {
    pub account_id: AccountId,
    pub public_key: PublicKey,
}
///Describes the cost of creating a specific action, `Action`. Includes all
/// variants.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the cost of creating a specific action,
/// `Action`. Includes all variants.",
///  "type": "object",
///  "properties": {
///    "add_key_cost": {
///      "description": "Base cost of adding a key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKeyCreationConfigView"
///        }
///      ]
///    },
///    "create_account_cost": {
///      "description": "Base cost of creating an account.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "delegate_cost": {
///      "description": "Base cost for processing a delegate action.\n\nThis
/// is on top of the costs for the actions inside the delegate action.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "delete_account_cost": {
///      "description": "Base cost of deleting an account.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "delete_key_cost": {
///      "description": "Base cost of deleting a key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "deploy_contract_cost": {
///      "description": "Base cost of deploying a contract.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "deploy_contract_cost_per_byte": {
///      "description": "Cost per byte of deploying a contract.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "function_call_cost": {
///      "description": "Base cost of calling a function.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "function_call_cost_per_byte": {
///      "description": "Cost per byte of method name and arguments of
/// calling a function.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "stake_cost": {
///      "description": "Base cost of staking.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "transfer_cost": {
///      "description": "Base cost of making a transfer.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ActionCreationConfigView {
    ///Base cost of adding a key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub add_key_cost: ::std::option::Option<AccessKeyCreationConfigView>,
    ///Base cost of creating an account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub create_account_cost: ::std::option::Option<Fee>,
    ///Base cost for processing a delegate action.
    ///
    ///This is on top of the costs for the actions inside the delegate
    /// action.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub delegate_cost: ::std::option::Option<Fee>,
    ///Base cost of deleting an account.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub delete_account_cost: ::std::option::Option<Fee>,
    ///Base cost of deleting a key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub delete_key_cost: ::std::option::Option<Fee>,
    ///Base cost of deploying a contract.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deploy_contract_cost: ::std::option::Option<Fee>,
    ///Cost per byte of deploying a contract.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deploy_contract_cost_per_byte: ::std::option::Option<Fee>,
    ///Base cost of calling a function.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call_cost: ::std::option::Option<Fee>,
    ///Cost per byte of method name and arguments of calling a function.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub function_call_cost_per_byte: ::std::option::Option<Fee>,
    ///Base cost of staking.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub stake_cost: ::std::option::Option<Fee>,
    ///Base cost of making a transfer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transfer_cost: ::std::option::Option<Fee>,
}
impl ::std::default::Default for ActionCreationConfigView {
    fn default() -> Self {
        Self {
            add_key_cost: Default::default(),
            create_account_cost: Default::default(),
            delegate_cost: Default::default(),
            delete_account_cost: Default::default(),
            delete_key_cost: Default::default(),
            deploy_contract_cost: Default::default(),
            deploy_contract_cost_per_byte: Default::default(),
            function_call_cost: Default::default(),
            function_call_cost_per_byte: Default::default(),
            stake_cost: Default::default(),
            transfer_cost: Default::default(),
        }
    }
}
///An error happened during Action execution
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An error happened during Action execution",
///  "type": "object",
///  "required": [
///    "kind"
///  ],
///  "properties": {
///    "index": {
///      "description": "Index of the failed action in the
/// transaction.\nAction index is not defined if ActionError.kind is
/// `ActionErrorKind::LackBalanceForState`",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "kind": {
///      "description": "The kind of ActionError happened",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ActionErrorKind"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ActionError {
    ///Index of the failed action in the transaction.
    ///Action index is not defined if ActionError.kind is
    /// `ActionErrorKind::LackBalanceForState`
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub index: ::std::option::Option<u64>,
    ///The kind of ActionError happened
    pub kind: ActionErrorKind,
}
///`ActionErrorKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "Happens when CreateAccount action tries to create
/// an account with account_id which is already exists in the storage",
///      "type": "object",
///      "required": [
///        "AccountAlreadyExists"
///      ],
///      "properties": {
///        "AccountAlreadyExists": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Happens when TX receiver_id doesn't exist (but
/// action is not Action::CreateAccount)",
///      "type": "object",
///      "required": [
///        "AccountDoesNotExist"
///      ],
///      "properties": {
///        "AccountDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "A top-level account ID can only be created by
/// registrar.",
///      "type": "object",
///      "required": [
///        "CreateAccountOnlyByRegistrar"
///      ],
///      "properties": {
///        "CreateAccountOnlyByRegistrar": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "predecessor_id",
///            "registrar_account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "predecessor_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "registrar_account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "A newly created account must be under a namespace
/// of the creator account",
///      "type": "object",
///      "required": [
///        "CreateAccountNotAllowed"
///      ],
///      "properties": {
///        "CreateAccountNotAllowed": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "predecessor_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "predecessor_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Administrative actions like `DeployContract`, `Stake`, `AddKey`, `DeleteKey`. can be proceed only if sender=receiver\nor the first TX action is a `CreateAccount` action",
///      "type": "object",
///      "required": [
///        "ActorNoPermission"
///      ],
///      "properties": {
///        "ActorNoPermission": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "actor_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "actor_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Account tries to remove an access key that doesn't
/// exist",
///      "type": "object",
///      "required": [
///        "DeleteKeyDoesNotExist"
///      ],
///      "properties": {
///        "DeleteKeyDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The public key is already used for an existing
/// access key",
///      "type": "object",
///      "required": [
///        "AddKeyAlreadyExists"
///      ],
///      "properties": {
///        "AddKeyAlreadyExists": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Account is staking and can not be deleted",
///      "type": "object",
///      "required": [
///        "DeleteAccountStaking"
///      ],
///      "properties": {
///        "DeleteAccountStaking": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "ActionReceipt can't be completed, because the
/// remaining balance will not be enough to cover storage.",
///      "type": "object",
///      "required": [
///        "LackBalanceForState"
///      ],
///      "properties": {
///        "LackBalanceForState": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "amount"
///          ],
///          "properties": {
///            "account_id": {
///              "description": "An account which needs balance",
///              "allOf": [
///                {
///                  "$ref": "#/components/schemas/AccountId"
///                }
///              ]
///            },
///            "amount": {
///              "description": "Balance required to complete an action.",
///              "allOf": [
///                {
///                  "$ref": "#/components/schemas/NearToken"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Account is not yet staked, but tries to unstake",
///      "type": "object",
///      "required": [
///        "TriesToUnstake"
///      ],
///      "properties": {
///        "TriesToUnstake": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The account doesn't have enough balance to increase
/// the stake.",
///      "type": "object",
///      "required": [
///        "TriesToStake"
///      ],
///      "properties": {
///        "TriesToStake": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "balance",
///            "locked",
///            "stake"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "balance": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "locked": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "stake": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "InsufficientStake"
///      ],
///      "properties": {
///        "InsufficientStake": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "minimum_stake",
///            "stake"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "minimum_stake": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "stake": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An error occurred during a `FunctionCall` Action,
/// parameter is debug message.",
///      "type": "object",
///      "required": [
///        "FunctionCallError"
///      ],
///      "properties": {
///        "FunctionCallError": {
///          "$ref": "#/components/schemas/FunctionCallError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Error occurs when a new `ActionReceipt` created by
/// the `FunctionCall` action fails\nreceipt validation.",
///      "type": "object",
///      "required": [
///        "NewReceiptValidationError"
///      ],
///      "properties": {
///        "NewReceiptValidationError": {
///          "$ref": "#/components/schemas/ReceiptValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Error occurs when a `CreateAccount` action is called on a NEAR-implicit or ETH-implicit account.\nSee NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>.\nAlso, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.\n\nTODO(#8598): This error is named very poorly. A better name would be\n`OnlyNamedAccountCreationAllowed`.",
///      "type": "object",
///      "required": [
///        "OnlyImplicitAccountCreationAllowed"
///      ],
///      "properties": {
///        "OnlyImplicitAccountCreationAllowed": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Delete account whose state is large is temporarily
/// banned.",
///      "type": "object",
///      "required": [
///        "DeleteAccountWithLargeState"
///      ],
///      "properties": {
///        "DeleteAccountWithLargeState": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Signature does not match the provided actions and
/// given signer public key.",
///      "type": "string",
///      "enum": [
///        "DelegateActionInvalidSignature"
///      ]
///    },
///    {
///      "description": "Receiver of the transaction doesn't match Sender of
/// the delegate action",
///      "type": "object",
///      "required": [
///        "DelegateActionSenderDoesNotMatchTxReceiver"
///      ],
///      "properties": {
///        "DelegateActionSenderDoesNotMatchTxReceiver": {
///          "type": "object",
///          "required": [
///            "receiver_id",
///            "sender_id"
///          ],
///          "properties": {
///            "receiver_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "sender_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Delegate action has expired. `max_block_height` is
/// less than actual block height.",
///      "type": "string",
///      "enum": [
///        "DelegateActionExpired"
///      ]
///    },
///    {
///      "description": "The given public key doesn't exist for Sender
/// account",
///      "type": "object",
///      "required": [
///        "DelegateActionAccessKeyError"
///      ],
///      "properties": {
///        "DelegateActionAccessKeyError": {
///          "$ref": "#/components/schemas/InvalidAccessKeyError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "DelegateAction nonce must be greater
/// sender[public_key].nonce",
///      "type": "object",
///      "required": [
///        "DelegateActionInvalidNonce"
///      ],
///      "properties": {
///        "DelegateActionInvalidNonce": {
///          "type": "object",
///          "required": [
///            "ak_nonce",
///            "delegate_nonce"
///          ],
///          "properties": {
///            "ak_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "delegate_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "DelegateAction nonce is larger than the upper bound
/// given by the block height",
///      "type": "object",
///      "required": [
///        "DelegateActionNonceTooLarge"
///      ],
///      "properties": {
///        "DelegateActionNonceTooLarge": {
///          "type": "object",
///          "required": [
///            "delegate_nonce",
///            "upper_bound"
///          ],
///          "properties": {
///            "delegate_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "upper_bound": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GlobalContractDoesNotExist"
///      ],
///      "properties": {
///        "GlobalContractDoesNotExist": {
///          "type": "object",
///          "required": [
///            "identifier"
///          ],
///          "properties": {
///            "identifier": {
///              "$ref": "#/components/schemas/GlobalContractIdentifier"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GasKeyDoesNotExist"
///      ],
///      "properties": {
///        "GasKeyDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GasKeyAlreadyExists"
///      ],
///      "properties": {
///        "GasKeyAlreadyExists": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ActionErrorKind {
    ///Happens when CreateAccount action tries to create an account with
    /// account_id which is already exists in the storage
    AccountAlreadyExists { account_id: AccountId },
    ///Happens when TX receiver_id doesn't exist (but action is not
    /// Action::CreateAccount)
    AccountDoesNotExist { account_id: AccountId },
    ///A top-level account ID can only be created by registrar.
    CreateAccountOnlyByRegistrar {
        account_id: AccountId,
        predecessor_id: AccountId,
        registrar_account_id: AccountId,
    },
    ///A newly created account must be under a namespace of the creator
    /// account
    CreateAccountNotAllowed {
        account_id: AccountId,
        predecessor_id: AccountId,
    },
    ///Administrative actions like `DeployContract`, `Stake`, `AddKey`,
    /// `DeleteKey`. can be proceed only if sender=receiver
    /// or the first TX action is a `CreateAccount` action
    ActorNoPermission {
        account_id: AccountId,
        actor_id: AccountId,
    },
    ///Account tries to remove an access key that doesn't exist
    DeleteKeyDoesNotExist {
        account_id: AccountId,
        public_key: PublicKey,
    },
    ///The public key is already used for an existing access key
    AddKeyAlreadyExists {
        account_id: AccountId,
        public_key: PublicKey,
    },
    ///Account is staking and can not be deleted
    DeleteAccountStaking { account_id: AccountId },
    ///ActionReceipt can't be completed, because the remaining balance will
    /// not be enough to cover storage.
    LackBalanceForState {
        ///An account which needs balance
        account_id: AccountId,
        ///Balance required to complete an action.
        amount: NearToken,
    },
    ///Account is not yet staked, but tries to unstake
    TriesToUnstake { account_id: AccountId },
    ///The account doesn't have enough balance to increase the stake.
    TriesToStake {
        account_id: AccountId,
        balance: NearToken,
        locked: NearToken,
        stake: NearToken,
    },
    InsufficientStake {
        account_id: AccountId,
        minimum_stake: NearToken,
        stake: NearToken,
    },
    ///An error occurred during a `FunctionCall` Action, parameter is debug
    /// message.
    FunctionCallError(FunctionCallError),
    ///Error occurs when a new `ActionReceipt` created by the
    /// `FunctionCall` action fails receipt validation.
    NewReceiptValidationError(ReceiptValidationError),
    ///Error occurs when a `CreateAccount` action is called on a
    /// NEAR-implicit or ETH-implicit account. See NEAR-implicit account creation NEP: <https://github.com/nearprotocol/NEPs/pull/71>.
    ///Also, see ETH-implicit account creation NEP: <https://github.com/near/NEPs/issues/518>.
    ///
    ///TODO(#8598): This error is named very poorly. A better name would be
    ///`OnlyNamedAccountCreationAllowed`.
    OnlyImplicitAccountCreationAllowed { account_id: AccountId },
    ///Delete account whose state is large is temporarily banned.
    DeleteAccountWithLargeState { account_id: AccountId },
    ///Signature does not match the provided actions and given signer
    /// public key.
    DelegateActionInvalidSignature,
    ///Receiver of the transaction doesn't match Sender of the delegate
    /// action
    DelegateActionSenderDoesNotMatchTxReceiver {
        receiver_id: AccountId,
        sender_id: AccountId,
    },
    ///Delegate action has expired. `max_block_height` is less than actual
    /// block height.
    DelegateActionExpired,
    ///The given public key doesn't exist for Sender account
    DelegateActionAccessKeyError(InvalidAccessKeyError),
    ///DelegateAction nonce must be greater sender[public_key].nonce
    DelegateActionInvalidNonce { ak_nonce: u64, delegate_nonce: u64 },
    ///DelegateAction nonce is larger than the upper bound given by the
    /// block height
    DelegateActionNonceTooLarge {
        delegate_nonce: u64,
        upper_bound: u64,
    },
    GlobalContractDoesNotExist {
        identifier: GlobalContractIdentifier,
    },
    GasKeyDoesNotExist {
        account_id: AccountId,
        public_key: PublicKey,
    },
    GasKeyAlreadyExists {
        account_id: AccountId,
        public_key: PublicKey,
    },
}
impl ::std::convert::From<FunctionCallError> for ActionErrorKind {
    fn from(value: FunctionCallError) -> Self {
        Self::FunctionCallError(value)
    }
}
impl ::std::convert::From<ReceiptValidationError> for ActionErrorKind {
    fn from(value: ReceiptValidationError) -> Self {
        Self::NewReceiptValidationError(value)
    }
}
impl ::std::convert::From<InvalidAccessKeyError> for ActionErrorKind {
    fn from(value: InvalidAccessKeyError) -> Self {
        Self::DelegateActionAccessKeyError(value)
    }
}
///`ActionView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "CreateAccount"
///      ]
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeployContract"
///      ],
///      "properties": {
///        "DeployContract": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "type": "object",
///          "required": [
///            "args",
///            "deposit",
///            "gas",
///            "method_name"
///          ],
///          "properties": {
///            "args": {
///              "$ref": "#/components/schemas/FunctionArgs"
///            },
///            "deposit": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "gas": {
///              "$ref": "#/components/schemas/NearGas"
///            },
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Transfer"
///      ],
///      "properties": {
///        "Transfer": {
///          "type": "object",
///          "required": [
///            "deposit"
///          ],
///          "properties": {
///            "deposit": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Stake"
///      ],
///      "properties": {
///        "Stake": {
///          "type": "object",
///          "required": [
///            "public_key",
///            "stake"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "stake": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "AddKey"
///      ],
///      "properties": {
///        "AddKey": {
///          "type": "object",
///          "required": [
///            "access_key",
///            "public_key"
///          ],
///          "properties": {
///            "access_key": {
///              "$ref": "#/components/schemas/AccessKeyView"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteKey"
///      ],
///      "properties": {
///        "DeleteKey": {
///          "type": "object",
///          "required": [
///            "public_key"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteAccount"
///      ],
///      "properties": {
///        "DeleteAccount": {
///          "type": "object",
///          "required": [
///            "beneficiary_id"
///          ],
///          "properties": {
///            "beneficiary_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Delegate"
///      ],
///      "properties": {
///        "Delegate": {
///          "type": "object",
///          "required": [
///            "delegate_action",
///            "signature"
///          ],
///          "properties": {
///            "delegate_action": {
///              "$ref": "#/components/schemas/DelegateAction"
///            },
///            "signature": {
///              "$ref": "#/components/schemas/Signature"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeployGlobalContract"
///      ],
///      "properties": {
///        "DeployGlobalContract": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeployGlobalContractByAccountId"
///      ],
///      "properties": {
///        "DeployGlobalContractByAccountId": {
///          "type": "object",
///          "required": [
///            "code"
///          ],
///          "properties": {
///            "code": {
///              "type": "string",
///              "format": "bytes"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "UseGlobalContract"
///      ],
///      "properties": {
///        "UseGlobalContract": {
///          "type": "object",
///          "required": [
///            "code_hash"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "UseGlobalContractByAccountId"
///      ],
///      "properties": {
///        "UseGlobalContractByAccountId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeterministicStateInit"
///      ],
///      "properties": {
///        "DeterministicStateInit": {
///          "type": "object",
///          "required": [
///            "code",
///            "data",
///            "deposit"
///          ],
///          "properties": {
///            "code": {
///              "$ref": "#/components/schemas/GlobalContractIdentifierView"
///            },
///            "data": {
///              "type": "object",
///              "additionalProperties": {
///                "type": "string"
///              }
///            },
///            "deposit": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "AddGasKey"
///      ],
///      "properties": {
///        "AddGasKey": {
///          "type": "object",
///          "required": [
///            "num_nonces",
///            "permission",
///            "public_key"
///          ],
///          "properties": {
///            "num_nonces": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            },
///            "permission": {
///              "$ref": "#/components/schemas/AccessKeyPermissionView"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteGasKey"
///      ],
///      "properties": {
///        "DeleteGasKey": {
///          "type": "object",
///          "required": [
///            "public_key"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "TransferToGasKey"
///      ],
///      "properties": {
///        "TransferToGasKey": {
///          "type": "object",
///          "required": [
///            "amount",
///            "public_key"
///          ],
///          "properties": {
///            "amount": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ActionView {
    CreateAccount,
    DeployContract {
        code: ::std::string::String,
    },
    FunctionCall {
        args: FunctionArgs,
        deposit: NearToken,
        gas: NearGas,
        method_name: ::std::string::String,
    },
    Transfer {
        deposit: NearToken,
    },
    Stake {
        public_key: PublicKey,
        stake: NearToken,
    },
    AddKey {
        access_key: AccessKeyView,
        public_key: PublicKey,
    },
    DeleteKey {
        public_key: PublicKey,
    },
    DeleteAccount {
        beneficiary_id: AccountId,
    },
    Delegate {
        delegate_action: DelegateAction,
        signature: Signature,
    },
    DeployGlobalContract {
        code: ::std::string::String,
    },
    DeployGlobalContractByAccountId {
        code: ::std::string::String,
    },
    UseGlobalContract {
        code_hash: CryptoHash,
    },
    UseGlobalContractByAccountId {
        account_id: AccountId,
    },
    DeterministicStateInit {
        code: GlobalContractIdentifierView,
        data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        deposit: NearToken,
    },
    AddGasKey {
        num_nonces: u32,
        permission: AccessKeyPermissionView,
        public_key: PublicKey,
    },
    DeleteGasKey {
        public_key: PublicKey,
    },
    TransferToGasKey {
        amount: NearToken,
        public_key: PublicKey,
    },
}
///Describes the error for validating a list of actions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the error for validating a list of actions.",
///  "oneOf": [
///    {
///      "description": "The delete action must be a final action in
/// transaction",
///      "type": "string",
///      "enum": [
///        "DeleteActionMustBeFinal"
///      ]
///    },
///    {
///      "description": "The total prepaid gas (for all given actions)
/// exceeded the limit.",
///      "type": "object",
///      "required": [
///        "TotalPrepaidGasExceeded"
///      ],
///      "properties": {
///        "TotalPrepaidGasExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_prepaid_gas"
///          ],
///          "properties": {
///            "limit": {
///              "$ref": "#/components/schemas/NearGas"
///            },
///            "total_prepaid_gas": {
///              "$ref": "#/components/schemas/NearGas"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The number of actions exceeded the given limit.",
///      "type": "object",
///      "required": [
///        "TotalNumberOfActionsExceeded"
///      ],
///      "properties": {
///        "TotalNumberOfActionsExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_number_of_actions"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "total_number_of_actions": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The total number of bytes of the method names
/// exceeded the limit in a Add Key action.",
///      "type": "object",
///      "required": [
///        "AddKeyMethodNamesNumberOfBytesExceeded"
///      ],
///      "properties": {
///        "AddKeyMethodNamesNumberOfBytesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "total_number_of_bytes"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "total_number_of_bytes": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The length of some method name exceeded the limit
/// in a Add Key action.",
///      "type": "object",
///      "required": [
///        "AddKeyMethodNameLengthExceeded"
///      ],
///      "properties": {
///        "AddKeyMethodNameLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Integer overflow during a compute.",
///      "type": "string",
///      "enum": [
///        "IntegerOverflow"
///      ]
///    },
///    {
///      "description": "Invalid account ID.",
///      "type": "object",
///      "required": [
///        "InvalidAccountId"
///      ],
///      "properties": {
///        "InvalidAccountId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The size of the contract code exceeded the limit in
/// a DeployContract action.",
///      "type": "object",
///      "required": [
///        "ContractSizeExceeded"
///      ],
///      "properties": {
///        "ContractSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The length of the method name exceeded the limit in
/// a Function Call action.",
///      "type": "object",
///      "required": [
///        "FunctionCallMethodNameLengthExceeded"
///      ],
///      "properties": {
///        "FunctionCallMethodNameLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The length of the arguments exceeded the limit in a
/// Function Call action.",
///      "type": "object",
///      "required": [
///        "FunctionCallArgumentsLengthExceeded"
///      ],
///      "properties": {
///        "FunctionCallArgumentsLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An attempt to stake with a public key that is not
/// convertible to ristretto.",
///      "type": "object",
///      "required": [
///        "UnsuitableStakingKey"
///      ],
///      "properties": {
///        "UnsuitableStakingKey": {
///          "type": "object",
///          "required": [
///            "public_key"
///          ],
///          "properties": {
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The attached amount of gas in a FunctionCall action
/// has to be a positive number.",
///      "type": "string",
///      "enum": [
///        "FunctionCallZeroAttachedGas"
///      ]
///    },
///    {
///      "description": "There should be the only one DelegateAction",
///      "type": "string",
///      "enum": [
///        "DelegateActionMustBeOnlyOne"
///      ]
///    },
///    {
///      "description": "The transaction includes a feature that the current
/// protocol version\ndoes not support.\n\nNote: we stringify the protocol
/// feature name instead of using\n`ProtocolFeature` here because we don't
/// want to leak the internals of\nthat type into observable borsh
/// serialization.",
///      "type": "object",
///      "required": [
///        "UnsupportedProtocolFeature"
///      ],
///      "properties": {
///        "UnsupportedProtocolFeature": {
///          "type": "object",
///          "required": [
///            "protocol_feature",
///            "version"
///          ],
///          "properties": {
///            "protocol_feature": {
///              "type": "string"
///            },
///            "version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "InvalidDeterministicStateInitReceiver"
///      ],
///      "properties": {
///        "InvalidDeterministicStateInitReceiver": {
///          "type": "object",
///          "required": [
///            "derived_id",
///            "receiver_id"
///          ],
///          "properties": {
///            "derived_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "receiver_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeterministicStateInitKeyLengthExceeded"
///      ],
///      "properties": {
///        "DeterministicStateInitKeyLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeterministicStateInitValueLengthExceeded"
///      ],
///      "properties": {
///        "DeterministicStateInitValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GasKeyPermissionInvalid"
///      ],
///      "properties": {
///        "GasKeyPermissionInvalid": {
///          "type": "object",
///          "required": [
///            "permission"
///          ],
///          "properties": {
///            "permission": {
///              "$ref": "#/components/schemas/AccessKeyPermission"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GasKeyTooManyNoncesRequested"
///      ],
///      "properties": {
///        "GasKeyTooManyNoncesRequested": {
///          "type": "object",
///          "required": [
///            "limit",
///            "requested_nonces"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            },
///            "requested_nonces": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum ActionsValidationError {
    ///The delete action must be a final action in transaction
    DeleteActionMustBeFinal,
    ///The total prepaid gas (for all given actions) exceeded the limit.
    TotalPrepaidGasExceeded {
        limit: NearGas,
        total_prepaid_gas: NearGas,
    },
    ///The number of actions exceeded the given limit.
    TotalNumberOfActionsExceeded {
        limit: u64,
        total_number_of_actions: u64,
    },
    ///The total number of bytes of the method names exceeded the limit in
    /// a Add Key action.
    AddKeyMethodNamesNumberOfBytesExceeded {
        limit: u64,
        total_number_of_bytes: u64,
    },
    ///The length of some method name exceeded the limit in a Add Key
    /// action.
    AddKeyMethodNameLengthExceeded {
        length: u64,
        limit: u64,
    },
    ///Integer overflow during a compute.
    IntegerOverflow,
    ///Invalid account ID.
    InvalidAccountId {
        account_id: ::std::string::String,
    },
    ///The size of the contract code exceeded the limit in a DeployContract
    /// action.
    ContractSizeExceeded {
        limit: u64,
        size: u64,
    },
    ///The length of the method name exceeded the limit in a Function Call
    /// action.
    FunctionCallMethodNameLengthExceeded {
        length: u64,
        limit: u64,
    },
    ///The length of the arguments exceeded the limit in a Function Call
    /// action.
    FunctionCallArgumentsLengthExceeded {
        length: u64,
        limit: u64,
    },
    ///An attempt to stake with a public key that is not convertible to
    /// ristretto.
    UnsuitableStakingKey {
        public_key: PublicKey,
    },
    ///The attached amount of gas in a FunctionCall action has to be a
    /// positive number.
    FunctionCallZeroAttachedGas,
    ///There should be the only one DelegateAction
    DelegateActionMustBeOnlyOne,
    ///The transaction includes a feature that the current protocol version
    ///does not support.
    ///
    ///Note: we stringify the protocol feature name instead of using
    ///`ProtocolFeature` here because we don't want to leak the internals
    /// of that type into observable borsh serialization.
    UnsupportedProtocolFeature {
        protocol_feature: ::std::string::String,
        version: u32,
    },
    InvalidDeterministicStateInitReceiver {
        derived_id: AccountId,
        receiver_id: AccountId,
    },
    DeterministicStateInitKeyLengthExceeded {
        length: u64,
        limit: u64,
    },
    DeterministicStateInitValueLengthExceeded {
        length: u64,
        limit: u64,
    },
    GasKeyPermissionInvalid {
        permission: AccessKeyPermission,
    },
    GasKeyTooManyNoncesRequested {
        limit: u32,
        requested_nonces: u32,
    },
}
///`AddGasKeyAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "num_nonces",
///    "permission",
///    "public_key"
///  ],
///  "properties": {
///    "num_nonces": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "permission": {
///      "$ref": "#/components/schemas/AccessKeyPermission"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AddGasKeyAction {
    pub num_nonces: u32,
    pub permission: AccessKeyPermission,
    pub public_key: PublicKey,
}
///An action that adds key with public key associated
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An action that adds key with public key associated",
///  "type": "object",
///  "required": [
///    "access_key",
///    "public_key"
///  ],
///  "properties": {
///    "access_key": {
///      "description": "An access key with the permission",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKey"
///        }
///      ]
///    },
///    "public_key": {
///      "description": "A public key which will be associated with an
/// access_key",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AddKeyAction {
    ///An access key with the permission
    pub access_key: AccessKey,
    ///A public key which will be associated with an access_key
    pub public_key: PublicKey,
}
///`AllAccessKeyChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllAccessKeyChangesByBlockIdChangesType {
    #[serde(rename = "all_access_key_changes")]
    AllAccessKeyChanges,
}
impl ::std::fmt::Display for AllAccessKeyChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllAccessKeyChanges => f.write_str("all_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllAccessKeyChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_access_key_changes" => Ok(Self::AllAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllAccessKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllAccessKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllAccessKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AllAccessKeyChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllAccessKeyChangesByFinalityChangesType {
    #[serde(rename = "all_access_key_changes")]
    AllAccessKeyChanges,
}
impl ::std::fmt::Display for AllAccessKeyChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllAccessKeyChanges => f.write_str("all_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllAccessKeyChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_access_key_changes" => Ok(Self::AllAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllAccessKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllAccessKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllAccessKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AllAccessKeyChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllAccessKeyChangesBySyncCheckpointChangesType {
    #[serde(rename = "all_access_key_changes")]
    AllAccessKeyChanges,
}
impl ::std::fmt::Display for AllAccessKeyChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllAccessKeyChanges => f.write_str("all_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllAccessKeyChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_access_key_changes" => Ok(Self::AllAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllAccessKeyChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for AllAccessKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for AllAccessKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AllGasKeyChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllGasKeyChangesByBlockIdChangesType {
    #[serde(rename = "all_gas_key_changes")]
    AllGasKeyChanges,
}
impl ::std::fmt::Display for AllGasKeyChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllGasKeyChanges => f.write_str("all_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllGasKeyChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_gas_key_changes" => Ok(Self::AllGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AllGasKeyChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllGasKeyChangesByFinalityChangesType {
    #[serde(rename = "all_gas_key_changes")]
    AllGasKeyChanges,
}
impl ::std::fmt::Display for AllGasKeyChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllGasKeyChanges => f.write_str("all_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllGasKeyChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_gas_key_changes" => Ok(Self::AllGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AllGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AllGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AllGasKeyChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "all_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AllGasKeyChangesBySyncCheckpointChangesType {
    #[serde(rename = "all_gas_key_changes")]
    AllGasKeyChanges,
}
impl ::std::fmt::Display for AllGasKeyChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AllGasKeyChanges => f.write_str("all_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for AllGasKeyChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "all_gas_key_changes" => Ok(Self::AllGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AllGasKeyChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for AllGasKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for AllGasKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`BandwidthRequest` describes the size of receipts that a shard would
/// like to send to another shard. When a shard wants to send a lot of
/// receipts to another shard, it needs to create a request and wait for
/// a bandwidth grant from the bandwidth scheduler.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`BandwidthRequest` describes the size of receipts that
/// a shard would like to send to another shard.\nWhen a shard wants to send
/// a lot of receipts to another shard, it needs to create a request and
/// wait\nfor a bandwidth grant from the bandwidth scheduler.",
///  "type": "object",
///  "required": [
///    "requested_values_bitmap",
///    "to_shard"
///  ],
///  "properties": {
///    "requested_values_bitmap": {
///      "description": "Bitmap which describes what values of bandwidth are
/// requested.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/BandwidthRequestBitmap"
///        }
///      ]
///    },
///    "to_shard": {
///      "description": "Requesting bandwidth to this shard.",
///      "type": "integer",
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequest {
    ///Bitmap which describes what values of bandwidth are requested.
    pub requested_values_bitmap: BandwidthRequestBitmap,
    ///Requesting bandwidth to this shard.
    pub to_shard: u16,
}
///Bitmap which describes which values from the predefined list are being
/// requested. The nth bit is set to 1 when the nth value from the list
/// is being requested.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Bitmap which describes which values from the predefined
/// list are being requested.\nThe nth bit is set to 1 when the nth value
/// from the list is being requested.",
///  "type": "object",
///  "required": [
///    "data"
///  ],
///  "properties": {
///    "data": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "maximum": 255.0,
///        "minimum": 0.0
///      },
///      "maxItems": 5,
///      "minItems": 5
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequestBitmap {
    pub data: [u8; 5usize],
}
///A list of shard's bandwidth requests.
///Describes how much the shard would like to send to other shards.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A list of shard's bandwidth requests.\nDescribes how
/// much the shard would like to send to other shards.",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/components/schemas/BandwidthRequestsV1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum BandwidthRequests {
    V1(BandwidthRequestsV1),
}
impl ::std::convert::From<BandwidthRequestsV1> for BandwidthRequests {
    fn from(value: BandwidthRequestsV1) -> Self {
        Self::V1(value)
    }
}
///Version 1 of [`BandwidthRequest`].
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Version 1 of [`BandwidthRequest`].",
///  "type": "object",
///  "required": [
///    "requests"
///  ],
///  "properties": {
///    "requests": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/BandwidthRequest"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BandwidthRequestsV1 {
    pub requests: ::std::vec::Vec<BandwidthRequest>,
}
///A part of a state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A part of a state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).",
///  "type": "object",
///  "required": [
///    "block_merkle_root",
///    "epoch_id",
///    "height",
///    "next_bp_hash",
///    "next_epoch_id",
///    "outcome_root",
///    "prev_state_root",
///    "timestamp",
///    "timestamp_nanosec"
///  ],
///  "properties": {
///    "block_merkle_root": {
///      "description": "The merkle root of all the block hashes",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "epoch_id": {
///      "description": "The epoch to which the block that is the current
/// known head belongs",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "next_bp_hash": {
///      "description": "The hash of the block producers set for the next
/// epoch",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "next_epoch_id": {
///      "description": "The epoch that will follow the current epoch",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "outcome_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "prev_state_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "timestamp": {
///      "description": "Legacy json number. Should not be used.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp_nanosec": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockHeaderInnerLiteView {
    ///The merkle root of all the block hashes
    pub block_merkle_root: CryptoHash,
    ///The epoch to which the block that is the current known head belongs
    pub epoch_id: CryptoHash,
    pub height: u64,
    ///The hash of the block producers set for the next epoch
    pub next_bp_hash: CryptoHash,
    ///The epoch that will follow the current epoch
    pub next_epoch_id: CryptoHash,
    pub outcome_root: CryptoHash,
    pub prev_state_root: CryptoHash,
    ///Legacy json number. Should not be used.
    pub timestamp: u64,
    pub timestamp_nanosec: ::std::string::String,
}
///Contains main info about the block.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contains main info about the block.",
///  "type": "object",
///  "required": [
///    "approvals",
///    "block_merkle_root",
///    "challenges_result",
///    "challenges_root",
///    "chunk_headers_root",
///    "chunk_mask",
///    "chunk_receipts_root",
///    "chunk_tx_root",
///    "chunks_included",
///    "epoch_id",
///    "gas_price",
///    "hash",
///    "height",
///    "last_ds_final_block",
///    "last_final_block",
///    "latest_protocol_version",
///    "next_bp_hash",
///    "next_epoch_id",
///    "outcome_root",
///    "prev_hash",
///    "prev_state_root",
///    "random_value",
///    "signature",
///    "timestamp",
///    "timestamp_nanosec",
///    "total_supply",
///    "validator_proposals"
///  ],
///  "properties": {
///    "approvals": {
///      "type": "array",
///      "items": {
///        "anyOf": [
///          {
///            "$ref": "#/components/schemas/Signature"
///          },
///          {
///            "type": "null"
///          }
///        ]
///      }
///    },
///    "block_body_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "block_merkle_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "block_ordinal": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "challenges_result": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/SlashedValidator"
///      }
///    },
///    "challenges_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "chunk_endorsements": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "array",
///        "items": {
///          "type": "integer",
///          "format": "uint8",
///          "maximum": 255.0,
///          "minimum": 0.0
///        }
///      }
///    },
///    "chunk_headers_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "chunk_mask": {
///      "type": "array",
///      "items": {
///        "type": "boolean"
///      }
///    },
///    "chunk_receipts_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "chunk_tx_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "chunks_included": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "epoch_sync_data_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "gas_price": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "last_ds_final_block": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "last_final_block": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "latest_protocol_version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "next_bp_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "next_epoch_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "outcome_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "prev_hash": {
///      "description": "The hash of the previous Block",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "prev_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "prev_state_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "random_value": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "rent_paid": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "signature": {
///      "description": "Signature of the block producer.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Signature"
///        }
///      ]
///    },
///    "timestamp": {
///      "description": "Legacy json number. Should not be used.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp_nanosec": {
///      "type": "string"
///    },
///    "total_supply": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "validator_proposals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "validator_reward": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockHeaderView {
    pub approvals: ::std::vec::Vec<::std::option::Option<Signature>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_body_hash: ::std::option::Option<CryptoHash>,
    pub block_merkle_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_ordinal: ::std::option::Option<u64>,
    pub challenges_result: ::std::vec::Vec<SlashedValidator>,
    pub challenges_root: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_endorsements: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<u8>>>,
    pub chunk_headers_root: CryptoHash,
    pub chunk_mask: ::std::vec::Vec<bool>,
    pub chunk_receipts_root: CryptoHash,
    pub chunk_tx_root: CryptoHash,
    pub chunks_included: u64,
    pub epoch_id: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_sync_data_hash: ::std::option::Option<CryptoHash>,
    pub gas_price: NearToken,
    pub hash: CryptoHash,
    pub height: u64,
    pub last_ds_final_block: CryptoHash,
    pub last_final_block: CryptoHash,
    pub latest_protocol_version: u32,
    pub next_bp_hash: CryptoHash,
    pub next_epoch_id: CryptoHash,
    pub outcome_root: CryptoHash,
    ///The hash of the previous Block
    pub prev_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prev_height: ::std::option::Option<u64>,
    pub prev_state_root: CryptoHash,
    pub random_value: CryptoHash,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::block_header_view_rent_paid")]
    pub rent_paid: NearToken,
    ///Signature of the block producer.
    pub signature: Signature,
    ///Legacy json number. Should not be used.
    pub timestamp: u64,
    pub timestamp_nanosec: ::std::string::String,
    pub total_supply: NearToken,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::block_header_view_validator_reward")]
    pub validator_reward: NearToken,
}
///`BlockId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "title": "block_height",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum BlockId {
    BlockHeight(u64),
    CryptoHash(CryptoHash),
}
impl ::std::fmt::Display for BlockId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::BlockHeight(x) => x.fmt(f),
            Self::CryptoHash(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<u64> for BlockId {
    fn from(value: u64) -> Self {
        Self::BlockHeight(value)
    }
}
///`BlockReference`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/components/schemas/Finality"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/components/schemas/SyncCheckpoint"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum BlockReference {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for BlockReference {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for BlockReference {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for BlockReference {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///Height and hash of a block
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Height and hash of a block",
///  "type": "object",
///  "required": [
///    "hash",
///    "height"
///  ],
///  "properties": {
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BlockStatusView {
    pub hash: CryptoHash,
    pub height: u64,
}
///`CallFunctionByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "call_function"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CallFunctionByBlockIdRequestType {
    #[serde(rename = "call_function")]
    CallFunction,
}
impl ::std::fmt::Display for CallFunctionByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CallFunction => f.write_str("call_function"),
        }
    }
}
impl ::std::str::FromStr for CallFunctionByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call_function" => Ok(Self::CallFunction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallFunctionByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CallFunctionByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallFunctionByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CallFunctionByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "call_function"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CallFunctionByFinalityRequestType {
    #[serde(rename = "call_function")]
    CallFunction,
}
impl ::std::fmt::Display for CallFunctionByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CallFunction => f.write_str("call_function"),
        }
    }
}
impl ::std::str::FromStr for CallFunctionByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call_function" => Ok(Self::CallFunction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallFunctionByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CallFunctionByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallFunctionByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CallFunctionBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "call_function"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum CallFunctionBySyncCheckpointRequestType {
    #[serde(rename = "call_function")]
    CallFunction,
}
impl ::std::fmt::Display for CallFunctionBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CallFunction => f.write_str("call_function"),
        }
    }
}
impl ::std::str::FromStr for CallFunctionBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "call_function" => Ok(Self::CallFunction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CallFunctionBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CallFunctionBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CallFunctionBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A result returned by contract method
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A result returned by contract method",
///  "type": "object",
///  "required": [
///    "logs",
///    "result"
///  ],
///  "properties": {
///    "logs": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "result": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "maximum": 255.0,
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CallResult {
    pub logs: ::std::vec::Vec<::std::string::String>,
    pub result: ::std::vec::Vec<u8>,
}
///Status of the [catchup](https://near.github.io/nearcore/architecture/how/sync.html#catchup) process
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Status of the [catchup](https://near.github.io/nearcore/architecture/how/sync.html#catchup) process",
///  "type": "object",
///  "required": [
///    "blocks_to_catchup",
///    "shard_sync_status",
///    "sync_block_hash",
///    "sync_block_height"
///  ],
///  "properties": {
///    "blocks_to_catchup": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/BlockStatusView"
///      }
///    },
///    "shard_sync_status": {
///      "type": "object",
///      "additionalProperties": false
///    },
///    "sync_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "sync_block_height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CatchupStatusView {
    pub blocks_to_catchup: ::std::vec::Vec<BlockStatusView>,
    pub shard_sync_status: CatchupStatusViewShardSyncStatus,
    pub sync_block_hash: CryptoHash,
    pub sync_block_height: u64,
}
///`CatchupStatusViewShardSyncStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CatchupStatusViewShardSyncStatus {}
impl ::std::default::Default for CatchupStatusViewShardSyncStatus {
    fn default() -> Self {
        Self {}
    }
}
///Config for the Chunk Distribution Network feature.
///This allows nodes to push and pull chunks from a central stream.
///The two benefits of this approach are: (1) less request/response traffic
///on the peer-to-peer network and (2) lower latency for RPC nodes indexing
/// the chain.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Config for the Chunk Distribution Network
/// feature.\nThis allows nodes to push and pull chunks from a central
/// stream.\nThe two benefits of this approach are: (1) less
/// request/response traffic\non the peer-to-peer network and (2) lower
/// latency for RPC nodes indexing the chain.",
///  "type": "object",
///  "properties": {
///    "enabled": {
///      "type": "boolean"
///    },
///    "uris": {
///      "$ref": "#/components/schemas/ChunkDistributionUris"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkDistributionNetworkConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub uris: ::std::option::Option<ChunkDistributionUris>,
}
impl ::std::default::Default for ChunkDistributionNetworkConfig {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            uris: Default::default(),
        }
    }
}
///URIs for the Chunk Distribution Network feature.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "URIs for the Chunk Distribution Network feature.",
///  "type": "object",
///  "properties": {
///    "get": {
///      "description": "URI for pulling chunks from the stream.",
///      "type": "string"
///    },
///    "set": {
///      "description": "URI for publishing chunks to the stream.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkDistributionUris {
    ///URI for pulling chunks from the stream.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub get: ::std::option::Option<::std::string::String>,
    ///URI for publishing chunks to the stream.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub set: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ChunkDistributionUris {
    fn default() -> Self {
        Self {
            get: Default::default(),
            set: Default::default(),
        }
    }
}
///`ChunkHash`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/components/schemas/CryptoHash"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ChunkHash(pub CryptoHash);
impl ::std::ops::Deref for ChunkHash {
    type Target = CryptoHash;
    fn deref(&self) -> &CryptoHash {
        &self.0
    }
}
impl ::std::str::FromStr for ChunkHash {
    type Err = <CryptoHash as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ChunkHash {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ChunkHash {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ChunkHash {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///Contains main info about the chunk.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contains main info about the chunk.",
///  "type": "object",
///  "required": [
///    "balance_burnt",
///    "chunk_hash",
///    "encoded_length",
///    "encoded_merkle_root",
///    "gas_limit",
///    "gas_used",
///    "height_created",
///    "height_included",
///    "outcome_root",
///    "outgoing_receipts_root",
///    "prev_block_hash",
///    "prev_state_root",
///    "shard_id",
///    "signature",
///    "tx_root",
///    "validator_proposals"
///  ],
///  "properties": {
///    "balance_burnt": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "bandwidth_requests": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/BandwidthRequests"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "chunk_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "congestion_info": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CongestionInfoView"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "encoded_length": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "encoded_merkle_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "gas_limit": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "gas_used": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "height_created": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "height_included": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "outcome_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "outgoing_receipts_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "prev_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "prev_state_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "rent_paid": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "shard_id": {
///      "$ref": "#/components/schemas/ShardId"
///    },
///    "signature": {
///      "$ref": "#/components/schemas/Signature"
///    },
///    "tx_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "validator_proposals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "validator_reward": {
///      "description": "TODO(2271): deprecated.",
///      "default": "0",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChunkHeaderView {
    pub balance_burnt: NearToken,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bandwidth_requests: ::std::option::Option<BandwidthRequests>,
    pub chunk_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub congestion_info: ::std::option::Option<CongestionInfoView>,
    pub encoded_length: u64,
    pub encoded_merkle_root: CryptoHash,
    pub gas_limit: NearGas,
    pub gas_used: NearGas,
    pub height_created: u64,
    pub height_included: u64,
    pub outcome_root: CryptoHash,
    pub outgoing_receipts_root: CryptoHash,
    pub prev_block_hash: CryptoHash,
    pub prev_state_root: CryptoHash,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::chunk_header_view_rent_paid")]
    pub rent_paid: NearToken,
    pub shard_id: ShardId,
    pub signature: Signature,
    pub tx_root: CryptoHash,
    pub validator_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///TODO(2271): deprecated.
    #[serde(default = "defaults::chunk_header_view_validator_reward")]
    pub validator_reward: NearToken,
}
///Configuration for a cloud-based archival writer. If this config is
/// present, the writer is enabled and writes chunk-related data based
/// on the tracked shards. This config also controls additional archival
/// behavior such as block data and polling interval.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration for a cloud-based archival writer. If
/// this config is present, the writer is enabled and\nwrites chunk-related
/// data based on the tracked shards. This config also controls additional
/// archival\nbehavior such as block data and polling interval.",
///  "type": "object",
///  "properties": {
///    "archive_block_data": {
///      "description": "Determines whether block-related data should be
/// written to cloud storage.",
///      "default": false,
///      "type": "boolean"
///    },
///    "polling_interval": {
///      "description": "Interval at which the system checks for new blocks
/// or chunks to archive.",
///      "default": {
///        "nanos": 0,
///        "secs": 1
///      },
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CloudArchivalWriterConfig {
    ///Determines whether block-related data should be written to cloud
    /// storage.
    #[serde(default)]
    pub archive_block_data: bool,
    ///Interval at which the system checks for new blocks or chunks to
    /// archive.
    #[serde(default = "defaults::cloud_archival_writer_config_polling_interval")]
    pub polling_interval: DurationAsStdSchemaProvider,
}
impl ::std::default::Default for CloudArchivalWriterConfig {
    fn default() -> Self {
        Self {
            archive_block_data: Default::default(),
            polling_interval: defaults::cloud_archival_writer_config_polling_interval(),
        }
    }
}
///`CompilationError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "CodeDoesNotExist"
///      ],
///      "properties": {
///        "CodeDoesNotExist": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "PrepareError"
///      ],
///      "properties": {
///        "PrepareError": {
///          "$ref": "#/components/schemas/PrepareError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "This is for defense in depth.\nWe expect our
/// runtime-independent preparation code to fully catch all invalid
/// wasms,\nbut, if it ever misses something we’ll emit this error",
///      "type": "object",
///      "required": [
///        "WasmerCompileError"
///      ],
///      "properties": {
///        "WasmerCompileError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum CompilationError {
    CodeDoesNotExist {
        account_id: AccountId,
    },
    PrepareError(PrepareError),
    ///This is for defense in depth.
    ///We expect our runtime-independent preparation code to fully catch
    /// all invalid wasms, but, if it ever misses something we’ll
    /// emit this error
    WasmerCompileError {
        msg: ::std::string::String,
    },
}
impl ::std::convert::From<PrepareError> for CompilationError {
    fn from(value: PrepareError) -> Self {
        Self::PrepareError(value)
    }
}
///The configuration for congestion control. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The configuration for congestion control. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)",
///  "type": "object",
///  "properties": {
///    "allowed_shard_outgoing_gas": {
///      "description": "How much gas the chosen allowed shard can send to a
/// 100% congested shard.\n\nSee [`CongestionControlConfig`] for more
/// details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_congestion_incoming_gas": {
///      "description": "How much gas in delayed receipts of a shard is 100%
/// incoming congestion.\n\nSee [`CongestionControlConfig`] for more
/// details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_congestion_memory_consumption": {
///      "description": "How much memory space of all delayed and buffered
/// receipts in a shard is\nconsidered 100% congested.\n\nSee
/// [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_congestion_missed_chunks": {
///      "description": "How many missed chunks in a row in a shard is
/// considered 100% congested.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_congestion_outgoing_gas": {
///      "description": "How much gas in outgoing buffered receipts of a
/// shard is 100% congested.\n\nOutgoing congestion contributes to overall
/// congestion, which reduces how\nmuch other shards are allowed to forward
/// to this shard.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_outgoing_gas": {
///      "description": "The maximum amount of gas attached to receipts a
/// shard can forward to\nanother shard per chunk.\n\nSee
/// [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_tx_gas": {
///      "description": "The maximum amount of gas in a chunk spent on
/// converting new transactions to\nreceipts.\n\nSee
/// [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "min_outgoing_gas": {
///      "description": "The minimum gas each shard can send to a shard that
/// is not fully congested.\n\nSee [`CongestionControlConfig`] for more
/// details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "min_tx_gas": {
///      "description": "The minimum amount of gas in a chunk spent on converting new transactions\nto receipts, as long as the receiving shard is not congested.\n\nSee [`CongestionControlConfig`] for more details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "outgoing_receipts_big_size_limit": {
///      "description": "Large size limit for outgoing receipts to a shard,
/// used when it's safe\nto send a lot of receipts without making the state
/// witness too large.\nIt limits the total sum of outgoing receipts, not
/// individual receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "outgoing_receipts_usual_size_limit": {
///      "description": "The standard size limit for outgoing receipts aimed
/// at a single shard.\nThis limit is pretty small to keep the size of
/// source_receipt_proofs under control.\nIt limits the total sum of
/// outgoing receipts, not individual receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "reject_tx_congestion_threshold": {
///      "description": "How much congestion a shard can tolerate before it
/// stops all shards from\naccepting new transactions with the receiver set
/// to the congested shard.",
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CongestionControlConfigView {
    ///How much gas the chosen allowed shard can send to a 100% congested
    /// shard.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allowed_shard_outgoing_gas: ::std::option::Option<NearGas>,
    ///How much gas in delayed receipts of a shard is 100% incoming
    /// congestion.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_congestion_incoming_gas: ::std::option::Option<NearGas>,
    ///How much memory space of all delayed and buffered receipts in a
    /// shard is considered 100% congested.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_congestion_memory_consumption: ::std::option::Option<u64>,
    ///How many missed chunks in a row in a shard is considered 100%
    /// congested.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_congestion_missed_chunks: ::std::option::Option<u64>,
    ///How much gas in outgoing buffered receipts of a shard is 100%
    /// congested.
    ///
    ///Outgoing congestion contributes to overall congestion, which reduces
    /// how much other shards are allowed to forward to this shard.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_congestion_outgoing_gas: ::std::option::Option<NearGas>,
    ///The maximum amount of gas attached to receipts a shard can forward
    /// to another shard per chunk.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_outgoing_gas: ::std::option::Option<NearGas>,
    ///The maximum amount of gas in a chunk spent on converting new
    /// transactions to receipts.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tx_gas: ::std::option::Option<NearGas>,
    ///The minimum gas each shard can send to a shard that is not fully
    /// congested.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_outgoing_gas: ::std::option::Option<NearGas>,
    ///The minimum amount of gas in a chunk spent on converting new
    /// transactions to receipts, as long as the receiving shard is
    /// not congested.
    ///
    ///See [`CongestionControlConfig`] for more details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_tx_gas: ::std::option::Option<NearGas>,
    ///Large size limit for outgoing receipts to a shard, used when it's
    /// safe to send a lot of receipts without making the state
    /// witness too large. It limits the total sum of outgoing
    /// receipts, not individual receipts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub outgoing_receipts_big_size_limit: ::std::option::Option<u64>,
    ///The standard size limit for outgoing receipts aimed at a single
    /// shard. This limit is pretty small to keep the size of
    /// source_receipt_proofs under control. It limits the total sum
    /// of outgoing receipts, not individual receipts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub outgoing_receipts_usual_size_limit: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reject_tx_congestion_threshold: ::std::option::Option<f64>,
}
impl ::std::default::Default for CongestionControlConfigView {
    fn default() -> Self {
        Self {
            allowed_shard_outgoing_gas: Default::default(),
            max_congestion_incoming_gas: Default::default(),
            max_congestion_memory_consumption: Default::default(),
            max_congestion_missed_chunks: Default::default(),
            max_congestion_outgoing_gas: Default::default(),
            max_outgoing_gas: Default::default(),
            max_tx_gas: Default::default(),
            min_outgoing_gas: Default::default(),
            min_tx_gas: Default::default(),
            outgoing_receipts_big_size_limit: Default::default(),
            outgoing_receipts_usual_size_limit: Default::default(),
            reject_tx_congestion_threshold: Default::default(),
        }
    }
}
///Stores the congestion level of a shard. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Stores the congestion level of a shard. More info about congestion [here](https://near.github.io/nearcore/architecture/how/receipt-congestion.html?highlight=congestion#receipt-congestion)",
///  "type": "object",
///  "required": [
///    "allowed_shard",
///    "buffered_receipts_gas",
///    "delayed_receipts_gas",
///    "receipt_bytes"
///  ],
///  "properties": {
///    "allowed_shard": {
///      "type": "integer",
///      "format": "uint16",
///      "maximum": 65535.0,
///      "minimum": 0.0
///    },
///    "buffered_receipts_gas": {
///      "type": "string"
///    },
///    "delayed_receipts_gas": {
///      "type": "string"
///    },
///    "receipt_bytes": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CongestionInfoView {
    pub allowed_shard: u16,
    pub buffered_receipts_gas: ::std::string::String,
    pub delayed_receipts_gas: ::std::string::String,
    pub receipt_bytes: u64,
}
///`ContractCodeChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "contract_code_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContractCodeChangesByBlockIdChangesType {
    #[serde(rename = "contract_code_changes")]
    ContractCodeChanges,
}
impl ::std::fmt::Display for ContractCodeChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContractCodeChanges => f.write_str("contract_code_changes"),
        }
    }
}
impl ::std::str::FromStr for ContractCodeChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "contract_code_changes" => Ok(Self::ContractCodeChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContractCodeChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ContractCodeChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContractCodeChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ContractCodeChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "contract_code_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContractCodeChangesByFinalityChangesType {
    #[serde(rename = "contract_code_changes")]
    ContractCodeChanges,
}
impl ::std::fmt::Display for ContractCodeChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContractCodeChanges => f.write_str("contract_code_changes"),
        }
    }
}
impl ::std::str::FromStr for ContractCodeChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "contract_code_changes" => Ok(Self::ContractCodeChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContractCodeChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ContractCodeChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ContractCodeChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ContractCodeChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "contract_code_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ContractCodeChangesBySyncCheckpointChangesType {
    #[serde(rename = "contract_code_changes")]
    ContractCodeChanges,
}
impl ::std::fmt::Display for ContractCodeChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ContractCodeChanges => f.write_str("contract_code_changes"),
        }
    }
}
impl ::std::str::FromStr for ContractCodeChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "contract_code_changes" => Ok(Self::ContractCodeChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ContractCodeChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ContractCodeChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ContractCodeChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A view of the contract code.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A view of the contract code.",
///  "type": "object",
///  "required": [
///    "code_base64",
///    "hash"
///  ],
///  "properties": {
///    "code_base64": {
///      "type": "string"
///    },
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ContractCodeView {
    pub code_base64: ::std::string::String,
    pub hash: CryptoHash,
}
///Shows gas profile. More info [here](https://near.github.io/nearcore/architecture/gas/gas_profile.html?highlight=WASM_HOST_COST#example-transaction-gas-profile).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Shows gas profile. More info [here](https://near.github.io/nearcore/architecture/gas/gas_profile.html?highlight=WASM_HOST_COST#example-transaction-gas-profile).",
///  "type": "object",
///  "required": [
///    "cost",
///    "cost_category",
///    "gas_used"
///  ],
///  "properties": {
///    "cost": {
///      "type": "string"
///    },
///    "cost_category": {
///      "description": "Either ACTION_COST or WASM_HOST_COST.",
///      "type": "string"
///    },
///    "gas_used": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CostGasUsed {
    pub cost: ::std::string::String,
    ///Either ACTION_COST or WASM_HOST_COST.
    pub cost_category: ::std::string::String,
    pub gas_used: ::std::string::String,
}
///Create account action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Create account action",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CreateAccountAction(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for CreateAccountAction {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<CreateAccountAction>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: CreateAccountAction) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for CreateAccountAction
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
///Describes information about the current epoch validator
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes information about the current epoch
/// validator",
///  "type": "object",
///  "required": [
///    "account_id",
///    "is_slashed",
///    "num_expected_blocks",
///    "num_produced_blocks",
///    "public_key",
///    "shards",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "is_slashed": {
///      "type": "boolean"
///    },
///    "num_expected_blocks": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_chunks": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_chunks_per_shard": {
///      "description": "Number of chunks this validator was expected to
/// produce in each shard.\nEach entry in the array corresponds to the shard
/// in the `shards_produced` array.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_expected_endorsements": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_expected_endorsements_per_shard": {
///      "description": "Number of chunks this validator was expected to
/// validate and endorse in each shard.\nEach entry in the array corresponds
/// to the shard in the `shards_endorsed` array.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_produced_blocks": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_chunks": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_chunks_per_shard": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_produced_endorsements": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_produced_endorsements_per_shard": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    },
///    "shards": {
///      "description": "Shards this validator is assigned to as chunk
/// producer in the current epoch.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "shards_endorsed": {
///      "description": "Shards this validator is assigned to as chunk
/// validator in the current epoch.",
///      "default": [],
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "stake": {
///      "$ref": "#/components/schemas/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CurrentEpochValidatorInfo {
    pub account_id: AccountId,
    pub is_slashed: bool,
    pub num_expected_blocks: u64,
    #[serde(default)]
    pub num_expected_chunks: u64,
    ///Number of chunks this validator was expected to produce in each
    /// shard. Each entry in the array corresponds to the shard in
    /// the `shards_produced` array.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_expected_chunks_per_shard: ::std::vec::Vec<u64>,
    #[serde(default)]
    pub num_expected_endorsements: u64,
    ///Number of chunks this validator was expected to validate and endorse
    /// in each shard. Each entry in the array corresponds to the
    /// shard in the `shards_endorsed` array.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_expected_endorsements_per_shard: ::std::vec::Vec<u64>,
    pub num_produced_blocks: u64,
    #[serde(default)]
    pub num_produced_chunks: u64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_produced_chunks_per_shard: ::std::vec::Vec<u64>,
    #[serde(default)]
    pub num_produced_endorsements: u64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_produced_endorsements_per_shard: ::std::vec::Vec<u64>,
    pub public_key: PublicKey,
    ///Shards this validator is assigned to as chunk producer in the
    /// current epoch.
    pub shards: ::std::vec::Vec<ShardId>,
    ///Shards this validator is assigned to as chunk validator in the
    /// current epoch.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub shards_endorsed: ::std::vec::Vec<ShardId>,
    pub stake: NearToken,
}
///`DataChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "data_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DataChangesByBlockIdChangesType {
    #[serde(rename = "data_changes")]
    DataChanges,
}
impl ::std::fmt::Display for DataChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DataChanges => f.write_str("data_changes"),
        }
    }
}
impl ::std::str::FromStr for DataChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "data_changes" => Ok(Self::DataChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DataChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DataChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DataChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DataChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "data_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DataChangesByFinalityChangesType {
    #[serde(rename = "data_changes")]
    DataChanges,
}
impl ::std::fmt::Display for DataChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DataChanges => f.write_str("data_changes"),
        }
    }
}
impl ::std::str::FromStr for DataChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "data_changes" => Ok(Self::DataChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DataChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DataChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DataChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DataChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "data_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DataChangesBySyncCheckpointChangesType {
    #[serde(rename = "data_changes")]
    DataChanges,
}
impl ::std::fmt::Display for DataChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DataChanges => f.write_str("data_changes"),
        }
    }
}
impl ::std::str::FromStr for DataChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "data_changes" => Ok(Self::DataChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DataChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DataChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DataChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The fees settings for a data receipt creation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The fees settings for a data receipt creation",
///  "type": "object",
///  "properties": {
///    "base_cost": {
///      "description": "Base cost of creating a data receipt.\nBoth `send`
/// and `exec` costs are burned when a new receipt has input dependencies.
/// The gas\nis charged for each input dependency. The dependencies are
/// specified when a receipt is\ncreated using `promise_then` and
/// `promise_batch_then`.\nNOTE: Any receipt with output dependencies will
/// produce data receipts. Even if it fails.\nEven if the last action is not
/// a function call (in case of success it will return empty\nvalue).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "cost_per_byte": {
///      "description": "Additional cost per byte sent.\nBoth `send` and
/// `exec` costs are burned when a function call finishes execution and
/// returns\n`N` bytes of data to every output dependency. For each output
/// dependency the cost is\n`(send(sir) + exec()) * N`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DataReceiptCreationConfigView {
    ///Base cost of creating a data receipt.
    ///Both `send` and `exec` costs are burned when a new receipt has input
    /// dependencies. The gas is charged for each input dependency.
    /// The dependencies are specified when a receipt is
    /// created using `promise_then` and `promise_batch_then`.
    ///NOTE: Any receipt with output dependencies will produce data
    /// receipts. Even if it fails. Even if the last action is not a
    /// function call (in case of success it will return empty
    /// value).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base_cost: ::std::option::Option<Fee>,
    ///Additional cost per byte sent.
    ///Both `send` and `exec` costs are burned when a function call
    /// finishes execution and returns `N` bytes of data to every
    /// output dependency. For each output dependency the cost is
    /// `(send(sir) + exec()) * N`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cost_per_byte: ::std::option::Option<Fee>,
}
impl ::std::default::Default for DataReceiptCreationConfigView {
    fn default() -> Self {
        Self {
            base_cost: Default::default(),
            cost_per_byte: Default::default(),
        }
    }
}
///`DataReceiverView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "data_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "data_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DataReceiverView {
    pub data_id: CryptoHash,
    pub receiver_id: AccountId,
}
///This action allows to execute the inner actions behalf of the defined
/// sender.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This action allows to execute the inner actions behalf
/// of the defined sender.",
///  "type": "object",
///  "required": [
///    "actions",
///    "max_block_height",
///    "nonce",
///    "public_key",
///    "receiver_id",
///    "sender_id"
///  ],
///  "properties": {
///    "actions": {
///      "description": "List of actions to be executed.\n\nWith the meta
/// transactions MVP defined in NEP-366, nested\nDelegateActions are not
/// allowed. A separate type is used to enforce it.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/NonDelegateAction"
///      }
///    },
///    "max_block_height": {
///      "description": "The maximal height of the block in the blockchain
/// below which the given DelegateAction is valid.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "description": "Nonce to ensure that the same delegate action is
/// not sent twice by a\nrelayer and should match for given account's
/// `public_key`.\nAfter this action is processed it will increment.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "public_key": {
///      "description": "Public key used to sign this delegated action.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    },
///    "receiver_id": {
///      "description": "Receiver of the delegated actions.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    },
///    "sender_id": {
///      "description": "Signer of the delegated actions",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DelegateAction {
    ///List of actions to be executed.
    ///
    ///With the meta transactions MVP defined in NEP-366, nested
    ///DelegateActions are not allowed. A separate type is used to enforce
    /// it.
    pub actions: ::std::vec::Vec<NonDelegateAction>,
    ///The maximal height of the block in the blockchain below which the
    /// given DelegateAction is valid.
    pub max_block_height: u64,
    ///Nonce to ensure that the same delegate action is not sent twice by a
    ///relayer and should match for given account's `public_key`.
    ///After this action is processed it will increment.
    pub nonce: u64,
    ///Public key used to sign this delegated action.
    pub public_key: PublicKey,
    ///Receiver of the delegated actions.
    pub receiver_id: AccountId,
    ///Signer of the delegated actions
    pub sender_id: AccountId,
}
///`DeleteAccountAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "beneficiary_id"
///  ],
///  "properties": {
///    "beneficiary_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeleteAccountAction {
    pub beneficiary_id: AccountId,
}
///`DeleteGasKeyAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "public_key"
///  ],
///  "properties": {
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeleteGasKeyAction {
    pub public_key: PublicKey,
}
///`DeleteKeyAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "public_key"
///  ],
///  "properties": {
///    "public_key": {
///      "description": "A public key associated with the access_key to be
/// deleted.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeleteKeyAction {
    ///A public key associated with the access_key to be deleted.
    pub public_key: PublicKey,
}
///Deploy contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Deploy contract action",
///  "type": "object",
///  "required": [
///    "code"
///  ],
///  "properties": {
///    "code": {
///      "description": "WebAssembly binary",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeployContractAction {
    ///WebAssembly binary
    pub code: ::std::string::String,
}
///Deploy global contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Deploy global contract action",
///  "type": "object",
///  "required": [
///    "code",
///    "deploy_mode"
///  ],
///  "properties": {
///    "code": {
///      "description": "WebAssembly binary",
///      "type": "string"
///    },
///    "deploy_mode": {
///      "$ref": "#/components/schemas/GlobalContractDeployMode"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeployGlobalContractAction {
    ///WebAssembly binary
    pub code: ::std::string::String,
    pub deploy_mode: GlobalContractDeployMode,
}
///`DetailedDebugStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_production_delay_millis",
///    "catchup_status",
///    "current_head_status",
///    "current_header_head_status",
///    "network_info",
///    "sync_status"
///  ],
///  "properties": {
///    "block_production_delay_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "catchup_status": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/CatchupStatusView"
///      }
///    },
///    "current_head_status": {
///      "$ref": "#/components/schemas/BlockStatusView"
///    },
///    "current_header_head_status": {
///      "$ref": "#/components/schemas/BlockStatusView"
///    },
///    "network_info": {
///      "$ref": "#/components/schemas/NetworkInfoView"
///    },
///    "sync_status": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DetailedDebugStatus {
    pub block_production_delay_millis: u64,
    pub catchup_status: ::std::vec::Vec<CatchupStatusView>,
    pub current_head_status: BlockStatusView,
    pub current_header_head_status: BlockStatusView,
    pub network_info: NetworkInfoView,
    pub sync_status: ::std::string::String,
}
///`DeterministicAccountStateInit`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/components/schemas/DeterministicAccountStateInitV1"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum DeterministicAccountStateInit {
    V1(DeterministicAccountStateInitV1),
}
impl ::std::convert::From<DeterministicAccountStateInitV1> for DeterministicAccountStateInit {
    fn from(value: DeterministicAccountStateInitV1) -> Self {
        Self::V1(value)
    }
}
///`DeterministicAccountStateInitV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "code",
///    "data"
///  ],
///  "properties": {
///    "code": {
///      "$ref": "#/components/schemas/GlobalContractIdentifier"
///    },
///    "data": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeterministicAccountStateInitV1 {
    pub code: GlobalContractIdentifier,
    pub data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
///`DeterministicStateInitAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposit",
///    "state_init"
///  ],
///  "properties": {
///    "deposit": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "state_init": {
///      "$ref": "#/components/schemas/DeterministicAccountStateInit"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DeterministicStateInitAction {
    pub deposit: NearToken,
    pub state_init: DeterministicAccountStateInit,
}
///`Direction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "Left",
///    "Right"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Direction {
    Left,
    Right,
}
impl ::std::fmt::Display for Direction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Left => f.write_str("Left"),
            Self::Right => f.write_str("Right"),
        }
    }
}
impl ::std::str::FromStr for Direction {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Left" => Ok(Self::Left),
            "Right" => Ok(Self::Right),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configures how to dump state to external storage.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures how to dump state to external storage.",
///  "type": "object",
///  "properties": {
///    "credentials_file": {
///      "description": "Location of a json file with credentials allowing
/// access to the bucket.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "iteration_delay": {
///      "description": "How often to check if a new epoch has
/// started.\nFeel free to set to `None`, defaults are sensible.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/DurationAsStdSchemaProvider"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "location": {
///      "description": "Specifies where to write the obtained state
/// parts.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExternalStorageLocation"
///        }
///      ]
///    },
///    "restart_dump_for_shards": {
///      "description": "Use in case a node that dumps state to the external
/// storage\ngets in trouble.",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DumpConfig {
    ///Location of a json file with credentials allowing access to the
    /// bucket.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credentials_file: ::std::option::Option<::std::string::String>,
    ///How often to check if a new epoch has started.
    ///Feel free to set to `None`, defaults are sensible.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub iteration_delay: ::std::option::Option<DurationAsStdSchemaProvider>,
    ///Specifies where to write the obtained state parts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<ExternalStorageLocation>,
    ///Use in case a node that dumps state to the external storage
    ///gets in trouble.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub restart_dump_for_shards: ::std::option::Option<::std::vec::Vec<ShardId>>,
}
impl ::std::default::Default for DumpConfig {
    fn default() -> Self {
        Self {
            credentials_file: Default::default(),
            iteration_delay: Default::default(),
            location: Default::default(),
            restart_dump_for_shards: Default::default(),
        }
    }
}
///`DurationAsStdSchemaProvider`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "nanos",
///    "secs"
///  ],
///  "properties": {
///    "nanos": {
///      "type": "integer",
///      "format": "int32"
///    },
///    "secs": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DurationAsStdSchemaProvider {
    pub nanos: i32,
    pub secs: i64,
}
///Configuration for dynamic resharding feature
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration for dynamic resharding feature",
///  "type": "object",
///  "required": [
///    "max_number_of_shards",
///    "memory_usage_threshold",
///    "min_child_memory_usage",
///    "min_epochs_between_resharding"
///  ],
///  "properties": {
///    "max_number_of_shards": {
///      "description": "Maximum number of shards in the network.\n\nSee
/// [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "memory_usage_threshold": {
///      "description": "Memory threshold over which a shard is marked for a
/// split.\n\nSee [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "min_child_memory_usage": {
///      "description": "Minimum memory usage of a child shard.\n\nSee
/// [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "min_epochs_between_resharding": {
///      "description": "Minimum number of epochs until next resharding can
/// be scheduled.\n\nSee [`CongestionControlConfig`] for more details.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DynamicReshardingConfigView {
    ///Maximum number of shards in the network.
    ///
    ///See [`CongestionControlConfig`] for more details.
    pub max_number_of_shards: u64,
    ///Memory threshold over which a shard is marked for a split.
    ///
    ///See [`CongestionControlConfig`] for more details.
    pub memory_usage_threshold: u64,
    ///Minimum memory usage of a child shard.
    ///
    ///See [`CongestionControlConfig`] for more details.
    pub min_child_memory_usage: u64,
    ///Minimum number of epochs until next resharding can be scheduled.
    ///
    ///See [`CongestionControlConfig`] for more details.
    pub min_epochs_between_resharding: u64,
}
///Epoch identifier -- wrapped hash, to make it easier to distinguish.
///EpochId of epoch T is the hash of last block in T-2
///EpochId of first two epochs is 0
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Epoch identifier -- wrapped hash, to make it easier to
/// distinguish.\nEpochId of epoch T is the hash of last block in
/// T-2\nEpochId of first two epochs is 0",
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EpochId(pub CryptoHash);
impl ::std::ops::Deref for EpochId {
    type Target = CryptoHash;
    fn deref(&self) -> &CryptoHash {
        &self.0
    }
}
impl ::std::str::FromStr for EpochId {
    type Err = <CryptoHash as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for EpochId {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for EpochId {
    type Error = <CryptoHash as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for EpochId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`EpochSyncConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "disable_epoch_sync_for_bootstrapping": {
///      "description": "If true, even if the node started from genesis, it
/// will not perform epoch sync.\nThere should be no reason to set this flag
/// in production, because on both mainnet\nand testnet it would be
/// infeasible to catch up from genesis without epoch sync.",
///      "default": false,
///      "type": "boolean"
///    },
///    "epoch_sync_horizon": {
///      "description": "This serves as two purposes: (1) the node will not
/// epoch sync and instead resort to\nheader sync, if the genesis block is
/// within this many blocks from the current block;\n(2) the node will
/// reject an epoch sync proof if the provided proof is for an epoch\nthat
/// is more than this many blocks behind the current block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ignore_epoch_sync_network_requests": {
///      "description": "If true, the node will ignore epoch sync requests
/// from the network. It is strongly\nrecommended not to set this flag,
/// because it will prevent other nodes from\nbootstrapping. This flag is
/// only included as a kill-switch and may be removed in a\nfuture release.
/// Please note that epoch sync requests are heavily rate limited
/// and\ncached, and therefore should not affect the performance of the node
/// or introduce\nany non-negligible increase in network traffic.",
///      "default": false,
///      "type": "boolean"
///    },
///    "timeout_for_epoch_sync": {
///      "description": "Timeout for epoch sync requests. The node will
/// continue retrying indefinitely even\nif this timeout is exceeded.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct EpochSyncConfig {
    ///If true, even if the node started from genesis, it will not perform
    /// epoch sync. There should be no reason to set this flag in
    /// production, because on both mainnet and testnet it would be
    /// infeasible to catch up from genesis without epoch sync.
    #[serde(default)]
    pub disable_epoch_sync_for_bootstrapping: bool,
    ///This serves as two purposes: (1) the node will not epoch sync and
    /// instead resort to header sync, if the genesis block is
    /// within this many blocks from the current block; (2) the node
    /// will reject an epoch sync proof if the provided proof is for an
    /// epoch that is more than this many blocks behind the current
    /// block.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_sync_horizon: ::std::option::Option<u64>,
    ///If true, the node will ignore epoch sync requests from the network.
    /// It is strongly recommended not to set this flag, because it
    /// will prevent other nodes from bootstrapping. This flag is
    /// only included as a kill-switch and may be removed in a
    /// future release. Please note that epoch sync requests are heavily
    /// rate limited and cached, and therefore should not affect the
    /// performance of the node or introduce any non-negligible
    /// increase in network traffic.
    #[serde(default)]
    pub ignore_epoch_sync_network_requests: bool,
    ///Timeout for epoch sync requests. The node will continue retrying
    /// indefinitely even if this timeout is exceeded.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timeout_for_epoch_sync: ::std::option::Option<DurationAsStdSchemaProvider>,
}
impl ::std::default::Default for EpochSyncConfig {
    fn default() -> Self {
        Self {
            disable_epoch_sync_for_bootstrapping: Default::default(),
            epoch_sync_horizon: Default::default(),
            ignore_epoch_sync_network_requests: Default::default(),
            timeout_for_epoch_sync: Default::default(),
        }
    }
}
///`ErrorWrapperForGenesisConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/GenesisConfigError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForGenesisConfigError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(GenesisConfigError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForGenesisConfigError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<GenesisConfigError> for ErrorWrapperForGenesisConfigError {
    fn from(value: GenesisConfigError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForGenesisConfigError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcBlockError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcBlockError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcBlockError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcBlockError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcBlockError> for ErrorWrapperForRpcBlockError {
    fn from(value: RpcBlockError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcBlockError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcChunkError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcChunkError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcChunkError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcChunkError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcChunkError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcChunkError> for ErrorWrapperForRpcChunkError {
    fn from(value: RpcChunkError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcChunkError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcClientConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcClientConfigError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcClientConfigError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcClientConfigError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcClientConfigError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcClientConfigError> for ErrorWrapperForRpcClientConfigError {
    fn from(value: RpcClientConfigError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcClientConfigError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcGasPriceError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcGasPriceError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcGasPriceError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcGasPriceError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcGasPriceError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcGasPriceError> for ErrorWrapperForRpcGasPriceError {
    fn from(value: RpcGasPriceError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcGasPriceError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcLightClientNextBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcLightClientNextBlockError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcLightClientNextBlockError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcLightClientNextBlockError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind>
    for ErrorWrapperForRpcLightClientNextBlockError
{
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcLightClientNextBlockError>
    for ErrorWrapperForRpcLightClientNextBlockError
{
    fn from(value: RpcLightClientNextBlockError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcLightClientNextBlockError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcLightClientProofError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcLightClientProofError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcLightClientProofError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcLightClientProofError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind>
    for ErrorWrapperForRpcLightClientProofError
{
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcLightClientProofError> for ErrorWrapperForRpcLightClientProofError {
    fn from(value: RpcLightClientProofError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcLightClientProofError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcMaintenanceWindowsError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcMaintenanceWindowsError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcMaintenanceWindowsError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcMaintenanceWindowsError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind>
    for ErrorWrapperForRpcMaintenanceWindowsError
{
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcMaintenanceWindowsError>
    for ErrorWrapperForRpcMaintenanceWindowsError
{
    fn from(value: RpcMaintenanceWindowsError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcMaintenanceWindowsError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcNetworkInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcNetworkInfoError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcNetworkInfoError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcNetworkInfoError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcNetworkInfoError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcNetworkInfoError> for ErrorWrapperForRpcNetworkInfoError {
    fn from(value: RpcNetworkInfoError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcNetworkInfoError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcProtocolConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcProtocolConfigError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcProtocolConfigError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcProtocolConfigError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcProtocolConfigError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcProtocolConfigError> for ErrorWrapperForRpcProtocolConfigError {
    fn from(value: RpcProtocolConfigError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcProtocolConfigError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcQueryError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcQueryError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcQueryError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcQueryError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcQueryError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcQueryError> for ErrorWrapperForRpcQueryError {
    fn from(value: RpcQueryError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcQueryError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcReceiptError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcReceiptError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcReceiptError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcReceiptError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcReceiptError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcReceiptError> for ErrorWrapperForRpcReceiptError {
    fn from(value: RpcReceiptError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcReceiptError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcSplitStorageInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcSplitStorageInfoError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcSplitStorageInfoError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcSplitStorageInfoError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind>
    for ErrorWrapperForRpcSplitStorageInfoError
{
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcSplitStorageInfoError> for ErrorWrapperForRpcSplitStorageInfoError {
    fn from(value: RpcSplitStorageInfoError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcSplitStorageInfoError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcStateChangesError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcStateChangesError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcStateChangesError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcStateChangesError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcStateChangesError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcStateChangesError> for ErrorWrapperForRpcStateChangesError {
    fn from(value: RpcStateChangesError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcStateChangesError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcStatusError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcStatusError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcStatusError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcStatusError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcStatusError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcStatusError> for ErrorWrapperForRpcStatusError {
    fn from(value: RpcStatusError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcStatusError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcTransactionError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcTransactionError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcTransactionError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcTransactionError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcTransactionError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcTransactionError> for ErrorWrapperForRpcTransactionError {
    fn from(value: RpcTransactionError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcTransactionError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ErrorWrapperForRpcValidatorError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcRequestValidationErrorKind"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_VALIDATION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/RpcValidatorError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "HANDLER_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "name"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/InternalError"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "cause")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum ErrorWrapperForRpcValidatorError {
    #[serde(rename = "REQUEST_VALIDATION_ERROR")]
    RequestValidationError(RpcRequestValidationErrorKind),
    #[serde(rename = "HANDLER_ERROR")]
    HandlerError(RpcValidatorError),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError(InternalError),
}
impl ::std::convert::From<RpcRequestValidationErrorKind> for ErrorWrapperForRpcValidatorError {
    fn from(value: RpcRequestValidationErrorKind) -> Self {
        Self::RequestValidationError(value)
    }
}
impl ::std::convert::From<RpcValidatorError> for ErrorWrapperForRpcValidatorError {
    fn from(value: RpcValidatorError) -> Self {
        Self::HandlerError(value)
    }
}
impl ::std::convert::From<InternalError> for ErrorWrapperForRpcValidatorError {
    fn from(value: InternalError) -> Self {
        Self::InternalError(value)
    }
}
///`ExecutionMetadataView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "version"
///  ],
///  "properties": {
///    "gas_profile": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/components/schemas/CostGasUsed"
///      }
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionMetadataView {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gas_profile: ::std::option::Option<::std::vec::Vec<CostGasUsed>>,
    pub version: u32,
}
///`ExecutionOutcomeView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "executor_id",
///    "gas_burnt",
///    "logs",
///    "receipt_ids",
///    "status",
///    "tokens_burnt"
///  ],
///  "properties": {
///    "executor_id": {
///      "description": "The id of the account on which the execution
/// happens. For transaction this is signer_id,\nfor receipt this is
/// receiver_id.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    },
///    "gas_burnt": {
///      "description": "The amount of the gas burnt by the given
/// transaction or receipt.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "logs": {
///      "description": "Logs from this transaction or receipt.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "metadata": {
///      "description": "Execution metadata, versioned",
///      "default": {
///        "version": 1
///      },
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExecutionMetadataView"
///        }
///      ]
///    },
///    "receipt_ids": {
///      "description": "Receipt IDs generated by this transaction or
/// receipt.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/CryptoHash"
///      }
///    },
///    "status": {
///      "description": "Execution status. Contains the result in case of
/// successful execution.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExecutionStatusView"
///        }
///      ]
///    },
///    "tokens_burnt": {
///      "description": "The amount of tokens burnt corresponding to the
/// burnt gas amount.\nThis value doesn't always equal to the `gas_burnt`
/// multiplied by the gas price, because\nthe prepaid gas price might be
/// lower than the actual gas price and it creates a
/// deficit.\n`tokens_burnt` also contains the penalty subtracted from
/// refunds, while\n`gas_burnt` only contains the gas that we actually burn
/// for the execution.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionOutcomeView {
    ///The id of the account on which the execution happens. For
    /// transaction this is signer_id, for receipt this is
    /// receiver_id.
    pub executor_id: AccountId,
    ///The amount of the gas burnt by the given transaction or receipt.
    pub gas_burnt: NearGas,
    ///Logs from this transaction or receipt.
    pub logs: ::std::vec::Vec<::std::string::String>,
    ///Execution metadata, versioned
    #[serde(default = "defaults::execution_outcome_view_metadata")]
    pub metadata: ExecutionMetadataView,
    ///Receipt IDs generated by this transaction or receipt.
    pub receipt_ids: ::std::vec::Vec<CryptoHash>,
    ///Execution status. Contains the result in case of successful
    /// execution.
    pub status: ExecutionStatusView,
    ///The amount of tokens burnt corresponding to the burnt gas amount.
    ///This value doesn't always equal to the `gas_burnt` multiplied by the
    /// gas price, because the prepaid gas price might be lower than
    /// the actual gas price and it creates a deficit.
    /// `tokens_burnt` also contains the penalty subtracted from refunds,
    /// while `gas_burnt` only contains the gas that we actually
    /// burn for the execution.
    pub tokens_burnt: NearToken,
}
///`ExecutionOutcomeWithIdView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_hash",
///    "id",
///    "outcome",
///    "proof"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "outcome": {
///      "$ref": "#/components/schemas/ExecutionOutcomeView"
///    },
///    "proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecutionOutcomeWithIdView {
    pub block_hash: CryptoHash,
    pub id: CryptoHash,
    pub outcome: ExecutionOutcomeView,
    pub proof: ::std::vec::Vec<MerklePathItem>,
}
///`ExecutionStatusView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The execution is pending or unknown.",
///      "type": "string",
///      "enum": [
///        "Unknown"
///      ]
///    },
///    {
///      "description": "The execution has failed.",
///      "type": "object",
///      "required": [
///        "Failure"
///      ],
///      "properties": {
///        "Failure": {
///          "$ref": "#/components/schemas/TxExecutionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The final action succeeded and returned some value
/// or an empty vec encoded in base64.",
///      "type": "object",
///      "required": [
///        "SuccessValue"
///      ],
///      "properties": {
///        "SuccessValue": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The final action of the receipt returned a promise
/// or the signed transaction was converted\nto a receipt. Contains the
/// receipt_id of the generated receipt.",
///      "type": "object",
///      "required": [
///        "SuccessReceiptId"
///      ],
///      "properties": {
///        "SuccessReceiptId": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ExecutionStatusView {
    ///The execution is pending or unknown.
    Unknown,
    ///The execution has failed.
    Failure(TxExecutionError),
    ///The final action succeeded and returned some value or an empty vec
    /// encoded in base64.
    SuccessValue(::std::string::String),
    ///The final action of the receipt returned a promise or the signed
    /// transaction was converted to a receipt. Contains the
    /// receipt_id of the generated receipt.
    SuccessReceiptId(CryptoHash),
}
impl ::std::convert::From<TxExecutionError> for ExecutionStatusView {
    fn from(value: TxExecutionError) -> Self {
        Self::Failure(value)
    }
}
///Typed view of ExtCostsConfig to preserve JSON output field names in
/// protocol config RPC output.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Typed view of ExtCostsConfig to preserve JSON output
/// field names in protocol\nconfig RPC output.",
///  "type": "object",
///  "properties": {
///    "alt_bn128_g1_multiexp_base": {
///      "description": "Base cost for multiexp",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_multiexp_element": {
///      "description": "Per element cost for multiexp",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_sum_base": {
///      "description": "Base cost for sum",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_g1_sum_element": {
///      "description": "Per element cost for sum",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_pairing_check_base": {
///      "description": "Base cost for pairing check",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "alt_bn128_pairing_check_element": {
///      "description": "Per element cost for pairing check",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "base": {
///      "description": "Base cost for calling a host function.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "bls12381_g1_multiexp_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_g1_multiexp_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_g2_multiexp_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_g2_multiexp_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_map_fp2_to_g2_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_map_fp2_to_g2_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_map_fp_to_g1_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_map_fp_to_g1_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p1_decompress_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p1_decompress_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p1_sum_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p1_sum_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p2_decompress_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p2_decompress_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p2_sum_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_p2_sum_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_pairing_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "bls12381_pairing_element": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "contract_compile_base": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "contract_compile_bytes": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "contract_loading_base": {
///      "description": "Base cost of loading a pre-compiled contract",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "contract_loading_bytes": {
///      "description": "Cost per byte of loading a pre-compiled contract",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "ecrecover_base": {
///      "description": "Cost of calling ecrecover",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "ed25519_verify_base": {
///      "description": "Cost of getting ed25519 base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "ed25519_verify_byte": {
///      "description": "Cost of getting ed25519 per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "keccak256_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "keccak256_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "keccak512_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "keccak512_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "log_base": {
///      "description": "Cost for calling logging.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "log_byte": {
///      "description": "Cost for logging per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "promise_and_base": {
///      "description": "Cost for calling `promise_and`",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "promise_and_per_promise": {
///      "description": "Cost for calling `promise_and` for each promise",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "promise_return": {
///      "description": "Cost for calling `promise_return`",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "read_cached_trie_node": {
///      "description": "Cost for reading trie node from memory",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "read_memory_base": {
///      "description": "Base cost for guest memory read",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "read_memory_byte": {
///      "description": "Cost for guest memory read",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "read_register_base": {
///      "description": "Base cost for reading from register",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "read_register_byte": {
///      "description": "Cost for reading byte from register",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "ripemd160_base": {
///      "description": "Cost of getting ripemd160 base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "ripemd160_block": {
///      "description": "Cost of getting ripemd160 per message block",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "sha256_base": {
///      "description": "Cost of getting sha256 base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "sha256_byte": {
///      "description": "Cost of getting sha256 per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_has_key_base": {
///      "description": "Storage trie check for key existence cost base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_has_key_byte": {
///      "description": "Storage trie check for key existence per key byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_from_byte": {
///      "description": "Create trie range iterator cost per byte of from
/// key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_prefix_base": {
///      "description": "Create trie prefix iterator cost base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_prefix_byte": {
///      "description": "Create trie prefix iterator cost per byte.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_range_base": {
///      "description": "Create trie range iterator cost base",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_create_to_byte": {
///      "description": "Create trie range iterator cost per byte of to
/// key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_base": {
///      "description": "Trie iterator per key base cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_key_byte": {
///      "description": "Trie iterator next key byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_iter_next_value_byte": {
///      "description": "Trie iterator next key byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_large_read_overhead_base": {
///      "description": "Storage trie read key overhead base cost, when
/// doing large reads",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_large_read_overhead_byte": {
///      "description": "Storage trie read key overhead  per-byte cost, when
/// doing large reads",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_read_base": {
///      "description": "Storage trie read key base cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_read_key_byte": {
///      "description": "Storage trie read key per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_read_value_byte": {
///      "description": "Storage trie read value cost per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_remove_base": {
///      "description": "Remove key from trie base cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_remove_key_byte": {
///      "description": "Remove key from trie per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_remove_ret_value_byte": {
///      "description": "Remove key from trie ret value byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_write_base": {
///      "description": "Storage trie write key base cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_write_evicted_byte": {
///      "description": "Storage trie write cost per byte of evicted
/// value.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_write_key_byte": {
///      "description": "Storage trie write key per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "storage_write_value_byte": {
///      "description": "Storage trie write value per byte cost",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "touching_trie_node": {
///      "description": "Cost per reading trie node from DB",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "utf16_decoding_base": {
///      "description": "Base cost of decoding utf16. It's used for
/// `log_utf16`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "utf16_decoding_byte": {
///      "description": "Cost per byte of decoding utf16. It's used for
/// `log_utf16`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "utf8_decoding_base": {
///      "description": "Base cost of decoding utf8. It's used for
/// `log_utf8` and `panic_utf8`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "utf8_decoding_byte": {
///      "description": "Cost per byte of decoding utf8. It's used for
/// `log_utf8` and `panic_utf8`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "validator_stake_base": {
///      "description": "Cost of calling `validator_stake`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "validator_total_stake_base": {
///      "description": "Cost of calling `validator_total_stake`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "write_memory_base": {
///      "description": "Base cost for guest memory write",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "write_memory_byte": {
///      "description": "Cost for guest memory write per byte",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "write_register_base": {
///      "description": "Base cost for writing into register",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "write_register_byte": {
///      "description": "Cost for writing byte into register",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "yield_create_base": {
///      "description": "Base cost for creating a yield promise.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "yield_create_byte": {
///      "description": "Per byte cost of arguments and method name.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "yield_resume_base": {
///      "description": "Base cost for resuming a yield receipt.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "yield_resume_byte": {
///      "description": "Per byte cost of resume payload.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExtCostsConfigView {
    ///Base cost for multiexp
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_g1_multiexp_base: ::std::option::Option<NearGas>,
    ///Per element cost for multiexp
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_g1_multiexp_element: ::std::option::Option<NearGas>,
    ///Base cost for sum
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_g1_sum_base: ::std::option::Option<NearGas>,
    ///Per element cost for sum
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_g1_sum_element: ::std::option::Option<NearGas>,
    ///Base cost for pairing check
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_pairing_check_base: ::std::option::Option<NearGas>,
    ///Per element cost for pairing check
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub alt_bn128_pairing_check_element: ::std::option::Option<NearGas>,
    ///Base cost for calling a host function.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_g1_multiexp_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_g1_multiexp_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_g2_multiexp_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_g2_multiexp_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_map_fp2_to_g2_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_map_fp2_to_g2_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_map_fp_to_g1_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_map_fp_to_g1_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p1_decompress_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p1_decompress_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p1_sum_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p1_sum_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p2_decompress_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p2_decompress_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p2_sum_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_p2_sum_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_pairing_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub bls12381_pairing_element: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_compile_base: ::std::option::Option<NearGas>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_compile_bytes: ::std::option::Option<NearGas>,
    ///Base cost of loading a pre-compiled contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_loading_base: ::std::option::Option<NearGas>,
    ///Cost per byte of loading a pre-compiled contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub contract_loading_bytes: ::std::option::Option<NearGas>,
    ///Cost of calling ecrecover
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ecrecover_base: ::std::option::Option<NearGas>,
    ///Cost of getting ed25519 base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ed25519_verify_base: ::std::option::Option<NearGas>,
    ///Cost of getting ed25519 per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ed25519_verify_byte: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keccak256_base: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keccak256_byte: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keccak512_base: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub keccak512_byte: ::std::option::Option<NearGas>,
    ///Cost for calling logging.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_base: ::std::option::Option<NearGas>,
    ///Cost for logging per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_byte: ::std::option::Option<NearGas>,
    ///Cost for calling `promise_and`
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub promise_and_base: ::std::option::Option<NearGas>,
    ///Cost for calling `promise_and` for each promise
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub promise_and_per_promise: ::std::option::Option<NearGas>,
    ///Cost for calling `promise_return`
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub promise_return: ::std::option::Option<NearGas>,
    ///Cost for reading trie node from memory
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_cached_trie_node: ::std::option::Option<NearGas>,
    ///Base cost for guest memory read
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_memory_base: ::std::option::Option<NearGas>,
    ///Cost for guest memory read
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_memory_byte: ::std::option::Option<NearGas>,
    ///Base cost for reading from register
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_register_base: ::std::option::Option<NearGas>,
    ///Cost for reading byte from register
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_register_byte: ::std::option::Option<NearGas>,
    ///Cost of getting ripemd160 base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ripemd160_base: ::std::option::Option<NearGas>,
    ///Cost of getting ripemd160 per message block
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ripemd160_block: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sha256_base: ::std::option::Option<NearGas>,
    ///Cost of getting sha256 per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sha256_byte: ::std::option::Option<NearGas>,
    ///Storage trie check for key existence cost base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_has_key_base: ::std::option::Option<NearGas>,
    ///Storage trie check for key existence per key byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_has_key_byte: ::std::option::Option<NearGas>,
    ///Create trie range iterator cost per byte of from key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_create_from_byte: ::std::option::Option<NearGas>,
    ///Create trie prefix iterator cost base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_create_prefix_base: ::std::option::Option<NearGas>,
    ///Create trie prefix iterator cost per byte.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_create_prefix_byte: ::std::option::Option<NearGas>,
    ///Create trie range iterator cost base
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_create_range_base: ::std::option::Option<NearGas>,
    ///Create trie range iterator cost per byte of to key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_create_to_byte: ::std::option::Option<NearGas>,
    ///Trie iterator per key base cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_next_base: ::std::option::Option<NearGas>,
    ///Trie iterator next key byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_next_key_byte: ::std::option::Option<NearGas>,
    ///Trie iterator next key byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_iter_next_value_byte: ::std::option::Option<NearGas>,
    ///Storage trie read key overhead base cost, when doing large reads
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_large_read_overhead_base: ::std::option::Option<NearGas>,
    ///Storage trie read key overhead  per-byte cost, when doing large
    /// reads
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_large_read_overhead_byte: ::std::option::Option<NearGas>,
    ///Storage trie read key base cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_read_base: ::std::option::Option<NearGas>,
    ///Storage trie read key per byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_read_key_byte: ::std::option::Option<NearGas>,
    ///Storage trie read value cost per byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_read_value_byte: ::std::option::Option<NearGas>,
    ///Remove key from trie base cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_remove_base: ::std::option::Option<NearGas>,
    ///Remove key from trie per byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_remove_key_byte: ::std::option::Option<NearGas>,
    ///Remove key from trie ret value byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_remove_ret_value_byte: ::std::option::Option<NearGas>,
    ///Storage trie write key base cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_write_base: ::std::option::Option<NearGas>,
    ///Storage trie write cost per byte of evicted value.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_write_evicted_byte: ::std::option::Option<NearGas>,
    ///Storage trie write key per byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_write_key_byte: ::std::option::Option<NearGas>,
    ///Storage trie write value per byte cost
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_write_value_byte: ::std::option::Option<NearGas>,
    ///Cost per reading trie node from DB
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub touching_trie_node: ::std::option::Option<NearGas>,
    ///Base cost of decoding utf16. It's used for `log_utf16`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub utf16_decoding_base: ::std::option::Option<NearGas>,
    ///Cost per byte of decoding utf16. It's used for `log_utf16`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub utf16_decoding_byte: ::std::option::Option<NearGas>,
    ///Base cost of decoding utf8. It's used for `log_utf8` and
    /// `panic_utf8`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub utf8_decoding_base: ::std::option::Option<NearGas>,
    ///Cost per byte of decoding utf8. It's used for `log_utf8` and
    /// `panic_utf8`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub utf8_decoding_byte: ::std::option::Option<NearGas>,
    ///Cost of calling `validator_stake`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_stake_base: ::std::option::Option<NearGas>,
    ///Cost of calling `validator_total_stake`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_total_stake_base: ::std::option::Option<NearGas>,
    ///Base cost for guest memory write
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_memory_base: ::std::option::Option<NearGas>,
    ///Cost for guest memory write per byte
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_memory_byte: ::std::option::Option<NearGas>,
    ///Base cost for writing into register
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_register_base: ::std::option::Option<NearGas>,
    ///Cost for writing byte into register
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_register_byte: ::std::option::Option<NearGas>,
    ///Base cost for creating a yield promise.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yield_create_base: ::std::option::Option<NearGas>,
    ///Per byte cost of arguments and method name.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yield_create_byte: ::std::option::Option<NearGas>,
    ///Base cost for resuming a yield receipt.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yield_resume_base: ::std::option::Option<NearGas>,
    ///Per byte cost of resume payload.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yield_resume_byte: ::std::option::Option<NearGas>,
}
impl ::std::default::Default for ExtCostsConfigView {
    fn default() -> Self {
        Self {
            alt_bn128_g1_multiexp_base: Default::default(),
            alt_bn128_g1_multiexp_element: Default::default(),
            alt_bn128_g1_sum_base: Default::default(),
            alt_bn128_g1_sum_element: Default::default(),
            alt_bn128_pairing_check_base: Default::default(),
            alt_bn128_pairing_check_element: Default::default(),
            base: Default::default(),
            bls12381_g1_multiexp_base: Default::default(),
            bls12381_g1_multiexp_element: Default::default(),
            bls12381_g2_multiexp_base: Default::default(),
            bls12381_g2_multiexp_element: Default::default(),
            bls12381_map_fp2_to_g2_base: Default::default(),
            bls12381_map_fp2_to_g2_element: Default::default(),
            bls12381_map_fp_to_g1_base: Default::default(),
            bls12381_map_fp_to_g1_element: Default::default(),
            bls12381_p1_decompress_base: Default::default(),
            bls12381_p1_decompress_element: Default::default(),
            bls12381_p1_sum_base: Default::default(),
            bls12381_p1_sum_element: Default::default(),
            bls12381_p2_decompress_base: Default::default(),
            bls12381_p2_decompress_element: Default::default(),
            bls12381_p2_sum_base: Default::default(),
            bls12381_p2_sum_element: Default::default(),
            bls12381_pairing_base: Default::default(),
            bls12381_pairing_element: Default::default(),
            contract_compile_base: Default::default(),
            contract_compile_bytes: Default::default(),
            contract_loading_base: Default::default(),
            contract_loading_bytes: Default::default(),
            ecrecover_base: Default::default(),
            ed25519_verify_base: Default::default(),
            ed25519_verify_byte: Default::default(),
            keccak256_base: Default::default(),
            keccak256_byte: Default::default(),
            keccak512_base: Default::default(),
            keccak512_byte: Default::default(),
            log_base: Default::default(),
            log_byte: Default::default(),
            promise_and_base: Default::default(),
            promise_and_per_promise: Default::default(),
            promise_return: Default::default(),
            read_cached_trie_node: Default::default(),
            read_memory_base: Default::default(),
            read_memory_byte: Default::default(),
            read_register_base: Default::default(),
            read_register_byte: Default::default(),
            ripemd160_base: Default::default(),
            ripemd160_block: Default::default(),
            sha256_base: Default::default(),
            sha256_byte: Default::default(),
            storage_has_key_base: Default::default(),
            storage_has_key_byte: Default::default(),
            storage_iter_create_from_byte: Default::default(),
            storage_iter_create_prefix_base: Default::default(),
            storage_iter_create_prefix_byte: Default::default(),
            storage_iter_create_range_base: Default::default(),
            storage_iter_create_to_byte: Default::default(),
            storage_iter_next_base: Default::default(),
            storage_iter_next_key_byte: Default::default(),
            storage_iter_next_value_byte: Default::default(),
            storage_large_read_overhead_base: Default::default(),
            storage_large_read_overhead_byte: Default::default(),
            storage_read_base: Default::default(),
            storage_read_key_byte: Default::default(),
            storage_read_value_byte: Default::default(),
            storage_remove_base: Default::default(),
            storage_remove_key_byte: Default::default(),
            storage_remove_ret_value_byte: Default::default(),
            storage_write_base: Default::default(),
            storage_write_evicted_byte: Default::default(),
            storage_write_key_byte: Default::default(),
            storage_write_value_byte: Default::default(),
            touching_trie_node: Default::default(),
            utf16_decoding_base: Default::default(),
            utf16_decoding_byte: Default::default(),
            utf8_decoding_base: Default::default(),
            utf8_decoding_byte: Default::default(),
            validator_stake_base: Default::default(),
            validator_total_stake_base: Default::default(),
            write_memory_base: Default::default(),
            write_memory_byte: Default::default(),
            write_register_base: Default::default(),
            write_register_byte: Default::default(),
            yield_create_base: Default::default(),
            yield_create_byte: Default::default(),
            yield_resume_base: Default::default(),
            yield_resume_byte: Default::default(),
        }
    }
}
///`ExternalStorageConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "external_storage_fallback_threshold": {
///      "description": "The number of attempts the node will make to obtain
/// a part from peers in\nthe network before it fetches from external
/// storage.",
///      "default": 3,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "location": {
///      "description": "Location of state parts.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExternalStorageLocation"
///        }
///      ]
///    },
///    "num_concurrent_requests": {
///      "description": "When fetching state parts from external storage,
/// throttle fetch requests\nto this many concurrent requests.",
///      "default": 25,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "num_concurrent_requests_during_catchup": {
///      "description": "During catchup, the node will use a different
/// number of concurrent requests\nto reduce the performance impact of state
/// sync.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExternalStorageConfig {
    ///The number of attempts the node will make to obtain a part from
    /// peers in the network before it fetches from external
    /// storage.
    #[serde(default = "defaults::default_u64::<u64, 3>")]
    pub external_storage_fallback_threshold: u64,
    ///Location of state parts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub location: ::std::option::Option<ExternalStorageLocation>,
    ///When fetching state parts from external storage, throttle fetch
    /// requests to this many concurrent requests.
    #[serde(default = "defaults::default_u64::<u8, 25>")]
    pub num_concurrent_requests: u8,
    ///During catchup, the node will use a different number of concurrent
    /// requests to reduce the performance impact of state sync.
    #[serde(default = "defaults::default_u64::<u8, 5>")]
    pub num_concurrent_requests_during_catchup: u8,
}
impl ::std::default::Default for ExternalStorageConfig {
    fn default() -> Self {
        Self {
            external_storage_fallback_threshold: defaults::default_u64::<u64, 3>(),
            location: Default::default(),
            num_concurrent_requests: defaults::default_u64::<u8, 25>(),
            num_concurrent_requests_during_catchup: defaults::default_u64::<u8, 5>(),
        }
    }
}
///Supported external storage backends and their minimal config.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Supported external storage backends and their minimal
/// config.",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "S3"
///      ],
///      "properties": {
///        "S3": {
///          "type": "object",
///          "required": [
///            "bucket",
///            "region"
///          ],
///          "properties": {
///            "bucket": {
///              "description": "Location on S3.",
///              "type": "string"
///            },
///            "region": {
///              "description": "Data may only be available in certain
/// locations.",
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Local filesystem root for storing data.",
///      "type": "object",
///      "required": [
///        "Filesystem"
///      ],
///      "properties": {
///        "Filesystem": {
///          "type": "object",
///          "required": [
///            "root_dir"
///          ],
///          "properties": {
///            "root_dir": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Google Cloud Storage bucket name.",
///      "type": "object",
///      "required": [
///        "GCS"
///      ],
///      "properties": {
///        "GCS": {
///          "type": "object",
///          "required": [
///            "bucket"
///          ],
///          "properties": {
///            "bucket": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ExternalStorageLocation {
    S3 {
        ///Location on S3.
        bucket: ::std::string::String,
        ///Data may only be available in certain locations.
        region: ::std::string::String,
    },
    ///Local filesystem root for storing data.
    Filesystem { root_dir: ::std::string::String },
    ///Google Cloud Storage bucket name.
    #[serde(rename = "GCS")]
    Gcs { bucket: ::std::string::String },
}
///Costs associated with an object that can only be sent over the network
/// (and executed by the receiver).
///NOTE: `send_sir` or `send_not_sir` fees are usually burned when the item
/// is being created. And `execution` fee is burned when the item is
/// being executed.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Costs associated with an object that can only be sent
/// over the network (and executed\nby the receiver).\nNOTE: `send_sir` or
/// `send_not_sir` fees are usually burned when the item is being
/// created.\nAnd `execution` fee is burned when the item is being
/// executed.",
///  "type": "object",
///  "required": [
///    "execution",
///    "send_not_sir",
///    "send_sir"
///  ],
///  "properties": {
///    "execution": {
///      "description": "Fee for executing the object.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "send_not_sir": {
///      "description": "Fee for sending an object potentially across the
/// shards.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "send_sir": {
///      "description": "Fee for sending an object from the sender to
/// itself, guaranteeing that it does not leave\nthe shard.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Fee {
    ///Fee for executing the object.
    pub execution: NearGas,
    ///Fee for sending an object potentially across the shards.
    pub send_not_sir: NearGas,
    ///Fee for sending an object from the sender to itself, guaranteeing
    /// that it does not leave the shard.
    pub send_sir: NearGas,
}
///Execution outcome of the transaction and all the subsequent receipts.
///Could be not finalized yet
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Execution outcome of the transaction and all the
/// subsequent receipts.\nCould be not finalized yet",
///  "type": "object",
///  "required": [
///    "receipts_outcome",
///    "status",
///    "transaction",
///    "transaction_outcome"
///  ],
///  "properties": {
///    "receipts_outcome": {
///      "description": "The execution outcome of receipts.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
///      }
///    },
///    "status": {
///      "description": "Execution status defined by
/// chain.rs:get_final_transaction_result\nFinalExecutionStatus::NotStarted
/// - the tx is not converted to the receipt
/// yet\nFinalExecutionStatus::Started - we have at least 1 receipt, but the
/// first leaf receipt_id (using dfs) hasn't finished the
/// execution\nFinalExecutionStatus::Failure - the result of the first leaf
/// receipt_id\nFinalExecutionStatus::SuccessValue - the result of the first
/// leaf receipt_id",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/FinalExecutionStatus"
///        }
///      ]
///    },
///    "transaction": {
///      "description": "Signed Transaction",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/SignedTransactionView"
///        }
///      ]
///    },
///    "transaction_outcome": {
///      "description": "The execution outcome of the signed transaction.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FinalExecutionOutcomeView {
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    ///Execution status defined by chain.rs:get_final_transaction_result
    ///FinalExecutionStatus::NotStarted - the tx is not converted to the
    /// receipt yet FinalExecutionStatus::Started - we have at least
    /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished
    /// the execution FinalExecutionStatus::Failure - the result of
    /// the first leaf receipt_id FinalExecutionStatus::SuccessValue
    /// - the result of the first leaf receipt_id
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
///Final execution outcome of the transaction and all of subsequent the
/// receipts. Also includes the generated receipt.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Final execution outcome of the transaction and all of
/// subsequent the receipts. Also includes\nthe generated receipt.",
///  "type": "object",
///  "required": [
///    "receipts",
///    "receipts_outcome",
///    "status",
///    "transaction",
///    "transaction_outcome"
///  ],
///  "properties": {
///    "receipts": {
///      "description": "Receipts generated from the transaction",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ReceiptView"
///      }
///    },
///    "receipts_outcome": {
///      "description": "The execution outcome of receipts.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
///      }
///    },
///    "status": {
///      "description": "Execution status defined by
/// chain.rs:get_final_transaction_result\nFinalExecutionStatus::NotStarted
/// - the tx is not converted to the receipt
/// yet\nFinalExecutionStatus::Started - we have at least 1 receipt, but the
/// first leaf receipt_id (using dfs) hasn't finished the
/// execution\nFinalExecutionStatus::Failure - the result of the first leaf
/// receipt_id\nFinalExecutionStatus::SuccessValue - the result of the first
/// leaf receipt_id",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/FinalExecutionStatus"
///        }
///      ]
///    },
///    "transaction": {
///      "description": "Signed Transaction",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/SignedTransactionView"
///        }
///      ]
///    },
///    "transaction_outcome": {
///      "description": "The execution outcome of the signed transaction.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FinalExecutionOutcomeWithReceiptView {
    ///Receipts generated from the transaction
    pub receipts: ::std::vec::Vec<ReceiptView>,
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    ///Execution status defined by chain.rs:get_final_transaction_result
    ///FinalExecutionStatus::NotStarted - the tx is not converted to the
    /// receipt yet FinalExecutionStatus::Started - we have at least
    /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished
    /// the execution FinalExecutionStatus::Failure - the result of
    /// the first leaf receipt_id FinalExecutionStatus::SuccessValue
    /// - the result of the first leaf receipt_id
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
///`FinalExecutionStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The execution has not yet started.",
///      "type": "string",
///      "enum": [
///        "NotStarted"
///      ]
///    },
///    {
///      "description": "The execution has started and still going.",
///      "type": "string",
///      "enum": [
///        "Started"
///      ]
///    },
///    {
///      "description": "The execution has failed with the given error.",
///      "type": "object",
///      "required": [
///        "Failure"
///      ],
///      "properties": {
///        "Failure": {
///          "$ref": "#/components/schemas/TxExecutionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The execution has succeeded and returned some value
/// or an empty vec encoded in base64.",
///      "type": "object",
///      "required": [
///        "SuccessValue"
///      ],
///      "properties": {
///        "SuccessValue": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum FinalExecutionStatus {
    ///The execution has not yet started.
    NotStarted,
    ///The execution has started and still going.
    Started,
    ///The execution has failed with the given error.
    Failure(TxExecutionError),
    ///The execution has succeeded and returned some value or an empty vec
    /// encoded in base64.
    SuccessValue(::std::string::String),
}
impl ::std::convert::From<TxExecutionError> for FinalExecutionStatus {
    fn from(value: TxExecutionError) -> Self {
        Self::Failure(value)
    }
}
///Different types of finality.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Different types of finality.",
///  "type": "string",
///  "enum": [
///    "optimistic",
///    "near-final",
///    "final"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Finality {
    #[serde(rename = "optimistic")]
    Optimistic,
    #[serde(rename = "near-final")]
    NearFinal,
    #[serde(rename = "final")]
    Final,
}
impl ::std::fmt::Display for Finality {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Optimistic => f.write_str("optimistic"),
            Self::NearFinal => f.write_str("near-final"),
            Self::Final => f.write_str("final"),
        }
    }
}
impl ::std::str::FromStr for Finality {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "optimistic" => Ok(Self::Optimistic),
            "near-final" => Ok(Self::NearFinal),
            "final" => Ok(Self::Final),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Finality {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///This type is used to mark function arguments.
///
///NOTE: The main reason for this to exist (except the type-safety) is that
/// the value is transparently serialized and deserialized as a
/// base64-encoded string when serde is used (serde_json).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This type is used to mark function arguments.\n\nNOTE:
/// The main reason for this to exist (except the type-safety) is that the
/// value is\ntransparently serialized and deserialized as a base64-encoded
/// string when serde is used\n(serde_json).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct FunctionArgs(pub ::std::string::String);
impl ::std::ops::Deref for FunctionArgs {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for FunctionArgs {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for FunctionArgs {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for FunctionArgs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`FunctionCallAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "args",
///    "deposit",
///    "gas",
///    "method_name"
///  ],
///  "properties": {
///    "args": {
///      "type": "string"
///    },
///    "deposit": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "gas": {
///      "$ref": "#/components/schemas/NearGas"
///    },
///    "method_name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FunctionCallAction {
    pub args: ::std::string::String,
    pub deposit: NearToken,
    pub gas: NearGas,
    pub method_name: ::std::string::String,
}
///Serializable version of `near-vm-runner::FunctionCallError`.
///
///Must never reorder/remove elements, can only add new variants at the end
/// (but do that very carefully). It describes stable serialization
/// format, and only used by serialization logic.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Serializable version of
/// `near-vm-runner::FunctionCallError`.\n\nMust never reorder/remove
/// elements, can only add new variants at the end (but do that
/// very\ncarefully). It describes stable serialization format, and only
/// used by serialization logic.",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "WasmUnknownError",
///        "_EVMError"
///      ]
///    },
///    {
///      "description": "Wasm compilation error",
///      "type": "object",
///      "required": [
///        "CompilationError"
///      ],
///      "properties": {
///        "CompilationError": {
///          "$ref": "#/components/schemas/CompilationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Wasm binary env link error\n\nNote: this is only to
/// deserialize old data, use execution error for new data",
///      "type": "object",
///      "required": [
///        "LinkError"
///      ],
///      "properties": {
///        "LinkError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Import/export resolve error",
///      "type": "object",
///      "required": [
///        "MethodResolveError"
///      ],
///      "properties": {
///        "MethodResolveError": {
///          "$ref": "#/components/schemas/MethodResolveError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "A trap happened during execution of a
/// binary\n\nNote: this is only to deserialize old data, use execution
/// error for new data",
///      "type": "object",
///      "required": [
///        "WasmTrap"
///      ],
///      "properties": {
///        "WasmTrap": {
///          "$ref": "#/components/schemas/WasmTrap"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Note: this is only to deserialize old data, use
/// execution error for new data",
///      "type": "object",
///      "required": [
///        "HostError"
///      ],
///      "properties": {
///        "HostError": {
///          "$ref": "#/components/schemas/HostError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "ExecutionError"
///      ],
///      "properties": {
///        "ExecutionError": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum FunctionCallError {
    WasmUnknownError,
    #[serde(rename = "_EVMError")]
    EvmError,
    ///Wasm compilation error
    CompilationError(CompilationError),
    ///Wasm binary env link error
    ///
    ///Note: this is only to deserialize old data, use execution error for
    /// new data
    LinkError {
        msg: ::std::string::String,
    },
    ///Import/export resolve error
    MethodResolveError(MethodResolveError),
    ///A trap happened during execution of a binary
    ///
    ///Note: this is only to deserialize old data, use execution error for
    /// new data
    WasmTrap(WasmTrap),
    ///Note: this is only to deserialize old data, use execution error for
    /// new data
    HostError(HostError),
    ExecutionError(::std::string::String),
}
impl ::std::convert::From<CompilationError> for FunctionCallError {
    fn from(value: CompilationError) -> Self {
        Self::CompilationError(value)
    }
}
impl ::std::convert::From<MethodResolveError> for FunctionCallError {
    fn from(value: MethodResolveError) -> Self {
        Self::MethodResolveError(value)
    }
}
impl ::std::convert::From<WasmTrap> for FunctionCallError {
    fn from(value: WasmTrap) -> Self {
        Self::WasmTrap(value)
    }
}
impl ::std::convert::From<HostError> for FunctionCallError {
    fn from(value: HostError) -> Self {
        Self::HostError(value)
    }
}
///Grants limited permission to make transactions with FunctionCallActions
///The permission can limit the allowed balance to be spent on the prepaid
/// gas. It also restrict the account ID of the receiver for this
/// function call. It also can restrict the method name for the allowed
/// function calls.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Grants limited permission to make transactions with
/// FunctionCallActions\nThe permission can limit the allowed balance to be
/// spent on the prepaid gas.\nIt also restrict the account ID of the
/// receiver for this function call.\nIt also can restrict the method name
/// for the allowed function calls.",
///  "type": "object",
///  "required": [
///    "method_names",
///    "receiver_id"
///  ],
///  "properties": {
///    "allowance": {
///      "description": "Allowance is a balance limit to use by this access
/// key to pay for function call gas and\ntransaction fees. When this access
/// key is used, both account balance and the allowance is\ndecreased by the
/// same value.\n`None` means unlimited allowance.\nNOTE: To change or
/// increase the allowance, the old access key needs to be deleted and a
/// new\naccess key should be created.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "method_names": {
///      "description": "A list of method names that can be used. The access
/// key only allows transactions with the\nfunction call of one of the given
/// method names.\nEmpty list means any method name can be used.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "receiver_id": {
///      "description": "The access key only allows transactions with the
/// given receiver's account id.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FunctionCallPermission {
    ///Allowance is a balance limit to use by this access key to pay for
    /// function call gas and transaction fees. When this access key
    /// is used, both account balance and the allowance is decreased
    /// by the same value. `None` means unlimited allowance.
    ///NOTE: To change or increase the allowance, the old access key needs
    /// to be deleted and a new access key should be created.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub allowance: ::std::option::Option<NearToken>,
    ///A list of method names that can be used. The access key only allows
    /// transactions with the function call of one of the given
    /// method names. Empty list means any method name can be used.
    pub method_names: ::std::vec::Vec<::std::string::String>,
    ///The access key only allows transactions with the given receiver's
    /// account id.
    pub receiver_id: ::std::string::String,
}
///Gas key is like an access key, except it stores a balance separately,
/// and transactions signed with it deduct their cost from the gas key
/// balance instead of the account balance.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Gas key is like an access key, except it stores a
/// balance separately, and transactions signed\nwith it deduct their cost
/// from the gas key balance instead of the account balance.",
///  "type": "object",
///  "required": [
///    "balance",
///    "num_nonces",
///    "permission"
///  ],
///  "properties": {
///    "balance": {
///      "description": "The balance of the gas key.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "num_nonces": {
///      "description": "The number of nonces this gas key has.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "permission": {
///      "description": "Defines the permissions for this gas key.\nIf this is a `FunctionCallPermission`, the allowance must be None (unlimited).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKeyPermission"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GasKey {
    ///The balance of the gas key.
    pub balance: NearToken,
    ///The number of nonces this gas key has.
    pub num_nonces: u32,
    ///Defines the permissions for this gas key.
    ///If this is a `FunctionCallPermission`, the allowance must be None
    /// (unlimited).
    pub permission: AccessKeyPermission,
}
///`GasKeyInfoView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "gas_key",
///    "public_key"
///  ],
///  "properties": {
///    "gas_key": {
///      "$ref": "#/components/schemas/GasKeyView"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GasKeyInfoView {
    pub gas_key: GasKeyView,
    pub public_key: PublicKey,
}
///`GasKeyList`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "keys"
///  ],
///  "properties": {
///    "keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/GasKeyInfoView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GasKeyList {
    pub keys: ::std::vec::Vec<GasKeyInfoView>,
}
///`GasKeyView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "balance",
///    "nonces",
///    "num_nonces",
///    "permission"
///  ],
///  "properties": {
///    "balance": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "nonces": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_nonces": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "permission": {
///      "$ref": "#/components/schemas/AccessKeyPermissionView"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GasKeyView {
    pub balance: NearToken,
    pub nonces: ::std::vec::Vec<u64>,
    pub num_nonces: u32,
    pub permission: AccessKeyPermissionView,
}
///Configuration for garbage collection.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration for garbage collection.",
///  "type": "object",
///  "properties": {
///    "gc_blocks_limit": {
///      "description": "Maximum number of blocks to garbage collect at
/// every garbage collection\ncall.",
///      "default": 2,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_fork_clean_step": {
///      "description": "Maximum number of height to go through at each
/// garbage collection step\nwhen cleaning forks during garbage
/// collection.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_num_epochs_to_keep": {
///      "description": "Number of epochs for which we keep store data.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "gc_step_period": {
///      "description": "How often gc should be run",
///      "default": {
///        "nanos": 500000000,
///        "secs": 0
///      },
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/DurationAsStdSchemaProvider"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GcConfig {
    ///Maximum number of blocks to garbage collect at every garbage
    /// collection call.
    #[serde(default = "defaults::default_u64::<u64, 2>")]
    pub gc_blocks_limit: u64,
    ///Maximum number of height to go through at each garbage collection
    /// step when cleaning forks during garbage collection.
    #[serde(default = "defaults::default_u64::<u64, 100>")]
    pub gc_fork_clean_step: u64,
    ///Number of epochs for which we keep store data.
    #[serde(default = "defaults::default_u64::<u64, 5>")]
    pub gc_num_epochs_to_keep: u64,
    ///How often gc should be run
    #[serde(default = "defaults::gc_config_gc_step_period")]
    pub gc_step_period: DurationAsStdSchemaProvider,
}
impl ::std::default::Default for GcConfig {
    fn default() -> Self {
        Self {
            gc_blocks_limit: defaults::default_u64::<u64, 2>(),
            gc_fork_clean_step: defaults::default_u64::<u64, 100>(),
            gc_num_epochs_to_keep: defaults::default_u64::<u64, 5>(),
            gc_step_period: defaults::gc_config_gc_step_period(),
        }
    }
}
///`GenesisConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "avg_hidden_validator_seats_per_shard",
///    "block_producer_kickout_threshold",
///    "chain_id",
///    "chunk_producer_kickout_threshold",
///    "dynamic_resharding",
///    "epoch_length",
///    "fishermen_threshold",
///    "gas_limit",
///    "gas_price_adjustment_rate",
///    "genesis_height",
///    "genesis_time",
///    "max_gas_price",
///    "max_inflation_rate",
///    "min_gas_price",
///    "num_block_producer_seats",
///    "num_block_producer_seats_per_shard",
///    "num_blocks_per_year",
///    "protocol_reward_rate",
///    "protocol_treasury_account",
///    "protocol_version",
///    "total_supply",
///    "transaction_validity_period",
///    "validators"
///  ],
///  "properties": {
///    "avg_hidden_validator_seats_per_shard": {
///      "description": "Expected number of hidden validators per shard.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "block_producer_kickout_threshold": {
///      "description": "Threshold for kicking out block producers, between
/// 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chain_id": {
///      "description": "ID of the blockchain. This must be unique for every
/// blockchain.\nIf your testnet blockchains do not have unique chain IDs,
/// you will have a bad time.",
///      "type": "string"
///    },
///    "chunk_producer_assignment_changes_limit": {
///      "description": "Limits the number of shard changes in chunk
/// producer assignments,\nif algorithm is able to choose assignment with
/// better balance of\nnumber of chunk producers for shards.",
///      "default": 5,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "chunk_producer_kickout_threshold": {
///      "description": "Threshold for kicking out chunk producers, between
/// 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chunk_validator_only_kickout_threshold": {
///      "description": "Threshold for kicking out nodes which are only
/// chunk validators, between 0 and 100.",
///      "default": 80,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "dynamic_resharding": {
///      "description": "Enable dynamic re-sharding.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length counted in block heights.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fishermen_threshold": {
///      "description": "Fishermen stake threshold.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "gas_limit": {
///      "description": "Initial gas limit.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "gas_price_adjustment_rate": {
///      "description": "Gas price adjustment rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "genesis_height": {
///      "description": "Height of genesis block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "genesis_time": {
///      "description": "Official time of blockchain start.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "max_gas_price": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "max_inflation_rate": {
///      "description": "Maximum inflation on the total supply every
/// epoch.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_kickout_stake_perc": {
///      "description": "Max stake percentage of the validators we will kick
/// out.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "min_gas_price": {
///      "description": "Minimum gas price. It is also the initial gas
/// price.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "minimum_stake_divisor": {
///      "description": "The minimum stake required for staking is last seat
/// price divided by this number.",
///      "default": 10,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "minimum_stake_ratio": {
///      "description": "The lowest ratio s/s_total any block producer can have.\nSee <https://github.com/near/NEPs/pull/167> for details",
///      "default": [
///        1,
///        6250
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "minimum_validators_per_shard": {
///      "description": "The minimum number of validators each shard must
/// have",
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats at genesis.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats_per_shard": {
///      "description": "Defines number of shards and number of block
/// producer seats per each shard at genesis.\nNote: not used with
/// protocol_feature_chunk_only_producers -- replaced by
/// minimum_validators_per_shard\nNote: not used before as all block
/// producers produce chunks for all shards",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_blocks_per_year": {
///      "description": "Expected number of blocks per year",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_only_producer_seats": {
///      "description": "Deprecated.",
///      "default": 300,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_producer_seats": {
///      "description": "Number of chunk producers.\nDon't mess it up with
/// chunk-only producers feature which is deprecated.",
///      "default": 100,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_chunk_validator_seats": {
///      "default": 300,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "online_max_threshold": {
///      "description": "Online maximum threshold above which validator gets
/// full reward.",
///      "default": [
///        99,
///        100
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "online_min_threshold": {
///      "description": "Online minimum threshold below which validator
/// doesn't receive reward.",
///      "default": [
///        9,
///        10
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_reward_rate": {
///      "description": "Protocol treasury rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_treasury_account": {
///      "description": "Protocol treasury account",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    },
///    "protocol_upgrade_stake_threshold": {
///      "description": "Threshold of stake that needs to indicate that they
/// ready for upgrade.",
///      "default": [
///        4,
///        5
///      ],
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_version": {
///      "description": "Protocol version that this genesis works with.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "shard_layout": {
///      "description": "Layout information regarding how to split accounts
/// to shards",
///      "default": {
///        "V2": {
///          "boundary_accounts": [],
///          "id_to_index_map": {
///            "0": 0
///          },
///          "index_to_id_map": {
///            "0": 0
///          },
///          "shard_ids": [
///            0
///          ],
///          "version": 0
///        }
///      },
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ShardLayout"
///        }
///      ]
///    },
///    "shuffle_shard_assignment_for_chunk_producers": {
///      "description": "If true, shuffle the chunk producers across shards.
/// In other words, if\nthe shard assignments were `[S_0, S_1, S_2, S_3]`
/// where `S_i` represents\nthe set of chunk producers for shard `i`, if
/// this flag were true, the\nshard assignments might become, for example,
/// `[S_2, S_0, S_3, S_1]`.",
///      "default": false,
///      "type": "boolean"
///    },
///    "target_validator_mandates_per_shard": {
///      "description": "Number of target chunk validator mandates for each
/// shard.",
///      "default": 68,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "total_supply": {
///      "description": "Total supply of tokens at genesis.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "transaction_validity_period": {
///      "description": "Number of blocks for which a given transaction is
/// valid",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "use_production_config": {
///      "description": "This is only for test purposes. We hard code some
/// configs for mainnet and testnet\nin AllEpochConfig, and we want to have
/// a way to test that code path. This flag is for that.\nIf set to true,
/// the node will use the same config override path as mainnet and
/// testnet.",
///      "default": false,
///      "type": "boolean"
///    },
///    "validators": {
///      "description": "List of initial validators.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccountInfo"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GenesisConfig {
    ///Expected number of hidden validators per shard.
    pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
    ///Threshold for kicking out block producers, between 0 and 100.
    pub block_producer_kickout_threshold: u8,
    ///ID of the blockchain. This must be unique for every blockchain.
    ///If your testnet blockchains do not have unique chain IDs, you will
    /// have a bad time.
    pub chain_id: ::std::string::String,
    ///Limits the number of shard changes in chunk producer assignments,
    ///if algorithm is able to choose assignment with better balance of
    ///number of chunk producers for shards.
    #[serde(default = "defaults::default_u64::<u64, 5>")]
    pub chunk_producer_assignment_changes_limit: u64,
    ///Threshold for kicking out chunk producers, between 0 and 100.
    pub chunk_producer_kickout_threshold: u8,
    ///Threshold for kicking out nodes which are only chunk validators,
    /// between 0 and 100.
    #[serde(default = "defaults::default_u64::<u8, 80>")]
    pub chunk_validator_only_kickout_threshold: u8,
    ///Enable dynamic re-sharding.
    pub dynamic_resharding: bool,
    ///Epoch length counted in block heights.
    pub epoch_length: u64,
    ///Fishermen stake threshold.
    pub fishermen_threshold: NearToken,
    ///Initial gas limit.
    pub gas_limit: NearGas,
    ///Gas price adjustment rate
    pub gas_price_adjustment_rate: [i32; 2usize],
    ///Height of genesis block.
    pub genesis_height: u64,
    ///Official time of blockchain start.
    pub genesis_time: ::chrono::DateTime<::chrono::offset::Utc>,
    pub max_gas_price: NearToken,
    ///Maximum inflation on the total supply every epoch.
    pub max_inflation_rate: [i32; 2usize],
    ///Max stake percentage of the validators we will kick out.
    #[serde(default = "defaults::default_u64::<u8, 100>")]
    pub max_kickout_stake_perc: u8,
    ///Minimum gas price. It is also the initial gas price.
    pub min_gas_price: NearToken,
    ///The minimum stake required for staking is last seat price divided by
    /// this number.
    #[serde(default = "defaults::default_u64::<u64, 10>")]
    pub minimum_stake_divisor: u64,
    ///The lowest ratio s/s_total any block producer can have.
    ///See <https://github.com/near/NEPs/pull/167> for details
    #[serde(default = "defaults::genesis_config_minimum_stake_ratio")]
    pub minimum_stake_ratio: [i32; 2usize],
    ///The minimum number of validators each shard must have
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub minimum_validators_per_shard: u64,
    ///Number of block producer seats at genesis.
    pub num_block_producer_seats: u64,
    ///Defines number of shards and number of block producer seats per each
    /// shard at genesis. Note: not used with
    /// protocol_feature_chunk_only_producers -- replaced by
    /// minimum_validators_per_shard Note: not used before as all
    /// block producers produce chunks for all shards
    pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
    ///Expected number of blocks per year
    pub num_blocks_per_year: u64,
    ///Deprecated.
    #[serde(default = "defaults::default_u64::<u64, 300>")]
    pub num_chunk_only_producer_seats: u64,
    ///Number of chunk producers.
    ///Don't mess it up with chunk-only producers feature which is
    /// deprecated.
    #[serde(default = "defaults::default_u64::<u64, 100>")]
    pub num_chunk_producer_seats: u64,
    #[serde(default = "defaults::default_u64::<u64, 300>")]
    pub num_chunk_validator_seats: u64,
    ///Online maximum threshold above which validator gets full reward.
    #[serde(default = "defaults::genesis_config_online_max_threshold")]
    pub online_max_threshold: [i32; 2usize],
    ///Online minimum threshold below which validator doesn't receive
    /// reward.
    #[serde(default = "defaults::genesis_config_online_min_threshold")]
    pub online_min_threshold: [i32; 2usize],
    ///Protocol treasury rate
    pub protocol_reward_rate: [i32; 2usize],
    ///Protocol treasury account
    pub protocol_treasury_account: AccountId,
    ///Threshold of stake that needs to indicate that they ready for
    /// upgrade.
    #[serde(default = "defaults::genesis_config_protocol_upgrade_stake_threshold")]
    pub protocol_upgrade_stake_threshold: [i32; 2usize],
    ///Protocol version that this genesis works with.
    pub protocol_version: u32,
    ///Layout information regarding how to split accounts to shards
    #[serde(default = "defaults::genesis_config_shard_layout")]
    pub shard_layout: ShardLayout,
    ///If true, shuffle the chunk producers across shards. In other words,
    /// if the shard assignments were `[S_0, S_1, S_2, S_3]` where
    /// `S_i` represents the set of chunk producers for shard `i`,
    /// if this flag were true, the shard assignments might become,
    /// for example, `[S_2, S_0, S_3, S_1]`.
    #[serde(default)]
    pub shuffle_shard_assignment_for_chunk_producers: bool,
    ///Number of target chunk validator mandates for each shard.
    #[serde(default = "defaults::default_u64::<u64, 68>")]
    pub target_validator_mandates_per_shard: u64,
    ///Total supply of tokens at genesis.
    pub total_supply: NearToken,
    ///Number of blocks for which a given transaction is valid
    pub transaction_validity_period: u64,
    ///This is only for test purposes. We hard code some configs for
    /// mainnet and testnet in AllEpochConfig, and we want to have a
    /// way to test that code path. This flag is for that. If set to
    /// true, the node will use the same config override path as mainnet and
    /// testnet.
    #[serde(default)]
    pub use_production_config: bool,
    ///List of initial validators.
    pub validators: ::std::vec::Vec<AccountInfo>,
}
///`GenesisConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GenesisConfigError(pub ());
impl ::std::ops::Deref for GenesisConfigError {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<GenesisConfigError> for () {
    fn from(value: GenesisConfigError) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for GenesisConfigError {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`GenesisConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "GenesisConfigRequest",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GenesisConfigRequest(pub ());
impl ::std::ops::Deref for GenesisConfigRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<GenesisConfigRequest> for () {
    fn from(value: GenesisConfigRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for GenesisConfigRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`GlobalContractDeployMode`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "Contract is deployed under its code hash.\nUsers
/// will be able reference it by that hash.\nThis effectively makes the
/// contract immutable.",
///      "type": "string",
///      "enum": [
///        "CodeHash"
///      ]
///    },
///    {
///      "description": "Contract is deployed under the owner account
/// id.\nUsers will be able reference it by that account id.\nThis allows
/// the owner to update the contract for all its users.",
///      "type": "string",
///      "enum": [
///        "AccountId"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum GlobalContractDeployMode {
    ///Contract is deployed under its code hash.
    ///Users will be able reference it by that hash.
    ///This effectively makes the contract immutable.
    CodeHash,
    ///Contract is deployed under the owner account id.
    ///Users will be able reference it by that account id.
    ///This allows the owner to update the contract for all its users.
    AccountId,
}
impl ::std::fmt::Display for GlobalContractDeployMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CodeHash => f.write_str("CodeHash"),
            Self::AccountId => f.write_str("AccountId"),
        }
    }
}
impl ::std::str::FromStr for GlobalContractDeployMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CodeHash" => Ok(Self::CodeHash),
            "AccountId" => Ok(Self::AccountId),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GlobalContractDeployMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`GlobalContractIdentifier`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "CodeHash"
///      ],
///      "properties": {
///        "CodeHash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "AccountId"
///      ],
///      "properties": {
///        "AccountId": {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum GlobalContractIdentifier {
    CodeHash(CryptoHash),
    AccountId(AccountId),
}
///`GlobalContractIdentifierView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GlobalContractIdentifierView {
    CryptoHash(CryptoHash),
    AccountId(AccountId),
}
impl ::std::fmt::Display for GlobalContractIdentifierView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::CryptoHash(x) => x.fmt(f),
            Self::AccountId(x) => x.fmt(f),
        }
    }
}
///`HostError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "String encoding is bad UTF-16 sequence",
///      "type": "string",
///      "enum": [
///        "BadUTF16"
///      ]
///    },
///    {
///      "description": "String encoding is bad UTF-8 sequence",
///      "type": "string",
///      "enum": [
///        "BadUTF8"
///      ]
///    },
///    {
///      "description": "Exceeded the prepaid gas",
///      "type": "string",
///      "enum": [
///        "GasExceeded"
///      ]
///    },
///    {
///      "description": "Exceeded the maximum amount of gas allowed to burn
/// per contract",
///      "type": "string",
///      "enum": [
///        "GasLimitExceeded"
///      ]
///    },
///    {
///      "description": "Exceeded the account balance",
///      "type": "string",
///      "enum": [
///        "BalanceExceeded"
///      ]
///    },
///    {
///      "description": "Tried to call an empty method name",
///      "type": "string",
///      "enum": [
///        "EmptyMethodName"
///      ]
///    },
///    {
///      "description": "Smart contract panicked",
///      "type": "object",
///      "required": [
///        "GuestPanic"
///      ],
///      "properties": {
///        "GuestPanic": {
///          "type": "object",
///          "required": [
///            "panic_msg"
///          ],
///          "properties": {
///            "panic_msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "IntegerOverflow happened during a contract
/// execution",
///      "type": "string",
///      "enum": [
///        "IntegerOverflow"
///      ]
///    },
///    {
///      "description": "`promise_idx` does not correspond to existing
/// promises",
///      "type": "object",
///      "required": [
///        "InvalidPromiseIndex"
///      ],
///      "properties": {
///        "InvalidPromiseIndex": {
///          "type": "object",
///          "required": [
///            "promise_idx"
///          ],
///          "properties": {
///            "promise_idx": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Actions can only be appended to non-joint
/// promise.",
///      "type": "string",
///      "enum": [
///        "CannotAppendActionToJointPromise"
///      ]
///    },
///    {
///      "description": "Returning joint promise is currently prohibited",
///      "type": "string",
///      "enum": [
///        "CannotReturnJointPromise"
///      ]
///    },
///    {
///      "description": "Accessed invalid promise result index",
///      "type": "object",
///      "required": [
///        "InvalidPromiseResultIndex"
///      ],
///      "properties": {
///        "InvalidPromiseResultIndex": {
///          "type": "object",
///          "required": [
///            "result_idx"
///          ],
///          "properties": {
///            "result_idx": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Accessed invalid register id",
///      "type": "object",
///      "required": [
///        "InvalidRegisterId"
///      ],
///      "properties": {
///        "InvalidRegisterId": {
///          "type": "object",
///          "required": [
///            "register_id"
///          ],
///          "properties": {
///            "register_id": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Iterator `iterator_index` was invalidated after its
/// creation by performing a mutable operation on trie",
///      "type": "object",
///      "required": [
///        "IteratorWasInvalidated"
///      ],
///      "properties": {
///        "IteratorWasInvalidated": {
///          "type": "object",
///          "required": [
///            "iterator_index"
///          ],
///          "properties": {
///            "iterator_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Accessed memory outside the bounds",
///      "type": "string",
///      "enum": [
///        "MemoryAccessViolation"
///      ]
///    },
///    {
///      "description": "VM Logic returned an invalid receipt index",
///      "type": "object",
///      "required": [
///        "InvalidReceiptIndex"
///      ],
///      "properties": {
///        "InvalidReceiptIndex": {
///          "type": "object",
///          "required": [
///            "receipt_index"
///          ],
///          "properties": {
///            "receipt_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Iterator index `iterator_index` does not exist",
///      "type": "object",
///      "required": [
///        "InvalidIteratorIndex"
///      ],
///      "properties": {
///        "InvalidIteratorIndex": {
///          "type": "object",
///          "required": [
///            "iterator_index"
///          ],
///          "properties": {
///            "iterator_index": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "VM Logic returned an invalid account id",
///      "type": "string",
///      "enum": [
///        "InvalidAccountId"
///      ]
///    },
///    {
///      "description": "VM Logic returned an invalid method name",
///      "type": "string",
///      "enum": [
///        "InvalidMethodName"
///      ]
///    },
///    {
///      "description": "VM Logic provided an invalid public key",
///      "type": "string",
///      "enum": [
///        "InvalidPublicKey"
///      ]
///    },
///    {
///      "description": "`method_name` is not allowed in view calls",
///      "type": "object",
///      "required": [
///        "ProhibitedInView"
///      ],
///      "properties": {
///        "ProhibitedInView": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The total number of logs will exceed the limit.",
///      "type": "object",
///      "required": [
///        "NumberOfLogsExceeded"
///      ],
///      "properties": {
///        "NumberOfLogsExceeded": {
///          "type": "object",
///          "required": [
///            "limit"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The storage key length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "KeyLengthExceeded"
///      ],
///      "properties": {
///        "KeyLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The storage value length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ValueLengthExceeded"
///      ],
///      "properties": {
///        "ValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The total log length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "TotalLogLengthExceeded"
///      ],
///      "properties": {
///        "TotalLogLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The maximum number of promises within a
/// FunctionCall exceeded the limit.",
///      "type": "object",
///      "required": [
///        "NumberPromisesExceeded"
///      ],
///      "properties": {
///        "NumberPromisesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_promises"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_promises": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The maximum number of input data dependencies
/// exceeded the limit.",
///      "type": "object",
///      "required": [
///        "NumberInputDataDependenciesExceeded"
///      ],
///      "properties": {
///        "NumberInputDataDependenciesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_input_data_dependencies"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_input_data_dependencies": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The returned value length exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ReturnedValueLengthExceeded"
///      ],
///      "properties": {
///        "ReturnedValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The contract size for DeployContract action
/// exceeded the limit.",
///      "type": "object",
///      "required": [
///        "ContractSizeExceeded"
///      ],
///      "properties": {
///        "ContractSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The host function was deprecated.",
///      "type": "object",
///      "required": [
///        "Deprecated"
///      ],
///      "properties": {
///        "Deprecated": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "General errors for ECDSA recover.",
///      "type": "object",
///      "required": [
///        "ECRecoverError"
///      ],
///      "properties": {
///        "ECRecoverError": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Invalid input to alt_bn128 family of functions
/// (e.g., point which isn't\non the curve).",
///      "type": "object",
///      "required": [
///        "AltBn128InvalidInput"
///      ],
///      "properties": {
///        "AltBn128InvalidInput": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Invalid input to ed25519 signature verification
/// function (e.g. signature cannot be\nderived from bytes).",
///      "type": "object",
///      "required": [
///        "Ed25519VerifyInvalidInput"
///      ],
///      "properties": {
///        "Ed25519VerifyInvalidInput": {
///          "type": "object",
///          "required": [
///            "msg"
///          ],
///          "properties": {
///            "msg": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum HostError {
    ///String encoding is bad UTF-16 sequence
    #[serde(rename = "BadUTF16")]
    BadUtf16,
    ///String encoding is bad UTF-8 sequence
    #[serde(rename = "BadUTF8")]
    BadUtf8,
    ///Exceeded the prepaid gas
    GasExceeded,
    ///Exceeded the maximum amount of gas allowed to burn per contract
    GasLimitExceeded,
    ///Exceeded the account balance
    BalanceExceeded,
    ///Tried to call an empty method name
    EmptyMethodName,
    ///Smart contract panicked
    GuestPanic { panic_msg: ::std::string::String },
    ///IntegerOverflow happened during a contract execution
    IntegerOverflow,
    ///`promise_idx` does not correspond to existing promises
    InvalidPromiseIndex { promise_idx: u64 },
    ///Actions can only be appended to non-joint promise.
    CannotAppendActionToJointPromise,
    ///Returning joint promise is currently prohibited
    CannotReturnJointPromise,
    ///Accessed invalid promise result index
    InvalidPromiseResultIndex { result_idx: u64 },
    ///Accessed invalid register id
    InvalidRegisterId { register_id: u64 },
    ///Iterator `iterator_index` was invalidated after its creation by
    /// performing a mutable operation on trie
    IteratorWasInvalidated { iterator_index: u64 },
    ///Accessed memory outside the bounds
    MemoryAccessViolation,
    ///VM Logic returned an invalid receipt index
    InvalidReceiptIndex { receipt_index: u64 },
    ///Iterator index `iterator_index` does not exist
    InvalidIteratorIndex { iterator_index: u64 },
    ///VM Logic returned an invalid account id
    InvalidAccountId,
    ///VM Logic returned an invalid method name
    InvalidMethodName,
    ///VM Logic provided an invalid public key
    InvalidPublicKey,
    ///`method_name` is not allowed in view calls
    ProhibitedInView { method_name: ::std::string::String },
    ///The total number of logs will exceed the limit.
    NumberOfLogsExceeded { limit: u64 },
    ///The storage key length exceeded the limit.
    KeyLengthExceeded { length: u64, limit: u64 },
    ///The storage value length exceeded the limit.
    ValueLengthExceeded { length: u64, limit: u64 },
    ///The total log length exceeded the limit.
    TotalLogLengthExceeded { length: u64, limit: u64 },
    ///The maximum number of promises within a FunctionCall exceeded the
    /// limit.
    NumberPromisesExceeded { limit: u64, number_of_promises: u64 },
    ///The maximum number of input data dependencies exceeded the limit.
    NumberInputDataDependenciesExceeded {
        limit: u64,
        number_of_input_data_dependencies: u64,
    },
    ///The returned value length exceeded the limit.
    ReturnedValueLengthExceeded { length: u64, limit: u64 },
    ///The contract size for DeployContract action exceeded the limit.
    ContractSizeExceeded { limit: u64, size: u64 },
    ///The host function was deprecated.
    Deprecated { method_name: ::std::string::String },
    ///General errors for ECDSA recover.
    #[serde(rename = "ECRecoverError")]
    EcRecoverError { msg: ::std::string::String },
    ///Invalid input to alt_bn128 family of functions (e.g., point which
    /// isn't on the curve).
    AltBn128InvalidInput { msg: ::std::string::String },
    ///Invalid input to ed25519 signature verification function (e.g.
    /// signature cannot be derived from bytes).
    Ed25519VerifyInvalidInput { msg: ::std::string::String },
}
///`InternalError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum InternalError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`InvalidAccessKeyError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "The access key identified by the `public_key`
/// doesn't exist for the account",
///      "type": "object",
///      "required": [
///        "AccessKeyNotFound"
///      ],
///      "properties": {
///        "AccessKeyNotFound": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction `receiver_id` doesn't match the access
/// key receiver_id",
///      "type": "object",
///      "required": [
///        "ReceiverMismatch"
///      ],
///      "properties": {
///        "ReceiverMismatch": {
///          "type": "object",
///          "required": [
///            "ak_receiver",
///            "tx_receiver"
///          ],
///          "properties": {
///            "ak_receiver": {
///              "type": "string"
///            },
///            "tx_receiver": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction method name isn't allowed by the access
/// key",
///      "type": "object",
///      "required": [
///        "MethodNameMismatch"
///      ],
///      "properties": {
///        "MethodNameMismatch": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction requires a full permission access
/// key.",
///      "type": "string",
///      "enum": [
///        "RequiresFullAccess"
///      ]
///    },
///    {
///      "description": "Access Key does not have enough allowance to cover
/// transaction cost",
///      "type": "object",
///      "required": [
///        "NotEnoughAllowance"
///      ],
///      "properties": {
///        "NotEnoughAllowance": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "allowance",
///            "cost",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "allowance": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "cost": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Having a deposit with a function call action is not
/// allowed with a function call access key.",
///      "type": "string",
///      "enum": [
///        "DepositWithFunctionCall"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum InvalidAccessKeyError {
    ///The access key identified by the `public_key` doesn't exist for the
    /// account
    AccessKeyNotFound {
        account_id: AccountId,
        public_key: PublicKey,
    },
    ///Transaction `receiver_id` doesn't match the access key receiver_id
    ReceiverMismatch {
        ak_receiver: ::std::string::String,
        tx_receiver: AccountId,
    },
    ///Transaction method name isn't allowed by the access key
    MethodNameMismatch { method_name: ::std::string::String },
    ///Transaction requires a full permission access key.
    RequiresFullAccess,
    ///Access Key does not have enough allowance to cover transaction cost
    NotEnoughAllowance {
        account_id: AccountId,
        allowance: NearToken,
        cost: NearToken,
        public_key: PublicKey,
    },
    ///Having a deposit with a function call action is not allowed with a
    /// function call access key.
    DepositWithFunctionCall,
}
///An error happened during TX execution
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An error happened during TX execution",
///  "oneOf": [
///    {
///      "description": "Happens if a wrong AccessKey used or AccessKey has
/// not enough permissions",
///      "type": "object",
///      "required": [
///        "InvalidAccessKeyError"
///      ],
///      "properties": {
///        "InvalidAccessKeyError": {
///          "$ref": "#/components/schemas/InvalidAccessKeyError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "TX signer_id is not a valid [`AccountId`]",
///      "type": "object",
///      "required": [
///        "InvalidSignerId"
///      ],
///      "properties": {
///        "InvalidSignerId": {
///          "type": "object",
///          "required": [
///            "signer_id"
///          ],
///          "properties": {
///            "signer_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "TX signer_id is not found in a storage",
///      "type": "object",
///      "required": [
///        "SignerDoesNotExist"
///      ],
///      "properties": {
///        "SignerDoesNotExist": {
///          "type": "object",
///          "required": [
///            "signer_id"
///          ],
///          "properties": {
///            "signer_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction nonce must be strictly greater than
/// `account[access_key].nonce`.",
///      "type": "object",
///      "required": [
///        "InvalidNonce"
///      ],
///      "properties": {
///        "InvalidNonce": {
///          "type": "object",
///          "required": [
///            "ak_nonce",
///            "tx_nonce"
///          ],
///          "properties": {
///            "ak_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "tx_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction nonce is larger than the upper bound
/// given by the block height",
///      "type": "object",
///      "required": [
///        "NonceTooLarge"
///      ],
///      "properties": {
///        "NonceTooLarge": {
///          "type": "object",
///          "required": [
///            "tx_nonce",
///            "upper_bound"
///          ],
///          "properties": {
///            "tx_nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "upper_bound": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "TX receiver_id is not a valid AccountId",
///      "type": "object",
///      "required": [
///        "InvalidReceiverId"
///      ],
///      "properties": {
///        "InvalidReceiverId": {
///          "type": "object",
///          "required": [
///            "receiver_id"
///          ],
///          "properties": {
///            "receiver_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "TX signature is not valid",
///      "type": "string",
///      "enum": [
///        "InvalidSignature"
///      ]
///    },
///    {
///      "description": "Account does not have enough balance to cover TX
/// cost",
///      "type": "object",
///      "required": [
///        "NotEnoughBalance"
///      ],
///      "properties": {
///        "NotEnoughBalance": {
///          "type": "object",
///          "required": [
///            "balance",
///            "cost",
///            "signer_id"
///          ],
///          "properties": {
///            "balance": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "cost": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "signer_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Signer account doesn't have enough balance after
/// transaction.",
///      "type": "object",
///      "required": [
///        "LackBalanceForState"
///      ],
///      "properties": {
///        "LackBalanceForState": {
///          "type": "object",
///          "required": [
///            "amount",
///            "signer_id"
///          ],
///          "properties": {
///            "amount": {
///              "description": "Required balance to cover the state.",
///              "allOf": [
///                {
///                  "$ref": "#/components/schemas/NearToken"
///                }
///              ]
///            },
///            "signer_id": {
///              "description": "An account which doesn't have enough
/// balance to cover storage.",
///              "allOf": [
///                {
///                  "$ref": "#/components/schemas/AccountId"
///                }
///              ]
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An integer overflow occurred during transaction
/// cost estimation.",
///      "type": "string",
///      "enum": [
///        "CostOverflow"
///      ]
///    },
///    {
///      "description": "Transaction parent block hash doesn't belong to the
/// current chain",
///      "type": "string",
///      "enum": [
///        "InvalidChain"
///      ]
///    },
///    {
///      "description": "Transaction has expired",
///      "type": "string",
///      "enum": [
///        "Expired"
///      ]
///    },
///    {
///      "description": "An error occurred while validating actions of a
/// Transaction.",
///      "type": "object",
///      "required": [
///        "ActionsValidation"
///      ],
///      "properties": {
///        "ActionsValidation": {
///          "$ref": "#/components/schemas/ActionsValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The size of serialized transaction exceeded the
/// limit.",
///      "type": "object",
///      "required": [
///        "TransactionSizeExceeded"
///      ],
///      "properties": {
///        "TransactionSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Transaction version is invalid.",
///      "type": "string",
///      "enum": [
///        "InvalidTransactionVersion"
///      ]
///    },
///    {
///      "type": "object",
///      "required": [
///        "StorageError"
///      ],
///      "properties": {
///        "StorageError": {
///          "$ref": "#/components/schemas/StorageError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The receiver shard of the transaction is too
/// congested to accept new\ntransactions at the moment.",
///      "type": "object",
///      "required": [
///        "ShardCongested"
///      ],
///      "properties": {
///        "ShardCongested": {
///          "type": "object",
///          "required": [
///            "congestion_level",
///            "shard_id"
///          ],
///          "properties": {
///            "congestion_level": {
///              "description": "A value between 0 (no congestion) and 1
/// (max congestion).",
///              "type": "number",
///              "format": "double"
///            },
///            "shard_id": {
///              "description": "The congested shard.",
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The receiver shard of the transaction missed
/// several chunks and rejects\nnew transaction until it can make progress
/// again.",
///      "type": "object",
///      "required": [
///        "ShardStuck"
///      ],
///      "properties": {
///        "ShardStuck": {
///          "type": "object",
///          "required": [
///            "missed_chunks",
///            "shard_id"
///          ],
///          "properties": {
///            "missed_chunks": {
///              "description": "The number of blocks since the last
/// included chunk of the shard.",
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "shard_id": {
///              "description": "The shard that fails making progress.",
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum InvalidTxError {
    ///Happens if a wrong AccessKey used or AccessKey has not enough
    /// permissions
    InvalidAccessKeyError(InvalidAccessKeyError),
    ///TX signer_id is not a valid [`AccountId`]
    InvalidSignerId {
        signer_id: ::std::string::String,
    },
    ///TX signer_id is not found in a storage
    SignerDoesNotExist {
        signer_id: AccountId,
    },
    ///Transaction nonce must be strictly greater than
    /// `account[access_key].nonce`.
    InvalidNonce {
        ak_nonce: u64,
        tx_nonce: u64,
    },
    ///Transaction nonce is larger than the upper bound given by the block
    /// height
    NonceTooLarge {
        tx_nonce: u64,
        upper_bound: u64,
    },
    ///TX receiver_id is not a valid AccountId
    InvalidReceiverId {
        receiver_id: ::std::string::String,
    },
    ///TX signature is not valid
    InvalidSignature,
    ///Account does not have enough balance to cover TX cost
    NotEnoughBalance {
        balance: NearToken,
        cost: NearToken,
        signer_id: AccountId,
    },
    ///Signer account doesn't have enough balance after transaction.
    LackBalanceForState {
        ///Required balance to cover the state.
        amount: NearToken,
        ///An account which doesn't have enough balance to cover storage.
        signer_id: AccountId,
    },
    ///An integer overflow occurred during transaction cost estimation.
    CostOverflow,
    ///Transaction parent block hash doesn't belong to the current chain
    InvalidChain,
    ///Transaction has expired
    Expired,
    ///An error occurred while validating actions of a Transaction.
    ActionsValidation(ActionsValidationError),
    ///The size of serialized transaction exceeded the limit.
    TransactionSizeExceeded {
        limit: u64,
        size: u64,
    },
    ///Transaction version is invalid.
    InvalidTransactionVersion,
    StorageError(StorageError),
    ///The receiver shard of the transaction is too congested to accept new
    ///transactions at the moment.
    ShardCongested {
        congestion_level: f64,
        ///The congested shard.
        shard_id: u32,
    },
    ///The receiver shard of the transaction missed several chunks and
    /// rejects new transaction until it can make progress again.
    ShardStuck {
        ///The number of blocks since the last included chunk of the shard.
        missed_chunks: u64,
        ///The shard that fails making progress.
        shard_id: u32,
    },
}
impl ::std::convert::From<InvalidAccessKeyError> for InvalidTxError {
    fn from(value: InvalidAccessKeyError) -> Self {
        Self::InvalidAccessKeyError(value)
    }
}
impl ::std::convert::From<ActionsValidationError> for InvalidTxError {
    fn from(value: ActionsValidationError) -> Self {
        Self::ActionsValidation(value)
    }
}
impl ::std::convert::From<StorageError> for InvalidTxError {
    fn from(value: StorageError) -> Self {
        Self::StorageError(value)
    }
}
///`JsonRpcRequestForBlock`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_block",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "block"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcBlockRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForBlock {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForBlockMethod,
    pub params: RpcBlockRequest,
}
///`JsonRpcRequestForBlockEffects`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_block_effects",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "block_effects"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcStateChangesInBlockRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForBlockEffects {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForBlockEffectsMethod,
    pub params: RpcStateChangesInBlockRequest,
}
///`JsonRpcRequestForBlockEffectsMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "block_effects"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForBlockEffectsMethod {
    #[serde(rename = "block_effects")]
    BlockEffects,
}
impl ::std::fmt::Display for JsonRpcRequestForBlockEffectsMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BlockEffects => f.write_str("block_effects"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForBlockEffectsMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "block_effects" => Ok(Self::BlockEffects),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForBlockEffectsMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForBlockEffectsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForBlockEffectsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForBlockMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "block"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForBlockMethod {
    #[serde(rename = "block")]
    Block,
}
impl ::std::fmt::Display for JsonRpcRequestForBlockMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Block => f.write_str("block"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForBlockMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "block" => Ok(Self::Block),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForBlockMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForBlockMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForBlockMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForBroadcastTxAsync`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_broadcast_tx_async",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "broadcast_tx_async"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForBroadcastTxAsync {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForBroadcastTxAsyncMethod,
    pub params: RpcSendTransactionRequest,
}
///`JsonRpcRequestForBroadcastTxAsyncMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "broadcast_tx_async"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForBroadcastTxAsyncMethod {
    #[serde(rename = "broadcast_tx_async")]
    BroadcastTxAsync,
}
impl ::std::fmt::Display for JsonRpcRequestForBroadcastTxAsyncMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BroadcastTxAsync => f.write_str("broadcast_tx_async"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForBroadcastTxAsyncMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "broadcast_tx_async" => Ok(Self::BroadcastTxAsync),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForBroadcastTxAsyncMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForBroadcastTxAsyncMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForBroadcastTxAsyncMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForBroadcastTxCommit`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_broadcast_tx_commit",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "broadcast_tx_commit"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForBroadcastTxCommit {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForBroadcastTxCommitMethod,
    pub params: RpcSendTransactionRequest,
}
///`JsonRpcRequestForBroadcastTxCommitMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "broadcast_tx_commit"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForBroadcastTxCommitMethod {
    #[serde(rename = "broadcast_tx_commit")]
    BroadcastTxCommit,
}
impl ::std::fmt::Display for JsonRpcRequestForBroadcastTxCommitMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BroadcastTxCommit => f.write_str("broadcast_tx_commit"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForBroadcastTxCommitMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "broadcast_tx_commit" => Ok(Self::BroadcastTxCommit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForBroadcastTxCommitMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForBroadcastTxCommitMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForBroadcastTxCommitMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForChanges`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_changes",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "changes"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcStateChangesInBlockByTypeRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForChanges {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForChangesMethod,
    pub params: RpcStateChangesInBlockByTypeRequest,
}
///`JsonRpcRequestForChangesMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForChangesMethod {
    #[serde(rename = "changes")]
    Changes,
}
impl ::std::fmt::Display for JsonRpcRequestForChangesMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Changes => f.write_str("changes"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForChangesMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "changes" => Ok(Self::Changes),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForChangesMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForChangesMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForChangesMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForChunk`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_chunk",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "chunk"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcChunkRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForChunk {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForChunkMethod,
    pub params: RpcChunkRequest,
}
///`JsonRpcRequestForChunkMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "chunk"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForChunkMethod {
    #[serde(rename = "chunk")]
    Chunk,
}
impl ::std::fmt::Display for JsonRpcRequestForChunkMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Chunk => f.write_str("chunk"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForChunkMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "chunk" => Ok(Self::Chunk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForChunkMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForChunkMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForChunkMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForClientConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_client_config",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "client_config"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcClientConfigRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForClientConfig {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForClientConfigMethod,
    pub params: RpcClientConfigRequest,
}
///`JsonRpcRequestForClientConfigMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "client_config"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForClientConfigMethod {
    #[serde(rename = "client_config")]
    ClientConfig,
}
impl ::std::fmt::Display for JsonRpcRequestForClientConfigMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ClientConfig => f.write_str("client_config"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForClientConfigMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "client_config" => Ok(Self::ClientConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForClientConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForClientConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForClientConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalChanges`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_changes",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_changes"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcStateChangesInBlockByTypeRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalChanges {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalChangesMethod,
    pub params: RpcStateChangesInBlockByTypeRequest,
}
///`JsonRpcRequestForExperimentalChangesInBlock`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_changes_in_block",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_changes_in_block"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcStateChangesInBlockRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalChangesInBlock {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalChangesInBlockMethod,
    pub params: RpcStateChangesInBlockRequest,
}
///`JsonRpcRequestForExperimentalChangesInBlockMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_changes_in_block"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalChangesInBlockMethod {
    #[serde(rename = "EXPERIMENTAL_changes_in_block")]
    ExperimentalChangesInBlock,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalChangesInBlockMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalChangesInBlock => f.write_str("EXPERIMENTAL_changes_in_block"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalChangesInBlockMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_changes_in_block" => Ok(Self::ExperimentalChangesInBlock),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalChangesInBlockMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalChangesInBlockMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalChangesInBlockMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalChangesMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalChangesMethod {
    #[serde(rename = "EXPERIMENTAL_changes")]
    ExperimentalChanges,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalChangesMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalChanges => f.write_str("EXPERIMENTAL_changes"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalChangesMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_changes" => Ok(Self::ExperimentalChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalChangesMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalChangesMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForExperimentalChangesMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalCongestionLevel`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_congestion_level",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_congestion_level"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcCongestionLevelRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalCongestionLevel {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalCongestionLevelMethod,
    pub params: RpcCongestionLevelRequest,
}
///`JsonRpcRequestForExperimentalCongestionLevelMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_congestion_level"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalCongestionLevelMethod {
    #[serde(rename = "EXPERIMENTAL_congestion_level")]
    ExperimentalCongestionLevel,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalCongestionLevelMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalCongestionLevel => f.write_str("EXPERIMENTAL_congestion_level"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalCongestionLevelMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_congestion_level" => Ok(Self::ExperimentalCongestionLevel),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalCongestionLevelMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalCongestionLevelMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalCongestionLevelMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalGenesisConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_genesis_config",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_genesis_config"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/GenesisConfigRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalGenesisConfig {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalGenesisConfigMethod,
    pub params: GenesisConfigRequest,
}
///`JsonRpcRequestForExperimentalGenesisConfigMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_genesis_config"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalGenesisConfigMethod {
    #[serde(rename = "EXPERIMENTAL_genesis_config")]
    ExperimentalGenesisConfig,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalGenesisConfigMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalGenesisConfig => f.write_str("EXPERIMENTAL_genesis_config"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalGenesisConfigMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_genesis_config" => Ok(Self::ExperimentalGenesisConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalGenesisConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalGenesisConfigMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalGenesisConfigMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalLightClientBlockProof`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_light_client_block_proof",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_light_client_block_proof"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcLightClientBlockProofRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalLightClientBlockProof {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalLightClientBlockProofMethod,
    pub params: RpcLightClientBlockProofRequest,
}
///`JsonRpcRequestForExperimentalLightClientBlockProofMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_light_client_block_proof"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalLightClientBlockProofMethod {
    #[serde(rename = "EXPERIMENTAL_light_client_block_proof")]
    ExperimentalLightClientBlockProof,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalLightClientBlockProofMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalLightClientBlockProof => {
                f.write_str("EXPERIMENTAL_light_client_block_proof")
            }
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalLightClientBlockProofMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_light_client_block_proof" => Ok(Self::ExperimentalLightClientBlockProof),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalLightClientBlockProofMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalLightClientBlockProofMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalLightClientBlockProofMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalLightClientProof`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_light_client_proof",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_light_client_proof"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcLightClientExecutionProofRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalLightClientProof {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalLightClientProofMethod,
    pub params: RpcLightClientExecutionProofRequest,
}
///`JsonRpcRequestForExperimentalLightClientProofMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_light_client_proof"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalLightClientProofMethod {
    #[serde(rename = "EXPERIMENTAL_light_client_proof")]
    ExperimentalLightClientProof,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalLightClientProofMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalLightClientProof => f.write_str("EXPERIMENTAL_light_client_proof"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalLightClientProofMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_light_client_proof" => Ok(Self::ExperimentalLightClientProof),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalLightClientProofMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalLightClientProofMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalLightClientProofMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalMaintenanceWindows`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_maintenance_windows",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_maintenance_windows"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcMaintenanceWindowsRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalMaintenanceWindows {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalMaintenanceWindowsMethod,
    pub params: RpcMaintenanceWindowsRequest,
}
///`JsonRpcRequestForExperimentalMaintenanceWindowsMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_maintenance_windows"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalMaintenanceWindowsMethod {
    #[serde(rename = "EXPERIMENTAL_maintenance_windows")]
    ExperimentalMaintenanceWindows,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalMaintenanceWindowsMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalMaintenanceWindows => f.write_str("EXPERIMENTAL_maintenance_windows"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalMaintenanceWindowsMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_maintenance_windows" => Ok(Self::ExperimentalMaintenanceWindows),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalMaintenanceWindowsMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalMaintenanceWindowsMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalMaintenanceWindowsMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalProtocolConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_protocol_config",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_protocol_config"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcProtocolConfigRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalProtocolConfig {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalProtocolConfigMethod,
    pub params: RpcProtocolConfigRequest,
}
///`JsonRpcRequestForExperimentalProtocolConfigMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_protocol_config"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalProtocolConfigMethod {
    #[serde(rename = "EXPERIMENTAL_protocol_config")]
    ExperimentalProtocolConfig,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalProtocolConfigMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalProtocolConfig => f.write_str("EXPERIMENTAL_protocol_config"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalProtocolConfigMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_protocol_config" => Ok(Self::ExperimentalProtocolConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalProtocolConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalProtocolConfigMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalProtocolConfigMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalReceipt`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_receipt",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_receipt"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcReceiptRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalReceipt {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalReceiptMethod,
    pub params: RpcReceiptRequest,
}
///`JsonRpcRequestForExperimentalReceiptMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_receipt"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalReceiptMethod {
    #[serde(rename = "EXPERIMENTAL_receipt")]
    ExperimentalReceipt,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalReceiptMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalReceipt => f.write_str("EXPERIMENTAL_receipt"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalReceiptMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_receipt" => Ok(Self::ExperimentalReceipt),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalReceiptMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalReceiptMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForExperimentalReceiptMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalSplitStorageInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_split_storage_info",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_split_storage_info"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcSplitStorageInfoRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalSplitStorageInfo {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalSplitStorageInfoMethod,
    pub params: RpcSplitStorageInfoRequest,
}
///`JsonRpcRequestForExperimentalSplitStorageInfoMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_split_storage_info"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalSplitStorageInfoMethod {
    #[serde(rename = "EXPERIMENTAL_split_storage_info")]
    ExperimentalSplitStorageInfo,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalSplitStorageInfoMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalSplitStorageInfo => f.write_str("EXPERIMENTAL_split_storage_info"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalSplitStorageInfoMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_split_storage_info" => Ok(Self::ExperimentalSplitStorageInfo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalSplitStorageInfoMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalSplitStorageInfoMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalSplitStorageInfoMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalTxStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_tx_status",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_tx_status"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcTransactionStatusRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalTxStatus {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalTxStatusMethod,
    pub params: RpcTransactionStatusRequest,
}
///`JsonRpcRequestForExperimentalTxStatusMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_tx_status"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalTxStatusMethod {
    #[serde(rename = "EXPERIMENTAL_tx_status")]
    ExperimentalTxStatus,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalTxStatusMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalTxStatus => f.write_str("EXPERIMENTAL_tx_status"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalTxStatusMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_tx_status" => Ok(Self::ExperimentalTxStatus),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalTxStatusMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalTxStatusMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalTxStatusMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForExperimentalValidatorsOrdered`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_EXPERIMENTAL_validators_ordered",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "EXPERIMENTAL_validators_ordered"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcValidatorsOrderedRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForExperimentalValidatorsOrdered {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForExperimentalValidatorsOrderedMethod,
    pub params: RpcValidatorsOrderedRequest,
}
///`JsonRpcRequestForExperimentalValidatorsOrderedMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "EXPERIMENTAL_validators_ordered"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForExperimentalValidatorsOrderedMethod {
    #[serde(rename = "EXPERIMENTAL_validators_ordered")]
    ExperimentalValidatorsOrdered,
}
impl ::std::fmt::Display for JsonRpcRequestForExperimentalValidatorsOrderedMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ExperimentalValidatorsOrdered => f.write_str("EXPERIMENTAL_validators_ordered"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForExperimentalValidatorsOrderedMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "EXPERIMENTAL_validators_ordered" => Ok(Self::ExperimentalValidatorsOrdered),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForExperimentalValidatorsOrderedMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForExperimentalValidatorsOrderedMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForExperimentalValidatorsOrderedMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForGasPrice`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_gas_price",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "gas_price"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcGasPriceRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForGasPrice {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForGasPriceMethod,
    pub params: RpcGasPriceRequest,
}
///`JsonRpcRequestForGasPriceMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "gas_price"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForGasPriceMethod {
    #[serde(rename = "gas_price")]
    GasPrice,
}
impl ::std::fmt::Display for JsonRpcRequestForGasPriceMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GasPrice => f.write_str("gas_price"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForGasPriceMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gas_price" => Ok(Self::GasPrice),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForGasPriceMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForGasPriceMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForGasPriceMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForGenesisConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_genesis_config",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "genesis_config"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/GenesisConfigRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForGenesisConfig {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForGenesisConfigMethod,
    pub params: GenesisConfigRequest,
}
///`JsonRpcRequestForGenesisConfigMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "genesis_config"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForGenesisConfigMethod {
    #[serde(rename = "genesis_config")]
    GenesisConfig,
}
impl ::std::fmt::Display for JsonRpcRequestForGenesisConfigMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::GenesisConfig => f.write_str("genesis_config"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForGenesisConfigMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "genesis_config" => Ok(Self::GenesisConfig),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForGenesisConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForGenesisConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForGenesisConfigMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForHealth`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_health",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "health"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcHealthRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForHealth {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForHealthMethod,
    pub params: RpcHealthRequest,
}
///`JsonRpcRequestForHealthMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "health"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForHealthMethod {
    #[serde(rename = "health")]
    Health,
}
impl ::std::fmt::Display for JsonRpcRequestForHealthMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Health => f.write_str("health"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForHealthMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "health" => Ok(Self::Health),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForHealthMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForHealthMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForHealthMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForLightClientProof`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_light_client_proof",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "light_client_proof"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcLightClientExecutionProofRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForLightClientProof {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForLightClientProofMethod,
    pub params: RpcLightClientExecutionProofRequest,
}
///`JsonRpcRequestForLightClientProofMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "light_client_proof"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForLightClientProofMethod {
    #[serde(rename = "light_client_proof")]
    LightClientProof,
}
impl ::std::fmt::Display for JsonRpcRequestForLightClientProofMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::LightClientProof => f.write_str("light_client_proof"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForLightClientProofMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light_client_proof" => Ok(Self::LightClientProof),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForLightClientProofMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForLightClientProofMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForLightClientProofMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForMaintenanceWindows`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_maintenance_windows",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "maintenance_windows"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcMaintenanceWindowsRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForMaintenanceWindows {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForMaintenanceWindowsMethod,
    pub params: RpcMaintenanceWindowsRequest,
}
///`JsonRpcRequestForMaintenanceWindowsMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "maintenance_windows"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForMaintenanceWindowsMethod {
    #[serde(rename = "maintenance_windows")]
    MaintenanceWindows,
}
impl ::std::fmt::Display for JsonRpcRequestForMaintenanceWindowsMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MaintenanceWindows => f.write_str("maintenance_windows"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForMaintenanceWindowsMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "maintenance_windows" => Ok(Self::MaintenanceWindows),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForMaintenanceWindowsMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForMaintenanceWindowsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForMaintenanceWindowsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForNetworkInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_network_info",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "network_info"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcNetworkInfoRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForNetworkInfo {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForNetworkInfoMethod,
    pub params: RpcNetworkInfoRequest,
}
///`JsonRpcRequestForNetworkInfoMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "network_info"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForNetworkInfoMethod {
    #[serde(rename = "network_info")]
    NetworkInfo,
}
impl ::std::fmt::Display for JsonRpcRequestForNetworkInfoMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NetworkInfo => f.write_str("network_info"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForNetworkInfoMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "network_info" => Ok(Self::NetworkInfo),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForNetworkInfoMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForNetworkInfoMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForNetworkInfoMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForNextLightClientBlock`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_next_light_client_block",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "next_light_client_block"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcLightClientNextBlockRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForNextLightClientBlock {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForNextLightClientBlockMethod,
    pub params: RpcLightClientNextBlockRequest,
}
///`JsonRpcRequestForNextLightClientBlockMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "next_light_client_block"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForNextLightClientBlockMethod {
    #[serde(rename = "next_light_client_block")]
    NextLightClientBlock,
}
impl ::std::fmt::Display for JsonRpcRequestForNextLightClientBlockMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NextLightClientBlock => f.write_str("next_light_client_block"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForNextLightClientBlockMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "next_light_client_block" => Ok(Self::NextLightClientBlock),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForNextLightClientBlockMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for JsonRpcRequestForNextLightClientBlockMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for JsonRpcRequestForNextLightClientBlockMethod
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForQuery`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_query",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "query"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcQueryRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForQuery {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForQueryMethod,
    pub params: RpcQueryRequest,
}
///`JsonRpcRequestForQueryMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "query"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForQueryMethod {
    #[serde(rename = "query")]
    Query,
}
impl ::std::fmt::Display for JsonRpcRequestForQueryMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Query => f.write_str("query"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForQueryMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "query" => Ok(Self::Query),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForQueryMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForQueryMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForQueryMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForSendTx`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_send_tx",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "send_tx"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcSendTransactionRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForSendTx {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForSendTxMethod,
    pub params: RpcSendTransactionRequest,
}
///`JsonRpcRequestForSendTxMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "send_tx"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForSendTxMethod {
    #[serde(rename = "send_tx")]
    SendTx,
}
impl ::std::fmt::Display for JsonRpcRequestForSendTxMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SendTx => f.write_str("send_tx"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForSendTxMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "send_tx" => Ok(Self::SendTx),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForSendTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForSendTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForSendTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_status",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "status"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcStatusRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForStatus {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForStatusMethod,
    pub params: RpcStatusRequest,
}
///`JsonRpcRequestForStatusMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "status"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForStatusMethod {
    #[serde(rename = "status")]
    Status,
}
impl ::std::fmt::Display for JsonRpcRequestForStatusMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Status => f.write_str("status"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForStatusMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "status" => Ok(Self::Status),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForStatusMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForStatusMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForStatusMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForTx`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_tx",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "tx"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcTransactionStatusRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForTx {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForTxMethod,
    pub params: RpcTransactionStatusRequest,
}
///`JsonRpcRequestForTxMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "tx"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForTxMethod {
    #[serde(rename = "tx")]
    Tx,
}
impl ::std::fmt::Display for JsonRpcRequestForTxMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tx => f.write_str("tx"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForTxMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tx" => Ok(Self::Tx),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForTxMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcRequestForValidators`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "JsonRpcRequest_for_validators",
///  "type": "object",
///  "required": [
///    "id",
///    "jsonrpc",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "id": {
///      "type": "string"
///    },
///    "jsonrpc": {
///      "type": "string"
///    },
///    "method": {
///      "type": "string",
///      "enum": [
///        "validators"
///      ]
///    },
///    "params": {
///      "$ref": "#/components/schemas/RpcValidatorRequest"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct JsonRpcRequestForValidators {
    pub id: ::std::string::String,
    pub jsonrpc: ::std::string::String,
    pub method: JsonRpcRequestForValidatorsMethod,
    pub params: RpcValidatorRequest,
}
///`JsonRpcRequestForValidatorsMethod`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "validators"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum JsonRpcRequestForValidatorsMethod {
    #[serde(rename = "validators")]
    Validators,
}
impl ::std::fmt::Display for JsonRpcRequestForValidatorsMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Validators => f.write_str("validators"),
        }
    }
}
impl ::std::str::FromStr for JsonRpcRequestForValidatorsMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "validators" => Ok(Self::Validators),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JsonRpcRequestForValidatorsMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JsonRpcRequestForValidatorsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JsonRpcRequestForValidatorsMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcResponseForArrayOfRangeOfUint64AndRpcMaintenanceWindowsError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "type": "array",
///          "items": {
///            "$ref": "#/components/schemas/Range_of_uint64"
///          }
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcMaintenanceWindowsError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForArrayOfRangeOfUint64AndRpcMaintenanceWindowsError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: ::std::vec::Vec<RangeOfUint64>,
    },
    Variant1 {
        error: ErrorWrapperForRpcMaintenanceWindowsError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForArrayOfValidatorStakeViewAndRpcValidatorError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "type": "array",
///          "items": {
///            "$ref": "#/components/schemas/ValidatorStakeView"
///          }
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcValidatorError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForArrayOfValidatorStakeViewAndRpcValidatorError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: ::std::vec::Vec<ValidatorStakeView>,
    },
    Variant1 {
        error: ErrorWrapperForRpcValidatorError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForCryptoHashAndRpcTransactionError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcTransactionError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForCryptoHashAndRpcTransactionError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: CryptoHash,
    },
    Variant1 {
        error: ErrorWrapperForRpcTransactionError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForGenesisConfigAndGenesisConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/GenesisConfig"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_GenesisConfigError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForGenesisConfigAndGenesisConfigError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: GenesisConfig,
    },
    Variant1 {
        error: ErrorWrapperForGenesisConfigError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForNullableRpcHealthResponseAndRpcStatusError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "anyOf": [
///            {
///              "$ref": "#/components/schemas/RpcHealthResponse"
///            },
///            {
///              "type": "null"
///            }
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcStatusError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForNullableRpcHealthResponseAndRpcStatusError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: ::std::option::Option<RpcHealthResponse>,
    },
    Variant1 {
        error: ErrorWrapperForRpcStatusError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcBlockResponseAndRpcBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcBlockResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcBlockError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcBlockResponseAndRpcBlockError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcBlockResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcBlockError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcChunkResponseAndRpcChunkError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcChunkResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcChunkError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcChunkResponseAndRpcChunkError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcChunkResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcChunkError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcClientConfigResponseAndRpcClientConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcClientConfigResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcClientConfigError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcClientConfigResponseAndRpcClientConfigError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcClientConfigResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcClientConfigError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcCongestionLevelResponseAndRpcChunkError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcCongestionLevelResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcChunkError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcCongestionLevelResponseAndRpcChunkError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcCongestionLevelResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcChunkError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcGasPriceResponseAndRpcGasPriceError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcGasPriceResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcGasPriceError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcGasPriceResponseAndRpcGasPriceError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcGasPriceResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcGasPriceError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcLightClientProofError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcLightClientBlockProofResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcLightClientProofError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcLightClientBlockProofResponseAndRpcLightClientProofError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcLightClientBlockProofResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcLightClientProofError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcLightClientProofError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref":
/// "#/components/schemas/RpcLightClientExecutionProofResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcLightClientProofError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcLightClientExecutionProofResponseAndRpcLightClientProofError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcLightClientExecutionProofResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcLightClientProofError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcLightClientNextBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcLightClientNextBlockResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcLightClientNextBlockError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcLightClientNextBlockResponseAndRpcLightClientNextBlockError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcLightClientNextBlockResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcLightClientNextBlockError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcNetworkInfoResponseAndRpcNetworkInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcNetworkInfoResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcNetworkInfoError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcNetworkInfoResponseAndRpcNetworkInfoError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcNetworkInfoResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcNetworkInfoError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcProtocolConfigResponseAndRpcProtocolConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcProtocolConfigResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcProtocolConfigError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcProtocolConfigResponseAndRpcProtocolConfigError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcProtocolConfigResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcProtocolConfigError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcQueryResponseAndRpcQueryError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcQueryResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcQueryError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcQueryResponseAndRpcQueryError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcQueryResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcQueryError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcReceiptResponseAndRpcReceiptError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcReceiptResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcReceiptError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcReceiptResponseAndRpcReceiptError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcReceiptResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcReceiptError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcSplitStorageInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcSplitStorageInfoResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcSplitStorageInfoError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcSplitStorageInfoResponseAndRpcSplitStorageInfoError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcSplitStorageInfoResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcSplitStorageInfoError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcStateChangesError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref":
/// "#/components/schemas/RpcStateChangesInBlockByTypeResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcStateChangesError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcStateChangesInBlockByTypeResponseAndRpcStateChangesError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcStateChangesInBlockByTypeResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcStateChangesError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcStateChangesError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcStateChangesInBlockResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcStateChangesError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcStateChangesInBlockResponseAndRpcStateChangesError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcStateChangesInBlockResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcStateChangesError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcStatusResponseAndRpcStatusError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcStatusResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref": "#/components/schemas/ErrorWrapper_for_RpcStatusError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcStatusResponseAndRpcStatusError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcStatusResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcStatusError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcTransactionResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcTransactionError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcTransactionResponseAndRpcTransactionError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcTransactionResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcTransactionError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///`JsonRpcResponseForRpcValidatorResponseAndRpcValidatorError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "id",
///        "jsonrpc",
///        "result"
///      ],
///      "properties": {
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        },
///        "result": {
///          "$ref": "#/components/schemas/RpcValidatorResponse"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "error",
///        "id",
///        "jsonrpc"
///      ],
///      "properties": {
///        "error": {
///          "$ref":
/// "#/components/schemas/ErrorWrapper_for_RpcValidatorError"
///        },
///        "id": {
///          "type": "string"
///        },
///        "jsonrpc": {
///          "type": "string"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponseForRpcValidatorResponseAndRpcValidatorError {
    Variant0 {
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
        result: RpcValidatorResponse,
    },
    Variant1 {
        error: ErrorWrapperForRpcValidatorError,
        id: ::std::string::String,
        jsonrpc: ::std::string::String,
    },
}
///Information about a Producer: its account name, peer_id and a list of
/// connected peers that the node can use to send message for this
/// producer.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information about a Producer: its account name, peer_id
/// and a list of connected peers that\nthe node can use to send message for
/// this producer.",
///  "type": "object",
///  "required": [
///    "account_id",
///    "peer_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "next_hops": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/components/schemas/PublicKey"
///      }
///    },
///    "peer_id": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct KnownProducerView {
    pub account_id: AccountId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_hops: ::std::option::Option<::std::vec::Vec<PublicKey>>,
    pub peer_id: PublicKey,
}
///`LightClientBlockLiteView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "inner_lite",
///    "inner_rest_hash",
///    "prev_block_hash"
///  ],
///  "properties": {
///    "inner_lite": {
///      "$ref": "#/components/schemas/BlockHeaderInnerLiteView"
///    },
///    "inner_rest_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "prev_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LightClientBlockLiteView {
    pub inner_lite: BlockHeaderInnerLiteView,
    pub inner_rest_hash: CryptoHash,
    pub prev_block_hash: CryptoHash,
}
///Describes limits for VM and Runtime.
///TODO #4139: consider switching to strongly-typed wrappers instead of raw
/// quantities
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes limits for VM and Runtime.\nTODO #4139:
/// consider switching to strongly-typed wrappers instead of raw
/// quantities",
///  "type": "object",
///  "properties": {
///    "account_id_validity_rules_version": {
///      "description": "Whether to enforce account_id well-formed-ness
/// where it wasn't enforced\nhistorically.",
///      "default": 0,
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountIdValidityRulesVersion"
///        }
///      ]
///    },
///    "initial_memory_pages": {
///      "description": "The initial number of memory pages.\nNOTE: It's not a limiter itself, but it's a value we use for initial_memory_pages.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_actions_per_receipt": {
///      "description": "Max number of actions per receipt.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_arguments_length": {
///      "description": "Max length of arguments in a function call
/// action.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_contract_size": {
///      "description": "Max contract size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_elements_per_contract_table": {
///      "description": "If present, stores max number of elements in a
/// single contract's table",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "max_functions_number_per_contract": {
///      "description": "If present, stores max number of functions in one
/// contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_gas_burnt": {
///      "description": "Max amount of gas that can be used, excluding gas
/// attached to promises.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_length_method_name": {
///      "description": "Max length of any method name (without terminating
/// character).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_returned_data": {
///      "description": "Max length of returned data",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_storage_key": {
///      "description": "Max storage key size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_length_storage_value": {
///      "description": "Max storage value size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_locals_per_contract": {
///      "description": "If present, stores max number of locals declared
/// globally in one contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_memory_pages": {
///      "description": "What is the maximal memory pages amount is allowed
/// to have for a contract.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_number_bytes_method_names": {
///      "description": "Max total length of all method names (including
/// terminating character) for a function call\npermission access key.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_input_data_dependencies": {
///      "description": "Max number of input data dependencies",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_logs": {
///      "description": "Maximum number of log entries.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_number_registers": {
///      "description": "Maximum number of registers that can be used
/// simultaneously.\n\nNote that due to an implementation quirk [read: a
/// bug] in VMLogic, if we\nhave this number of registers, no subsequent
/// writes to the registers\nwill succeed even if they replace an existing
/// register.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_promises_per_function_call_action": {
///      "description": "Max number of promises that a function call can
/// create",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_receipt_size": {
///      "description": "Max receipt size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_register_size": {
///      "description": "Maximum number of bytes that can be stored in a
/// single register.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_stack_height": {
///      "description": "How tall the stack is allowed to grow?\n\nSee <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost\nis calculated.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_tables_per_contract": {
///      "description": "If present, stores max number of tables declared
/// globally in one contract",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "max_total_log_length": {
///      "description": "Maximum total length in bytes of all log
/// messages.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_total_prepaid_gas": {
///      "description": "Max total prepaid gas for all function call actions
/// per receipt.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "max_transaction_size": {
///      "description": "Max transaction size",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_yield_payload_size": {
///      "description": "Maximum number of bytes for payload passed over a
/// yield resume.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "per_receipt_storage_proof_size_limit": {
///      "description": "Hard limit on the size of storage proof generated
/// while executing a single receipt.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "registers_memory_limit": {
///      "description": "Limit of memory used by registers.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "yield_timeout_length_in_blocks": {
///      "description": "Number of blocks after which a yielded promise
/// times out.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LimitConfig {
    ///Whether to enforce account_id well-formed-ness where it wasn't
    /// enforced historically.
    #[serde(default = "defaults::limit_config_account_id_validity_rules_version")]
    pub account_id_validity_rules_version: AccountIdValidityRulesVersion,
    ///The initial number of memory pages.
    ///NOTE: It's not a limiter itself, but it's a value we use for
    /// initial_memory_pages.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub initial_memory_pages: ::std::option::Option<u32>,
    ///Max number of actions per receipt.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_actions_per_receipt: ::std::option::Option<u64>,
    ///Max length of arguments in a function call action.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_arguments_length: ::std::option::Option<u64>,
    ///Max contract size
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_contract_size: ::std::option::Option<u64>,
    ///If present, stores max number of elements in a single contract's
    /// table
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_elements_per_contract_table: ::std::option::Option<u32>,
    ///If present, stores max number of functions in one contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_functions_number_per_contract: ::std::option::Option<u64>,
    ///Max amount of gas that can be used, excluding gas attached to
    /// promises.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_gas_burnt: ::std::option::Option<NearGas>,
    ///Max length of any method name (without terminating character).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_length_method_name: ::std::option::Option<u64>,
    ///Max length of returned data
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_length_returned_data: ::std::option::Option<u64>,
    ///Max storage key size
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_length_storage_key: ::std::option::Option<u64>,
    ///Max storage value size
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_length_storage_value: ::std::option::Option<u64>,
    ///If present, stores max number of locals declared globally in one
    /// contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_locals_per_contract: ::std::option::Option<u64>,
    ///What is the maximal memory pages amount is allowed to have for a
    /// contract.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_memory_pages: ::std::option::Option<u32>,
    ///Max total length of all method names (including terminating
    /// character) for a function call permission access key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_number_bytes_method_names: ::std::option::Option<u64>,
    ///Max number of input data dependencies
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_number_input_data_dependencies: ::std::option::Option<u64>,
    ///Maximum number of log entries.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_number_logs: ::std::option::Option<u64>,
    ///Maximum number of registers that can be used simultaneously.
    ///
    ///Note that due to an implementation quirk [read: a bug] in VMLogic,
    /// if we have this number of registers, no subsequent writes to
    /// the registers will succeed even if they replace an existing
    /// register.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_number_registers: ::std::option::Option<u64>,
    ///Max number of promises that a function call can create
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_promises_per_function_call_action: ::std::option::Option<u64>,
    ///Max receipt size
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_receipt_size: ::std::option::Option<u64>,
    ///Maximum number of bytes that can be stored in a single register.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_register_size: ::std::option::Option<u64>,
    ///How tall the stack is allowed to grow?
    ///
    ///See <https://wiki.parity.io/WebAssembly-StackHeight> to find out how the stack frame cost
    ///is calculated.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_stack_height: ::std::option::Option<u32>,
    ///If present, stores max number of tables declared globally in one
    /// contract
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tables_per_contract: ::std::option::Option<u32>,
    ///Maximum total length in bytes of all log messages.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_total_log_length: ::std::option::Option<u64>,
    ///Max total prepaid gas for all function call actions per receipt.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_total_prepaid_gas: ::std::option::Option<NearGas>,
    ///Max transaction size
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_transaction_size: ::std::option::Option<u64>,
    ///Maximum number of bytes for payload passed over a yield resume.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_yield_payload_size: ::std::option::Option<u64>,
    ///Hard limit on the size of storage proof generated while executing a
    /// single receipt.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub per_receipt_storage_proof_size_limit: ::std::option::Option<u32>,
    ///Limit of memory used by registers.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub registers_memory_limit: ::std::option::Option<u64>,
    ///Number of blocks after which a yielded promise times out.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub yield_timeout_length_in_blocks: ::std::option::Option<u64>,
}
impl ::std::default::Default for LimitConfig {
    fn default() -> Self {
        Self {
            account_id_validity_rules_version:
                defaults::limit_config_account_id_validity_rules_version(),
            initial_memory_pages: Default::default(),
            max_actions_per_receipt: Default::default(),
            max_arguments_length: Default::default(),
            max_contract_size: Default::default(),
            max_elements_per_contract_table: Default::default(),
            max_functions_number_per_contract: Default::default(),
            max_gas_burnt: Default::default(),
            max_length_method_name: Default::default(),
            max_length_returned_data: Default::default(),
            max_length_storage_key: Default::default(),
            max_length_storage_value: Default::default(),
            max_locals_per_contract: Default::default(),
            max_memory_pages: Default::default(),
            max_number_bytes_method_names: Default::default(),
            max_number_input_data_dependencies: Default::default(),
            max_number_logs: Default::default(),
            max_number_registers: Default::default(),
            max_promises_per_function_call_action: Default::default(),
            max_receipt_size: Default::default(),
            max_register_size: Default::default(),
            max_stack_height: Default::default(),
            max_tables_per_contract: Default::default(),
            max_total_log_length: Default::default(),
            max_total_prepaid_gas: Default::default(),
            max_transaction_size: Default::default(),
            max_yield_payload_size: Default::default(),
            per_receipt_storage_proof_size_limit: Default::default(),
            registers_memory_limit: Default::default(),
            yield_timeout_length_in_blocks: Default::default(),
        }
    }
}
///`LogSummaryStyle`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "plain",
///    "colored"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum LogSummaryStyle {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "colored")]
    Colored,
}
impl ::std::fmt::Display for LogSummaryStyle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Plain => f.write_str("plain"),
            Self::Colored => f.write_str("colored"),
        }
    }
}
impl ::std::str::FromStr for LogSummaryStyle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "plain" => Ok(Self::Plain),
            "colored" => Ok(Self::Colored),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LogSummaryStyle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MerklePathItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "hash"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/components/schemas/Direction"
///    },
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MerklePathItem {
    pub direction: Direction,
    pub hash: CryptoHash,
}
///`MethodResolveError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "MethodEmptyName",
///    "MethodNotFound",
///    "MethodInvalidSignature"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MethodResolveError {
    MethodEmptyName,
    MethodNotFound,
    MethodInvalidSignature,
}
impl ::std::fmt::Display for MethodResolveError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MethodEmptyName => f.write_str("MethodEmptyName"),
            Self::MethodNotFound => f.write_str("MethodNotFound"),
            Self::MethodInvalidSignature => f.write_str("MethodInvalidSignature"),
        }
    }
}
impl ::std::str::FromStr for MethodResolveError {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MethodEmptyName" => Ok(Self::MethodEmptyName),
            "MethodNotFound" => Ok(Self::MethodNotFound),
            "MethodInvalidSignature" => Ok(Self::MethodInvalidSignature),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MethodResolveError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MissingTrieValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "context",
///    "hash"
///  ],
///  "properties": {
///    "context": {
///      "$ref": "#/components/schemas/MissingTrieValueContext"
///    },
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MissingTrieValue {
    pub context: MissingTrieValueContext,
    pub hash: CryptoHash,
}
///Contexts in which `StorageError::MissingTrieValue` error might occur.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contexts in which `StorageError::MissingTrieValue`
/// error might occur.",
///  "oneOf": [
///    {
///      "description": "Missing trie value when reading from
/// TrieIterator.",
///      "type": "string",
///      "enum": [
///        "TrieIterator"
///      ]
///    },
///    {
///      "description": "Missing trie value when reading from
/// TriePrefetchingStorage.",
///      "type": "string",
///      "enum": [
///        "TriePrefetchingStorage"
///      ]
///    },
///    {
///      "description": "Missing trie value when reading from
/// TrieMemoryPartialStorage.",
///      "type": "string",
///      "enum": [
///        "TrieMemoryPartialStorage"
///      ]
///    },
///    {
///      "description": "Missing trie value when reading from TrieStorage.",
///      "type": "string",
///      "enum": [
///        "TrieStorage"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MissingTrieValueContext {
    ///Missing trie value when reading from TrieIterator.
    TrieIterator,
    ///Missing trie value when reading from TriePrefetchingStorage.
    TriePrefetchingStorage,
    ///Missing trie value when reading from TrieMemoryPartialStorage.
    TrieMemoryPartialStorage,
    ///Missing trie value when reading from TrieStorage.
    TrieStorage,
}
impl ::std::fmt::Display for MissingTrieValueContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TrieIterator => f.write_str("TrieIterator"),
            Self::TriePrefetchingStorage => f.write_str("TriePrefetchingStorage"),
            Self::TrieMemoryPartialStorage => f.write_str("TrieMemoryPartialStorage"),
            Self::TrieStorage => f.write_str("TrieStorage"),
        }
    }
}
impl ::std::str::FromStr for MissingTrieValueContext {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TrieIterator" => Ok(Self::TrieIterator),
            "TriePrefetchingStorage" => Ok(Self::TriePrefetchingStorage),
            "TrieMemoryPartialStorage" => Ok(Self::TrieMemoryPartialStorage),
            "TrieStorage" => Ok(Self::TrieStorage),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MissingTrieValueContext {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MutableConfigValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct MutableConfigValue(pub ::std::string::String);
impl ::std::ops::Deref for MutableConfigValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for MutableConfigValue {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for MutableConfigValue {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for MutableConfigValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`NetworkInfoView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "connected_peers",
///    "known_producers",
///    "num_connected_peers",
///    "peer_max_count",
///    "tier1_accounts_data",
///    "tier1_accounts_keys",
///    "tier1_connections"
///  ],
///  "properties": {
///    "connected_peers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PeerInfoView"
///      }
///    },
///    "known_producers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/KnownProducerView"
///      }
///    },
///    "num_connected_peers": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "peer_max_count": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "tier1_accounts_data": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccountDataView"
///      }
///    },
///    "tier1_accounts_keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PublicKey"
///      }
///    },
///    "tier1_connections": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/PeerInfoView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NetworkInfoView {
    pub connected_peers: ::std::vec::Vec<PeerInfoView>,
    pub known_producers: ::std::vec::Vec<KnownProducerView>,
    pub num_connected_peers: u32,
    pub peer_max_count: u32,
    pub tier1_accounts_data: ::std::vec::Vec<AccountDataView>,
    pub tier1_accounts_keys: ::std::vec::Vec<PublicKey>,
    pub tier1_connections: ::std::vec::Vec<PeerInfoView>,
}
///`NextEpochValidatorInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key",
///    "shards",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    },
///    "shards": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "stake": {
///      "$ref": "#/components/schemas/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct NextEpochValidatorInfo {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub shards: ::std::vec::Vec<ShardId>,
    pub stake: NearToken,
}
///An Action that can be included in a transaction or receipt, excluding
/// delegate actions. This type represents all possible action types except
/// DelegateAction to prevent infinite recursion in meta-transactions.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An Action that can be included in a transaction or
/// receipt, excluding delegate actions. This type represents all possible
/// action types except DelegateAction to prevent infinite recursion in
/// meta-transactions.",
///  "oneOf": [
///    {
///      "description": "Create an (sub)account using a transaction `receiver_id` as an ID for\na new account ID must pass validation rules described here\n<https://nomicon.io/DataStructures/Account>.",
///      "type": "object",
///      "required": [
///        "CreateAccount"
///      ],
///      "properties": {
///        "CreateAccount": {
///          "$ref": "#/components/schemas/CreateAccountAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Sets a Wasm code to a receiver_id",
///      "type": "object",
///      "required": [
///        "DeployContract"
///      ],
///      "properties": {
///        "DeployContract": {
///          "$ref": "#/components/schemas/DeployContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "FunctionCall"
///      ],
///      "properties": {
///        "FunctionCall": {
///          "$ref": "#/components/schemas/FunctionCallAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Transfer"
///      ],
///      "properties": {
///        "Transfer": {
///          "$ref": "#/components/schemas/TransferAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Stake"
///      ],
///      "properties": {
///        "Stake": {
///          "$ref": "#/components/schemas/StakeAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "AddKey"
///      ],
///      "properties": {
///        "AddKey": {
///          "$ref": "#/components/schemas/AddKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteKey"
///      ],
///      "properties": {
///        "DeleteKey": {
///          "$ref": "#/components/schemas/DeleteKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteAccount"
///      ],
///      "properties": {
///        "DeleteAccount": {
///          "$ref": "#/components/schemas/DeleteAccountAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeployGlobalContract"
///      ],
///      "properties": {
///        "DeployGlobalContract": {
///          "$ref": "#/components/schemas/DeployGlobalContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "UseGlobalContract"
///      ],
///      "properties": {
///        "UseGlobalContract": {
///          "$ref": "#/components/schemas/UseGlobalContractAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeterministicStateInit"
///      ],
///      "properties": {
///        "DeterministicStateInit": {
///          "$ref": "#/components/schemas/DeterministicStateInitAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "AddGasKey"
///      ],
///      "properties": {
///        "AddGasKey": {
///          "$ref": "#/components/schemas/AddGasKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "DeleteGasKey"
///      ],
///      "properties": {
///        "DeleteGasKey": {
///          "$ref": "#/components/schemas/DeleteGasKeyAction"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "TransferToGasKey"
///      ],
///      "properties": {
///        "TransferToGasKey": {
///          "$ref": "#/components/schemas/TransferToGasKeyAction"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum NonDelegateAction {
    ///Create an (sub)account using a transaction `receiver_id` as an ID
    /// for a new account ID must pass validation rules described
    /// here <https://nomicon.io/DataStructures/Account>.
    CreateAccount(CreateAccountAction),
    ///Sets a Wasm code to a receiver_id
    DeployContract(DeployContractAction),
    FunctionCall(FunctionCallAction),
    Transfer(TransferAction),
    Stake(StakeAction),
    AddKey(AddKeyAction),
    DeleteKey(DeleteKeyAction),
    DeleteAccount(DeleteAccountAction),
    DeployGlobalContract(DeployGlobalContractAction),
    UseGlobalContract(UseGlobalContractAction),
    DeterministicStateInit(DeterministicStateInitAction),
    AddGasKey(AddGasKeyAction),
    DeleteGasKey(DeleteGasKeyAction),
    TransferToGasKey(TransferToGasKeyAction),
}
impl ::std::convert::From<CreateAccountAction> for NonDelegateAction {
    fn from(value: CreateAccountAction) -> Self {
        Self::CreateAccount(value)
    }
}
impl ::std::convert::From<DeployContractAction> for NonDelegateAction {
    fn from(value: DeployContractAction) -> Self {
        Self::DeployContract(value)
    }
}
impl ::std::convert::From<FunctionCallAction> for NonDelegateAction {
    fn from(value: FunctionCallAction) -> Self {
        Self::FunctionCall(value)
    }
}
impl ::std::convert::From<TransferAction> for NonDelegateAction {
    fn from(value: TransferAction) -> Self {
        Self::Transfer(value)
    }
}
impl ::std::convert::From<StakeAction> for NonDelegateAction {
    fn from(value: StakeAction) -> Self {
        Self::Stake(value)
    }
}
impl ::std::convert::From<AddKeyAction> for NonDelegateAction {
    fn from(value: AddKeyAction) -> Self {
        Self::AddKey(value)
    }
}
impl ::std::convert::From<DeleteKeyAction> for NonDelegateAction {
    fn from(value: DeleteKeyAction) -> Self {
        Self::DeleteKey(value)
    }
}
impl ::std::convert::From<DeleteAccountAction> for NonDelegateAction {
    fn from(value: DeleteAccountAction) -> Self {
        Self::DeleteAccount(value)
    }
}
impl ::std::convert::From<DeployGlobalContractAction> for NonDelegateAction {
    fn from(value: DeployGlobalContractAction) -> Self {
        Self::DeployGlobalContract(value)
    }
}
impl ::std::convert::From<UseGlobalContractAction> for NonDelegateAction {
    fn from(value: UseGlobalContractAction) -> Self {
        Self::UseGlobalContract(value)
    }
}
impl ::std::convert::From<DeterministicStateInitAction> for NonDelegateAction {
    fn from(value: DeterministicStateInitAction) -> Self {
        Self::DeterministicStateInit(value)
    }
}
impl ::std::convert::From<AddGasKeyAction> for NonDelegateAction {
    fn from(value: AddGasKeyAction) -> Self {
        Self::AddGasKey(value)
    }
}
impl ::std::convert::From<DeleteGasKeyAction> for NonDelegateAction {
    fn from(value: DeleteGasKeyAction) -> Self {
        Self::DeleteGasKey(value)
    }
}
impl ::std::convert::From<TransferToGasKeyAction> for NonDelegateAction {
    fn from(value: TransferToGasKeyAction) -> Self {
        Self::TransferToGasKey(value)
    }
}
///Peer id is the public key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Peer id is the public key.",
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PeerId(pub PublicKey);
impl ::std::ops::Deref for PeerId {
    type Target = PublicKey;
    fn deref(&self) -> &PublicKey {
        &self.0
    }
}
impl ::std::convert::From<PeerId> for PublicKey {
    fn from(value: PeerId) -> Self {
        value.0
    }
}
impl ::std::convert::From<PublicKey> for PeerId {
    fn from(value: PublicKey) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PeerId {
    type Err = <PublicKey as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for PeerId {
    type Error = <PublicKey as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for PeerId {
    type Error = <PublicKey as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for PeerId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`PeerInfoView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "addr",
///    "archival",
///    "connection_established_time_millis",
///    "is_highest_block_invalid",
///    "is_outbound_peer",
///    "last_time_peer_requested_millis",
///    "last_time_received_message_millis",
///    "nonce",
///    "peer_id",
///    "received_bytes_per_sec",
///    "sent_bytes_per_sec",
///    "tracked_shards"
///  ],
///  "properties": {
///    "account_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "addr": {
///      "type": "string"
///    },
///    "archival": {
///      "type": "boolean"
///    },
///    "block_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "connection_established_time_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "is_highest_block_invalid": {
///      "type": "boolean"
///    },
///    "is_outbound_peer": {
///      "type": "boolean"
///    },
///    "last_time_peer_requested_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "last_time_received_message_millis": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "description": "Connection nonce.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "peer_id": {
///      "$ref": "#/components/schemas/PublicKey"
///    },
///    "received_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sent_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "tracked_shards": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PeerInfoView {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_id: ::std::option::Option<AccountId>,
    pub addr: ::std::string::String,
    pub archival: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_hash: ::std::option::Option<CryptoHash>,
    pub connection_established_time_millis: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub height: ::std::option::Option<u64>,
    pub is_highest_block_invalid: bool,
    pub is_outbound_peer: bool,
    pub last_time_peer_requested_millis: u64,
    pub last_time_received_message_millis: u64,
    ///Connection nonce.
    pub nonce: u64,
    pub peer_id: PublicKey,
    pub received_bytes_per_sec: u64,
    pub sent_bytes_per_sec: u64,
    pub tracked_shards: ::std::vec::Vec<ShardId>,
}
///Error that can occur while preparing or executing Wasm smart-contract.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Error that can occur while preparing or executing Wasm
/// smart-contract.",
///  "oneOf": [
///    {
///      "description": "Error happened while serializing the module.",
///      "type": "string",
///      "enum": [
///        "Serialization"
///      ]
///    },
///    {
///      "description": "Error happened while deserializing the module.",
///      "type": "string",
///      "enum": [
///        "Deserialization"
///      ]
///    },
///    {
///      "description": "Internal memory declaration has been found in the
/// module.",
///      "type": "string",
///      "enum": [
///        "InternalMemoryDeclared"
///      ]
///    },
///    {
///      "description": "Gas instrumentation failed.\n\nThis most likely
/// indicates the module isn't valid.",
///      "type": "string",
///      "enum": [
///        "GasInstrumentation"
///      ]
///    },
///    {
///      "description": "Stack instrumentation failed.\n\nThis  most likely
/// indicates the module isn't valid.",
///      "type": "string",
///      "enum": [
///        "StackHeightInstrumentation"
///      ]
///    },
///    {
///      "description": "Error happened during instantiation.\n\nThis might
/// indicate that `start` function trapped, or module isn't\ninstantiable
/// and/or un-linkable.",
///      "type": "string",
///      "enum": [
///        "Instantiate"
///      ]
///    },
///    {
///      "description": "Error creating memory.",
///      "type": "string",
///      "enum": [
///        "Memory"
///      ]
///    },
///    {
///      "description": "Contract contains too many functions.",
///      "type": "string",
///      "enum": [
///        "TooManyFunctions"
///      ]
///    },
///    {
///      "description": "Contract contains too many locals.",
///      "type": "string",
///      "enum": [
///        "TooManyLocals"
///      ]
///    },
///    {
///      "description": "Contract contains too many tables.",
///      "type": "string",
///      "enum": [
///        "TooManyTables"
///      ]
///    },
///    {
///      "description": "Contract contains too many table elements.",
///      "type": "string",
///      "enum": [
///        "TooManyTableElements"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum PrepareError {
    ///Error happened while serializing the module.
    Serialization,
    ///Error happened while deserializing the module.
    Deserialization,
    ///Internal memory declaration has been found in the module.
    InternalMemoryDeclared,
    ///Gas instrumentation failed.
    ///
    ///This most likely indicates the module isn't valid.
    GasInstrumentation,
    ///Stack instrumentation failed.
    ///
    ///This  most likely indicates the module isn't valid.
    StackHeightInstrumentation,
    ///Error happened during instantiation.
    ///
    ///This might indicate that `start` function trapped, or module isn't
    ///instantiable and/or un-linkable.
    Instantiate,
    ///Error creating memory.
    Memory,
    ///Contract contains too many functions.
    TooManyFunctions,
    ///Contract contains too many locals.
    TooManyLocals,
    ///Contract contains too many tables.
    TooManyTables,
    ///Contract contains too many table elements.
    TooManyTableElements,
}
impl ::std::fmt::Display for PrepareError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Serialization => f.write_str("Serialization"),
            Self::Deserialization => f.write_str("Deserialization"),
            Self::InternalMemoryDeclared => f.write_str("InternalMemoryDeclared"),
            Self::GasInstrumentation => f.write_str("GasInstrumentation"),
            Self::StackHeightInstrumentation => f.write_str("StackHeightInstrumentation"),
            Self::Instantiate => f.write_str("Instantiate"),
            Self::Memory => f.write_str("Memory"),
            Self::TooManyFunctions => f.write_str("TooManyFunctions"),
            Self::TooManyLocals => f.write_str("TooManyLocals"),
            Self::TooManyTables => f.write_str("TooManyTables"),
            Self::TooManyTableElements => f.write_str("TooManyTableElements"),
        }
    }
}
impl ::std::str::FromStr for PrepareError {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Serialization" => Ok(Self::Serialization),
            "Deserialization" => Ok(Self::Deserialization),
            "InternalMemoryDeclared" => Ok(Self::InternalMemoryDeclared),
            "GasInstrumentation" => Ok(Self::GasInstrumentation),
            "StackHeightInstrumentation" => Ok(Self::StackHeightInstrumentation),
            "Instantiate" => Ok(Self::Instantiate),
            "Memory" => Ok(Self::Memory),
            "TooManyFunctions" => Ok(Self::TooManyFunctions),
            "TooManyLocals" => Ok(Self::TooManyLocals),
            "TooManyTables" => Ok(Self::TooManyTables),
            "TooManyTableElements" => Ok(Self::TooManyTableElements),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PrepareError {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configures whether the node checks the next or the next next epoch for
/// network version compatibility.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures whether the node checks the next or the next
/// next epoch for network version compatibility.",
///  "type": "string",
///  "enum": [
///    "Next",
///    "NextNext"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProtocolVersionCheckConfig {
    Next,
    NextNext,
}
impl ::std::fmt::Display for ProtocolVersionCheckConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Next => f.write_str("Next"),
            Self::NextNext => f.write_str("NextNext"),
        }
    }
}
impl ::std::str::FromStr for ProtocolVersionCheckConfig {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Next" => Ok(Self::Next),
            "NextNext" => Ok(Self::NextNext),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProtocolVersionCheckConfig {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PublicKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct PublicKey(pub ::std::string::String);
impl ::std::ops::Deref for PublicKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for PublicKey {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for PublicKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`RangeOfUint64`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end",
///    "start"
///  ],
///  "properties": {
///    "end": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RangeOfUint64 {
    pub end: u64,
    pub start: u64,
}
///`ReceiptEnumView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "Action"
///      ],
///      "properties": {
///        "Action": {
///          "type": "object",
///          "required": [
///            "actions",
///            "gas_price",
///            "input_data_ids",
///            "output_data_receivers",
///            "signer_id",
///            "signer_public_key"
///          ],
///          "properties": {
///            "actions": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/ActionView"
///              }
///            },
///            "gas_price": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "input_data_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/CryptoHash"
///              }
///            },
///            "is_promise_yield": {
///              "default": false,
///              "type": "boolean"
///            },
///            "output_data_receivers": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/DataReceiverView"
///              }
///            },
///            "refund_to": {
///              "anyOf": [
///                {
///                  "$ref": "#/components/schemas/AccountId"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "signer_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "signer_public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "Data"
///      ],
///      "properties": {
///        "Data": {
///          "type": "object",
///          "required": [
///            "data_id"
///          ],
///          "properties": {
///            "data": {
///              "type": [
///                "string",
///                "null"
///              ]
///            },
///            "data_id": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "is_promise_resume": {
///              "default": false,
///              "type": "boolean"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "GlobalContractDistribution"
///      ],
///      "properties": {
///        "GlobalContractDistribution": {
///          "type": "object",
///          "required": [
///            "already_delivered_shards",
///            "code",
///            "id",
///            "target_shard"
///          ],
///          "properties": {
///            "already_delivered_shards": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/ShardId"
///              }
///            },
///            "code": {
///              "type": "string"
///            },
///            "id": {
///              "$ref": "#/components/schemas/GlobalContractIdentifier"
///            },
///            "target_shard": {
///              "$ref": "#/components/schemas/ShardId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ReceiptEnumView {
    Action {
        actions: ::std::vec::Vec<ActionView>,
        gas_price: NearToken,
        input_data_ids: ::std::vec::Vec<CryptoHash>,
        #[serde(default)]
        is_promise_yield: bool,
        output_data_receivers: ::std::vec::Vec<DataReceiverView>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        refund_to: ::std::option::Option<AccountId>,
        signer_id: AccountId,
        signer_public_key: PublicKey,
    },
    Data {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        data: ::std::option::Option<::std::string::String>,
        data_id: CryptoHash,
        #[serde(default)]
        is_promise_resume: bool,
    },
    GlobalContractDistribution {
        already_delivered_shards: ::std::vec::Vec<ShardId>,
        code: ::std::string::String,
        id: GlobalContractIdentifier,
        target_shard: ShardId,
    },
}
///Describes the error for validating a receipt.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the error for validating a receipt.",
///  "oneOf": [
///    {
///      "description": "The `predecessor_id` of a Receipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidPredecessorId"
///      ],
///      "properties": {
///        "InvalidPredecessorId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The `receiver_id` of a Receipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidReceiverId"
///      ],
///      "properties": {
///        "InvalidReceiverId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The `signer_id` of an ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidSignerId"
///      ],
///      "properties": {
///        "InvalidSignerId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The `receiver_id` of a DataReceiver within an
/// ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidDataReceiverId"
///      ],
///      "properties": {
///        "InvalidDataReceiverId": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The length of the returned data exceeded the limit
/// in a DataReceipt.",
///      "type": "object",
///      "required": [
///        "ReturnedValueLengthExceeded"
///      ],
///      "properties": {
///        "ReturnedValueLengthExceeded": {
///          "type": "object",
///          "required": [
///            "length",
///            "limit"
///          ],
///          "properties": {
///            "length": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The number of input data dependencies exceeds the
/// limit in an ActionReceipt.",
///      "type": "object",
///      "required": [
///        "NumberInputDataDependenciesExceeded"
///      ],
///      "properties": {
///        "NumberInputDataDependenciesExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "number_of_input_data_dependencies"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "number_of_input_data_dependencies": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An error occurred while validating actions of an
/// ActionReceipt.",
///      "type": "object",
///      "required": [
///        "ActionsValidation"
///      ],
///      "properties": {
///        "ActionsValidation": {
///          "$ref": "#/components/schemas/ActionsValidationError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Receipt is bigger than the limit.",
///      "type": "object",
///      "required": [
///        "ReceiptSizeExceeded"
///      ],
///      "properties": {
///        "ReceiptSizeExceeded": {
///          "type": "object",
///          "required": [
///            "limit",
///            "size"
///          ],
///          "properties": {
///            "limit": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "size": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "The `refund_to` of an ActionReceipt is not valid.",
///      "type": "object",
///      "required": [
///        "InvalidRefundTo"
///      ],
///      "properties": {
///        "InvalidRefundTo": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "type": "string"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum ReceiptValidationError {
    ///The `predecessor_id` of a Receipt is not valid.
    InvalidPredecessorId { account_id: ::std::string::String },
    ///The `receiver_id` of a Receipt is not valid.
    InvalidReceiverId { account_id: ::std::string::String },
    ///The `signer_id` of an ActionReceipt is not valid.
    InvalidSignerId { account_id: ::std::string::String },
    ///The `receiver_id` of a DataReceiver within an ActionReceipt is not
    /// valid.
    InvalidDataReceiverId { account_id: ::std::string::String },
    ///The length of the returned data exceeded the limit in a DataReceipt.
    ReturnedValueLengthExceeded { length: u64, limit: u64 },
    ///The number of input data dependencies exceeds the limit in an
    /// ActionReceipt.
    NumberInputDataDependenciesExceeded {
        limit: u64,
        number_of_input_data_dependencies: u64,
    },
    ///An error occurred while validating actions of an ActionReceipt.
    ActionsValidation(ActionsValidationError),
    ///Receipt is bigger than the limit.
    ReceiptSizeExceeded { limit: u64, size: u64 },
    ///The `refund_to` of an ActionReceipt is not valid.
    InvalidRefundTo { account_id: ::std::string::String },
}
impl ::std::convert::From<ActionsValidationError> for ReceiptValidationError {
    fn from(value: ActionsValidationError) -> Self {
        Self::ActionsValidation(value)
    }
}
///`ReceiptView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "predecessor_id",
///    "receipt",
///    "receipt_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "predecessor_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "priority": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "receipt": {
///      "$ref": "#/components/schemas/ReceiptEnumView"
///    },
///    "receipt_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReceiptView {
    pub predecessor_id: AccountId,
    #[serde(default)]
    pub priority: u64,
    pub receipt: ReceiptEnumView,
    pub receipt_id: CryptoHash,
    pub receiver_id: AccountId,
}
///`RpcBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "NOT_SYNCED_YET"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcBlockError {
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "NOT_SYNCED_YET")]
    NotSyncedYet,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcBlockError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcBlockRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/components/schemas/Finality"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/components/schemas/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcBlockRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcBlockRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcBlockRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcBlockRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcBlockResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "author",
///    "chunks",
///    "header"
///  ],
///  "properties": {
///    "author": {
///      "description": "The AccountId of the author of the Block",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    },
///    "chunks": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ChunkHeaderView"
///      }
///    },
///    "header": {
///      "$ref": "#/components/schemas/BlockHeaderView"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcBlockResponse {
    ///The AccountId of the author of the Block
    pub author: AccountId,
    pub chunks: ::std::vec::Vec<ChunkHeaderView>,
    pub header: BlockHeaderView,
}
///`RpcChunkError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "shard_id"
///          ],
///          "properties": {
///            "shard_id": {
///              "$ref": "#/components/schemas/ShardId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INVALID_SHARD_ID"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "chunk_hash"
///          ],
///          "properties": {
///            "chunk_hash": {
///              "$ref": "#/components/schemas/ChunkHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_CHUNK"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcChunkError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "INVALID_SHARD_ID")]
    InvalidShardId { shard_id: ShardId },
    #[serde(rename = "UNKNOWN_CHUNK")]
    UnknownChunk { chunk_hash: ChunkHash },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcChunkError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcChunkRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcChunkRequest",
///  "type": "object",
///  "anyOf": [
///    {
///      "title": "block_shard_id",
///      "type": "object",
///      "required": [
///        "block_id",
///        "shard_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        },
///        "shard_id": {
///          "$ref": "#/components/schemas/ShardId"
///        }
///      }
///    },
///    {
///      "title": "chunk_hash",
///      "type": "object",
///      "required": [
///        "chunk_id"
///      ],
///      "properties": {
///        "chunk_id": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcChunkRequest {
    BlockShardId {
        block_id: BlockId,
        shard_id: ShardId,
    },
    ChunkHash {
        chunk_id: CryptoHash,
    },
}
///`RpcChunkResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "author",
///    "header",
///    "receipts",
///    "transactions"
///  ],
///  "properties": {
///    "author": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "header": {
///      "$ref": "#/components/schemas/ChunkHeaderView"
///    },
///    "receipts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ReceiptView"
///      }
///    },
///    "transactions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/SignedTransactionView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcChunkResponse {
    pub author: AccountId,
    pub header: ChunkHeaderView,
    pub receipts: ::std::vec::Vec<ReceiptView>,
    pub transactions: ::std::vec::Vec<SignedTransactionView>,
}
///`RpcClientConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcClientConfigError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcClientConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcClientConfigRequest",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcClientConfigRequest(pub ());
impl ::std::ops::Deref for RpcClientConfigRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcClientConfigRequest> for () {
    fn from(value: RpcClientConfigRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcClientConfigRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///ClientConfig where some fields can be updated at runtime.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "ClientConfig where some fields can be updated at
/// runtime.",
///  "type": "object",
///  "properties": {
///    "archive": {
///      "description": "Not clear old data, set `true` for archive nodes.",
///      "type": "boolean"
///    },
///    "block_fetch_horizon": {
///      "description": "Horizon at which instead of fetching block, fetch
/// full state.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "block_header_fetch_horizon": {
///      "description": "Behind this horizon header fetch kicks in.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "block_production_tracking_delay": {
///      "description": "Duration to check for producing / skipping block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "catchup_step_period": {
///      "description": "Time between check to perform catchup.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chain_id": {
///      "description": "Chain id for status.",
///      "type": "string"
///    },
///    "chunk_distribution_network": {
///      "description": "Optional config for the Chunk Distribution Network
/// feature.\nIf set to `None` then this node does not participate in the
/// Chunk Distribution Network.\nNodes not participating will still function
/// fine, but possibly with higher\nlatency due to the need of requesting
/// chunks over the peer-to-peer network.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/ChunkDistributionNetworkConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "chunk_request_retry_period": {
///      "description": "Time between checking to re-request chunks.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chunk_validation_threads": {
///      "description": "Number of threads for ChunkValidationActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "chunk_wait_mult": {
///      "description": "Multiplier for the wait time for all chunks to be
/// received.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "chunks_cache_height_horizon": {
///      "description": "Height horizon for the chunk cache. A chunk is
/// removed from the cache\nif its height + chunks_cache_height_horizon <
/// largest_seen_height.\nThe default value is
/// DEFAULT_CHUNKS_CACHE_HEIGHT_HORIZON.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "client_background_migration_threads": {
///      "description": "Number of threads to execute background migration
/// work in client.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "cloud_archival_writer": {
///      "description": "Configuration for a cloud-based archival writer. If
/// this config is present, the writer is enabled and\nwrites chunk-related
/// data based on the tracked shards.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CloudArchivalWriterConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "disable_tx_routing": {
///      "description": "If true, the node won't forward transactions to
/// next the chunk producers.",
///      "type": "boolean"
///    },
///    "doomslug_step_period": {
///      "description": "Time between running doomslug timer.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "dynamic_resharding_dry_run": {
///      "description": "If true, the runtime will do a dynamic resharding
/// 'dry run' at the last block of each epoch.\nThis means calculating
/// tentative boundary accounts for splitting the tracked shards.",
///      "type": "boolean"
///    },
///    "enable_early_prepare_transactions": {
///      "description": "If true, transactions for the next chunk will be
/// prepared early, right after the previous chunk's\npost-state is ready.
/// This can help produce chunks faster, for high-throughput chains.\nThe
/// current implementation increases latency on low-load chains, which will
/// be fixed in the future.\nThe default is disabled.",
///      "type": "boolean"
///    },
///    "enable_multiline_logging": {
///      "type": "boolean"
///    },
///    "enable_statistics_export": {
///      "description": "Re-export storage layer statistics as prometheus
/// metrics.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_sync": {
///      "description": "Options for epoch sync.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/EpochSyncConfig"
///        }
///      ]
///    },
///    "expected_shutdown": {
///      "description": "Graceful shutdown at expected block height.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/MutableConfigValue"
///        }
///      ]
///    },
///    "gc": {
///      "description": "Garbage collection configuration.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/GCConfig"
///        }
///      ]
///    },
///    "header_sync_expected_height_per_second": {
///      "description": "Expected increase of header head height per second
/// during header sync",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "header_sync_initial_timeout": {
///      "description": "How much time to wait after initial header sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "header_sync_progress_timeout": {
///      "description": "How much time to wait after some progress is made
/// in header sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "header_sync_stall_ban_timeout": {
///      "description": "How much time to wait before banning a peer in
/// header sync if sync is too slow",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "log_summary_period": {
///      "description": "Period between logging summary information.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "log_summary_style": {
///      "description": "Enable coloring of the logs",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/LogSummaryStyle"
///        }
///      ]
///    },
///    "max_block_production_delay": {
///      "description": "Maximum wait for approvals before producing
/// block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_block_wait_delay": {
///      "description": "Maximum duration before skipping given height.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_gas_burnt_view": {
///      "description": "Max burnt gas per view method.  If present,
/// overrides value stored in\ngenesis file.  The value only affects the
/// RPCs without influencing the\nprotocol thus changing it per-node doesn’t
/// affect the blockchain.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "min_block_production_delay": {
///      "description": "Minimum duration before producing block.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "min_num_peers": {
///      "description": "Minimum number of peers to start syncing.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "orphan_state_witness_max_size": {
///      "description": "Maximum size of state witnesses in the
/// OrphanStateWitnessPool.\n\nWe keep only orphan witnesses which are
/// smaller than this size.\nThis limits the maximum memory usage of
/// OrphanStateWitnessPool.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "orphan_state_witness_pool_size": {
///      "description": "OrphanStateWitnessPool keeps instances of
/// ChunkStateWitness which can't be processed\nbecause the previous block
/// isn't available. The witnesses wait in the pool until the\nrequired
/// block appears. This variable controls how many witnesses can be stored
/// in the pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "produce_chunk_add_transactions_time_limit": {
///      "description": "Limit the time of adding transactions to a
/// chunk.\nA node produces a chunk by adding transactions from the
/// transaction pool until\nsome limit is reached. This time limit ensures
/// that adding transactions won't take\nlonger than the specified duration,
/// which helps to produce the chunk quickly.",
///      "type": "string"
///    },
///    "produce_empty_blocks": {
///      "description": "Produce empty blocks, use `false` for testing.",
///      "type": "boolean"
///    },
///    "protocol_version_check": {
///      "description": "Determines whether client should exit if the protocol version is not supported\nfor the next or next next epoch.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ProtocolVersionCheckConfig"
///        }
///      ]
///    },
///    "resharding_config": {
///      "$ref": "#/components/schemas/MutableConfigValue"
///    },
///    "rpc_addr": {
///      "description": "Listening rpc port for status.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "save_invalid_witnesses": {
///      "description": "Save observed instances of invalid
/// ChunkStateWitness to the database in
/// DBCol::InvalidChunkStateWitnesses.\nSaving invalid witnesses is useful
/// for analysis and debugging.\nThis option can cause extra load on the
/// database and is not recommended for production use.",
///      "type": "boolean"
///    },
///    "save_latest_witnesses": {
///      "description": "Save observed instances of ChunkStateWitness to the database in DBCol::LatestChunkStateWitnesses.\nSaving the latest witnesses is useful for analysis and debugging.\nThis option can cause extra load on the database and is not recommended for production use.",
///      "type": "boolean"
///    },
///    "save_state_changes": {
///      "description": "Whether to persist state changes on disk or not.",
///      "type": "boolean"
///    },
///    "save_trie_changes": {
///      "description": "save_trie_changes should be set to true iff\n-
/// archive if false - non-archival nodes need trie changes to perform
/// garbage collection\n- archive is true, cold_store is configured and
/// migration to split_storage is finished - node\nworking in split storage
/// mode needs trie changes in order to do garbage collection on hot.",
///      "type": "boolean"
///    },
///    "save_tx_outcomes": {
///      "description": "Whether to persist transaction outcomes to disk or
/// not.",
///      "type": "boolean"
///    },
///    "save_untracked_partial_chunks_parts": {
///      "description": "Whether to persist partial chunk parts for
/// untracked shards or not.",
///      "type": "boolean"
///    },
///    "skip_sync_wait": {
///      "description": "Skip waiting for sync (for testing or single node
/// testnet).",
///      "type": "boolean"
///    },
///    "state_request_server_threads": {
///      "description": "Number of threads for StateRequestActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "state_request_throttle_period": {
///      "description": "Number of seconds between state requests for view
/// client.\nThrottling window for state requests (headers and parts).",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_requests_per_throttle_period": {
///      "description": "Maximum number of state requests served per
/// throttle period",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "state_sync": {
///      "description": "Options for syncing state.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/StateSyncConfig"
///        }
///      ]
///    },
///    "state_sync_enabled": {
///      "description": "Whether to use the State Sync mechanism.\nIf
/// disabled, the node will do Block Sync instead of State Sync.",
///      "type": "boolean"
///    },
///    "state_sync_external_backoff": {
///      "description": "Additional waiting period after a failed request to
/// external storage",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_external_timeout": {
///      "description": "How long to wait for a response from centralized
/// state sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_p2p_timeout": {
///      "description": "How long to wait for a response from p2p state
/// sync",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "state_sync_retry_backoff": {
///      "description": "How long to wait after a failed state sync
/// request",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "sync_check_period": {
///      "description": "How often to check that we are not out of sync.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "sync_height_threshold": {
///      "description": "Sync height threshold: below this difference in
/// height don't start syncing.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sync_max_block_requests": {
///      "description": "Maximum number of block requests to send to peers
/// to sync",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "sync_step_period": {
///      "description": "While syncing, how long to check for each step.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "tracked_shards_config": {
///      "$ref": "#/components/schemas/TrackedShardsConfig"
///    },
///    "transaction_pool_size_limit": {
///      "description": "Limit of the size of per-shard transaction pool
/// measured in bytes. If not set, the size\nwill be unbounded.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "transaction_request_handler_threads": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "trie_viewer_state_size_limit": {
///      "description": "Upper bound of the byte size of contract state that
/// is still viewable. None is no limit",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ttl_account_id_router": {
///      "description": "Time to persist Accounts Id in the router without
/// removing them.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "tx_routing_height_horizon": {
///      "description": "If the node is not a chunk producer within that
/// many blocks, then route\nto upcoming chunk producers.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "version": {
///      "description": "Version of the binary.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Version"
///        }
///      ]
///    },
///    "view_client_threads": {
///      "description": "Number of threads for ViewClientActor pool.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcClientConfigResponse {
    ///Not clear old data, set `true` for archive nodes.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub archive: ::std::option::Option<bool>,
    ///Horizon at which instead of fetching block, fetch full state.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_fetch_horizon: ::std::option::Option<u64>,
    ///Behind this horizon header fetch kicks in.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_header_fetch_horizon: ::std::option::Option<u64>,
    ///Duration to check for producing / skipping block.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_production_tracking_delay: ::std::option::Option<[u64; 2usize]>,
    ///Time between check to perform catchup.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub catchup_step_period: ::std::option::Option<[u64; 2usize]>,
    ///Chain id for status.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Optional config for the Chunk Distribution Network feature.
    ///If set to `None` then this node does not participate in the Chunk
    /// Distribution Network. Nodes not participating will still
    /// function fine, but possibly with higher latency due to the
    /// need of requesting chunks over the peer-to-peer network.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_distribution_network: ::std::option::Option<ChunkDistributionNetworkConfig>,
    ///Time between checking to re-request chunks.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_request_retry_period: ::std::option::Option<[u64; 2usize]>,
    ///Number of threads for ChunkValidationActor pool.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_validation_threads: ::std::option::Option<u32>,
    ///Multiplier for the wait time for all chunks to be received.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_wait_mult: ::std::option::Option<[i32; 2usize]>,
    ///Height horizon for the chunk cache. A chunk is removed from the
    /// cache if its height + chunks_cache_height_horizon <
    /// largest_seen_height. The default value is
    /// DEFAULT_CHUNKS_CACHE_HEIGHT_HORIZON.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunks_cache_height_horizon: ::std::option::Option<u64>,
    ///Number of threads to execute background migration work in client.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client_background_migration_threads: ::std::option::Option<u32>,
    ///Configuration for a cloud-based archival writer. If this config is
    /// present, the writer is enabled and writes chunk-related data
    /// based on the tracked shards.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cloud_archival_writer: ::std::option::Option<CloudArchivalWriterConfig>,
    ///If true, the node won't forward transactions to next the chunk
    /// producers.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub disable_tx_routing: ::std::option::Option<bool>,
    ///Time between running doomslug timer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub doomslug_step_period: ::std::option::Option<[u64; 2usize]>,
    ///If true, the runtime will do a dynamic resharding 'dry run' at the
    /// last block of each epoch. This means calculating tentative
    /// boundary accounts for splitting the tracked shards.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dynamic_resharding_dry_run: ::std::option::Option<bool>,
    ///If true, transactions for the next chunk will be prepared early,
    /// right after the previous chunk's post-state is ready. This
    /// can help produce chunks faster, for high-throughput chains.
    /// The current implementation increases latency on low-load chains,
    /// which will be fixed in the future. The default is disabled.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enable_early_prepare_transactions: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enable_multiline_logging: ::std::option::Option<bool>,
    ///Re-export storage layer statistics as prometheus metrics.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enable_statistics_export: ::std::option::Option<bool>,
    ///Epoch length.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_length: ::std::option::Option<u64>,
    ///Options for epoch sync.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_sync: ::std::option::Option<EpochSyncConfig>,
    ///Graceful shutdown at expected block height.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expected_shutdown: ::std::option::Option<MutableConfigValue>,
    ///Garbage collection configuration.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gc: ::std::option::Option<GcConfig>,
    ///Expected increase of header head height per second during header
    /// sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header_sync_expected_height_per_second: ::std::option::Option<u64>,
    ///How much time to wait after initial header sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header_sync_initial_timeout: ::std::option::Option<[u64; 2usize]>,
    ///How much time to wait after some progress is made in header sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header_sync_progress_timeout: ::std::option::Option<[u64; 2usize]>,
    ///How much time to wait before banning a peer in header sync if sync
    /// is too slow
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header_sync_stall_ban_timeout: ::std::option::Option<[u64; 2usize]>,
    ///Period between logging summary information.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_summary_period: ::std::option::Option<[u64; 2usize]>,
    ///Enable coloring of the logs
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub log_summary_style: ::std::option::Option<LogSummaryStyle>,
    ///Maximum wait for approvals before producing block.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_block_production_delay: ::std::option::Option<[u64; 2usize]>,
    ///Maximum duration before skipping given height.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_block_wait_delay: ::std::option::Option<[u64; 2usize]>,
    ///Max burnt gas per view method.  If present, overrides value stored
    /// in genesis file.  The value only affects the RPCs without
    /// influencing the protocol thus changing it per-node doesn’t
    /// affect the blockchain.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_gas_burnt_view: ::std::option::Option<NearGas>,
    ///Minimum duration before producing block.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_block_production_delay: ::std::option::Option<[u64; 2usize]>,
    ///Minimum number of peers to start syncing.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_num_peers: ::std::option::Option<u32>,
    ///Number of block producer seats
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub num_block_producer_seats: ::std::option::Option<u64>,
    ///Maximum size of state witnesses in the OrphanStateWitnessPool.
    ///
    ///We keep only orphan witnesses which are smaller than this size.
    ///This limits the maximum memory usage of OrphanStateWitnessPool.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orphan_state_witness_max_size: ::std::option::Option<u64>,
    ///OrphanStateWitnessPool keeps instances of ChunkStateWitness which
    /// can't be processed because the previous block isn't
    /// available. The witnesses wait in the pool until the required
    /// block appears. This variable controls how many witnesses can be
    /// stored in the pool.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub orphan_state_witness_pool_size: ::std::option::Option<u32>,
    ///Limit the time of adding transactions to a chunk.
    ///A node produces a chunk by adding transactions from the transaction
    /// pool until some limit is reached. This time limit ensures
    /// that adding transactions won't take longer than the
    /// specified duration, which helps to produce the chunk quickly.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub produce_chunk_add_transactions_time_limit: ::std::option::Option<::std::string::String>,
    ///Produce empty blocks, use `false` for testing.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub produce_empty_blocks: ::std::option::Option<bool>,
    ///Determines whether client should exit if the protocol version is not
    /// supported for the next or next next epoch.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol_version_check: ::std::option::Option<ProtocolVersionCheckConfig>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resharding_config: ::std::option::Option<MutableConfigValue>,
    ///Listening rpc port for status.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rpc_addr: ::std::option::Option<::std::string::String>,
    ///Save observed instances of invalid ChunkStateWitness to the database
    /// in DBCol::InvalidChunkStateWitnesses. Saving invalid
    /// witnesses is useful for analysis and debugging. This option
    /// can cause extra load on the database and is not recommended for
    /// production use.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_invalid_witnesses: ::std::option::Option<bool>,
    ///Save observed instances of ChunkStateWitness to the database in
    /// DBCol::LatestChunkStateWitnesses. Saving the latest
    /// witnesses is useful for analysis and debugging. This option
    /// can cause extra load on the database and is not recommended for
    /// production use.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_latest_witnesses: ::std::option::Option<bool>,
    ///Whether to persist state changes on disk or not.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_state_changes: ::std::option::Option<bool>,
    ///save_trie_changes should be set to true iff
    /// - archive if false - non-archival nodes need trie changes to perform
    ///   garbage collection
    /// - archive is true, cold_store is configured and migration to
    ///   split_storage is finished - node
    ///working in split storage mode needs trie changes in order to do
    /// garbage collection on hot.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_trie_changes: ::std::option::Option<bool>,
    ///Whether to persist transaction outcomes to disk or not.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_tx_outcomes: ::std::option::Option<bool>,
    ///Whether to persist partial chunk parts for untracked shards or not.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub save_untracked_partial_chunks_parts: ::std::option::Option<bool>,
    ///Skip waiting for sync (for testing or single node testnet).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub skip_sync_wait: ::std::option::Option<bool>,
    ///Number of threads for StateRequestActor pool.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_request_server_threads: ::std::option::Option<u32>,
    ///Number of seconds between state requests for view client.
    ///Throttling window for state requests (headers and parts).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_request_throttle_period: ::std::option::Option<[u64; 2usize]>,
    ///Maximum number of state requests served per throttle period
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_requests_per_throttle_period: ::std::option::Option<u32>,
    ///Options for syncing state.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync: ::std::option::Option<StateSyncConfig>,
    ///Whether to use the State Sync mechanism.
    ///If disabled, the node will do Block Sync instead of State Sync.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync_enabled: ::std::option::Option<bool>,
    ///Additional waiting period after a failed request to external storage
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync_external_backoff: ::std::option::Option<[u64; 2usize]>,
    ///How long to wait for a response from centralized state sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync_external_timeout: ::std::option::Option<[u64; 2usize]>,
    ///How long to wait for a response from p2p state sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync_p2p_timeout: ::std::option::Option<[u64; 2usize]>,
    ///How long to wait after a failed state sync request
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub state_sync_retry_backoff: ::std::option::Option<[u64; 2usize]>,
    ///How often to check that we are not out of sync.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync_check_period: ::std::option::Option<[u64; 2usize]>,
    ///Sync height threshold: below this difference in height don't start
    /// syncing.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync_height_threshold: ::std::option::Option<u64>,
    ///Maximum number of block requests to send to peers to sync
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync_max_block_requests: ::std::option::Option<u32>,
    ///While syncing, how long to check for each step.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync_step_period: ::std::option::Option<[u64; 2usize]>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tracked_shards_config: ::std::option::Option<TrackedShardsConfig>,
    ///Limit of the size of per-shard transaction pool measured in bytes.
    /// If not set, the size will be unbounded.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_pool_size_limit: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_request_handler_threads: ::std::option::Option<u32>,
    ///Upper bound of the byte size of contract state that is still
    /// viewable. None is no limit
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trie_viewer_state_size_limit: ::std::option::Option<u64>,
    ///Time to persist Accounts Id in the router without removing them.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ttl_account_id_router: ::std::option::Option<[u64; 2usize]>,
    ///If the node is not a chunk producer within that many blocks, then
    /// route to upcoming chunk producers.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_routing_height_horizon: ::std::option::Option<u64>,
    ///Version of the binary.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub version: ::std::option::Option<Version>,
    ///Number of threads for ViewClientActor pool.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub view_client_threads: ::std::option::Option<u32>,
}
impl ::std::default::Default for RpcClientConfigResponse {
    fn default() -> Self {
        Self {
            archive: Default::default(),
            block_fetch_horizon: Default::default(),
            block_header_fetch_horizon: Default::default(),
            block_production_tracking_delay: Default::default(),
            catchup_step_period: Default::default(),
            chain_id: Default::default(),
            chunk_distribution_network: Default::default(),
            chunk_request_retry_period: Default::default(),
            chunk_validation_threads: Default::default(),
            chunk_wait_mult: Default::default(),
            chunks_cache_height_horizon: Default::default(),
            client_background_migration_threads: Default::default(),
            cloud_archival_writer: Default::default(),
            disable_tx_routing: Default::default(),
            doomslug_step_period: Default::default(),
            dynamic_resharding_dry_run: Default::default(),
            enable_early_prepare_transactions: Default::default(),
            enable_multiline_logging: Default::default(),
            enable_statistics_export: Default::default(),
            epoch_length: Default::default(),
            epoch_sync: Default::default(),
            expected_shutdown: Default::default(),
            gc: Default::default(),
            header_sync_expected_height_per_second: Default::default(),
            header_sync_initial_timeout: Default::default(),
            header_sync_progress_timeout: Default::default(),
            header_sync_stall_ban_timeout: Default::default(),
            log_summary_period: Default::default(),
            log_summary_style: Default::default(),
            max_block_production_delay: Default::default(),
            max_block_wait_delay: Default::default(),
            max_gas_burnt_view: Default::default(),
            min_block_production_delay: Default::default(),
            min_num_peers: Default::default(),
            num_block_producer_seats: Default::default(),
            orphan_state_witness_max_size: Default::default(),
            orphan_state_witness_pool_size: Default::default(),
            produce_chunk_add_transactions_time_limit: Default::default(),
            produce_empty_blocks: Default::default(),
            protocol_version_check: Default::default(),
            resharding_config: Default::default(),
            rpc_addr: Default::default(),
            save_invalid_witnesses: Default::default(),
            save_latest_witnesses: Default::default(),
            save_state_changes: Default::default(),
            save_trie_changes: Default::default(),
            save_tx_outcomes: Default::default(),
            save_untracked_partial_chunks_parts: Default::default(),
            skip_sync_wait: Default::default(),
            state_request_server_threads: Default::default(),
            state_request_throttle_period: Default::default(),
            state_requests_per_throttle_period: Default::default(),
            state_sync: Default::default(),
            state_sync_enabled: Default::default(),
            state_sync_external_backoff: Default::default(),
            state_sync_external_timeout: Default::default(),
            state_sync_p2p_timeout: Default::default(),
            state_sync_retry_backoff: Default::default(),
            sync_check_period: Default::default(),
            sync_height_threshold: Default::default(),
            sync_max_block_requests: Default::default(),
            sync_step_period: Default::default(),
            tracked_shards_config: Default::default(),
            transaction_pool_size_limit: Default::default(),
            transaction_request_handler_threads: Default::default(),
            trie_viewer_state_size_limit: Default::default(),
            ttl_account_id_router: Default::default(),
            tx_routing_height_horizon: Default::default(),
            version: Default::default(),
            view_client_threads: Default::default(),
        }
    }
}
///`RpcCongestionLevelRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcCongestionLevelRequest",
///  "type": "object",
///  "anyOf": [
///    {
///      "title": "block_shard_id",
///      "type": "object",
///      "required": [
///        "block_id",
///        "shard_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        },
///        "shard_id": {
///          "$ref": "#/components/schemas/ShardId"
///        }
///      }
///    },
///    {
///      "title": "chunk_hash",
///      "type": "object",
///      "required": [
///        "chunk_id"
///      ],
///      "properties": {
///        "chunk_id": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcCongestionLevelRequest {
    BlockShardId {
        block_id: BlockId,
        shard_id: ShardId,
    },
    ChunkHash {
        chunk_id: CryptoHash,
    },
}
///`RpcCongestionLevelResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "congestion_level"
///  ],
///  "properties": {
///    "congestion_level": {
///      "type": "number",
///      "format": "double"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcCongestionLevelResponse {
    pub congestion_level: f64,
}
///`RpcGasPriceError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcGasPriceError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcGasPriceError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcGasPriceRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcGasPriceRequest",
///  "type": "object",
///  "properties": {
///    "block_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/BlockId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcGasPriceRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_id: ::std::option::Option<BlockId>,
}
impl ::std::default::Default for RpcGasPriceRequest {
    fn default() -> Self {
        Self {
            block_id: Default::default(),
        }
    }
}
///`RpcGasPriceResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "gas_price"
///  ],
///  "properties": {
///    "gas_price": {
///      "$ref": "#/components/schemas/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcGasPriceResponse {
    pub gas_price: NearToken,
}
///`RpcHealthRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcHealthRequest",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcHealthRequest(pub ());
impl ::std::ops::Deref for RpcHealthRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcHealthRequest> for () {
    fn from(value: RpcHealthRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcHealthRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`RpcHealthResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcHealthResponse(pub ());
impl ::std::ops::Deref for RpcHealthResponse {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcHealthResponse> for () {
    fn from(value: RpcHealthResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcHealthResponse {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`RpcKnownProducer`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id",
///    "peer_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "addr": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "peer_id": {
///      "$ref": "#/components/schemas/PeerId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcKnownProducer {
    pub account_id: AccountId,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addr: ::std::option::Option<::std::string::String>,
    pub peer_id: PeerId,
}
///`RpcLightClientBlockProofRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientBlockProofRequest",
///  "type": "object",
///  "required": [
///    "block_hash",
///    "light_client_head"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "light_client_head": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientBlockProofRequest {
    pub block_hash: CryptoHash,
    pub light_client_head: CryptoHash,
}
///`RpcLightClientBlockProofResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_header_lite",
///    "block_proof"
///  ],
///  "properties": {
///    "block_header_lite": {
///      "$ref": "#/components/schemas/LightClientBlockLiteView"
///    },
///    "block_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientBlockProofResponse {
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: ::std::vec::Vec<MerklePathItem>,
}
///`RpcLightClientExecutionProofRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "light_client_head",
///        "sender_id",
///        "transaction_hash",
///        "type"
///      ],
///      "properties": {
///        "light_client_head": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "sender_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "transaction_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "transaction"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "light_client_head",
///        "receipt_id",
///        "receiver_id",
///        "type"
///      ],
///      "properties": {
///        "light_client_head": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "receipt_id": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "receiver_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "receipt"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum RpcLightClientExecutionProofRequest {
    #[serde(rename = "transaction")]
    Transaction {
        light_client_head: CryptoHash,
        sender_id: AccountId,
        transaction_hash: CryptoHash,
    },
    #[serde(rename = "receipt")]
    Receipt {
        light_client_head: CryptoHash,
        receipt_id: CryptoHash,
        receiver_id: AccountId,
    },
}
///`RpcLightClientExecutionProofResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_header_lite",
///    "block_proof",
///    "outcome_proof",
///    "outcome_root_proof"
///  ],
///  "properties": {
///    "block_header_lite": {
///      "$ref": "#/components/schemas/LightClientBlockLiteView"
///    },
///    "block_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/MerklePathItem"
///      }
///    },
///    "outcome_proof": {
///      "$ref": "#/components/schemas/ExecutionOutcomeWithIdView"
///    },
///    "outcome_root_proof": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/MerklePathItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientExecutionProofResponse {
    pub block_header_lite: LightClientBlockLiteView,
    pub block_proof: ::std::vec::Vec<MerklePathItem>,
    pub outcome_proof: ExecutionOutcomeWithIdView,
    pub outcome_root_proof: ::std::vec::Vec<MerklePathItem>,
}
///`RpcLightClientNextBlockError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "epoch_id"
///          ],
///          "properties": {
///            "epoch_id": {
///              "$ref": "#/components/schemas/EpochId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "EPOCH_OUT_OF_BOUNDS"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcLightClientNextBlockError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "EPOCH_OUT_OF_BOUNDS")]
    EpochOutOfBounds { epoch_id: EpochId },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcLightClientNextBlockError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcLightClientNextBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcLightClientNextBlockRequest",
///  "type": "object",
///  "required": [
///    "last_block_hash"
///  ],
///  "properties": {
///    "last_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientNextBlockRequest {
    pub last_block_hash: CryptoHash,
}
///A state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A state for the current head of a light client. More info [here](https://nomicon.io/ChainSpec/LightClient).",
///  "type": "object",
///  "properties": {
///    "approvals_after_next": {
///      "type": "array",
///      "items": {
///        "anyOf": [
///          {
///            "$ref": "#/components/schemas/Signature"
///          },
///          {
///            "type": "null"
///          }
///        ]
///      }
///    },
///    "inner_lite": {
///      "description": "Inner part of the block header that gets hashed,
/// split into two parts, one that is sent\n   to light clients, and the
/// rest",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/BlockHeaderInnerLiteView"
///        }
///      ]
///    },
///    "inner_rest_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "next_block_inner_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "next_bps": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "prev_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcLightClientNextBlockResponse {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub approvals_after_next: ::std::vec::Vec<::std::option::Option<Signature>>,
    ///Inner part of the block header that gets hashed, split into two
    /// parts, one that is sent   to light clients, and the rest
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inner_lite: ::std::option::Option<BlockHeaderInnerLiteView>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub inner_rest_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_block_inner_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub next_bps: ::std::option::Option<::std::vec::Vec<ValidatorStakeView>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub prev_block_hash: ::std::option::Option<CryptoHash>,
}
impl ::std::default::Default for RpcLightClientNextBlockResponse {
    fn default() -> Self {
        Self {
            approvals_after_next: Default::default(),
            inner_lite: Default::default(),
            inner_rest_hash: Default::default(),
            next_block_inner_hash: Default::default(),
            next_bps: Default::default(),
            prev_block_hash: Default::default(),
        }
    }
}
///`RpcLightClientProofError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "execution_outcome_shard_id",
///            "number_or_shards"
///          ],
///          "properties": {
///            "execution_outcome_shard_id": {
///              "$ref": "#/components/schemas/ShardId"
///            },
///            "number_or_shards": {
///              "type": "integer",
///              "format": "uint",
///              "minimum": 0.0
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INCONSISTENT_STATE"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "transaction_or_receipt_id"
///          ],
///          "properties": {
///            "transaction_or_receipt_id": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "NOT_CONFIRMED"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "transaction_or_receipt_id"
///          ],
///          "properties": {
///            "transaction_or_receipt_id": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_TRANSACTION_OR_RECEIPT"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "shard_id",
///            "transaction_or_receipt_id"
///          ],
///          "properties": {
///            "shard_id": {
///              "$ref": "#/components/schemas/ShardId"
///            },
///            "transaction_or_receipt_id": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNAVAILABLE_SHARD"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcLightClientProofError {
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "INCONSISTENT_STATE")]
    InconsistentState {
        execution_outcome_shard_id: ShardId,
        number_or_shards: u32,
    },
    #[serde(rename = "NOT_CONFIRMED")]
    NotConfirmed {
        transaction_or_receipt_id: CryptoHash,
    },
    #[serde(rename = "UNKNOWN_TRANSACTION_OR_RECEIPT")]
    UnknownTransactionOrReceipt {
        transaction_or_receipt_id: CryptoHash,
    },
    #[serde(rename = "UNAVAILABLE_SHARD")]
    UnavailableShard {
        shard_id: ShardId,
        transaction_or_receipt_id: CryptoHash,
    },
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcLightClientProofError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcMaintenanceWindowsError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcMaintenanceWindowsError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcMaintenanceWindowsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcMaintenanceWindowsRequest",
///  "type": "object",
///  "required": [
///    "account_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcMaintenanceWindowsRequest {
    pub account_id: AccountId,
}
///`RpcNetworkInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcNetworkInfoError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcNetworkInfoRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcNetworkInfoRequest",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcNetworkInfoRequest(pub ());
impl ::std::ops::Deref for RpcNetworkInfoRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcNetworkInfoRequest> for () {
    fn from(value: RpcNetworkInfoRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcNetworkInfoRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`RpcNetworkInfoResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "active_peers",
///    "known_producers",
///    "num_active_peers",
///    "peer_max_count",
///    "received_bytes_per_sec",
///    "sent_bytes_per_sec"
///  ],
///  "properties": {
///    "active_peers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/RpcPeerInfo"
///      }
///    },
///    "known_producers": {
///      "description": "Accounts of known block and chunk producers from
/// routing table.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/RpcKnownProducer"
///      }
///    },
///    "num_active_peers": {
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "peer_max_count": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "received_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "sent_bytes_per_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcNetworkInfoResponse {
    pub active_peers: ::std::vec::Vec<RpcPeerInfo>,
    ///Accounts of known block and chunk producers from routing table.
    pub known_producers: ::std::vec::Vec<RpcKnownProducer>,
    pub num_active_peers: u32,
    pub peer_max_count: u32,
    pub received_bytes_per_sec: u64,
    pub sent_bytes_per_sec: u64,
}
///`RpcPeerInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id"
///  ],
///  "properties": {
///    "account_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "addr": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "id": {
///      "$ref": "#/components/schemas/PeerId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcPeerInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub addr: ::std::option::Option<::std::string::String>,
    pub id: PeerId,
}
///`RpcProtocolConfigError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcProtocolConfigError {
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcProtocolConfigError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///`RpcProtocolConfigRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcProtocolConfigRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/components/schemas/Finality"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/components/schemas/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcProtocolConfigRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcProtocolConfigRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcProtocolConfigRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcProtocolConfigRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcProtocolConfigResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "avg_hidden_validator_seats_per_shard": {
///      "description": "Expected number of hidden validators per shard.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "block_producer_kickout_threshold": {
///      "description": "Threshold for kicking out block producers, between
/// 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chain_id": {
///      "description": "ID of the blockchain. This must be unique for every
/// blockchain.\nIf your testnet blockchains do not have unique chain IDs,
/// you will have a bad time.",
///      "type": "string"
///    },
///    "chunk_producer_kickout_threshold": {
///      "description": "Threshold for kicking out chunk producers, between
/// 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "chunk_validator_only_kickout_threshold": {
///      "description": "Threshold for kicking out nodes which are only
/// chunk validators, between 0 and 100.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "dynamic_resharding": {
///      "description": "Enable dynamic re-sharding.",
///      "type": "boolean"
///    },
///    "epoch_length": {
///      "description": "Epoch length counted in block heights.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fishermen_threshold": {
///      "description": "Fishermen stake threshold.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "gas_limit": {
///      "description": "Initial gas limit.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearGas"
///        }
///      ]
///    },
///    "gas_price_adjustment_rate": {
///      "description": "Gas price adjustment rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "genesis_height": {
///      "description": "Height of genesis block.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "genesis_time": {
///      "description": "Official time of blockchain start.",
///      "type": "string",
///      "format": "date-time"
///    },
///    "max_gas_price": {
///      "description": "Maximum gas price.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "max_inflation_rate": {
///      "description": "Maximum inflation on the total supply every
/// epoch.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "max_kickout_stake_perc": {
///      "description": "Max stake percentage of the validators we will kick
/// out.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "min_gas_price": {
///      "description": "Minimum gas price. It is also the initial gas
/// price.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "minimum_stake_divisor": {
///      "description": "The minimum stake required for staking is last seat
/// price divided by this number.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "minimum_stake_ratio": {
///      "description": "The lowest ratio s/s_total any block producer can have.\nSee <https://github.com/near/NEPs/pull/167> for details",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "minimum_validators_per_shard": {
///      "description": "The minimum number of validators each shard must
/// have",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats": {
///      "description": "Number of block producer seats at genesis.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_block_producer_seats_per_shard": {
///      "description": "Defines number of shards and number of block
/// producer seats per each shard at genesis.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "num_blocks_per_year": {
///      "description": "Expected number of blocks per year",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "online_max_threshold": {
///      "description": "Online maximum threshold above which validator gets
/// full reward.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "online_min_threshold": {
///      "description": "Online minimum threshold below which validator
/// doesn't receive reward.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_reward_rate": {
///      "description": "Protocol treasury rate",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_treasury_account": {
///      "description": "Protocol treasury account",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      ]
///    },
///    "protocol_upgrade_stake_threshold": {
///      "description": "Threshold of stake that needs to indicate that they
/// ready for upgrade.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "protocol_version": {
///      "description": "Current Protocol Version",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "runtime_config": {
///      "description": "Runtime configuration (mostly economics
/// constants).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/RuntimeConfigView"
///        }
///      ]
///    },
///    "shard_layout": {
///      "description": "Layout information regarding how to split accounts
/// to shards",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ShardLayout"
///        }
///      ]
///    },
///    "shuffle_shard_assignment_for_chunk_producers": {
///      "description": "If true, shuffle the chunk producers across shards.
/// In other words, if\nthe shard assignments were `[S_0, S_1, S_2, S_3]`
/// where `S_i` represents\nthe set of chunk producers for shard `i`, if
/// this flag were true, the\nshard assignments might become, for example,
/// `[S_2, S_0, S_3, S_1]`.",
///      "type": "boolean"
///    },
///    "target_validator_mandates_per_shard": {
///      "description": "Number of target chunk validator mandates for each
/// shard.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "transaction_validity_period": {
///      "description": "Number of blocks for which a given transaction is
/// valid",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcProtocolConfigResponse {
    ///Expected number of hidden validators per shard.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub avg_hidden_validator_seats_per_shard: ::std::vec::Vec<u64>,
    ///Threshold for kicking out block producers, between 0 and 100.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_producer_kickout_threshold: ::std::option::Option<u8>,
    ///ID of the blockchain. This must be unique for every blockchain.
    ///If your testnet blockchains do not have unique chain IDs, you will
    /// have a bad time.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chain_id: ::std::option::Option<::std::string::String>,
    ///Threshold for kicking out chunk producers, between 0 and 100.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_producer_kickout_threshold: ::std::option::Option<u8>,
    ///Threshold for kicking out nodes which are only chunk validators,
    /// between 0 and 100.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub chunk_validator_only_kickout_threshold: ::std::option::Option<u8>,
    ///Enable dynamic re-sharding.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dynamic_resharding: ::std::option::Option<bool>,
    ///Epoch length counted in block heights.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_length: ::std::option::Option<u64>,
    ///Fishermen stake threshold.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fishermen_threshold: ::std::option::Option<NearToken>,
    ///Initial gas limit.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gas_limit: ::std::option::Option<NearGas>,
    ///Gas price adjustment rate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub gas_price_adjustment_rate: ::std::option::Option<[i32; 2usize]>,
    ///Height of genesis block.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub genesis_height: ::std::option::Option<u64>,
    ///Official time of blockchain start.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub genesis_time: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    ///Maximum gas price.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_gas_price: ::std::option::Option<NearToken>,
    ///Maximum inflation on the total supply every epoch.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_inflation_rate: ::std::option::Option<[i32; 2usize]>,
    ///Max stake percentage of the validators we will kick out.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_kickout_stake_perc: ::std::option::Option<u8>,
    ///Minimum gas price. It is also the initial gas price.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_gas_price: ::std::option::Option<NearToken>,
    ///The minimum stake required for staking is last seat price divided by
    /// this number.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum_stake_divisor: ::std::option::Option<u64>,
    ///The lowest ratio s/s_total any block producer can have.
    ///See <https://github.com/near/NEPs/pull/167> for details
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum_stake_ratio: ::std::option::Option<[i32; 2usize]>,
    ///The minimum number of validators each shard must have
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum_validators_per_shard: ::std::option::Option<u64>,
    ///Number of block producer seats at genesis.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub num_block_producer_seats: ::std::option::Option<u64>,
    ///Defines number of shards and number of block producer seats per each
    /// shard at genesis.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub num_block_producer_seats_per_shard: ::std::vec::Vec<u64>,
    ///Expected number of blocks per year
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub num_blocks_per_year: ::std::option::Option<u64>,
    ///Online maximum threshold above which validator gets full reward.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub online_max_threshold: ::std::option::Option<[i32; 2usize]>,
    ///Online minimum threshold below which validator doesn't receive
    /// reward.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub online_min_threshold: ::std::option::Option<[i32; 2usize]>,
    ///Protocol treasury rate
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol_reward_rate: ::std::option::Option<[i32; 2usize]>,
    ///Protocol treasury account
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol_treasury_account: ::std::option::Option<AccountId>,
    ///Threshold of stake that needs to indicate that they ready for
    /// upgrade.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol_upgrade_stake_threshold: ::std::option::Option<[i32; 2usize]>,
    ///Current Protocol Version
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub protocol_version: ::std::option::Option<u32>,
    ///Runtime configuration (mostly economics constants).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub runtime_config: ::std::option::Option<RuntimeConfigView>,
    ///Layout information regarding how to split accounts to shards
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shard_layout: ::std::option::Option<ShardLayout>,
    ///If true, shuffle the chunk producers across shards. In other words,
    /// if the shard assignments were `[S_0, S_1, S_2, S_3]` where
    /// `S_i` represents the set of chunk producers for shard `i`,
    /// if this flag were true, the shard assignments might become,
    /// for example, `[S_2, S_0, S_3, S_1]`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shuffle_shard_assignment_for_chunk_producers: ::std::option::Option<bool>,
    ///Number of target chunk validator mandates for each shard.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub target_validator_mandates_per_shard: ::std::option::Option<u64>,
    ///Number of blocks for which a given transaction is valid
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_validity_period: ::std::option::Option<u64>,
}
impl ::std::default::Default for RpcProtocolConfigResponse {
    fn default() -> Self {
        Self {
            avg_hidden_validator_seats_per_shard: Default::default(),
            block_producer_kickout_threshold: Default::default(),
            chain_id: Default::default(),
            chunk_producer_kickout_threshold: Default::default(),
            chunk_validator_only_kickout_threshold: Default::default(),
            dynamic_resharding: Default::default(),
            epoch_length: Default::default(),
            fishermen_threshold: Default::default(),
            gas_limit: Default::default(),
            gas_price_adjustment_rate: Default::default(),
            genesis_height: Default::default(),
            genesis_time: Default::default(),
            max_gas_price: Default::default(),
            max_inflation_rate: Default::default(),
            max_kickout_stake_perc: Default::default(),
            min_gas_price: Default::default(),
            minimum_stake_divisor: Default::default(),
            minimum_stake_ratio: Default::default(),
            minimum_validators_per_shard: Default::default(),
            num_block_producer_seats: Default::default(),
            num_block_producer_seats_per_shard: Default::default(),
            num_blocks_per_year: Default::default(),
            online_max_threshold: Default::default(),
            online_min_threshold: Default::default(),
            protocol_reward_rate: Default::default(),
            protocol_treasury_account: Default::default(),
            protocol_upgrade_stake_threshold: Default::default(),
            protocol_version: Default::default(),
            runtime_config: Default::default(),
            shard_layout: Default::default(),
            shuffle_shard_assignment_for_chunk_producers: Default::default(),
            target_validator_mandates_per_shard: Default::default(),
            transaction_validity_period: Default::default(),
        }
    }
}
///`RpcQueryError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "NO_SYNCED_BLOCKS"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "requested_shard_id"
///          ],
///          "properties": {
///            "requested_shard_id": {
///              "$ref": "#/components/schemas/ShardId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNAVAILABLE_SHARD"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "GARBAGE_COLLECTED_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_reference"
///          ],
///          "properties": {
///            "block_reference": {
///              "$ref": "#/components/schemas/BlockReference"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "requested_account_id"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "requested_account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INVALID_ACCOUNT"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "requested_account_id"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "requested_account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_ACCOUNT"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "contract_account_id"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "contract_account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "NO_CONTRACT_CODE"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "contract_account_id"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "contract_account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "TOO_LARGE_CONTRACT_STATE"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "public_key"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_ACCESS_KEY"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "public_key"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_GAS_KEY"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "vm_error"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "vm_error": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "CONTRACT_EXECUTION_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height",
///            "identifier"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "identifier": {
///              "$ref": "#/components/schemas/GlobalContractIdentifier"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "NO_GLOBAL_CONTRACT_CODE"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcQueryError {
    #[serde(rename = "NO_SYNCED_BLOCKS")]
    NoSyncedBlocks,
    #[serde(rename = "UNAVAILABLE_SHARD")]
    UnavailableShard { requested_shard_id: ShardId },
    #[serde(rename = "GARBAGE_COLLECTED_BLOCK")]
    GarbageCollectedBlock {
        block_hash: CryptoHash,
        block_height: u64,
    },
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock { block_reference: BlockReference },
    #[serde(rename = "INVALID_ACCOUNT")]
    InvalidAccount {
        block_hash: CryptoHash,
        block_height: u64,
        requested_account_id: AccountId,
    },
    #[serde(rename = "UNKNOWN_ACCOUNT")]
    UnknownAccount {
        block_hash: CryptoHash,
        block_height: u64,
        requested_account_id: AccountId,
    },
    #[serde(rename = "NO_CONTRACT_CODE")]
    NoContractCode {
        block_hash: CryptoHash,
        block_height: u64,
        contract_account_id: AccountId,
    },
    #[serde(rename = "TOO_LARGE_CONTRACT_STATE")]
    TooLargeContractState {
        block_hash: CryptoHash,
        block_height: u64,
        contract_account_id: AccountId,
    },
    #[serde(rename = "UNKNOWN_ACCESS_KEY")]
    UnknownAccessKey {
        block_hash: CryptoHash,
        block_height: u64,
        public_key: PublicKey,
    },
    #[serde(rename = "UNKNOWN_GAS_KEY")]
    UnknownGasKey {
        block_hash: CryptoHash,
        block_height: u64,
        public_key: PublicKey,
    },
    #[serde(rename = "CONTRACT_EXECUTION_ERROR")]
    ContractExecutionError {
        block_hash: CryptoHash,
        block_height: u64,
        vm_error: ::std::string::String,
    },
    #[serde(rename = "NO_GLOBAL_CONTRACT_CODE")]
    NoGlobalContractCode {
        block_hash: CryptoHash,
        block_height: u64,
        identifier: GlobalContractIdentifier,
    },
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcQueryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcQueryRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "view_account_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_account"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_code_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_state_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_state"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_list_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_list_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "call_function_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/components/schemas/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "call_function"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_global_contract_code_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_global_contract_code_by_account_id_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code_by_account_id"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_account_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_account"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_code_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_state_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_state"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_list_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_list_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "call_function_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/components/schemas/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "call_function"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_global_contract_code_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_global_contract_code_by_account_id_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code_by_account_id"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_account_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_account"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_code_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_state_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "prefix_base64",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "include_proof": {
///              "type": "boolean"
///            },
///            "prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_state"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_access_key_list_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_access_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_gas_key_list_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_gas_key_list"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "call_function_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "args_base64",
///            "method_name",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "args_base64": {
///              "$ref": "#/components/schemas/FunctionArgs"
///            },
///            "method_name": {
///              "type": "string"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "call_function"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "view_global_contract_code_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "code_hash",
///            "request_type"
///          ],
///          "properties": {
///            "code_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title":
/// "view_global_contract_code_by_account_id_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_id",
///            "request_type"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "request_type": {
///              "type": "string",
///              "enum": [
///                "view_global_contract_code_by_account_id"
///              ]
///            }
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcQueryRequest {
    ViewAccountByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ViewAccountByBlockIdRequestType,
    },
    ViewCodeByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ViewCodeByBlockIdRequestType,
    },
    ViewStateByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ViewStateByBlockIdRequestType,
    },
    ViewAccessKeyByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        public_key: PublicKey,
        request_type: ViewAccessKeyByBlockIdRequestType,
    },
    ViewAccessKeyListByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ViewAccessKeyListByBlockIdRequestType,
    },
    ViewGasKeyByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        public_key: PublicKey,
        request_type: ViewGasKeyByBlockIdRequestType,
    },
    ViewGasKeyListByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ViewGasKeyListByBlockIdRequestType,
    },
    CallFunctionByBlockId {
        account_id: AccountId,
        args_base64: FunctionArgs,
        block_id: BlockId,
        method_name: ::std::string::String,
        request_type: CallFunctionByBlockIdRequestType,
    },
    ViewGlobalContractCodeByBlockId {
        block_id: BlockId,
        code_hash: CryptoHash,
        request_type: ViewGlobalContractCodeByBlockIdRequestType,
    },
    ViewGlobalContractCodeByAccountIdByBlockId {
        account_id: AccountId,
        block_id: BlockId,
        request_type: ViewGlobalContractCodeByAccountIdByBlockIdRequestType,
    },
    ViewAccountByFinality {
        account_id: AccountId,
        finality: Finality,
        request_type: ViewAccountByFinalityRequestType,
    },
    ViewCodeByFinality {
        account_id: AccountId,
        finality: Finality,
        request_type: ViewCodeByFinalityRequestType,
    },
    ViewStateByFinality {
        account_id: AccountId,
        finality: Finality,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ViewStateByFinalityRequestType,
    },
    ViewAccessKeyByFinality {
        account_id: AccountId,
        finality: Finality,
        public_key: PublicKey,
        request_type: ViewAccessKeyByFinalityRequestType,
    },
    ViewAccessKeyListByFinality {
        account_id: AccountId,
        finality: Finality,
        request_type: ViewAccessKeyListByFinalityRequestType,
    },
    ViewGasKeyByFinality {
        account_id: AccountId,
        finality: Finality,
        public_key: PublicKey,
        request_type: ViewGasKeyByFinalityRequestType,
    },
    ViewGasKeyListByFinality {
        account_id: AccountId,
        finality: Finality,
        request_type: ViewGasKeyListByFinalityRequestType,
    },
    CallFunctionByFinality {
        account_id: AccountId,
        args_base64: FunctionArgs,
        finality: Finality,
        method_name: ::std::string::String,
        request_type: CallFunctionByFinalityRequestType,
    },
    ViewGlobalContractCodeByFinality {
        code_hash: CryptoHash,
        finality: Finality,
        request_type: ViewGlobalContractCodeByFinalityRequestType,
    },
    ViewGlobalContractCodeByAccountIdByFinality {
        account_id: AccountId,
        finality: Finality,
        request_type: ViewGlobalContractCodeByAccountIdByFinalityRequestType,
    },
    ViewAccountBySyncCheckpoint {
        account_id: AccountId,
        request_type: ViewAccountBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewCodeBySyncCheckpoint {
        account_id: AccountId,
        request_type: ViewCodeBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewStateBySyncCheckpoint {
        account_id: AccountId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        include_proof: ::std::option::Option<bool>,
        prefix_base64: StoreKey,
        request_type: ViewStateBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewAccessKeyBySyncCheckpoint {
        account_id: AccountId,
        public_key: PublicKey,
        request_type: ViewAccessKeyBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewAccessKeyListBySyncCheckpoint {
        account_id: AccountId,
        request_type: ViewAccessKeyListBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewGasKeyBySyncCheckpoint {
        account_id: AccountId,
        public_key: PublicKey,
        request_type: ViewGasKeyBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewGasKeyListBySyncCheckpoint {
        account_id: AccountId,
        request_type: ViewGasKeyListBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    CallFunctionBySyncCheckpoint {
        account_id: AccountId,
        args_base64: FunctionArgs,
        method_name: ::std::string::String,
        request_type: CallFunctionBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewGlobalContractCodeBySyncCheckpoint {
        code_hash: CryptoHash,
        request_type: ViewGlobalContractCodeBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
    ViewGlobalContractCodeByAccountIdBySyncCheckpoint {
        account_id: AccountId,
        request_type: ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType,
        sync_checkpoint: SyncCheckpoint,
    },
}
///`RpcQueryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ContractCodeView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ViewStateResult"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CallResult"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKeyView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccessKeyList"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/GasKeyView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/GasKeyList"
///        },
///        {
///          "type": "object",
///          "required": [
///            "block_hash",
///            "block_height"
///          ],
///          "properties": {
///            "block_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "block_height": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponse {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<RpcQueryResponseSubtype0>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<RpcQueryResponseSubtype1>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<RpcQueryResponseSubtype2>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<RpcQueryResponseSubtype3>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_4: ::std::option::Option<RpcQueryResponseSubtype4>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_5: ::std::option::Option<RpcQueryResponseSubtype5>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_6: ::std::option::Option<RpcQueryResponseSubtype6>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_7: ::std::option::Option<RpcQueryResponseSubtype7>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_8: ::std::option::Option<RpcQueryResponseSubtype8>,
}
impl ::std::default::Default for RpcQueryResponse {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
            subtype_4: Default::default(),
            subtype_5: Default::default(),
            subtype_6: Default::default(),
            subtype_7: Default::default(),
            subtype_8: Default::default(),
        }
    }
}
///`RpcQueryResponseSubtype0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_hash",
///    "block_height"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "block_height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RpcQueryResponseSubtype0 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
}
///`RpcQueryResponseSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/AccountView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype1 {
    pub amount: NearToken,
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub code_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_hash: ::std::option::Option<CryptoHash>,
    pub locked: NearToken,
    ///TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: u64,
    pub storage_usage: u64,
}
///`RpcQueryResponseSubtype2`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/ContractCodeView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype2 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub code_base64: ::std::string::String,
    pub hash: CryptoHash,
}
///`RpcQueryResponseSubtype3`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/ViewStateResult"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype3 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub proof: ::std::vec::Vec<::std::string::String>,
    pub values: ::std::vec::Vec<StateItem>,
}
///`RpcQueryResponseSubtype4`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/CallResult"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype4 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub logs: ::std::vec::Vec<::std::string::String>,
    pub result: ::std::vec::Vec<u8>,
}
///`RpcQueryResponseSubtype5`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/AccessKeyView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype5 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub nonce: u64,
    pub permission: AccessKeyPermissionView,
}
///`RpcQueryResponseSubtype6`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/AccessKeyList"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype6 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub keys: ::std::vec::Vec<AccessKeyInfoView>,
}
///`RpcQueryResponseSubtype7`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/GasKeyView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype7 {
    pub balance: NearToken,
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub nonces: ::std::vec::Vec<u64>,
    pub num_nonces: u32,
    pub permission: AccessKeyPermissionView,
}
///`RpcQueryResponseSubtype8`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/GasKeyList"
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_hash",
///        "block_height"
///      ],
///      "properties": {
///        "block_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "block_height": {
///          "type": "integer",
///          "format": "uint64",
///          "minimum": 0.0
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcQueryResponseSubtype8 {
    pub block_hash: CryptoHash,
    pub block_height: u64,
    pub keys: ::std::vec::Vec<GasKeyInfoView>,
}
///`RpcReceiptError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "receipt_id"
///          ],
///          "properties": {
///            "receipt_id": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_RECEIPT"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcReceiptError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
    #[serde(rename = "UNKNOWN_RECEIPT")]
    UnknownReceipt { receipt_id: CryptoHash },
}
///`RpcReceiptRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcReceiptRequest",
///  "type": "object",
///  "required": [
///    "receipt_id"
///  ],
///  "properties": {
///    "receipt_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcReceiptRequest {
    pub receipt_id: CryptoHash,
}
///`RpcReceiptResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "predecessor_id",
///    "receipt",
///    "receipt_id",
///    "receiver_id"
///  ],
///  "properties": {
///    "predecessor_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "priority": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "receipt": {
///      "$ref": "#/components/schemas/ReceiptEnumView"
///    },
///    "receipt_id": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "receiver_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcReceiptResponse {
    pub predecessor_id: AccountId,
    #[serde(default)]
    pub priority: u64,
    pub receipt: ReceiptEnumView,
    pub receipt_id: CryptoHash,
    pub receiver_id: AccountId,
}
///`RpcRequestValidationErrorKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "method_name"
///          ],
///          "properties": {
///            "method_name": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "METHOD_NOT_FOUND"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "PARSE_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
pub enum RpcRequestValidationErrorKind {
    #[serde(rename = "METHOD_NOT_FOUND")]
    MethodNotFound { method_name: ::std::string::String },
    #[serde(rename = "PARSE_ERROR")]
    ParseError {
        error_message: ::std::string::String,
    },
}
///`RpcSendTransactionRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcSendTransactionRequest",
///  "type": "object",
///  "required": [
///    "signed_tx_base64"
///  ],
///  "properties": {
///    "signed_tx_base64": {
///      "$ref": "#/components/schemas/SignedTransaction"
///    },
///    "wait_until": {
///      "default": "EXECUTED_OPTIMISTIC",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/TxExecutionStatus"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcSendTransactionRequest {
    pub signed_tx_base64: SignedTransaction,
    #[serde(default = "defaults::rpc_send_transaction_request_wait_until")]
    pub wait_until: TxExecutionStatus,
}
///`RpcSplitStorageInfoError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcSplitStorageInfoError {
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcSplitStorageInfoRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcSplitStorageInfoRequest",
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcSplitStorageInfoRequest(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for RpcSplitStorageInfoRequest {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<RpcSplitStorageInfoRequest>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: RpcSplitStorageInfoRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcSplitStorageInfoRequest
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
///Contains the split storage information.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Contains the split storage information.",
///  "type": "object",
///  "properties": {
///    "cold_head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "final_head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "head_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "hot_db_kind": {
///      "type": [
///        "string",
///        "null"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcSplitStorageInfoResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cold_head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub final_head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub head_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hot_db_kind: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for RpcSplitStorageInfoResponse {
    fn default() -> Self {
        Self {
            cold_head_height: Default::default(),
            final_head_height: Default::default(),
            head_height: Default::default(),
            hot_db_kind: Default::default(),
        }
    }
}
///`RpcStateChangesError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_BLOCK"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "NOT_SYNCED_YET"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcStateChangesError {
    #[serde(rename = "UNKNOWN_BLOCK")]
    UnknownBlock(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "NOT_SYNCED_YET")]
    NotSyncedYet,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcStateChangesError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::UnknownBlock(value)
    }
}
///It is a [serializable view] of [`StateChangesRequest`].
///
///[serializable view]: ./index.html
///[`StateChangesRequest`]: ../types/struct.StateChangesRequest.html
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockByTypeRequest",
///  "description": "It is a [serializable view] of
/// [`StateChangesRequest`].\n\n[serializable view]:
/// ./index.html\n[`StateChangesRequest`]:
/// ../types/struct.StateChangesRequest.html",
///  "type": "object",
///  "oneOf": [
///    {
///      "title": "account_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "account_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_access_key_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_access_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_gas_key_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_gas_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_access_key_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_access_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_gas_key_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_gas_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "contract_code_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "contract_code_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "data_changes_by_block_id",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "block_id"
///          ],
///          "properties": {
///            "block_id": {
///              "$ref": "#/components/schemas/BlockId"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "data_changes"
///              ]
///            },
///            "key_prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "account_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "account_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_access_key_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_access_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_gas_key_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_gas_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_access_key_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_access_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_gas_key_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_gas_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "contract_code_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "contract_code_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "data_changes_by_finality",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "finality"
///          ],
///          "properties": {
///            "finality": {
///              "$ref": "#/components/schemas/Finality"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "data_changes"
///              ]
///            },
///            "key_prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "account_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "account_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_access_key_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_access_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "single_gas_key_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "changes_type",
///            "keys"
///          ],
///          "properties": {
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "single_gas_key_changes"
///              ]
///            },
///            "keys": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountWithPublicKey"
///              }
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_access_key_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_access_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "all_gas_key_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "all_gas_key_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "contract_code_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "contract_code_changes"
///              ]
///            }
///          }
///        }
///      ]
///    },
///    {
///      "title": "data_changes_by_sync_checkpoint",
///      "allOf": [
///        {
///          "type": "object",
///          "required": [
///            "sync_checkpoint"
///          ],
///          "properties": {
///            "sync_checkpoint": {
///              "$ref": "#/components/schemas/SyncCheckpoint"
///            }
///          }
///        },
///        {
///          "type": "object",
///          "required": [
///            "account_ids",
///            "changes_type",
///            "key_prefix_base64"
///          ],
///          "properties": {
///            "account_ids": {
///              "type": "array",
///              "items": {
///                "$ref": "#/components/schemas/AccountId"
///              }
///            },
///            "changes_type": {
///              "type": "string",
///              "enum": [
///                "data_changes"
///              ]
///            },
///            "key_prefix_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            }
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcStateChangesInBlockByTypeRequest {
    AccountChangesByBlockId {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: AccountChangesByBlockIdChangesType,
    },
    SingleAccessKeyChangesByBlockId {
        block_id: BlockId,
        changes_type: SingleAccessKeyChangesByBlockIdChangesType,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    SingleGasKeyChangesByBlockId {
        block_id: BlockId,
        changes_type: SingleGasKeyChangesByBlockIdChangesType,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    AllAccessKeyChangesByBlockId {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: AllAccessKeyChangesByBlockIdChangesType,
    },
    AllGasKeyChangesByBlockId {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: AllGasKeyChangesByBlockIdChangesType,
    },
    ContractCodeChangesByBlockId {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: ContractCodeChangesByBlockIdChangesType,
    },
    DataChangesByBlockId {
        account_ids: ::std::vec::Vec<AccountId>,
        block_id: BlockId,
        changes_type: DataChangesByBlockIdChangesType,
        key_prefix_base64: StoreKey,
    },
    AccountChangesByFinality {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AccountChangesByFinalityChangesType,
        finality: Finality,
    },
    SingleAccessKeyChangesByFinality {
        changes_type: SingleAccessKeyChangesByFinalityChangesType,
        finality: Finality,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    SingleGasKeyChangesByFinality {
        changes_type: SingleGasKeyChangesByFinalityChangesType,
        finality: Finality,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
    },
    AllAccessKeyChangesByFinality {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AllAccessKeyChangesByFinalityChangesType,
        finality: Finality,
    },
    AllGasKeyChangesByFinality {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AllGasKeyChangesByFinalityChangesType,
        finality: Finality,
    },
    ContractCodeChangesByFinality {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ContractCodeChangesByFinalityChangesType,
        finality: Finality,
    },
    DataChangesByFinality {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: DataChangesByFinalityChangesType,
        finality: Finality,
        key_prefix_base64: StoreKey,
    },
    AccountChangesBySyncCheckpoint {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AccountChangesBySyncCheckpointChangesType,
        sync_checkpoint: SyncCheckpoint,
    },
    SingleAccessKeyChangesBySyncCheckpoint {
        changes_type: SingleAccessKeyChangesBySyncCheckpointChangesType,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
        sync_checkpoint: SyncCheckpoint,
    },
    SingleGasKeyChangesBySyncCheckpoint {
        changes_type: SingleGasKeyChangesBySyncCheckpointChangesType,
        keys: ::std::vec::Vec<AccountWithPublicKey>,
        sync_checkpoint: SyncCheckpoint,
    },
    AllAccessKeyChangesBySyncCheckpoint {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AllAccessKeyChangesBySyncCheckpointChangesType,
        sync_checkpoint: SyncCheckpoint,
    },
    AllGasKeyChangesBySyncCheckpoint {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: AllGasKeyChangesBySyncCheckpointChangesType,
        sync_checkpoint: SyncCheckpoint,
    },
    ContractCodeChangesBySyncCheckpoint {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: ContractCodeChangesBySyncCheckpointChangesType,
        sync_checkpoint: SyncCheckpoint,
    },
    DataChangesBySyncCheckpoint {
        account_ids: ::std::vec::Vec<AccountId>,
        changes_type: DataChangesBySyncCheckpointChangesType,
        key_prefix_base64: StoreKey,
        sync_checkpoint: SyncCheckpoint,
    },
}
///`RpcStateChangesInBlockByTypeResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_hash",
///    "changes"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "changes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/StateChangeKindView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStateChangesInBlockByTypeResponse {
    pub block_hash: CryptoHash,
    pub changes: ::std::vec::Vec<StateChangeKindView>,
}
///`RpcStateChangesInBlockRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStateChangesInBlockRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "finality"
///      ],
///      "properties": {
///        "finality": {
///          "$ref": "#/components/schemas/Finality"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "sync_checkpoint"
///      ],
///      "properties": {
///        "sync_checkpoint": {
///          "$ref": "#/components/schemas/SyncCheckpoint"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcStateChangesInBlockRequest {
    #[serde(rename = "block_id")]
    BlockId(BlockId),
    #[serde(rename = "finality")]
    Finality(Finality),
    #[serde(rename = "sync_checkpoint")]
    SyncCheckpoint(SyncCheckpoint),
}
impl ::std::convert::From<BlockId> for RpcStateChangesInBlockRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
impl ::std::convert::From<Finality> for RpcStateChangesInBlockRequest {
    fn from(value: Finality) -> Self {
        Self::Finality(value)
    }
}
impl ::std::convert::From<SyncCheckpoint> for RpcStateChangesInBlockRequest {
    fn from(value: SyncCheckpoint) -> Self {
        Self::SyncCheckpoint(value)
    }
}
///`RpcStateChangesInBlockResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "block_hash",
///    "changes"
///  ],
///  "properties": {
///    "block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "changes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/StateChangeWithCauseView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStateChangesInBlockResponse {
    pub block_hash: CryptoHash,
    pub changes: ::std::vec::Vec<StateChangeWithCauseView>,
}
///`RpcStatusError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "NODE_IS_SYNCING"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "elapsed"
///          ],
///          "properties": {
///            "elapsed": {
///              "type": "array",
///              "items": {
///                "type": "integer",
///                "format": "uint64",
///                "minimum": 0.0
///              },
///              "maxItems": 2,
///              "minItems": 2
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "NO_NEW_BLOCKS"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "epoch_id"
///          ],
///          "properties": {
///            "epoch_id": {
///              "$ref": "#/components/schemas/EpochId"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "EPOCH_OUT_OF_BOUNDS"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcStatusError {
    #[serde(rename = "NODE_IS_SYNCING")]
    NodeIsSyncing,
    #[serde(rename = "NO_NEW_BLOCKS")]
    NoNewBlocks { elapsed: [u64; 2usize] },
    #[serde(rename = "EPOCH_OUT_OF_BOUNDS")]
    EpochOutOfBounds { epoch_id: EpochId },
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcStatusRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcStatusRequest",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct RpcStatusRequest(pub ());
impl ::std::ops::Deref for RpcStatusRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<RpcStatusRequest> for () {
    fn from(value: RpcStatusRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for RpcStatusRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`RpcStatusResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "chain_id",
///    "genesis_hash",
///    "latest_protocol_version",
///    "node_public_key",
///    "protocol_version",
///    "sync_info",
///    "uptime_sec",
///    "validators",
///    "version"
///  ],
///  "properties": {
///    "chain_id": {
///      "description": "Unique chain id.",
///      "type": "string"
///    },
///    "detailed_debug_status": {
///      "description": "Information about last blocks, network, epoch and
/// chain & chunk info.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/DetailedDebugStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "genesis_hash": {
///      "description": "Genesis hash of the chain.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      ]
///    },
///    "latest_protocol_version": {
///      "description": "Latest protocol version that this client
/// supports.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "node_key": {
///      "description": "Deprecated; same as `validator_public_key` which
/// you should use instead.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "node_public_key": {
///      "description": "Public key of the node.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    },
///    "protocol_version": {
///      "description": "Currently active protocol version.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "rpc_addr": {
///      "description": "Address for RPC server.  None if node doesn't have
/// RPC endpoint enabled.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "sync_info": {
///      "description": "Sync status of the node.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/StatusSyncInfo"
///        }
///      ]
///    },
///    "uptime_sec": {
///      "description": "Uptime of the node.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "validator_account_id": {
///      "description": "Validator id of the node",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "validator_public_key": {
///      "description": "Public key of the validator.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "validators": {
///      "description": "Current epoch validators.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorInfo"
///      }
///    },
///    "version": {
///      "description": "Binary version.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Version"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcStatusResponse {
    ///Unique chain id.
    pub chain_id: ::std::string::String,
    ///Information about last blocks, network, epoch and chain & chunk
    /// info.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub detailed_debug_status: ::std::option::Option<DetailedDebugStatus>,
    ///Genesis hash of the chain.
    pub genesis_hash: CryptoHash,
    ///Latest protocol version that this client supports.
    pub latest_protocol_version: u32,
    ///Deprecated; same as `validator_public_key` which you should use
    /// instead.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub node_key: ::std::option::Option<PublicKey>,
    ///Public key of the node.
    pub node_public_key: PublicKey,
    ///Currently active protocol version.
    pub protocol_version: u32,
    ///Address for RPC server.  None if node doesn't have RPC endpoint
    /// enabled.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rpc_addr: ::std::option::Option<::std::string::String>,
    ///Sync status of the node.
    pub sync_info: StatusSyncInfo,
    ///Uptime of the node.
    pub uptime_sec: i64,
    ///Validator id of the node
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_account_id: ::std::option::Option<AccountId>,
    ///Public key of the validator.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub validator_public_key: ::std::option::Option<PublicKey>,
    ///Current epoch validators.
    pub validators: ::std::vec::Vec<ValidatorInfo>,
    ///Binary version.
    pub version: Version,
}
///`RpcTransactionError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object"
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INVALID_TRANSACTION"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "DOES_NOT_TRACK_SHARD"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "transaction_hash"
///          ],
///          "properties": {
///            "transaction_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "REQUEST_ROUTED"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "requested_transaction_hash"
///          ],
///          "properties": {
///            "requested_transaction_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_TRANSACTION"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "debug_info"
///          ],
///          "properties": {
///            "debug_info": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "TIMEOUT_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcTransactionError {
    #[serde(rename = "INVALID_TRANSACTION")]
    InvalidTransaction(::serde_json::Map<::std::string::String, ::serde_json::Value>),
    #[serde(rename = "DOES_NOT_TRACK_SHARD")]
    DoesNotTrackShard,
    #[serde(rename = "REQUEST_ROUTED")]
    RequestRouted { transaction_hash: CryptoHash },
    #[serde(rename = "UNKNOWN_TRANSACTION")]
    UnknownTransaction {
        requested_transaction_hash: CryptoHash,
    },
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError { debug_info: ::std::string::String },
    #[serde(rename = "TIMEOUT_ERROR")]
    TimeoutError,
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for RpcTransactionError
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self::InvalidTransaction(value)
    }
}
///`RpcTransactionResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "object",
///      "required": [
///        "final_execution_status"
///      ],
///      "properties": {
///        "final_execution_status": {
///          "$ref": "#/components/schemas/TxExecutionStatus"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "allOf": [
///        {
///          "$ref":
/// "#/components/schemas/FinalExecutionOutcomeWithReceiptView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "final_execution_status"
///          ],
///          "properties": {
///            "final_execution_status": {
///              "$ref": "#/components/schemas/TxExecutionStatus"
///            }
///          }
///        }
///      ]
///    },
///    {
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/FinalExecutionOutcomeView"
///        },
///        {
///          "type": "object",
///          "required": [
///            "final_execution_status"
///          ],
///          "properties": {
///            "final_execution_status": {
///              "$ref": "#/components/schemas/TxExecutionStatus"
///            }
///          }
///        }
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcTransactionResponse {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<RpcTransactionResponseSubtype0>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<RpcTransactionResponseSubtype1>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<RpcTransactionResponseSubtype2>,
}
impl ::std::default::Default for RpcTransactionResponse {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
        }
    }
}
///`RpcTransactionResponseSubtype0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "final_execution_status"
///  ],
///  "properties": {
///    "final_execution_status": {
///      "$ref": "#/components/schemas/TxExecutionStatus"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RpcTransactionResponseSubtype0 {
    pub final_execution_status: TxExecutionStatus,
}
///`RpcTransactionResponseSubtype1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/FinalExecutionOutcomeWithReceiptView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "final_execution_status"
///      ],
///      "properties": {
///        "final_execution_status": {
///          "$ref": "#/components/schemas/TxExecutionStatus"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcTransactionResponseSubtype1 {
    pub final_execution_status: TxExecutionStatus,
    ///Receipts generated from the transaction
    pub receipts: ::std::vec::Vec<ReceiptView>,
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    ///Execution status defined by chain.rs:get_final_transaction_result
    ///FinalExecutionStatus::NotStarted - the tx is not converted to the
    /// receipt yet FinalExecutionStatus::Started - we have at least
    /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished
    /// the execution FinalExecutionStatus::Failure - the result of
    /// the first leaf receipt_id FinalExecutionStatus::SuccessValue
    /// - the result of the first leaf receipt_id
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
///`RpcTransactionResponseSubtype2`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/FinalExecutionOutcomeView"
///    },
///    {
///      "type": "object",
///      "required": [
///        "final_execution_status"
///      ],
///      "properties": {
///        "final_execution_status": {
///          "$ref": "#/components/schemas/TxExecutionStatus"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcTransactionResponseSubtype2 {
    pub final_execution_status: TxExecutionStatus,
    ///The execution outcome of receipts.
    pub receipts_outcome: ::std::vec::Vec<ExecutionOutcomeWithIdView>,
    ///Execution status defined by chain.rs:get_final_transaction_result
    ///FinalExecutionStatus::NotStarted - the tx is not converted to the
    /// receipt yet FinalExecutionStatus::Started - we have at least
    /// 1 receipt, but the first leaf receipt_id (using dfs) hasn't finished
    /// the execution FinalExecutionStatus::Failure - the result of
    /// the first leaf receipt_id FinalExecutionStatus::SuccessValue
    /// - the result of the first leaf receipt_id
    pub status: FinalExecutionStatus,
    ///Signed Transaction
    pub transaction: SignedTransactionView,
    ///The execution outcome of the signed transaction.
    pub transaction_outcome: ExecutionOutcomeWithIdView,
}
///`RpcTransactionStatusRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcTransactionStatusRequest",
///  "type": "object",
///  "anyOf": [
///    {
///      "type": "object",
///      "required": [
///        "signed_tx_base64"
///      ],
///      "properties": {
///        "signed_tx_base64": {
///          "$ref": "#/components/schemas/SignedTransaction"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "sender_account_id",
///        "tx_hash"
///      ],
///      "properties": {
///        "sender_account_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "tx_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        }
///      }
///    }
///  ],
///  "properties": {
///    "wait_until": {
///      "default": "EXECUTED_OPTIMISTIC",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/TxExecutionStatus"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum RpcTransactionStatusRequest {
    Variant0 {
        signed_tx_base64: SignedTransaction,
        #[serde(default = "defaults::rpc_transaction_status_request_variant0_wait_until")]
        wait_until: TxExecutionStatus,
    },
    Variant1 {
        sender_account_id: AccountId,
        tx_hash: CryptoHash,
        #[serde(default = "defaults::rpc_transaction_status_request_variant1_wait_until")]
        wait_until: TxExecutionStatus,
    },
}
///`RpcValidatorError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "UNKNOWN_EPOCH"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "name"
///      ],
///      "properties": {
///        "name": {
///          "type": "string",
///          "enum": [
///            "VALIDATOR_INFO_UNAVAILABLE"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "info",
///        "name"
///      ],
///      "properties": {
///        "info": {
///          "type": "object",
///          "required": [
///            "error_message"
///          ],
///          "properties": {
///            "error_message": {
///              "type": "string"
///            }
///          }
///        },
///        "name": {
///          "type": "string",
///          "enum": [
///            "INTERNAL_ERROR"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "name", content = "info")]
#[derive(strum_macros::Display, thiserror::Error)]
pub enum RpcValidatorError {
    #[serde(rename = "UNKNOWN_EPOCH")]
    UnknownEpoch,
    #[serde(rename = "VALIDATOR_INFO_UNAVAILABLE")]
    ValidatorInfoUnavailable,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError {
        error_message: ::std::string::String,
    },
}
///`RpcValidatorRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcValidatorRequest",
///  "type": "object",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "latest"
///      ]
///    },
///    {
///      "type": "object",
///      "required": [
///        "epoch_id"
///      ],
///      "properties": {
///        "epoch_id": {
///          "$ref": "#/components/schemas/EpochId"
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "block_id"
///      ],
///      "properties": {
///        "block_id": {
///          "$ref": "#/components/schemas/BlockId"
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum RpcValidatorRequest {
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "epoch_id")]
    EpochId(EpochId),
    #[serde(rename = "block_id")]
    BlockId(BlockId),
}
impl ::std::convert::From<EpochId> for RpcValidatorRequest {
    fn from(value: EpochId) -> Self {
        Self::EpochId(value)
    }
}
impl ::std::convert::From<BlockId> for RpcValidatorRequest {
    fn from(value: BlockId) -> Self {
        Self::BlockId(value)
    }
}
///Information about this epoch validators and next epoch validators
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Information about this epoch validators and next epoch
/// validators",
///  "type": "object",
///  "required": [
///    "current_fishermen",
///    "current_proposals",
///    "current_validators",
///    "epoch_height",
///    "epoch_start_height",
///    "next_fishermen",
///    "next_validators",
///    "prev_epoch_kickout"
///  ],
///  "properties": {
///    "current_fishermen": {
///      "description": "Fishermen for the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "current_proposals": {
///      "description": "Proposals in the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "current_validators": {
///      "description": "Validators for the current epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/CurrentEpochValidatorInfo"
///      }
///    },
///    "epoch_height": {
///      "description": "Epoch height",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "epoch_start_height": {
///      "description": "Epoch start block height",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "next_fishermen": {
///      "description": "Fishermen for the next epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorStakeView"
///      }
///    },
///    "next_validators": {
///      "description": "Validators for the next epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/NextEpochValidatorInfo"
///      }
///    },
///    "prev_epoch_kickout": {
///      "description": "Kickout in the previous epoch",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ValidatorKickoutView"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcValidatorResponse {
    ///Fishermen for the current epoch
    pub current_fishermen: ::std::vec::Vec<ValidatorStakeView>,
    ///Proposals in the current epoch
    pub current_proposals: ::std::vec::Vec<ValidatorStakeView>,
    ///Validators for the current epoch
    pub current_validators: ::std::vec::Vec<CurrentEpochValidatorInfo>,
    ///Epoch height
    pub epoch_height: u64,
    ///Epoch start block height
    pub epoch_start_height: u64,
    ///Fishermen for the next epoch
    pub next_fishermen: ::std::vec::Vec<ValidatorStakeView>,
    ///Validators for the next epoch
    pub next_validators: ::std::vec::Vec<NextEpochValidatorInfo>,
    ///Kickout in the previous epoch
    pub prev_epoch_kickout: ::std::vec::Vec<ValidatorKickoutView>,
}
///`RpcValidatorsOrderedRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "title": "RpcValidatorsOrderedRequest",
///  "type": "object",
///  "properties": {
///    "block_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/BlockId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcValidatorsOrderedRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub block_id: ::std::option::Option<BlockId>,
}
impl ::std::default::Default for RpcValidatorsOrderedRequest {
    fn default() -> Self {
        Self {
            block_id: Default::default(),
        }
    }
}
///View that preserves JSON format of the runtime config.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "View that preserves JSON format of the runtime
/// config.",
///  "type": "object",
///  "properties": {
///    "account_creation_config": {
///      "description": "Config that defines rules for account creation.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/AccountCreationConfigView"
///        }
///      ]
///    },
///    "congestion_control_config": {
///      "description": "The configuration for congestion control.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/CongestionControlConfigView"
///        }
///      ]
///    },
///    "dynamic_resharding_config": {
///      "description": "Configuration for dynamic resharding feature.",
///      "default": {
///        "max_number_of_shards": 999999999999999,
///        "memory_usage_threshold": 999999999999999,
///        "min_child_memory_usage": 999999999999999,
///        "min_epochs_between_resharding": 999999999999999
///      },
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/DynamicReshardingConfigView"
///        }
///      ]
///    },
///    "storage_amount_per_byte": {
///      "description": "Amount of yN per byte required to have on the account.  See\n<https://nomicon.io/Economics/Economics.html#state-stake> for details.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    },
///    "transaction_costs": {
///      "description": "Costs of different actions that need to be
/// performed when sending and\nprocessing transaction and receipts.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/RuntimeFeesConfigView"
///        }
///      ]
///    },
///    "wasm_config": {
///      "description": "Config of wasm operations.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/VMConfigView"
///        }
///      ]
///    },
///    "witness_config": {
///      "description": "Configuration specific to ChunkStateWitness.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/WitnessConfigView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RuntimeConfigView {
    ///Config that defines rules for account creation.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub account_creation_config: ::std::option::Option<AccountCreationConfigView>,
    ///The configuration for congestion control.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub congestion_control_config: ::std::option::Option<CongestionControlConfigView>,
    ///Configuration for dynamic resharding feature.
    #[serde(default = "defaults::runtime_config_view_dynamic_resharding_config")]
    pub dynamic_resharding_config: DynamicReshardingConfigView,
    ///Amount of yN per byte required to have on the account.  See
    ///<https://nomicon.io/Economics/Economics.html#state-stake> for details.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_amount_per_byte: ::std::option::Option<NearToken>,
    ///Costs of different actions that need to be performed when sending
    /// and processing transaction and receipts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_costs: ::std::option::Option<RuntimeFeesConfigView>,
    ///Config of wasm operations.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wasm_config: ::std::option::Option<VmConfigView>,
    ///Configuration specific to ChunkStateWitness.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub witness_config: ::std::option::Option<WitnessConfigView>,
}
impl ::std::default::Default for RuntimeConfigView {
    fn default() -> Self {
        Self {
            account_creation_config: Default::default(),
            congestion_control_config: Default::default(),
            dynamic_resharding_config: defaults::runtime_config_view_dynamic_resharding_config(),
            storage_amount_per_byte: Default::default(),
            transaction_costs: Default::default(),
            wasm_config: Default::default(),
            witness_config: Default::default(),
        }
    }
}
///Describes different fees for the runtime
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes different fees for the runtime",
///  "type": "object",
///  "properties": {
///    "action_creation_config": {
///      "description": "Describes the cost of creating a certain action,
/// `Action`. Includes all variants.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ActionCreationConfigView"
///        }
///      ]
///    },
///    "action_receipt_creation_config": {
///      "description": "Describes the cost of creating an action receipt,
/// `ActionReceipt`, excluding the actual cost\nof actions.\n- `send` cost
/// is burned when a receipt is created using `promise_create` or\n
/// `promise_batch_create`\n- `exec` cost is burned when the receipt is
/// being executed.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/Fee"
///        }
///      ]
///    },
///    "burnt_gas_reward": {
///      "description": "Fraction of the burnt gas to reward to the contract
/// account for execution.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "data_receipt_creation_config": {
///      "description": "Describes the cost of creating a data receipt,
/// `DataReceipt`.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/DataReceiptCreationConfigView"
///        }
///      ]
///    },
///    "pessimistic_gas_price_inflation_ratio": {
///      "description": "Pessimistic gas price inflation ratio.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "int32"
///      },
///      "maxItems": 2,
///      "minItems": 2
///    },
///    "storage_usage_config": {
///      "description": "Describes fees for storage.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/StorageUsageConfigView"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RuntimeFeesConfigView {
    ///Describes the cost of creating a certain action, `Action`. Includes
    /// all variants.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action_creation_config: ::std::option::Option<ActionCreationConfigView>,
    ///Describes the cost of creating an action receipt, `ActionReceipt`,
    /// excluding the actual cost of actions.
    /// - `send` cost is burned when a receipt is created using
    ///   `promise_create` or `promise_batch_create`
    /// - `exec` cost is burned when the receipt is being executed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action_receipt_creation_config: ::std::option::Option<Fee>,
    ///Fraction of the burnt gas to reward to the contract account for
    /// execution.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub burnt_gas_reward: ::std::option::Option<[i32; 2usize]>,
    ///Describes the cost of creating a data receipt, `DataReceipt`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data_receipt_creation_config: ::std::option::Option<DataReceiptCreationConfigView>,
    ///Pessimistic gas price inflation ratio.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pessimistic_gas_price_inflation_ratio: ::std::option::Option<[i32; 2usize]>,
    ///Describes fees for storage.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_usage_config: ::std::option::Option<StorageUsageConfigView>,
}
impl ::std::default::Default for RuntimeFeesConfigView {
    fn default() -> Self {
        Self {
            action_creation_config: Default::default(),
            action_receipt_creation_config: Default::default(),
            burnt_gas_reward: Default::default(),
            data_receipt_creation_config: Default::default(),
            pessimistic_gas_price_inflation_ratio: Default::default(),
            storage_usage_config: Default::default(),
        }
    }
}
///The shard identifier. It may be an arbitrary number - it does not need
/// to be a number in the range 0..NUM_SHARDS. The shard ids do not need
/// to be sequential or contiguous.
///
///The shard id is wrapped in a new type to prevent the old pattern of
/// using indices in range 0..NUM_SHARDS and casting to ShardId. Once
/// the transition if fully complete it potentially may be simplified to
/// a regular type alias.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The shard identifier. It may be an arbitrary number -
/// it does not need to be\na number in the range 0..NUM_SHARDS. The shard
/// ids do not need to be\nsequential or contiguous.\n\nThe shard id is
/// wrapped in a new type to prevent the old pattern of using\nindices in
/// range 0..NUM_SHARDS and casting to ShardId. Once the transition\nif
/// fully complete it potentially may be simplified to a regular type
/// alias.",
///  "type": "integer",
///  "format": "uint64",
///  "minimum": 0.0
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ShardId(pub u64);
impl ::std::ops::Deref for ShardId {
    type Target = u64;
    fn deref(&self) -> &u64 {
        &self.0
    }
}
impl ::std::convert::From<u64> for ShardId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for ShardId {
    type Err = <u64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for ShardId {
    type Error = <u64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for ShardId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///A versioned struct that contains all information needed to assign
/// accounts to shards.
///
///Because of re-sharding, the chain may use different shard layout to
/// split shards at different times. Currently, `ShardLayout` is stored
/// as part of `EpochConfig`, which is generated each epoch given the
/// epoch protocol version. In mainnet/testnet, we use two shard layouts
/// since re-sharding has only happened once. It is stored as part of
/// genesis config, see default_simple_nightshade_shard_layout() Below
/// is an overview for some important functionalities of ShardLayout
/// interface.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A versioned struct that contains all information needed
/// to assign accounts to shards.\n\nBecause of re-sharding, the chain may
/// use different shard layout to split shards at different\ntimes.
/// Currently, `ShardLayout` is stored as part of `EpochConfig`, which is
/// generated each\nepoch given the epoch protocol version. In
/// mainnet/testnet, we use two shard layouts since\nre-sharding has only
/// happened once. It is stored as part of genesis config,
/// see\ndefault_simple_nightshade_shard_layout() Below is an overview for
/// some important\nfunctionalities of ShardLayout interface.",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "V0"
///      ],
///      "properties": {
///        "V0": {
///          "$ref": "#/components/schemas/ShardLayoutV0"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "V1"
///      ],
///      "properties": {
///        "V1": {
///          "$ref": "#/components/schemas/ShardLayoutV1"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "V2"
///      ],
///      "properties": {
///        "V2": {
///          "$ref": "#/components/schemas/ShardLayoutV2"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "type": "object",
///      "required": [
///        "V3"
///      ],
///      "properties": {
///        "V3": {
///          "$ref": "#/components/schemas/ShardLayoutV3"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ShardLayout {
    V0(ShardLayoutV0),
    V1(ShardLayoutV1),
    V2(ShardLayoutV2),
    V3(ShardLayoutV3),
}
impl ::std::convert::From<ShardLayoutV0> for ShardLayout {
    fn from(value: ShardLayoutV0) -> Self {
        Self::V0(value)
    }
}
impl ::std::convert::From<ShardLayoutV1> for ShardLayout {
    fn from(value: ShardLayoutV1) -> Self {
        Self::V1(value)
    }
}
impl ::std::convert::From<ShardLayoutV2> for ShardLayout {
    fn from(value: ShardLayoutV2) -> Self {
        Self::V2(value)
    }
}
impl ::std::convert::From<ShardLayoutV3> for ShardLayout {
    fn from(value: ShardLayoutV3) -> Self {
        Self::V3(value)
    }
}
///A shard layout that maps accounts evenly across all shards -- by
/// calculate the hash of account id and mod number of shards. This is
/// added to capture the old `account_id_to_shard_id` algorithm, to keep
/// backward compatibility for some existing tests. `parent_shards` for
/// `ShardLayoutV1` is always `None`, meaning it can only be the first shard
/// layout a chain uses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A shard layout that maps accounts evenly across all
/// shards -- by calculate the hash of account\nid and mod number of shards.
/// This is added to capture the old `account_id_to_shard_id` algorithm,\nto
/// keep backward compatibility for some existing tests.\n`parent_shards`
/// for `ShardLayoutV1` is always `None`, meaning it can only be the first
/// shard layout\na chain uses.",
///  "type": "object",
///  "required": [
///    "num_shards",
///    "version"
///  ],
///  "properties": {
///    "num_shards": {
///      "description": "Map accounts evenly across all shards",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "version": {
///      "description": "Version of the shard layout, this is useful for
/// uniquely identify the shard layout",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV0 {
    ///Map accounts evenly across all shards
    pub num_shards: u64,
    ///Version of the shard layout, this is useful for uniquely identify
    /// the shard layout
    pub version: u32,
}
///`ShardLayoutV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "version"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "description": "The boundary accounts are the accounts on
/// boundaries between shards.\nEach shard contains a range of accounts from
/// one boundary account to\nanother - or the smallest or largest account
/// possible. The total\nnumber of shards is equal to the number of boundary
/// accounts plus 1.",
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccountId"
///      }
///    },
///    "shards_split_map": {
///      "description": "Maps shards from the last shard layout to shards
/// that it splits to in this shard layout,\nUseful for constructing states
/// for the shards.\nNone for the genesis shard layout",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "array",
///        "items": {
///          "$ref": "#/components/schemas/ShardId"
///        }
///      }
///    },
///    "to_parent_shard_map": {
///      "description": "Maps shard in this shard layout to their parent
/// shard\nSince shard_ids always range from 0 to num_shards - 1, we use vec
/// instead of a hashmap",
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "version": {
///      "description": "Version of the shard layout, this is useful for
/// uniquely identify the shard layout",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV1 {
    ///The boundary accounts are the accounts on boundaries between shards.
    ///Each shard contains a range of accounts from one boundary account to
    ///another - or the smallest or largest account possible. The total
    ///number of shards is equal to the number of boundary accounts plus 1.
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    ///Maps shards from the last shard layout to shards that it splits to
    /// in this shard layout, Useful for constructing states for the
    /// shards. None for the genesis shard layout
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_split_map: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<ShardId>>>,
    ///Maps shard in this shard layout to their parent shard
    ///Since shard_ids always range from 0 to num_shards - 1, we use vec
    /// instead of a hashmap
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to_parent_shard_map: ::std::option::Option<::std::vec::Vec<ShardId>>,
    ///Version of the shard layout, this is useful for uniquely identify
    /// the shard layout
    pub version: u32,
}
///Counterpart to `ShardLayoutV2` composed of maps with string keys to aid
///serde serialization.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Counterpart to `ShardLayoutV2` composed of maps with
/// string keys to aid\nserde serialization.",
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "id_to_index_map",
///    "index_to_id_map",
///    "shard_ids",
///    "version"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccountId"
///      }
///    },
///    "id_to_index_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "uint",
///        "minimum": 0.0
///      }
///    },
///    "index_to_id_map": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "shard_ids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "shards_parent_map": {
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "shards_split_map": {
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/components/schemas/ShardId"
///        }
///      }
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV2 {
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    pub id_to_index_map: ::std::collections::HashMap<::std::string::String, u32>,
    pub index_to_id_map: ::std::collections::HashMap<::std::string::String, ShardId>,
    pub shard_ids: ::std::vec::Vec<ShardId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_parent_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, ShardId>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub shards_split_map: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<ShardId>>,
    >,
    pub version: u32,
}
///Counterpart to `ShardLayoutV3` composed of maps with string keys to aid
///serde serialization.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Counterpart to `ShardLayoutV3` composed of maps with
/// string keys to aid\nserde serialization.",
///  "type": "object",
///  "required": [
///    "boundary_accounts",
///    "id_to_index_map",
///    "last_split",
///    "shard_ids",
///    "shards_split_map"
///  ],
///  "properties": {
///    "boundary_accounts": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/AccountId"
///      }
///    },
///    "id_to_index_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "uint",
///        "minimum": 0.0
///      }
///    },
///    "last_split": {
///      "$ref": "#/components/schemas/ShardId"
///    },
///    "shard_ids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ShardId"
///      }
///    },
///    "shards_split_map": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "array",
///        "items": {
///          "$ref": "#/components/schemas/ShardId"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardLayoutV3 {
    pub boundary_accounts: ::std::vec::Vec<AccountId>,
    pub id_to_index_map: ::std::collections::HashMap<::std::string::String, u32>,
    pub last_split: ShardId,
    pub shard_ids: ::std::vec::Vec<ShardId>,
    pub shards_split_map:
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<ShardId>>,
}
///`ShardUId` is a unique representation for shards from different shard
/// layouts.
///
///Comparing to `ShardId`, which is just an ordinal number ranging from 0
/// to NUM_SHARDS-1, `ShardUId` provides a way to unique identify shards
/// when shard layouts may change across epochs. This is important
/// because we store states indexed by shards in our database, so we need a
/// way to unique identify shard even when shards change across epochs.
///Another difference between `ShardUId` and `ShardId` is that `ShardUId`
/// should only exist in a node's internal state while `ShardId` can be
/// exposed to outside APIs and used in protocol level information (for
/// example, `ShardChunkHeader` contains `ShardId` instead of `ShardUId`)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`ShardUId` is a unique representation for shards from
/// different shard layouts.\n\nComparing to `ShardId`, which is just an
/// ordinal number ranging from 0 to NUM_SHARDS-1,\n`ShardUId` provides a
/// way to unique identify shards when shard layouts may change across
/// epochs.\nThis is important because we store states indexed by shards in
/// our database, so we need a\nway to unique identify shard even when
/// shards change across epochs.\nAnother difference between `ShardUId` and
/// `ShardId` is that `ShardUId` should only exist in\na node's internal
/// state while `ShardId` can be exposed to outside APIs and used in
/// protocol\nlevel information (for example, `ShardChunkHeader` contains
/// `ShardId` instead of `ShardUId`)",
///  "type": "object",
///  "required": [
///    "shard_id",
///    "version"
///  ],
///  "properties": {
///    "shard_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "version": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ShardUId {
    pub shard_id: u32,
    pub version: u32,
}
///`Signature`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct Signature(pub ::std::string::String);
impl ::std::ops::Deref for Signature {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for Signature {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Signature {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for Signature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SignedDelegateAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "delegate_action",
///    "signature"
///  ],
///  "properties": {
///    "delegate_action": {
///      "$ref": "#/components/schemas/DelegateAction"
///    },
///    "signature": {
///      "$ref": "#/components/schemas/Signature"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedDelegateAction {
    pub delegate_action: DelegateAction,
    pub signature: Signature,
}
///`SignedTransaction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "format": "byte"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct SignedTransaction(pub ::std::string::String);
impl ::std::ops::Deref for SignedTransaction {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for SignedTransaction {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SignedTransaction {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for SignedTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SignedTransactionView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "actions",
///    "hash",
///    "nonce",
///    "public_key",
///    "receiver_id",
///    "signature",
///    "signer_id"
///  ],
///  "properties": {
///    "actions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/ActionView"
///      }
///    },
///    "hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "priority_fee": {
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    },
///    "receiver_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "signature": {
///      "$ref": "#/components/schemas/Signature"
///    },
///    "signer_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedTransactionView {
    pub actions: ::std::vec::Vec<ActionView>,
    pub hash: CryptoHash,
    pub nonce: u64,
    #[serde(default)]
    pub priority_fee: u64,
    pub public_key: PublicKey,
    pub receiver_id: AccountId,
    pub signature: Signature,
    pub signer_id: AccountId,
}
///`SingleAccessKeyChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleAccessKeyChangesByBlockIdChangesType {
    #[serde(rename = "single_access_key_changes")]
    SingleAccessKeyChanges,
}
impl ::std::fmt::Display for SingleAccessKeyChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleAccessKeyChanges => f.write_str("single_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleAccessKeyChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_access_key_changes" => Ok(Self::SingleAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleAccessKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for SingleAccessKeyChangesByBlockIdChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SingleAccessKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingleAccessKeyChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleAccessKeyChangesByFinalityChangesType {
    #[serde(rename = "single_access_key_changes")]
    SingleAccessKeyChanges,
}
impl ::std::fmt::Display for SingleAccessKeyChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleAccessKeyChanges => f.write_str("single_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleAccessKeyChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_access_key_changes" => Ok(Self::SingleAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleAccessKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for SingleAccessKeyChangesByFinalityChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for SingleAccessKeyChangesByFinalityChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingleAccessKeyChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_access_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleAccessKeyChangesBySyncCheckpointChangesType {
    #[serde(rename = "single_access_key_changes")]
    SingleAccessKeyChanges,
}
impl ::std::fmt::Display for SingleAccessKeyChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleAccessKeyChanges => f.write_str("single_access_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleAccessKeyChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_access_key_changes" => Ok(Self::SingleAccessKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleAccessKeyChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for SingleAccessKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for SingleAccessKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingleGasKeyChangesByBlockIdChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleGasKeyChangesByBlockIdChangesType {
    #[serde(rename = "single_gas_key_changes")]
    SingleGasKeyChanges,
}
impl ::std::fmt::Display for SingleGasKeyChangesByBlockIdChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleGasKeyChanges => f.write_str("single_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleGasKeyChangesByBlockIdChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_gas_key_changes" => Ok(Self::SingleGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SingleGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SingleGasKeyChangesByBlockIdChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingleGasKeyChangesByFinalityChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleGasKeyChangesByFinalityChangesType {
    #[serde(rename = "single_gas_key_changes")]
    SingleGasKeyChanges,
}
impl ::std::fmt::Display for SingleGasKeyChangesByFinalityChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleGasKeyChanges => f.write_str("single_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleGasKeyChangesByFinalityChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_gas_key_changes" => Ok(Self::SingleGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SingleGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SingleGasKeyChangesByFinalityChangesType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingleGasKeyChangesBySyncCheckpointChangesType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "single_gas_key_changes"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SingleGasKeyChangesBySyncCheckpointChangesType {
    #[serde(rename = "single_gas_key_changes")]
    SingleGasKeyChanges,
}
impl ::std::fmt::Display for SingleGasKeyChangesBySyncCheckpointChangesType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SingleGasKeyChanges => f.write_str("single_gas_key_changes"),
        }
    }
}
impl ::std::str::FromStr for SingleGasKeyChangesBySyncCheckpointChangesType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "single_gas_key_changes" => Ok(Self::SingleGasKeyChanges),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SingleGasKeyChangesBySyncCheckpointChangesType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for SingleGasKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for SingleGasKeyChangesBySyncCheckpointChangesType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SlashedValidator`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id",
///    "is_double_sign"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "is_double_sign": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SlashedValidator {
    pub account_id: AccountId,
    pub is_double_sign: bool,
}
///An action which stakes signer_id tokens and setup's validator public key
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An action which stakes signer_id tokens and setup's
/// validator public key",
///  "type": "object",
///  "required": [
///    "public_key",
///    "stake"
///  ],
///  "properties": {
///    "public_key": {
///      "description": "Validator key which will be used to sign
/// transactions on behalf of signer_id",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/PublicKey"
///        }
///      ]
///    },
///    "stake": {
///      "description": "Amount of tokens to stake.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/NearToken"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StakeAction {
    ///Validator key which will be used to sign transactions on behalf of
    /// signer_id
    pub public_key: PublicKey,
    ///Amount of tokens to stake.
    pub stake: NearToken,
}
///See crate::types::StateChangeCause for details.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "See crate::types::StateChangeCause for details.",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "not_writable_to_disk"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "initial_state"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "tx_hash",
///        "type"
///      ],
///      "properties": {
///        "tx_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "transaction_processing"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "action_receipt_processing_started"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "action_receipt_gas_reward"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "receipt_processing"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "receipt_hash",
///        "type"
///      ],
///      "properties": {
///        "receipt_hash": {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "postponed_receipt"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "updated_delayed_receipts"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "validator_accounts_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "migration"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "type"
///      ],
///      "properties": {
///        "type": {
///          "type": "string",
///          "enum": [
///            "bandwidth_scheduler_state_update"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StateChangeCauseView {
    #[serde(rename = "not_writable_to_disk")]
    NotWritableToDisk,
    #[serde(rename = "initial_state")]
    InitialState,
    #[serde(rename = "transaction_processing")]
    TransactionProcessing { tx_hash: CryptoHash },
    #[serde(rename = "action_receipt_processing_started")]
    ActionReceiptProcessingStarted { receipt_hash: CryptoHash },
    #[serde(rename = "action_receipt_gas_reward")]
    ActionReceiptGasReward { receipt_hash: CryptoHash },
    #[serde(rename = "receipt_processing")]
    ReceiptProcessing { receipt_hash: CryptoHash },
    #[serde(rename = "postponed_receipt")]
    PostponedReceipt { receipt_hash: CryptoHash },
    #[serde(rename = "updated_delayed_receipts")]
    UpdatedDelayedReceipts,
    #[serde(rename = "validator_accounts_update")]
    ValidatorAccountsUpdate,
    #[serde(rename = "migration")]
    Migration,
    #[serde(rename = "bandwidth_scheduler_state_update")]
    BandwidthSchedulerStateUpdate,
}
///It is a [serializable view] of [`StateChangeKind`].
///
///[serializable view]: ./index.html
///[`StateChangeKind`]: ../types/struct.StateChangeKind.html
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "It is a [serializable view] of
/// [`StateChangeKind`].\n\n[serializable view]:
/// ./index.html\n[`StateChangeKind`]:
/// ../types/struct.StateChangeKind.html",
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "account_touched"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "access_key_touched"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "data_touched"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "account_id",
///        "type"
///      ],
///      "properties": {
///        "account_id": {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "contract_code_touched"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type", content = "account_id")]
pub enum StateChangeKindView {
    #[serde(rename = "account_touched")]
    AccountTouched(AccountId),
    #[serde(rename = "access_key_touched")]
    AccessKeyTouched(AccountId),
    #[serde(rename = "data_touched")]
    DataTouched(AccountId),
    #[serde(rename = "contract_code_touched")]
    ContractCodeTouched(AccountId),
}
///`StateChangeWithCauseView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "description": "A view of the account",
///          "type": "object",
///          "required": [
///            "account_id",
///            "amount",
///            "code_hash",
///            "locked",
///            "storage_usage"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "amount": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "code_hash": {
///              "$ref": "#/components/schemas/CryptoHash"
///            },
///            "global_contract_account_id": {
///              "anyOf": [
///                {
///                  "$ref": "#/components/schemas/AccountId"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "global_contract_hash": {
///              "anyOf": [
///                {
///                  "$ref": "#/components/schemas/CryptoHash"
///                },
///                {
///                  "type": "null"
///                }
///              ]
///            },
///            "locked": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "storage_paid_at": {
///              "description": "TODO(2271): deprecated.",
///              "default": 0,
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "storage_usage": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "account_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "account_deletion"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "access_key",
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "access_key": {
///              "$ref": "#/components/schemas/AccessKeyView"
///            },
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "access_key_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "access_key_deletion"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "gas_key",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "gas_key": {
///              "$ref": "#/components/schemas/GasKey"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "gas_key_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "index",
///            "nonce",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "index": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            },
///            "nonce": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "gas_key_nonce_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "public_key"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "public_key": {
///              "$ref": "#/components/schemas/PublicKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "gas_key_deletion"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "key_base64",
///            "value_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "key_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            },
///            "value_base64": {
///              "$ref": "#/components/schemas/StoreValue"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "data_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "key_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "key_base64": {
///              "$ref": "#/components/schemas/StoreKey"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "data_deletion"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id",
///            "code_base64"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            },
///            "code_base64": {
///              "type": "string"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "contract_code_update"
///          ]
///        }
///      }
///    },
///    {
///      "type": "object",
///      "required": [
///        "cause",
///        "change",
///        "type"
///      ],
///      "properties": {
///        "cause": {
///          "$ref": "#/components/schemas/StateChangeCauseView"
///        },
///        "change": {
///          "type": "object",
///          "required": [
///            "account_id"
///          ],
///          "properties": {
///            "account_id": {
///              "$ref": "#/components/schemas/AccountId"
///            }
///          }
///        },
///        "type": {
///          "type": "string",
///          "enum": [
///            "contract_code_deletion"
///          ]
///        }
///      }
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StateChangeWithCauseView {
    #[serde(rename = "account_update")]
    AccountUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "account_deletion")]
    AccountDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "access_key_update")]
    AccessKeyUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "access_key_deletion")]
    AccessKeyDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "gas_key_update")]
    GasKeyUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "gas_key_nonce_update")]
    GasKeyNonceUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "gas_key_deletion")]
    GasKeyDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "data_update")]
    DataUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "data_deletion")]
    DataDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "contract_code_update")]
    ContractCodeUpdate {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
    #[serde(rename = "contract_code_deletion")]
    ContractCodeDeletion {
        cause: StateChangeCauseView,
        change: StateChangeWithCauseViewChange,
    },
}
///A view of the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A view of the account",
///  "type": "object",
///  "required": [
///    "account_id",
///    "amount",
///    "code_hash",
///    "locked",
///    "storage_usage"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "amount": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "code_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "global_contract_account_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/AccountId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "global_contract_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "locked": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "storage_paid_at": {
///      "description": "TODO(2271): deprecated.",
///      "default": 0,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "storage_usage": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateChangeWithCauseViewChange {
    pub account_id: AccountId,
    pub amount: NearToken,
    pub code_hash: CryptoHash,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_account_id: ::std::option::Option<AccountId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_hash: ::std::option::Option<CryptoHash>,
    pub locked: NearToken,
    ///TODO(2271): deprecated.
    #[serde(default)]
    pub storage_paid_at: u64,
    pub storage_usage: u64,
}
///Item of the state, key and value are serialized in base64 and proof for
/// inclusion of given state item.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Item of the state, key and value are serialized in
/// base64 and proof for inclusion of given state item.",
///  "type": "object",
///  "required": [
///    "key",
///    "value"
///  ],
///  "properties": {
///    "key": {
///      "$ref": "#/components/schemas/StoreKey"
///    },
///    "value": {
///      "$ref": "#/components/schemas/StoreValue"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateItem {
    pub key: StoreKey,
    pub value: StoreValue,
}
///`StateSyncConfig`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "concurrency": {
///      "$ref": "#/components/schemas/SyncConcurrency"
///    },
///    "dump": {
///      "description": "`none` value disables state dump to external
/// storage.",
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/DumpConfig"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "parts_compression_lvl": {
///      "description": "Zstd compression level for state parts.",
///      "default": 1,
///      "type": "integer",
///      "format": "int32"
///    },
///    "sync": {
///      "$ref": "#/components/schemas/SyncConfig"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StateSyncConfig {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub concurrency: ::std::option::Option<SyncConcurrency>,
    ///`none` value disables state dump to external storage.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub dump: ::std::option::Option<DumpConfig>,
    ///Zstd compression level for state parts.
    #[serde(default = "defaults::default_u64::<i32, 1>")]
    pub parts_compression_lvl: i32,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sync: ::std::option::Option<SyncConfig>,
}
impl ::std::default::Default for StateSyncConfig {
    fn default() -> Self {
        Self {
            concurrency: Default::default(),
            dump: Default::default(),
            parts_compression_lvl: defaults::default_u64::<i32, 1>(),
            sync: Default::default(),
        }
    }
}
///`StatusSyncInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "latest_block_hash",
///    "latest_block_height",
///    "latest_block_time",
///    "latest_state_root",
///    "syncing"
///  ],
///  "properties": {
///    "earliest_block_hash": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/CryptoHash"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "earliest_block_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "earliest_block_time": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "epoch_id": {
///      "anyOf": [
///        {
///          "$ref": "#/components/schemas/EpochId"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "epoch_start_height": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "latest_block_hash": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "latest_block_height": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "latest_block_time": {
///      "type": "string"
///    },
///    "latest_state_root": {
///      "$ref": "#/components/schemas/CryptoHash"
///    },
///    "syncing": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StatusSyncInfo {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_hash: ::std::option::Option<CryptoHash>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_height: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub earliest_block_time: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_id: ::std::option::Option<EpochId>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub epoch_start_height: ::std::option::Option<u64>,
    pub latest_block_hash: CryptoHash,
    pub latest_block_height: u64,
    pub latest_block_time: ::std::string::String,
    pub latest_state_root: CryptoHash,
    pub syncing: bool,
}
///Errors which may occur during working with trie storages, storing
///trie values (trie nodes and state values) by their hashes.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Errors which may occur during working with trie
/// storages, storing\ntrie values (trie nodes and state values) by their
/// hashes.",
///  "oneOf": [
///    {
///      "description": "Key-value db internal failure",
///      "type": "string",
///      "enum": [
///        "StorageInternalError"
///      ]
///    },
///    {
///      "description": "Requested trie value by its hash which is missing
/// in storage.",
///      "type": "object",
///      "required": [
///        "MissingTrieValue"
///      ],
///      "properties": {
///        "MissingTrieValue": {
///          "$ref": "#/components/schemas/MissingTrieValue"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Found trie node which shouldn't be part of state.
/// Raised during\nvalidation of state sync parts where incorrect node was
/// passed.\nTODO (#8997): consider including hash of trie node.",
///      "type": "string",
///      "enum": [
///        "UnexpectedTrieValue"
///      ]
///    },
///    {
///      "description": "Either invalid state or key-value db is
/// corrupted.\nFor PartialStorage it cannot be corrupted.\nError message is
/// unreliable and for debugging purposes only. It's also probably ok
/// to\npanic in every place that produces this error.\nWe can check if db
/// is corrupted by verifying everything in the state trie.",
///      "type": "object",
///      "required": [
///        "StorageInconsistentState"
///      ],
///      "properties": {
///        "StorageInconsistentState": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Flat storage error, meaning that it doesn't support
/// some block anymore.\nWe guarantee that such block cannot become final,
/// thus block processing\nmust resume normally.",
///      "type": "object",
///      "required": [
///        "FlatStorageBlockNotSupported"
///      ],
///      "properties": {
///        "FlatStorageBlockNotSupported": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "In-memory trie could not be loaded for some
/// reason.",
///      "type": "object",
///      "required": [
///        "MemTrieLoadingError"
///      ],
///      "properties": {
///        "MemTrieLoadingError": {
///          "type": "string"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum StorageError {
    ///Key-value db internal failure
    StorageInternalError,
    ///Requested trie value by its hash which is missing in storage.
    MissingTrieValue(MissingTrieValue),
    ///Found trie node which shouldn't be part of state. Raised during
    ///validation of state sync parts where incorrect node was passed.
    ///TODO (#8997): consider including hash of trie node.
    UnexpectedTrieValue,
    ///Either invalid state or key-value db is corrupted.
    ///For PartialStorage it cannot be corrupted.
    ///Error message is unreliable and for debugging purposes only. It's
    /// also probably ok to panic in every place that produces this
    /// error. We can check if db is corrupted by verifying
    /// everything in the state trie.
    StorageInconsistentState(::std::string::String),
    ///Flat storage error, meaning that it doesn't support some block
    /// anymore. We guarantee that such block cannot become final,
    /// thus block processing must resume normally.
    FlatStorageBlockNotSupported(::std::string::String),
    ///In-memory trie could not be loaded for some reason.
    MemTrieLoadingError(::std::string::String),
}
impl ::std::convert::From<MissingTrieValue> for StorageError {
    fn from(value: MissingTrieValue) -> Self {
        Self::MissingTrieValue(value)
    }
}
///This enum represents if a storage_get call will be performed through
/// flat storage or trie
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This enum represents if a storage_get call will be
/// performed through flat storage or trie",
///  "type": "string",
///  "enum": [
///    "FlatStorage",
///    "Trie"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum StorageGetMode {
    FlatStorage,
    Trie,
}
impl ::std::fmt::Display for StorageGetMode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::FlatStorage => f.write_str("FlatStorage"),
            Self::Trie => f.write_str("Trie"),
        }
    }
}
impl ::std::str::FromStr for StorageGetMode {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "FlatStorage" => Ok(Self::FlatStorage),
            "Trie" => Ok(Self::Trie),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StorageGetMode {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Describes cost of storage per block
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes cost of storage per block",
///  "type": "object",
///  "properties": {
///    "num_bytes_account": {
///      "description": "Number of bytes for an account record, including
/// rounding up for account id.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_extra_bytes_record": {
///      "description": "Additional number of bytes for a k/v record",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct StorageUsageConfigView {
    ///Number of bytes for an account record, including rounding up for
    /// account id.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub num_bytes_account: ::std::option::Option<u64>,
    ///Additional number of bytes for a k/v record
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub num_extra_bytes_record: ::std::option::Option<u64>,
}
impl ::std::default::Default for StorageUsageConfigView {
    fn default() -> Self {
        Self {
            num_bytes_account: Default::default(),
            num_extra_bytes_record: Default::default(),
        }
    }
}
///This type is used to mark keys (arrays of bytes) that are queried from
/// store.
///
///NOTE: Currently, this type is only used in the view_client and RPC to be
/// able to transparently pretty-serialize the bytes arrays as
/// base64-encoded strings (see `serialize.rs`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This type is used to mark keys (arrays of bytes) that
/// are queried from store.\n\nNOTE: Currently, this type is only used in
/// the view_client and RPC to be able to transparently\npretty-serialize
/// the bytes arrays as base64-encoded strings (see `serialize.rs`).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct StoreKey(pub ::std::string::String);
impl ::std::ops::Deref for StoreKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for StoreKey {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StoreKey {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StoreKey {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///This type is used to mark values returned from store (arrays of bytes).
///
///NOTE: Currently, this type is only used in the view_client and RPC to be
/// able to transparently pretty-serialize the bytes arrays as
/// base64-encoded strings (see `serialize.rs`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This type is used to mark values returned from store
/// (arrays of bytes).\n\nNOTE: Currently, this type is only used in the
/// view_client and RPC to be able to transparently\npretty-serialize the
/// bytes arrays as base64-encoded strings (see `serialize.rs`).",
///  "type": "string",
///  "format": "bytes"
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
#[serde(transparent)]
pub struct StoreValue(pub ::std::string::String);
impl ::std::ops::Deref for StoreValue {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<::std::string::String> for StoreValue {
    fn from(value: ::std::string::String) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for StoreValue {
    type Err = ::std::convert::Infallible;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.to_string()))
    }
}
impl ::std::fmt::Display for StoreValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SyncCheckpoint`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "genesis",
///    "earliest_available"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum SyncCheckpoint {
    #[serde(rename = "genesis")]
    Genesis,
    #[serde(rename = "earliest_available")]
    EarliestAvailable,
}
impl ::std::fmt::Display for SyncCheckpoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Genesis => f.write_str("genesis"),
            Self::EarliestAvailable => f.write_str("earliest_available"),
        }
    }
}
impl ::std::str::FromStr for SyncCheckpoint {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "genesis" => Ok(Self::Genesis),
            "earliest_available" => Ok(Self::EarliestAvailable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SyncCheckpoint {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SyncConcurrency`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "apply": {
///      "description": "Maximum number of \"apply parts\" tasks that can be
/// performed in parallel.\nThis is a very disk-heavy task and therefore we
/// set this to a low limit,\nor else the rocksdb contention makes the whole
/// server freeze up.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "apply_during_catchup": {
///      "description": "Maximum number of \"apply parts\" tasks that can be
/// performed in parallel\nduring catchup. We set this to a very low value
/// to avoid overloading the\nnode while it is still performing normal
/// tasks.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "peer_downloads": {
///      "description": "Maximum number of outstanding requests for
/// decentralized state sync.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    },
///    "per_shard": {
///      "description": "The maximum parallelism to use per shard. This is mostly for fairness, because\nthe actual rate limiting is done by the TaskTrackers, but this is useful for\nbalancing the shards a little.",
///      "type": "integer",
///      "format": "uint8",
///      "maximum": 255.0,
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SyncConcurrency {
    ///Maximum number of "apply parts" tasks that can be performed in
    /// parallel. This is a very disk-heavy task and therefore we
    /// set this to a low limit, or else the rocksdb contention
    /// makes the whole server freeze up.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub apply: ::std::option::Option<u8>,
    ///Maximum number of "apply parts" tasks that can be performed in
    /// parallel during catchup. We set this to a very low value to
    /// avoid overloading the node while it is still performing
    /// normal tasks.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub apply_during_catchup: ::std::option::Option<u8>,
    ///Maximum number of outstanding requests for decentralized state sync.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub peer_downloads: ::std::option::Option<u8>,
    ///The maximum parallelism to use per shard. This is mostly for
    /// fairness, because the actual rate limiting is done by the
    /// TaskTrackers, but this is useful for balancing the shards a
    /// little.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub per_shard: ::std::option::Option<u8>,
}
impl ::std::default::Default for SyncConcurrency {
    fn default() -> Self {
        Self {
            apply: Default::default(),
            apply_during_catchup: Default::default(),
            peer_downloads: Default::default(),
            per_shard: Default::default(),
        }
    }
}
///Configures how to fetch state parts during state sync.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configures how to fetch state parts during state
/// sync.",
///  "oneOf": [
///    {
///      "description": "Syncs state from the peers without reading anything
/// from external storage.",
///      "type": "string",
///      "enum": [
///        "Peers"
///      ]
///    },
///    {
///      "description": "Expects parts to be available in external
/// storage.\n\nUsually as a fallback after some number of attempts to use
/// peers.",
///      "type": "object",
///      "required": [
///        "ExternalStorage"
///      ],
///      "properties": {
///        "ExternalStorage": {
///          "$ref": "#/components/schemas/ExternalStorageConfig"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum SyncConfig {
    ///Syncs state from the peers without reading anything from external
    /// storage.
    Peers,
    ///Expects parts to be available in external storage.
    ///
    ///Usually as a fallback after some number of attempts to use peers.
    ExternalStorage(ExternalStorageConfig),
}
impl ::std::convert::From<ExternalStorageConfig> for SyncConfig {
    fn from(value: ExternalStorageConfig) -> Self {
        Self::ExternalStorage(value)
    }
}
///`Tier1ProxyView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "addr",
///    "peer_id"
///  ],
///  "properties": {
///    "addr": {
///      "type": "string"
///    },
///    "peer_id": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Tier1ProxyView {
    pub addr: ::std::string::String,
    pub peer_id: PublicKey,
}
///Describes the expected behavior of the node regarding shard tracking.
///If the node is an active validator, it will also track the shards it is
/// responsible for as a validator.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Describes the expected behavior of the node regarding
/// shard tracking.\nIf the node is an active validator, it will also track
/// the shards it is responsible for as a validator.",
///  "oneOf": [
///    {
///      "description": "Tracks no shards (light client).",
///      "type": "string",
///      "enum": [
///        "NoShards"
///      ]
///    },
///    {
///      "description": "Tracks arbitrary shards.",
///      "type": "object",
///      "required": [
///        "Shards"
///      ],
///      "properties": {
///        "Shards": {
///          "type": "array",
///          "items": {
///            "$ref": "#/components/schemas/ShardUId"
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Tracks all shards.",
///      "type": "string",
///      "enum": [
///        "AllShards"
///      ]
///    },
///    {
///      "description": "Tracks shards that are assigned to given validator
/// account.",
///      "type": "object",
///      "required": [
///        "ShadowValidator"
///      ],
///      "properties": {
///        "ShadowValidator": {
///          "$ref": "#/components/schemas/AccountId"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Rotate between these sets of tracked shards.\nUsed
/// to simulate the behavior of chunk only producers without staking
/// tokens.",
///      "type": "object",
///      "required": [
///        "Schedule"
///      ],
///      "properties": {
///        "Schedule": {
///          "type": "array",
///          "items": {
///            "type": "array",
///            "items": {
///              "$ref": "#/components/schemas/ShardId"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Tracks shards that contain one of the given
/// account.",
///      "type": "object",
///      "required": [
///        "Accounts"
///      ],
///      "properties": {
///        "Accounts": {
///          "type": "array",
///          "items": {
///            "$ref": "#/components/schemas/AccountId"
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum TrackedShardsConfig {
    ///Tracks no shards (light client).
    NoShards,
    ///Tracks arbitrary shards.
    Shards(::std::vec::Vec<ShardUId>),
    ///Tracks all shards.
    AllShards,
    ///Tracks shards that are assigned to given validator account.
    ShadowValidator(AccountId),
    ///Rotate between these sets of tracked shards.
    ///Used to simulate the behavior of chunk only producers without
    /// staking tokens.
    Schedule(::std::vec::Vec<::std::vec::Vec<ShardId>>),
    ///Tracks shards that contain one of the given account.
    Accounts(::std::vec::Vec<AccountId>),
}
impl ::std::convert::From<::std::vec::Vec<ShardUId>> for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<ShardUId>) -> Self {
        Self::Shards(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::vec::Vec<ShardId>>> for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<::std::vec::Vec<ShardId>>) -> Self {
        Self::Schedule(value)
    }
}
impl ::std::convert::From<::std::vec::Vec<AccountId>> for TrackedShardsConfig {
    fn from(value: ::std::vec::Vec<AccountId>) -> Self {
        Self::Accounts(value)
    }
}
///`TransferAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposit"
///  ],
///  "properties": {
///    "deposit": {
///      "$ref": "#/components/schemas/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferAction {
    pub deposit: NearToken,
}
///`TransferToGasKeyAction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposit",
///    "public_key"
///  ],
///  "properties": {
///    "deposit": {
///      "$ref": "#/components/schemas/NearToken"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferToGasKeyAction {
    pub deposit: NearToken,
    pub public_key: PublicKey,
}
///Error returned in the ExecutionOutcome in case of failure
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Error returned in the ExecutionOutcome in case of
/// failure",
///  "oneOf": [
///    {
///      "description": "An error happened during Action execution",
///      "type": "object",
///      "required": [
///        "ActionError"
///      ],
///      "properties": {
///        "ActionError": {
///          "$ref": "#/components/schemas/ActionError"
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "An error happened during Transaction execution",
///      "type": "object",
///      "required": [
///        "InvalidTxError"
///      ],
///      "properties": {
///        "InvalidTxError": {
///          "$ref": "#/components/schemas/InvalidTxError"
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Debug, strum_macros::Display, thiserror::Error,
)]
pub enum TxExecutionError {
    ///An error happened during Action execution
    ActionError(ActionError),
    ///An error happened during Transaction execution
    InvalidTxError(InvalidTxError),
}
impl ::std::convert::From<ActionError> for TxExecutionError {
    fn from(value: ActionError) -> Self {
        Self::ActionError(value)
    }
}
impl ::std::convert::From<InvalidTxError> for TxExecutionError {
    fn from(value: InvalidTxError) -> Self {
        Self::InvalidTxError(value)
    }
}
///`TxExecutionStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "Transaction is waiting to be included into the
/// block",
///      "type": "string",
///      "enum": [
///        "NONE"
///      ]
///    },
///    {
///      "description": "Transaction is included into the block. The block
/// may be not finalized yet",
///      "type": "string",
///      "enum": [
///        "INCLUDED"
///      ]
///    },
///    {
///      "description": "Transaction is included into the block +\nAll non-refund transaction receipts finished their execution.\nThe corresponding blocks for tx and each receipt may be not finalized yet",
///      "type": "string",
///      "enum": [
///        "EXECUTED_OPTIMISTIC"
///      ]
///    },
///    {
///      "description": "Transaction is included into finalized block",
///      "type": "string",
///      "enum": [
///        "INCLUDED_FINAL"
///      ]
///    },
///    {
///      "description": "Transaction is included into finalized block +\nAll
/// non-refund transaction receipts finished their execution.\nThe
/// corresponding blocks for each receipt may be not finalized yet",
///      "type": "string",
///      "enum": [
///        "EXECUTED"
///      ]
///    },
///    {
///      "description": "Transaction is included into finalized block
/// +\nExecution of all transaction receipts is finalized, including refund
/// receipts",
///      "type": "string",
///      "enum": [
///        "FINAL"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TxExecutionStatus {
    ///Transaction is waiting to be included into the block
    #[serde(rename = "NONE")]
    None,
    ///Transaction is included into the block. The block may be not
    /// finalized yet
    #[serde(rename = "INCLUDED")]
    Included,
    ///Transaction is included into the block +
    ///All non-refund transaction receipts finished their execution.
    ///The corresponding blocks for tx and each receipt may be not
    /// finalized yet
    #[serde(rename = "EXECUTED_OPTIMISTIC")]
    ExecutedOptimistic,
    ///Transaction is included into finalized block
    #[serde(rename = "INCLUDED_FINAL")]
    IncludedFinal,
    ///Transaction is included into finalized block +
    ///All non-refund transaction receipts finished their execution.
    ///The corresponding blocks for each receipt may be not finalized yet
    #[serde(rename = "EXECUTED")]
    Executed,
    ///Transaction is included into finalized block +
    ///Execution of all transaction receipts is finalized, including refund
    /// receipts
    #[serde(rename = "FINAL")]
    Final,
}
impl ::std::fmt::Display for TxExecutionStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("NONE"),
            Self::Included => f.write_str("INCLUDED"),
            Self::ExecutedOptimistic => f.write_str("EXECUTED_OPTIMISTIC"),
            Self::IncludedFinal => f.write_str("INCLUDED_FINAL"),
            Self::Executed => f.write_str("EXECUTED"),
            Self::Final => f.write_str("FINAL"),
        }
    }
}
impl ::std::str::FromStr for TxExecutionStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NONE" => Ok(Self::None),
            "INCLUDED" => Ok(Self::Included),
            "EXECUTED_OPTIMISTIC" => Ok(Self::ExecutedOptimistic),
            "INCLUDED_FINAL" => Ok(Self::IncludedFinal),
            "EXECUTED" => Ok(Self::Executed),
            "FINAL" => Ok(Self::Final),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TxExecutionStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Use global contract action
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Use global contract action",
///  "type": "object",
///  "required": [
///    "contract_identifier"
///  ],
///  "properties": {
///    "contract_identifier": {
///      "$ref": "#/components/schemas/GlobalContractIdentifier"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UseGlobalContractAction {
    pub contract_identifier: GlobalContractIdentifier,
}
///`ValidatorInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorInfo {
    pub account_id: AccountId,
}
///Reasons for removing a validator from the validator set.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Reasons for removing a validator from the validator
/// set.",
///  "oneOf": [
///    {
///      "description": "Deprecated",
///      "type": "string",
///      "enum": [
///        "_UnusedSlashed"
///      ]
///    },
///    {
///      "description": "Validator didn't produce enough blocks.",
///      "type": "object",
///      "required": [
///        "NotEnoughBlocks"
///      ],
///      "properties": {
///        "NotEnoughBlocks": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Validator didn't produce enough chunks.",
///      "type": "object",
///      "required": [
///        "NotEnoughChunks"
///      ],
///      "properties": {
///        "NotEnoughChunks": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Validator unstaked themselves.",
///      "type": "string",
///      "enum": [
///        "Unstaked"
///      ]
///    },
///    {
///      "description": "Validator stake is now below threshold",
///      "type": "object",
///      "required": [
///        "NotEnoughStake"
///      ],
///      "properties": {
///        "NotEnoughStake": {
///          "type": "object",
///          "required": [
///            "stake_u128",
///            "threshold_u128"
///          ],
///          "properties": {
///            "stake_u128": {
///              "$ref": "#/components/schemas/NearToken"
///            },
///            "threshold_u128": {
///              "$ref": "#/components/schemas/NearToken"
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Enough stake but is not chosen because of seat
/// limits.",
///      "type": "string",
///      "enum": [
///        "DidNotGetASeat"
///      ]
///    },
///    {
///      "description": "Validator didn't produce enough chunk
/// endorsements.",
///      "type": "object",
///      "required": [
///        "NotEnoughChunkEndorsements"
///      ],
///      "properties": {
///        "NotEnoughChunkEndorsements": {
///          "type": "object",
///          "required": [
///            "expected",
///            "produced"
///          ],
///          "properties": {
///            "expected": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            },
///            "produced": {
///              "type": "integer",
///              "format": "uint64",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    },
///    {
///      "description": "Validator's last block proposal was for a protocol
/// version older than\nthe network's voted protocol version.",
///      "type": "object",
///      "required": [
///        "ProtocolVersionTooOld"
///      ],
///      "properties": {
///        "ProtocolVersionTooOld": {
///          "type": "object",
///          "required": [
///            "network_version",
///            "version"
///          ],
///          "properties": {
///            "network_version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            },
///            "version": {
///              "type": "integer",
///              "format": "uint32",
///              "minimum": 0.0
///            }
///          }
///        }
///      },
///      "additionalProperties": false
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub enum ValidatorKickoutReason {
    ///Deprecated
    #[serde(rename = "_UnusedSlashed")]
    UnusedSlashed,
    ///Validator didn't produce enough blocks.
    NotEnoughBlocks { expected: u64, produced: u64 },
    ///Validator didn't produce enough chunks.
    NotEnoughChunks { expected: u64, produced: u64 },
    ///Validator unstaked themselves.
    Unstaked,
    ///Validator stake is now below threshold
    NotEnoughStake {
        stake_u128: NearToken,
        threshold_u128: NearToken,
    },
    ///Enough stake but is not chosen because of seat limits.
    DidNotGetASeat,
    ///Validator didn't produce enough chunk endorsements.
    NotEnoughChunkEndorsements { expected: u64, produced: u64 },
    ///Validator's last block proposal was for a protocol version older
    /// than the network's voted protocol version.
    ProtocolVersionTooOld { network_version: u32, version: u32 },
}
///`ValidatorKickoutView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id",
///    "reason"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "reason": {
///      "$ref": "#/components/schemas/ValidatorKickoutReason"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorKickoutView {
    pub account_id: AccountId,
    pub reason: ValidatorKickoutReason,
}
///`ValidatorStakeView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "allOf": [
///    {
///      "$ref": "#/components/schemas/ValidatorStakeViewV1"
///    }
///  ],
///  "required": [
///    "validator_stake_struct_version"
///  ],
///  "properties": {
///    "validator_stake_struct_version": {
///      "type": "string",
///      "enum": [
///        "V1"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorStakeView {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: NearToken,
    pub validator_stake_struct_version: ValidatorStakeViewValidatorStakeStructVersion,
}
///`ValidatorStakeViewV1`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "account_id",
///    "public_key",
///    "stake"
///  ],
///  "properties": {
///    "account_id": {
///      "$ref": "#/components/schemas/AccountId"
///    },
///    "public_key": {
///      "$ref": "#/components/schemas/PublicKey"
///    },
///    "stake": {
///      "$ref": "#/components/schemas/NearToken"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValidatorStakeViewV1 {
    pub account_id: AccountId,
    pub public_key: PublicKey,
    pub stake: NearToken,
}
///`ValidatorStakeViewValidatorStakeStructVersion`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "V1"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ValidatorStakeViewValidatorStakeStructVersion {
    V1,
}
impl ::std::fmt::Display for ValidatorStakeViewValidatorStakeStructVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::V1 => f.write_str("V1"),
        }
    }
}
impl ::std::str::FromStr for ValidatorStakeViewValidatorStakeStructVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "V1" => Ok(Self::V1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ValidatorStakeViewValidatorStakeStructVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ValidatorStakeViewValidatorStakeStructVersion
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ValidatorStakeViewValidatorStakeStructVersion
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Data structure for semver version and github tag or commit.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Data structure for semver version and github tag or
/// commit.",
///  "type": "object",
///  "required": [
///    "build",
///    "commit",
///    "version"
///  ],
///  "properties": {
///    "build": {
///      "type": "string"
///    },
///    "commit": {
///      "type": "string"
///    },
///    "rustc_version": {
///      "default": "",
///      "type": "string"
///    },
///    "version": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Version {
    pub build: ::std::string::String,
    pub commit: ::std::string::String,
    #[serde(default)]
    pub rustc_version: ::std::string::String,
    pub version: ::std::string::String,
}
///`ViewAccessKeyByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyByBlockIdRequestType {
    #[serde(rename = "view_access_key")]
    ViewAccessKey,
}
impl ::std::fmt::Display for ViewAccessKeyByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKey => f.write_str("view_access_key"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key" => Ok(Self::ViewAccessKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccessKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccessKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccessKeyByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyByFinalityRequestType {
    #[serde(rename = "view_access_key")]
    ViewAccessKey,
}
impl ::std::fmt::Display for ViewAccessKeyByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKey => f.write_str("view_access_key"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key" => Ok(Self::ViewAccessKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccessKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccessKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccessKeyBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyBySyncCheckpointRequestType {
    #[serde(rename = "view_access_key")]
    ViewAccessKey,
}
impl ::std::fmt::Display for ViewAccessKeyBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKey => f.write_str("view_access_key"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key" => Ok(Self::ViewAccessKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccessKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccessKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccessKeyListByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyListByBlockIdRequestType {
    #[serde(rename = "view_access_key_list")]
    ViewAccessKeyList,
}
impl ::std::fmt::Display for ViewAccessKeyListByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKeyList => f.write_str("view_access_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyListByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key_list" => Ok(Self::ViewAccessKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccessKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccessKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccessKeyListByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyListByFinalityRequestType {
    #[serde(rename = "view_access_key_list")]
    ViewAccessKeyList,
}
impl ::std::fmt::Display for ViewAccessKeyListByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKeyList => f.write_str("view_access_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyListByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key_list" => Ok(Self::ViewAccessKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccessKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccessKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccessKeyListBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_access_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccessKeyListBySyncCheckpointRequestType {
    #[serde(rename = "view_access_key_list")]
    ViewAccessKeyList,
}
impl ::std::fmt::Display for ViewAccessKeyListBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccessKeyList => f.write_str("view_access_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewAccessKeyListBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_access_key_list" => Ok(Self::ViewAccessKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccessKeyListBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewAccessKeyListBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewAccessKeyListBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccountByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_account"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccountByBlockIdRequestType {
    #[serde(rename = "view_account")]
    ViewAccount,
}
impl ::std::fmt::Display for ViewAccountByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccount => f.write_str("view_account"),
        }
    }
}
impl ::std::str::FromStr for ViewAccountByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_account" => Ok(Self::ViewAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccountByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccountByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccountByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccountByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_account"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccountByFinalityRequestType {
    #[serde(rename = "view_account")]
    ViewAccount,
}
impl ::std::fmt::Display for ViewAccountByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccount => f.write_str("view_account"),
        }
    }
}
impl ::std::str::FromStr for ViewAccountByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_account" => Ok(Self::ViewAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccountByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccountByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccountByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewAccountBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_account"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewAccountBySyncCheckpointRequestType {
    #[serde(rename = "view_account")]
    ViewAccount,
}
impl ::std::fmt::Display for ViewAccountBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewAccount => f.write_str("view_account"),
        }
    }
}
impl ::std::str::FromStr for ViewAccountBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_account" => Ok(Self::ViewAccount),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewAccountBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewAccountBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewAccountBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewCodeByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewCodeByBlockIdRequestType {
    #[serde(rename = "view_code")]
    ViewCode,
}
impl ::std::fmt::Display for ViewCodeByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewCode => f.write_str("view_code"),
        }
    }
}
impl ::std::str::FromStr for ViewCodeByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_code" => Ok(Self::ViewCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewCodeByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewCodeByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewCodeByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewCodeByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewCodeByFinalityRequestType {
    #[serde(rename = "view_code")]
    ViewCode,
}
impl ::std::fmt::Display for ViewCodeByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewCode => f.write_str("view_code"),
        }
    }
}
impl ::std::str::FromStr for ViewCodeByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_code" => Ok(Self::ViewCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewCodeByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewCodeByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewCodeByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewCodeBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewCodeBySyncCheckpointRequestType {
    #[serde(rename = "view_code")]
    ViewCode,
}
impl ::std::fmt::Display for ViewCodeBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewCode => f.write_str("view_code"),
        }
    }
}
impl ::std::str::FromStr for ViewCodeBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_code" => Ok(Self::ViewCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewCodeBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewCodeBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewCodeBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyByBlockIdRequestType {
    #[serde(rename = "view_gas_key")]
    ViewGasKey,
}
impl ::std::fmt::Display for ViewGasKeyByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKey => f.write_str("view_gas_key"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key" => Ok(Self::ViewGasKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyByFinalityRequestType {
    #[serde(rename = "view_gas_key")]
    ViewGasKey,
}
impl ::std::fmt::Display for ViewGasKeyByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKey => f.write_str("view_gas_key"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key" => Ok(Self::ViewGasKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyBySyncCheckpointRequestType {
    #[serde(rename = "view_gas_key")]
    ViewGasKey,
}
impl ::std::fmt::Display for ViewGasKeyBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKey => f.write_str("view_gas_key"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key" => Ok(Self::ViewGasKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyListByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyListByBlockIdRequestType {
    #[serde(rename = "view_gas_key_list")]
    ViewGasKeyList,
}
impl ::std::fmt::Display for ViewGasKeyListByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKeyList => f.write_str("view_gas_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyListByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key_list" => Ok(Self::ViewGasKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyListByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyListByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyListByFinalityRequestType {
    #[serde(rename = "view_gas_key_list")]
    ViewGasKeyList,
}
impl ::std::fmt::Display for ViewGasKeyListByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKeyList => f.write_str("view_gas_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyListByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key_list" => Ok(Self::ViewGasKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyListByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGasKeyListBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_gas_key_list"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGasKeyListBySyncCheckpointRequestType {
    #[serde(rename = "view_gas_key_list")]
    ViewGasKeyList,
}
impl ::std::fmt::Display for ViewGasKeyListBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGasKeyList => f.write_str("view_gas_key_list"),
        }
    }
}
impl ::std::str::FromStr for ViewGasKeyListBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_gas_key_list" => Ok(Self::ViewGasKeyList),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGasKeyListBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewGasKeyListBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGasKeyListBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeByAccountIdByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code_by_account_id"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeByAccountIdByBlockIdRequestType {
    #[serde(rename = "view_global_contract_code_by_account_id")]
    ViewGlobalContractCodeByAccountId,
}
impl ::std::fmt::Display for ViewGlobalContractCodeByAccountIdByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCodeByAccountId => {
                f.write_str("view_global_contract_code_by_account_id")
            }
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeByAccountIdByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code_by_account_id" => {
                Ok(Self::ViewGlobalContractCodeByAccountId)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGlobalContractCodeByAccountIdByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeByAccountIdByBlockIdRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewGlobalContractCodeByAccountIdByBlockIdRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeByAccountIdByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code_by_account_id"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeByAccountIdByFinalityRequestType {
    #[serde(rename = "view_global_contract_code_by_account_id")]
    ViewGlobalContractCodeByAccountId,
}
impl ::std::fmt::Display for ViewGlobalContractCodeByAccountIdByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCodeByAccountId => {
                f.write_str("view_global_contract_code_by_account_id")
            }
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeByAccountIdByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code_by_account_id" => {
                Ok(Self::ViewGlobalContractCodeByAccountId)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGlobalContractCodeByAccountIdByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeByAccountIdByFinalityRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewGlobalContractCodeByAccountIdByFinalityRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code_by_account_id"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType {
    #[serde(rename = "view_global_contract_code_by_account_id")]
    ViewGlobalContractCodeByAccountId,
}
impl ::std::fmt::Display for ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCodeByAccountId => {
                f.write_str("view_global_contract_code_by_account_id")
            }
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code_by_account_id" => {
                Ok(Self::ViewGlobalContractCodeByAccountId)
            }
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str>
    for ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewGlobalContractCodeByAccountIdBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeByBlockIdRequestType {
    #[serde(rename = "view_global_contract_code")]
    ViewGlobalContractCode,
}
impl ::std::fmt::Display for ViewGlobalContractCodeByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCode => f.write_str("view_global_contract_code"),
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code" => Ok(Self::ViewGlobalContractCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGlobalContractCodeByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeByBlockIdRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewGlobalContractCodeByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeByFinalityRequestType {
    #[serde(rename = "view_global_contract_code")]
    ViewGlobalContractCode,
}
impl ::std::fmt::Display for ViewGlobalContractCodeByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCode => f.write_str("view_global_contract_code"),
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code" => Ok(Self::ViewGlobalContractCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGlobalContractCodeByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeByFinalityRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewGlobalContractCodeByFinalityRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewGlobalContractCodeBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_global_contract_code"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewGlobalContractCodeBySyncCheckpointRequestType {
    #[serde(rename = "view_global_contract_code")]
    ViewGlobalContractCode,
}
impl ::std::fmt::Display for ViewGlobalContractCodeBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewGlobalContractCode => f.write_str("view_global_contract_code"),
        }
    }
}
impl ::std::str::FromStr for ViewGlobalContractCodeBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_global_contract_code" => Ok(Self::ViewGlobalContractCode),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewGlobalContractCodeBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for ViewGlobalContractCodeBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for ViewGlobalContractCodeBySyncCheckpointRequestType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewStateByBlockIdRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_state"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewStateByBlockIdRequestType {
    #[serde(rename = "view_state")]
    ViewState,
}
impl ::std::fmt::Display for ViewStateByBlockIdRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewState => f.write_str("view_state"),
        }
    }
}
impl ::std::str::FromStr for ViewStateByBlockIdRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_state" => Ok(Self::ViewState),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewStateByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewStateByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewStateByBlockIdRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewStateByFinalityRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_state"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewStateByFinalityRequestType {
    #[serde(rename = "view_state")]
    ViewState,
}
impl ::std::fmt::Display for ViewStateByFinalityRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewState => f.write_str("view_state"),
        }
    }
}
impl ::std::str::FromStr for ViewStateByFinalityRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_state" => Ok(Self::ViewState),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewStateByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewStateByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewStateByFinalityRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ViewStateBySyncCheckpointRequestType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "view_state"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ViewStateBySyncCheckpointRequestType {
    #[serde(rename = "view_state")]
    ViewState,
}
impl ::std::fmt::Display for ViewStateBySyncCheckpointRequestType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ViewState => f.write_str("view_state"),
        }
    }
}
impl ::std::str::FromStr for ViewStateBySyncCheckpointRequestType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "view_state" => Ok(Self::ViewState),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ViewStateBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ViewStateBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ViewStateBySyncCheckpointRequestType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Resulting state values for a view state query request
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Resulting state values for a view state query request",
///  "type": "object",
///  "required": [
///    "values"
///  ],
///  "properties": {
///    "proof": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "values": {
///      "type": "array",
///      "items": {
///        "$ref": "#/components/schemas/StateItem"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ViewStateResult {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub proof: ::std::vec::Vec<::std::string::String>,
    pub values: ::std::vec::Vec<StateItem>,
}
///`VmConfigView`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "deterministic_account_ids": {
///      "description": "See
/// [VMConfig::deterministic_account_ids](crate::vm::Config::deterministic_account_ids).
/// ",
///      "type": "boolean"
///    },
///    "discard_custom_sections": {
///      "description": "See
/// [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
/// ",
///      "type": "boolean"
///    },
///    "eth_implicit_accounts": {
///      "description": "See
/// [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
/// ",
///      "type": "boolean"
///    },
///    "ext_costs": {
///      "description": "Costs for runtime externals",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/ExtCostsConfigView"
///        }
///      ]
///    },
///    "fix_contract_loading_cost": {
///      "description": "See
/// [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
/// ",
///      "type": "boolean"
///    },
///    "global_contract_host_fns": {
///      "description": "See
/// [VMConfig::global_contract_host_fns](crate::vm::Config::global_contract_host_fns).
/// ",
///      "type": "boolean"
///    },
///    "grow_mem_cost": {
///      "description": "Gas cost of a growing memory by single page.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "implicit_account_creation": {
///      "description": "Deprecated",
///      "type": "boolean"
///    },
///    "limit_config": {
///      "description": "Describes limits for VM and Runtime.\n\nTODO:
/// Consider changing this to `VMLimitConfigView` to avoid dependency\non
/// runtime.",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/LimitConfig"
///        }
///      ]
///    },
///    "linear_op_base_cost": {
///      "description": "Base gas cost of a linear operation",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "linear_op_unit_cost": {
///      "description": "Unit gas cost of a linear operation",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "reftypes_bulk_memory": {
///      "description": "See
/// [VMConfig::reftypes_bulk_memory](crate::vm::Config::reftypes_bulk_memory).
/// ",
///      "type": "boolean"
///    },
///    "regular_op_cost": {
///      "description": "Gas cost of a regular operation.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "saturating_float_to_int": {
///      "description": "See
/// [VMConfig::saturating_float_to_int](crate::vm::Config::saturating_float_to_int).
/// ",
///      "type": "boolean"
///    },
///    "storage_get_mode": {
///      "description": "See
/// [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/StorageGetMode"
///        }
///      ]
///    },
///    "vm_kind": {
///      "description": "See
/// [VMConfig::vm_kind](crate::vm::Config::vm_kind).",
///      "allOf": [
///        {
///          "$ref": "#/components/schemas/VMKind"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VmConfigView {
    ///See [VMConfig::deterministic_account_ids](crate::vm::Config::deterministic_account_ids).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deterministic_account_ids: ::std::option::Option<bool>,
    ///See [VMConfig::discard_custom_sections](crate::vm::Config::discard_custom_sections).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub discard_custom_sections: ::std::option::Option<bool>,
    ///See [VMConfig::eth_implicit_accounts](crate::vm::Config::eth_implicit_accounts).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub eth_implicit_accounts: ::std::option::Option<bool>,
    ///Costs for runtime externals
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ext_costs: ::std::option::Option<ExtCostsConfigView>,
    ///See [VMConfig::fix_contract_loading_cost](crate::vm::Config::fix_contract_loading_cost).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fix_contract_loading_cost: ::std::option::Option<bool>,
    ///See [VMConfig::global_contract_host_fns](crate::vm::Config::global_contract_host_fns).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub global_contract_host_fns: ::std::option::Option<bool>,
    ///Gas cost of a growing memory by single page.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub grow_mem_cost: ::std::option::Option<u32>,
    ///Deprecated
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub implicit_account_creation: ::std::option::Option<bool>,
    ///Describes limits for VM and Runtime.
    ///
    ///TODO: Consider changing this to `VMLimitConfigView` to avoid
    /// dependency on runtime.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit_config: ::std::option::Option<LimitConfig>,
    ///Base gas cost of a linear operation
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub linear_op_base_cost: ::std::option::Option<u64>,
    ///Unit gas cost of a linear operation
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub linear_op_unit_cost: ::std::option::Option<u64>,
    ///See [VMConfig::reftypes_bulk_memory](crate::vm::Config::reftypes_bulk_memory).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reftypes_bulk_memory: ::std::option::Option<bool>,
    ///Gas cost of a regular operation.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub regular_op_cost: ::std::option::Option<u32>,
    ///See [VMConfig::saturating_float_to_int](crate::vm::Config::saturating_float_to_int).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub saturating_float_to_int: ::std::option::Option<bool>,
    ///See [VMConfig::storage_get_mode](crate::vm::Config::storage_get_mode).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub storage_get_mode: ::std::option::Option<StorageGetMode>,
    ///See [VMConfig::vm_kind](crate::vm::Config::vm_kind).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub vm_kind: ::std::option::Option<VmKind>,
}
impl ::std::default::Default for VmConfigView {
    fn default() -> Self {
        Self {
            deterministic_account_ids: Default::default(),
            discard_custom_sections: Default::default(),
            eth_implicit_accounts: Default::default(),
            ext_costs: Default::default(),
            fix_contract_loading_cost: Default::default(),
            global_contract_host_fns: Default::default(),
            grow_mem_cost: Default::default(),
            implicit_account_creation: Default::default(),
            limit_config: Default::default(),
            linear_op_base_cost: Default::default(),
            linear_op_unit_cost: Default::default(),
            reftypes_bulk_memory: Default::default(),
            regular_op_cost: Default::default(),
            saturating_float_to_int: Default::default(),
            storage_get_mode: Default::default(),
            vm_kind: Default::default(),
        }
    }
}
///`VmKind`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "description": "Wasmer 0.17.x VM. Gone now.",
///      "type": "string",
///      "enum": [
///        "Wasmer0"
///      ]
///    },
///    {
///      "description": "Wasmtime VM.",
///      "type": "string",
///      "enum": [
///        "Wasmtime"
///      ]
///    },
///    {
///      "description": "Wasmer 2.x VM.",
///      "type": "string",
///      "enum": [
///        "Wasmer2"
///      ]
///    },
///    {
///      "description": "NearVM.",
///      "type": "string",
///      "enum": [
///        "NearVm"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum VmKind {
    ///Wasmer 0.17.x VM. Gone now.
    Wasmer0,
    ///Wasmtime VM.
    Wasmtime,
    ///Wasmer 2.x VM.
    Wasmer2,
    ///NearVM.
    NearVm,
}
impl ::std::fmt::Display for VmKind {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Wasmer0 => f.write_str("Wasmer0"),
            Self::Wasmtime => f.write_str("Wasmtime"),
            Self::Wasmer2 => f.write_str("Wasmer2"),
            Self::NearVm => f.write_str("NearVm"),
        }
    }
}
impl ::std::str::FromStr for VmKind {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Wasmer0" => Ok(Self::Wasmer0),
            "Wasmtime" => Ok(Self::Wasmtime),
            "Wasmer2" => Ok(Self::Wasmer2),
            "NearVm" => Ok(Self::NearVm),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VmKind {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///A kind of a trap happened during execution of a binary
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A kind of a trap happened during execution of a
/// binary",
///  "oneOf": [
///    {
///      "description": "An `unreachable` opcode was executed.",
///      "type": "string",
///      "enum": [
///        "Unreachable"
///      ]
///    },
///    {
///      "description": "Call indirect incorrect signature trap.",
///      "type": "string",
///      "enum": [
///        "IncorrectCallIndirectSignature"
///      ]
///    },
///    {
///      "description": "Memory out of bounds trap.",
///      "type": "string",
///      "enum": [
///        "MemoryOutOfBounds"
///      ]
///    },
///    {
///      "description": "Call indirect out of bounds trap.",
///      "type": "string",
///      "enum": [
///        "CallIndirectOOB"
///      ]
///    },
///    {
///      "description": "An arithmetic exception, e.g. divided by zero.",
///      "type": "string",
///      "enum": [
///        "IllegalArithmetic"
///      ]
///    },
///    {
///      "description": "Misaligned atomic access trap.",
///      "type": "string",
///      "enum": [
///        "MisalignedAtomicAccess"
///      ]
///    },
///    {
///      "description": "Indirect call to null.",
///      "type": "string",
///      "enum": [
///        "IndirectCallToNull"
///      ]
///    },
///    {
///      "description": "Stack overflow.",
///      "type": "string",
///      "enum": [
///        "StackOverflow"
///      ]
///    },
///    {
///      "description": "Generic trap.",
///      "type": "string",
///      "enum": [
///        "GenericTrap"
///      ]
///    }
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WasmTrap {
    ///An `unreachable` opcode was executed.
    Unreachable,
    ///Call indirect incorrect signature trap.
    IncorrectCallIndirectSignature,
    ///Memory out of bounds trap.
    MemoryOutOfBounds,
    ///Call indirect out of bounds trap.
    #[serde(rename = "CallIndirectOOB")]
    CallIndirectOob,
    ///An arithmetic exception, e.g. divided by zero.
    IllegalArithmetic,
    ///Misaligned atomic access trap.
    MisalignedAtomicAccess,
    ///Indirect call to null.
    IndirectCallToNull,
    ///Stack overflow.
    StackOverflow,
    ///Generic trap.
    GenericTrap,
}
impl ::std::fmt::Display for WasmTrap {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Unreachable => f.write_str("Unreachable"),
            Self::IncorrectCallIndirectSignature => f.write_str("IncorrectCallIndirectSignature"),
            Self::MemoryOutOfBounds => f.write_str("MemoryOutOfBounds"),
            Self::CallIndirectOob => f.write_str("CallIndirectOOB"),
            Self::IllegalArithmetic => f.write_str("IllegalArithmetic"),
            Self::MisalignedAtomicAccess => f.write_str("MisalignedAtomicAccess"),
            Self::IndirectCallToNull => f.write_str("IndirectCallToNull"),
            Self::StackOverflow => f.write_str("StackOverflow"),
            Self::GenericTrap => f.write_str("GenericTrap"),
        }
    }
}
impl ::std::str::FromStr for WasmTrap {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Unreachable" => Ok(Self::Unreachable),
            "IncorrectCallIndirectSignature" => Ok(Self::IncorrectCallIndirectSignature),
            "MemoryOutOfBounds" => Ok(Self::MemoryOutOfBounds),
            "CallIndirectOOB" => Ok(Self::CallIndirectOob),
            "IllegalArithmetic" => Ok(Self::IllegalArithmetic),
            "MisalignedAtomicAccess" => Ok(Self::MisalignedAtomicAccess),
            "IndirectCallToNull" => Ok(Self::IndirectCallToNull),
            "StackOverflow" => Ok(Self::StackOverflow),
            "GenericTrap" => Ok(Self::GenericTrap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WasmTrap {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Configuration specific to ChunkStateWitness.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration specific to ChunkStateWitness.",
///  "type": "object",
///  "properties": {
///    "combined_transactions_size_limit": {
///      "description": "Maximum size of transactions contained inside
/// ChunkStateWitness.\n\nA witness contains transactions from both the
/// previous chunk and the current one.\nThis parameter limits the sum of
/// sizes of transactions from both of those chunks.",
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "main_storage_proof_size_soft_limit": {
///      "description": "Size limit for storage proof generated while
/// executing receipts in a chunk.\nAfter this limit is reached we defer
/// execution of any new receipts.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "new_transactions_validation_state_size_soft_limit": {
///      "description": "Soft size limit of storage proof used to validate
/// new transactions in ChunkStateWitness.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WitnessConfigView {
    ///Maximum size of transactions contained inside ChunkStateWitness.
    ///
    ///A witness contains transactions from both the previous chunk and the
    /// current one. This parameter limits the sum of sizes of
    /// transactions from both of those chunks.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub combined_transactions_size_limit: ::std::option::Option<u32>,
    ///Size limit for storage proof generated while executing receipts in a
    /// chunk. After this limit is reached we defer execution of any
    /// new receipts.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub main_storage_proof_size_soft_limit: ::std::option::Option<u64>,
    ///Soft size limit of storage proof used to validate new transactions
    /// in ChunkStateWitness.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub new_transactions_validation_state_size_soft_limit: ::std::option::Option<u64>,
}
impl ::std::default::Default for WitnessConfigView {
    fn default() -> Self {
        Self {
            combined_transactions_size_limit: Default::default(),
            main_storage_proof_size_soft_limit: Default::default(),
            new_transactions_validation_state_size_soft_limit: Default::default(),
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn block_header_view_rent_paid() -> super::NearToken {
        super::NearToken::from_yoctonear(0)
    }
    pub(super) fn block_header_view_validator_reward() -> super::NearToken {
        super::NearToken::from_yoctonear(0)
    }
    pub(super) fn chunk_header_view_rent_paid() -> super::NearToken {
        super::NearToken::from_yoctonear(0)
    }
    pub(super) fn chunk_header_view_validator_reward() -> super::NearToken {
        super::NearToken::from_yoctonear(0)
    }
    pub(super) fn cloud_archival_writer_config_polling_interval(
    ) -> super::DurationAsStdSchemaProvider {
        super::DurationAsStdSchemaProvider {
            nanos: 0_i32,
            secs: 1_i64,
        }
    }
    pub(super) fn execution_outcome_view_metadata() -> super::ExecutionMetadataView {
        super::ExecutionMetadataView {
            gas_profile: Default::default(),
            version: 1_u32,
        }
    }
    pub(super) fn gc_config_gc_step_period() -> super::DurationAsStdSchemaProvider {
        super::DurationAsStdSchemaProvider {
            nanos: 500000000_i32,
            secs: 0_i64,
        }
    }
    pub(super) fn genesis_config_minimum_stake_ratio() -> [i32; 2usize] {
        [1_i32, 6250_i32]
    }
    pub(super) fn genesis_config_online_max_threshold() -> [i32; 2usize] {
        [99_i32, 100_i32]
    }
    pub(super) fn genesis_config_online_min_threshold() -> [i32; 2usize] {
        [9_i32, 10_i32]
    }
    pub(super) fn genesis_config_protocol_upgrade_stake_threshold() -> [i32; 2usize] {
        [4_i32, 5_i32]
    }
    pub(super) fn genesis_config_shard_layout() -> super::ShardLayout {
        super::ShardLayout::V2(super::ShardLayoutV2 {
            boundary_accounts: vec![],
            id_to_index_map: [("0".to_string(), 0_u32)].into_iter().collect(),
            index_to_id_map: [("0".to_string(), super::ShardId(0_u64))]
                .into_iter()
                .collect(),
            shard_ids: vec![super::ShardId(0_u64)],
            shards_parent_map: Default::default(),
            shards_split_map: Default::default(),
            version: 0_u32,
        })
    }
    pub(super) fn limit_config_account_id_validity_rules_version(
    ) -> super::AccountIdValidityRulesVersion {
        super::AccountIdValidityRulesVersion(0_u8)
    }
    pub(super) fn rpc_send_transaction_request_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
    pub(super) fn rpc_transaction_status_request_variant0_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
    pub(super) fn rpc_transaction_status_request_variant1_wait_until() -> super::TxExecutionStatus {
        super::TxExecutionStatus::ExecutedOptimistic
    }
    pub(super) fn runtime_config_view_dynamic_resharding_config(
    ) -> super::DynamicReshardingConfigView {
        super::DynamicReshardingConfigView {
            max_number_of_shards: 999999999999999_u64,
            memory_usage_threshold: 999999999999999_u64,
            min_child_memory_usage: 999999999999999_u64,
            min_epochs_between_resharding: 999999999999999_u64,
        }
    }
}
