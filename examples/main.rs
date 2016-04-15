extern crate tinyfiledialogs;

fn main() {
    
    let choice = tinyfiledialogs::message_box("hello", "yes or no?", "yesno", "question", 0);

    let user_input: String;
    match tinyfiledialogs::input_box("Enter user name", "Username:", "") {
        Some(input) => user_input = input,
        None => user_input = "null".to_string(),
    }

    let save_file: String;
    match tinyfiledialogs::save_file_dialog("Save", "password.txt", 1, "test.png", "") {
        Some(file) => save_file = file,
        None => save_file = "null".to_string(),
    }

    let open_file: String;
    match tinyfiledialogs::open_file_dialog("Open", "password.txt", 0, "", "", 0) {
        Some(file) => open_file = file,
        None => open_file = "null".to_string(),
    }

    let folder: String;
    match tinyfiledialogs::select_folder_dialog("Select folder", "") {
        Some(result) => folder = result,
        None => folder = "null".to_string(),
    }

    let default_rgb: [u8; 3]  = [1, 0, 0];
    let result_rgb:  [u8; 3]  = [1, 0, 0];

    let color: String;
    match tinyfiledialogs::color_chooser_dialog("Choose a Color", "FF0000", default_rgb, result_rgb) {
        Some(result) => color = result,
        None => color = "null".to_string(),
    }

    println!("Choice {}", choice);
    println!("User input {}", user_input);
    println!("Save file {}", save_file);
    println!("Open file {}", open_file);
    println!("folder {}", folder);
    println!("color {}", color);
}
