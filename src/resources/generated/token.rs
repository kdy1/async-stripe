// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, TokenId};
use crate::params::{Expand, Metadata, Object, Timestamp};
use crate::resources::{Address, BankAccount, Card, CompanyParams, PersonParams, TokenType};

/// The resource representing a Stripe "Token".
///
/// For more details see <https://stripe.com/docs/api/tokens/object>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Token {
    /// Unique identifier for the object.
    pub id: TokenId,

    pub bank_account: Box<Option<BankAccount>>,

    pub card: Box<Option<Card>>,

    /// IP address of the client that generated the token.
    pub client_ip: Box<Option<String>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Type of the token: `account`, `bank_account`, `card`, or `pii`.
    #[serde(rename = "type")]
    pub type_: TokenType,

    /// Whether this token has already been used (tokens can be used only once).
    pub used: bool,
}

impl Token {
    /// Creates a single-use token that represents a bank account’s details.
    /// This token can be used with any API method in place of a bank account dictionary.
    ///
    /// This token can be used only once, by attaching it to a [Custom account](https://stripe.com/docs/api#accounts).
    pub fn create(client: &Client, params: CreateToken<'_>) -> Response<Token> {
        client.post_form("/tokens", &params)
    }

    /// Retrieves the token with the given ID.
    pub fn retrieve(client: &Client, id: &TokenId, expand: &[&str]) -> Response<Token> {
        client.get_query(&format!("/tokens/{}", id), &Expand { expand })
    }
}

impl Object for Token {
    type Id = TokenId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "token"
    }
}

/// The parameters for `Token::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateToken<'a> {
    /// Information for the account this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Box<Option<CreateTokenAccount>>,

    /// The customer (owned by the application's account) for which to create a token.
    ///
    /// This can be used only with an [OAuth access token](https://stripe.com/docs/connect/standard-accounts) or [Stripe-Account header](https://stripe.com/docs/connect/authentication).
    /// For more details, see [Cloning Saved Payment Methods](https://stripe.com/docs/connect/cloning-saved-payment-methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// The updated CVC value this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_update: Box<Option<CreateTokenCvcUpdate>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Information for the person this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Box<Option<CreateTokenPerson>>,

    /// The PII this token will represent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii: Box<Option<CreateTokenPii>>,
}

impl<'a> CreateToken<'a> {
    pub fn new() -> Self {
        CreateToken {
            account: Default::default(),
            customer: Default::default(),
            cvc_update: Default::default(),
            expand: Default::default(),
            person: Default::default(),
            pii: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenAccount {
    pub business_type: Box<Option<CreateTokenAccountBusinessType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<CompanyParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<PersonParams>,

    pub tos_shown_and_accepted: Box<Option<bool>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenCvcUpdate {
    pub cvc: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPerson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    pub dob: Box<Option<CreateTokenPersonDob>>,

    pub documents: Box<Option<CreateTokenPersonDocuments>>,

    pub email: Box<Option<String>>,

    pub first_name: Box<Option<String>>,

    pub first_name_kana: Box<Option<String>>,

    pub first_name_kanji: Box<Option<String>>,

    pub full_name_aliases: Box<Option<Vec<String>>>,

    pub gender: Box<Option<String>>,

    pub id_number: Box<Option<String>>,

    pub last_name: Box<Option<String>>,

    pub last_name_kana: Box<Option<String>>,

    pub last_name_kanji: Box<Option<String>>,

    pub maiden_name: Box<Option<String>>,

    #[serde(default)]
    pub metadata: Metadata,

    pub nationality: Box<Option<String>>,

    pub phone: Box<Option<String>>,

    pub political_exposure: Box<Option<String>>,

    pub relationship: Box<Option<CreateTokenPersonRelationship>>,

    pub ssn_last_4: Box<Option<String>>,

    pub verification: Box<Option<PersonVerificationParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPii {
    pub id_number: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDob {
    pub day: i64,

    pub month: i64,

    pub year: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocuments {
    pub company_authorization: Box<Option<CreateTokenPersonDocumentsCompanyAuthorization>>,

    pub passport: Box<Option<CreateTokenPersonDocumentsPassport>>,

    pub visa: Box<Option<CreateTokenPersonDocumentsVisa>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonRelationship {
    pub director: Box<Option<bool>>,

    pub executive: Box<Option<bool>>,

    pub owner: Box<Option<bool>>,

    pub percent_ownership: Box<Option<f64>>,

    pub representative: Box<Option<bool>>,

    pub title: Box<Option<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationParams {
    pub additional_document: Box<Option<VerificationDocumentParams>>,

    pub document: Box<Option<VerificationDocumentParams>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsCompanyAuthorization {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsPassport {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTokenPersonDocumentsVisa {
    pub files: Box<Option<Vec<String>>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerificationDocumentParams {
    pub back: Box<Option<String>>,

    pub front: Box<Option<String>>,
}

/// An enum representing the possible values of an `CreateTokenAccount`'s `business_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateTokenAccountBusinessType {
    Company,
    GovernmentEntity,
    Individual,
    NonProfit,
}

impl CreateTokenAccountBusinessType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateTokenAccountBusinessType::Company => "company",
            CreateTokenAccountBusinessType::GovernmentEntity => "government_entity",
            CreateTokenAccountBusinessType::Individual => "individual",
            CreateTokenAccountBusinessType::NonProfit => "non_profit",
        }
    }
}

impl AsRef<str> for CreateTokenAccountBusinessType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateTokenAccountBusinessType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
