/*
 * File   : io.rs
 * Purpose: 
 * Program: red
 * About  : What does this program do?
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016
 */

// *** Bring in to namespace *** {{{
use std::fs::{File, OpenOptions};
// ^^^ Bring in to namespace ^^^ }}}
//
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
//
// *** Constants *** {{{
const S_FOPEN_MSG: &'static str = "successfully opened file!";
const LINE_CONT: &'static str = "\\\n";
const PROMPT: &'static str = "%";
const PROMPT_CONT: &'static str = ">";
// ^^^ Constants ^^^ }}}
//
// *** Data Structures *** {{{
#[derive(Default)]
pub struct FileMode {
    pub f_write:        bool,
    pub f_read:         bool,
    pub f_append:       bool,
    pub f_truncate:     bool,
    pub f_create:       bool,
    pub f_create_new:   bool,
}

struct FileCoordinate {
    line: usize,
    col: usize,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Return next occurrence of regular expression
regex_search( needle: &str, from: FileCoordinate ) -> FileCoordinate {
}

/// Opens file with user-specified name and mode {{{
///
/// Uses global definitions of mode flags in this file
///
/// Returns direct result of call to OpenOptions::new()
/// This is of type Result<File, io::Error>
pub fn file_opener( name: &str, mode: FileMode ) -> Result<File, io::Error> {

    // let's introduce OpenOptions now, though we don't need it
    // until we introduce more functionality
    OpenOptions::new()
        .read(mode.f_read)
        .write(mode.f_write)
        .append(mode.f_append)
        .truncate(mode.f_truncate)
        .create(mode.f_create)
        .create_new(mode.f_create_new)
        .open( name )
}
//}}}

// ^^^ Functions ^^^ }}}


