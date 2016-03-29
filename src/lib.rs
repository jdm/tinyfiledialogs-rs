extern crate libc;
use libc::{c_char, c_uchar, c_int};
use std::ffi::{CStr, CString};

extern {
    fn tinyfd_messageBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDialogType: *const c_char,
        aIconType: *const c_char,
        aDefaultButton: c_int) -> c_int;

    fn tinyfd_inputBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDefaultInput: *const c_char) -> *const c_char;

    fn tinyfd_saveFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char) -> *const c_char;

    fn tinyfd_openFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char,
        aAllowMultipleSelects: c_int) -> *const c_char;

    fn tinyfd_selectFolderDialog (
        aTitle: *const c_char,
        aDefaultPath: *const c_char) -> *const c_char;

    fn tinyfd_colorChooser (
        aTitle: *const c_char,
        aDefaultHexRGB: *const c_char,
        aDefaultRGB: *const c_uchar,
        aoResultRGB: *const c_uchar) -> *const c_char;
}

fn tfd_message_box(title: &CStr, message: &CStr,
               dialog_type: &CStr,
               icon: &CStr, button: c_int) -> c_int {

    unsafe {
        let result: c_int = tinyfd_messageBox(
            title.as_ptr(),
            message.as_ptr(),
            dialog_type.as_ptr(),
            icon.as_ptr(),
            button);

        result
    }
}

fn tfd_input_box(title: &CStr,
             message: &CStr,
             default: &CStr) -> *const c_char {

    unsafe {
        let result: *const c_char = tinyfd_inputBox(title.as_ptr(), message.as_ptr(), default.as_ptr());
        
        result
    }
}

fn tfd_save_file_dialog(title: &CStr, path: &CStr,
                        num_patterns: c_int,
                        filter_pattern_one: &CStr, des: &CStr) -> *const c_char {
    let filter_patterns : [&CStr; 1] = [filter_pattern_one];
    let ptr_filter_patterns = filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        let result: *const c_char = tinyfd_saveFileDialog(
            title.as_ptr(),
            path.as_ptr(),
            num_patterns,
            ptr_filter_patterns.as_ptr(),
            des.as_ptr());

        result
    }
}

fn tfd_open_file_dialog(title: &CStr, path: &CStr,
                        num_patterns: c_int, filter_pattern_one: &CStr,
                        des: &CStr, multi_select: c_int) -> *const c_char {
    let filter_patterns : [&CStr; 1] = [filter_pattern_one];
    let ptr_filter_patterns = filter_patterns.iter().map(|c| c.as_ptr()).collect::<Vec<*const c_char>>();

    unsafe {
        let result: *const c_char = tinyfd_openFileDialog(
            title.as_ptr(),
            path.as_ptr(),
            num_patterns,
            ptr_filter_patterns.as_ptr(),
            des.as_ptr(),
            multi_select);
        
        result
    }
}

fn tfd_select_folder_dialog(title: &CStr, path: &CStr) -> *const c_char {
    unsafe {
        let result: *const c_char = tinyfd_selectFolderDialog(title.as_ptr(), path.as_ptr());
        
        result
    }
}

fn tfd_color_chooser_dialog(title: &CStr, default_hex: &CStr,
                        default_rgb: &[c_uchar ; 3], result_rgb: &[c_uchar ; 3]) -> *const c_char {
    unsafe {
        let result: *const c_char = tinyfd_colorChooser(title.as_ptr(), default_hex.as_ptr(), default_rgb.as_ptr(), result_rgb.as_ptr());
        
        result
    }
}

pub fn message_box(title: &str, message: &str, box_type: &str, icon: &str, button: i32) -> i32 { 
    let message_box_title          = CString::new(title).unwrap();
    let message_box_message        = CString::new(message).unwrap();
    let message_box_type           = CString::new(box_type).unwrap();
    let message_box_icon           = CString::new(icon).unwrap();
    let message_box_button : c_int = button;

    let c_response = tfd_message_box(
                       &message_box_title,
                       &message_box_message,
                       &message_box_type,
                       &message_box_icon,
                       message_box_button);

    c_response
}

pub fn input_box(title: &str, message: &str, default: &str) -> Option<String> {
    let input_box_title   = CString::new(title).unwrap();
    let input_box_message = CString::new(message).unwrap();
    let input_box_default = CString::new(default).unwrap();

    let c_input = tfd_input_box(&input_box_title, &input_box_message, &input_box_default);

    unsafe {
        if !c_input.is_null() {
            Some(CStr::from_ptr(c_input).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

pub fn save_file_dialog(title: &str, path: &str, num_patterns: i32, filter_pattern: &str, description: &str) -> Option<String> {
    let save_dialog_title                = CString::new(title).unwrap();
    let save_dialog_path                 = CString::new(path).unwrap();
    let save_dialog_num_patterns : c_int = num_patterns;
    let save_dialog_filter_pattern_one   = CString::new(filter_pattern).unwrap();
    let save_dialog_des                  = CString::new(description).unwrap();

    let c_file_name = tfd_save_file_dialog(
                        &save_dialog_title,
                        &save_dialog_path,
                        save_dialog_num_patterns,
                        &save_dialog_filter_pattern_one,
                        &save_dialog_des);

    unsafe {
        if !c_file_name.is_null() {
            Some(CStr::from_ptr(c_file_name).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

pub fn open_file_dialog(title: &str, path: &str, num_patterns: i32, filter_pattern: &str, description: &str, multi: i32) -> Option<String> {
    let open_dialog_title                = CString::new(title).unwrap();
    let open_dialog_path                 = CString::new(path).unwrap();
    let open_dialog_num_patterns : c_int = num_patterns;
    let open_dialog_filter_pattern_one   = CString::new(filter_pattern).unwrap();  
    let open_dialog_des                  = CString::new(description).unwrap();
    let open_dialog_multi : c_int        = multi;

    let c_file_name = tfd_open_file_dialog(
                        &open_dialog_title,
                        &open_dialog_path,
                        open_dialog_num_patterns,
                        &open_dialog_filter_pattern_one,
                        &open_dialog_des,
                        open_dialog_multi);

    unsafe {
        if !c_file_name.is_null() {
            Some(CStr::from_ptr(c_file_name).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

pub fn select_folder_dialog(title: &str, path: &str) -> Option<String> {
    let select_folder_title = CString::new(title).unwrap();
    let select_folder_path  = CString::new(path).unwrap();

    let folder = tfd_select_folder_dialog(
                    &select_folder_title,
                    &select_folder_path);

    unsafe {
        if !folder.is_null() {
            Some(CStr::from_ptr(folder).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

pub fn color_chooser_dialog(title: &str, d_hex: &str, d_rgb: [u8; 3], r_rgb: [u8; 3]) -> Option<String> {
    let color_title                      = CString::new(title).unwrap();
    let color_default_hex                = CString::new(d_hex).unwrap();
    let color_default_rgb : [c_uchar; 3] = d_rgb;
    let color_result_rgb  : [c_uchar; 3] = r_rgb;

    let result = tfd_color_chooser_dialog(
                    &color_title,
                    &color_default_hex,
                    &color_default_rgb,
                    &color_result_rgb);
    
    unsafe {
        if !result.is_null() {
            Some(CStr::from_ptr(result).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}
