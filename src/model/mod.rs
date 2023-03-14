pub use authenticate::*;
pub use sign::*;

pub mod authenticate;
pub mod sign;

pub struct Requirement {
    pub card_reader: Option<String>,
    pub certificate_policies: Vec<String>,
    pub issuer_cn: Option<String>,
    pub auto_start_token_required: Option<bool>,
    pub allow_fingerprint: Option<bool>,
}
