use std::fs;
use std::io;

use crate::user;
use crate::user::User;

#[derive(Debug)]
pub enum SendEmailError {
    IOError(io::Error),
    UserNotFoundError,
    InvalidEmailError,
}

impl From<std::io::Error> for SendEmailError {
    fn from(error: std::io::Error) -> Self {
        SendEmailError::IOError(error)
    }
}

type SendEmailResult<T> = Result<T, SendEmailError>;





pub fn send_content_by_email(file_path: &str, user_id: &str) -> SendEmailResult<String> {
    let data = read_file(file_path)?;
    let user_info = match user::find_by_id(user_id) {
        Some(user) => user,
        None => return Err(SendEmailError::UserNotFoundError),
    };
    let validated_info = validate_user_info(&user_info)?;
    send_email(&validated_info.email_address, &validated_info.first_name, &validated_info.last_name, &data)?;
    Ok("Terminé".to_string())
}


fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}


fn get_user_info(user_id: &str) -> SendEmailResult<User> {

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



fn validate_user_info(user_info: &User) -> SendEmailResult<&User> {
    
    if user_info.email_address.contains('@') {
        Ok(user_info)
    } else {
        Err(SendEmailError::InvalidEmailError)
    }
}



fn send_email(email_address: &str, first_name: &str, last_name: &str, _data: &str) -> SendEmailResult<()> {
    println!("E-mail envoyé : Adresse e-mail : {}, Prénom : {}, Nom : {}", email_address, first_name, last_name);
    Ok(())
}
