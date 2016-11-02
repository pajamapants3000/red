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
//    FileClose(io::Error),
    SetLineOutOfBounds,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Return error code for given error type {{{
///
pub fn error_code( _error: RedError ) -> u32 {
    match _error {
        RedError::FileOpen(_) => 280,
        RedError::FileRename(_) => 281,
//        RedError::FileClose(_) => 282,
        RedError::SetLineOutOfBounds => 290,
    }
}
//}}}

// ^^^ Functions ^^^ }}}

