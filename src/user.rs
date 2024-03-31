use crate::send_content_by_email::SendEmailError;
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}


pub fn find_by_id(user_id: &str) -> Result<User, SendEmailError> {
    match user_id {
        "1" => Ok(User {
            id: "1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email_address: "john@example.com".to_string(),
        }),
        "2" => Ok(User {
            id: "2".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@example.com".to_string(),
        }),
        _ => Err(SendEmailError::UserNotFoundError),
    }
}
