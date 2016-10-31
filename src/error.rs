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

// }}}

// *** Data Structures *** {{{
pub enum RedError {
//    FileOpen,
    FileClose,
//    SetLineOutOfBounds,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Return error code for given error type {{{
///
pub fn error_code( _error: RedError ) -> u32 {
    match _error {
//        RedError::FileOpen => 280,
        RedError::FileClose => 281,
//        RedError::SetLineOutOfBounds => 290,
    }
}
//}}}

// ^^^ Functions ^^^ }}}

