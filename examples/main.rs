extern crate tinyfiledialogs;

use tinyfiledialogs::{YesNo, MessageBoxIcon, DefaultColorValue};

fn main() {

    let choice = tinyfiledialogs::message_box_yes_no("hello", "yes or no?",
                                                     MessageBoxIcon::Question, YesNo::No);

    let user_input: String;
    match tinyfiledialogs::input_box("Enter user name", "Username:", "") {
        Some(input) => user_input = input,
        None => user_input = "null".to_string(),
    }

    let save_file: String;
    match tinyfiledialogs::save_file_dialog("Save", "password.txt") {
        Some(file) => save_file = file,
        None => save_file = "null".to_string(),
    }

    let open_file: String;
    match tinyfiledialogs::open_file_dialog("Open", "password.txt", None) {
        Some(file) => open_file = file,
        None => open_file = "null".to_string(),
    }

    let folder: String;
    match tinyfiledialogs::select_folder_dialog("Select folder", "") {
        Some(result) => folder = result,
        None => folder = "null".to_string(),
    }

    let color: String;
    match tinyfiledialogs::color_chooser_dialog("Choose a Color", DefaultColorValue::Hex("#FF0000")) {
        Some((hex_result, _rgb)) => color = hex_result,
        None => color = "null".to_string(),
    }

    let list: String;
    match tinyfiledialogs::list_dialog("Test Dialog",
                                       &["Id", "Name"],
                                       Some(&["471", "Donald Duck",
                                              "1143", "Chris P. Bacon",
                                              "6509", "Moon Doge"])) {
        Some(result) => list = result,
        None => list = "null".to_string(),
    }

    println!("Choice {:?}", choice);
    println!("User input {:?}", user_input);
    println!("Save file {:?}", save_file);
    println!("Open file {:?}", open_file);
    println!("folder {:?}", folder);
    println!("color {:?}", color);
    println!("List {:?}", list);
}
