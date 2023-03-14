use crate::model::Requirement;

pub struct AuthenticateRequest {
    pub personal_number: Option<String>,
    pub end_user_ip: String,
    pub requirement: Option<Requirement>,
}

pub struct AuthenticateResponse {
    pub auto_start_token: String,
    pub order_ref: String,
    pub qr_start_token: String,
    pub qr_start_secret: String,
}
