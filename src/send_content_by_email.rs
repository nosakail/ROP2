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
