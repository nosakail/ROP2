
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email_address: String,
}


pub fn find_by_id(user_id: &str) -> Option<User> {
    
    match user_id {
        "1" => Some(User {
            id: "1".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email_address: "john@example.com".to_string(),
        }),
        "2" => Some(User {
            id: "2".to_string(),
            first_name: "Jane".to_string(),
            last_name: "Doe".to_string(),
            email_address: "jane@example.com".to_string(),
        }),
        _ => None,
    }
}