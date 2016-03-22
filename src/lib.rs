extern crate libc;
use libc::{c_char, c_uchar, c_int};

extern {
    pub fn tinyfd_messageBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDialogType: *const c_char,
        aIconType: *const c_char,
        aDefaultButton: c_int) -> c_int;

    pub fn tinyfd_inputBox (
        aTitle: *const c_char ,
        aMessage: *const c_char,
        aDefaultInput: *const c_char) -> *const c_char;

    pub fn tinyfd_saveFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char) -> *const c_char;

    pub fn tinyfd_openFileDialog (
        aTitle: *const c_char,
        aDefaultPathAndFile: *const c_char,
        aNumOfFilterPatterns: c_int,
        aFilterPatterns: *const *const c_char,
        aSingleFilterDescription: *const c_char,
        aAllowMultipleSelects: c_int) -> *const c_char;

    pub fn tinyfd_selectFolderDialog (
        aTitle: *const c_char,
        aDefaultPath: *const c_char) -> *const c_char;

    pub fn tinyfd_colorChooser (
        aTitle: *const c_char,
        aDefaultHexRGB: *const c_char,
        aDefaultRGB: *const c_uchar,
        aoResultRGB: *const c_uchar) -> *const c_char;
}
