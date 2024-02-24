use std::fs::File;
use std::io::{ErrorKind, Read, self};

fn read_user_name_from_file() -> Result<String, io::Error> {
    let user_name_file = File::open("username.txt");
    let mut user_name_file_result = match user_name_file {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut user_name = String::new();
    match user_name_file_result.read_to_string(&mut user_name) {
        Ok(_) => Ok(user_name),
        Err(e) => Err(e)
    }
}

fn read_user_name_from_file_shorter() -> Result<String, io::Error> {
    let mut user_name_file = File::open("user_name.txt")?;  // if error occurs, the error will be returned
    let mut user_name = String::new();
    user_name_file.read_to_string(&mut user_name)?;
    Ok(user_name)
}


fn main() {
    // handel all possible scenarios
    let greeting_file_result = File::open("hello.txt");  // return Result, so will not panic
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem createing the file {:?}", e),
            }
            other_error => {
                panic!("Problem create the file {:?}", other_error)
            }
        }
    };
    
    // unwrap
    let greeting_file = File::open("hello.txt").unwrap();  // if value is OK, will return value in side, otherwise panic
    let greeting_file = File::open("file.txt").expect("can not open the file"); // work the same way as unwrap but can return customized result
}
