mod send_content_by_email;
mod user;

use send_content_by_email::send_content_by_email;

fn main() {
    let file_path = "./ContentMail.txt";
    let user_id = "1";



    match send_content_by_email(file_path, user_id) {
        Ok(result) => println!("{}", result),
        Err(error) => eprintln!("Une erreur s'est produite : {:?}", error),
    }
}
