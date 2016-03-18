extern crate tinyfiledialogs;
extern crate libc;

use std::ffi::{CString, CStr};
use libc::c_int;
use libc::c_uchar;


fn message_box(title: &CStr, message: &CStr, dialog_type: &CStr, icon: &CStr, button: c_int) {

    unsafe{

        tinyfiledialogs::tinyfd_messageBox(
            title.as_ptr(),
            message.as_ptr(),
            dialog_type.as_ptr(),
            icon.as_ptr(),
            button);
    }
}
fn input_box(title: &CStr, message: &CStr, default: &CStr) {

    unsafe{

        tinyfiledialogs::tinyfd_inputBox(title.as_ptr(),message.as_ptr(),default.as_ptr());
    }
}
fn save_file_dialog(title: &CStr, path: &CStr, num_patterns: c_int, filter_patterns: &CStr, des: &CStr) {

    unsafe{

        tinyfiledialogs::tinyfd_saveFileDialog(
            title.as_ptr(),
            path.as_ptr(),
            num_patterns,
            filter_patterns.as_ptr(),
            des.as_ptr());
    }
}
fn open_file_dialog(title: &CStr, path: &CStr, num_patterns: c_int, filter_patterns: &CStr, des: &CStr, multi_select: c_int) {

    unsafe{

        tinyfiledialogs::tinyfd_openFileDialog(
            title.as_ptr(),
            path.as_ptr(),
            num_patterns,
            filter_patterns.as_ptr(),
            des.as_ptr(),
            multi_select);
    }
}
fn select_folder_dialog(title: &CStr, path: &CStr) {

    unsafe{

        tinyfiledialogs::tinyfd_selectFolderDialog(title.as_ptr(),path.as_ptr());
    }

}
fn color_chooser_dialog(title: &CStr, default_hex: &CStr, default_RGB: &[c_uchar ; 3], result_RGB: &[c_uchar ; 3]) {

    unsafe{

        tinyfiledialogs::tinyfd_colorChooser(title.as_ptr(),default_hex.as_ptr(),default_RGB,result_RGB);
    }

}
fn main()
{
    let message_box_title          = CString::new("Title").unwrap();
    let message_box_message        = CString::new("Hello, world!").unwrap();
    let message_box_type           = CString::new("yes").unwrap();
    let message_box_icon           = CString::new("yes").unwrap();
    let message_box_button : c_int = 0;

    message_box(
        &message_box_title,
        &message_box_message,
        &message_box_type,
        &message_box_icon,
        message_box_button);

    let input_box_title   = CString::new("Title").unwrap();
    let input_box_message = CString::new("Hello, world!").unwrap();
    let input_box_default = CString::new("yes").unwrap();

    input_box(&input_box_title, &input_box_message, &input_box_default);

    let save_dialog_title                = CString::new("Save this").unwrap();
    let save_dialog_path                 = CString::new("password.txt").unwrap();
    let save_dialog_num_patterns : c_int = 0;
    let save_dialog_filter_patterns      = CString::new("").unwrap();
    let save_dialog_des                  = CString::new("").unwrap();

    save_file_dialog(
        &save_dialog_title,
        &save_dialog_path,
        save_dialog_num_patterns,
        &save_dialog_filter_patterns,
        &save_dialog_des);

    let open_dialog_title                = CString::new("Open this").unwrap();
    let open_dialog_path                 = CString::new("password.txt").unwrap();
    let open_dialog_num_patterns : c_int = 0;
    let open_dialog_filter_patterns      = CString::new("").unwrap();
    let open_dialog_des                  = CString::new("").unwrap();
    let open_dialog_multi : c_int        = 0;

    open_file_dialog(
        &open_dialog_title,
        &open_dialog_path,
        open_dialog_num_patterns,
        &open_dialog_filter_patterns,
        &open_dialog_des,
        open_dialog_multi);

    let select_folder_title              = CString::new("Select Folder").unwrap();
    let select_folder_path               = CString::new("home").unwrap();

    select_folder_dialog(
        &select_folder_title,
        &select_folder_path);

    let color_title                      = CString::new("Color Choose").unwrap();
    let color_default_hex                = CString::new("Color Choose").unwrap();
    let color_default_RGB : [c_uchar ; 3] = [250,0,0];
    let color_result_RGB  : [c_uchar ; 3] = [100,0,0];

    color_chooser_dialog(
        &color_title,
        &color_default_hex,
        &color_default_RGB,
        &color_result_RGB);


}
