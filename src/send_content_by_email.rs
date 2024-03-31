use std::fs;
use std::io;

use crate::user;
use crate::user::User;

#[derive(Debug)]
pub enum SendEmailError {
    IOError(io::Error),
    UserNotFoundError,
    InvalidEmailError,
    InvalidUserInfoError(String),
}
//Cette partie définit une implémentation de la trait From, qui permet de convertir un type 
//en un autre. Ici, nous convertissons std::io::Error en SendEmailError.
impl From<std::io::Error> for SendEmailError { 
    fn from(error: std::io::Error) -> Self {
//C'est la méthode associée à l'implémentation de From. Elle prend une erreur de type std::io::Error 
//en entrée et renvoie notre propre type d'erreur SendEmailError.
        SendEmailError::IOError(error)
//Cette partie crée une nouvelle instance de SendEmailError avec la variante IOError, qui contient 
//l'erreur std::io::Error d'origine. 
    }
}



type SendEmailResult<T> = Result<T, SendEmailError>;




pub fn send_content_by_email(file_path: &str, user_id: &str) -> SendEmailResult<String> {
    let data = read_file(file_path)?;
    let user_info = user::find_by_id(user_id)?;
    let validated_info = validate_user_info(&user_info)?;
    send_email(&validated_info.email_address, &validated_info.first_name, &validated_info.last_name, &data)?;
    Ok("Terminé".to_string())
}


fn read_file(file_path: &str) -> io::Result<String> {
    fs::read_to_string(file_path)
}


fn validate_user_info(user_info: &User) -> SendEmailResult<&User> {
    if user_info.email_address.is_empty() || !user_info.email_address.contains('@') {
        return Err(SendEmailError::InvalidEmailError);
    }

    if user_info.first_name.is_empty() {
        return Err(SendEmailError::InvalidUserInfoError("Prénom vide".to_string()));
    }

    if user_info.last_name.is_empty() {
        return Err(SendEmailError::InvalidUserInfoError("Nom vide".to_string()));
    }

    if let Err(_) = user_info.id.parse::<i32>() {
        return Err(SendEmailError::InvalidUserInfoError("ID invalide".to_string()));
    }

    Ok(user_info)
}





fn send_email(email_address: &str, first_name: &str, last_name: &str, _data: &str) -> SendEmailResult<()> {
    println!("E-mail envoyé : Adresse e-mail : {}, Prénom : {}, Nom : {}", email_address, first_name, last_name);
    Ok(())
}
