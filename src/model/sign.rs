use crate::model::Requirement;

pub struct SignRequest {
    pub personal_number: Option<String>,
    pub end_user_ip: String,
    pub user_visible_data: String,
    pub user_non_visible_data: Option<String>,
    pub user_visible_data_format: Option<UserVisibleDataFormat>,
    pub requirement: Option<Requirement>,
}

pub struct SignResponse {
    pub auto_start_token: String,
    pub order_ref: String,
    pub qr_start_token: String,
    pub qr_start_secret: String,
}

pub enum UserVisibleDataFormat {
    SimpleMarkdownV1,
}
