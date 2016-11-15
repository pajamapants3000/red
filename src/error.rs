/*
 * File   : error.rs
 * Purpose: structures and routines related to error processing
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016
 */

// Bring in to namespace {{{
use std::io;
// }}}

// *** Data Structures *** {{{
#[derive(Debug)]
pub enum RedError {
    FileOpen(io::Error),
    FileRename(io::Error),
    FileWrite(io::Error),
//    FileRemove(io::Error),
    FileCopy(io::Error),
    FileExist(io::Error),
//    FileClose(io::Error),
    SetLineOutOfBounds{ line_num: usize },
    GetLineOutOfBounds{ line_num: usize },
//    ParseCommand,
    OpCharIndex,
    AddressSyntax{ address: String },
    ParameterSyntax{ parameter: String },
    InvalidOperation{ operation: char },
    NoDestruct,
    CriticalError(String),
    Quit,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Return error code for given error type {{{
///
pub fn error_code( _error: RedError ) -> u32 {
    match _error {
        RedError::FileOpen(_) => 280,
        RedError::FileRename(_) => 281,
        RedError::FileWrite(_) => 282,
//        RedError::FileRemove(_) => 283,
        RedError::FileCopy(_) => 284,
        RedError::FileExist(_) => 285,
//        RedError::FileClose(_) => 286,
        RedError::SetLineOutOfBounds{ line_num: _ } => 290,
        RedError::GetLineOutOfBounds{ line_num: _ } => 291,
//        RedError::ParseCommand => 300,
        RedError::OpCharIndex => 301,
        RedError::AddressSyntax{ address: _} => 302,
        RedError::ParameterSyntax{ parameter: _} => 303,
        RedError::InvalidOperation{ operation: _ } => 304,
        RedError::NoDestruct => 305,
        RedError::CriticalError(_) => 99,
        RedError::Quit => 0,
    }
}
//}}}

// ^^^ Functions ^^^ }}}

