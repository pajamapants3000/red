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
    FileRemove(io::Error),
//    FileClose(io::Error),
    SetLineOutOfBounds,
//    ParseCommand,
    OpCharIndex,
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
        RedError::FileRemove(_) => 283,
//        RedError::FileClose(_) => 284,
        RedError::SetLineOutOfBounds => 290,
//        RedError::ParseCommand => 300,
        RedError::OpCharIndex => 301,
    }
}
//}}}

// ^^^ Functions ^^^ }}}

