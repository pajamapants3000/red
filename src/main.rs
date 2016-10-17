/*
 * File   : main.rs
 * Purpose: reimplementation of the classic `ed` in Rust
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/16/2016
 * Updated: 10/16/2016
 */

//! A re-implementation of the classic `ed` program in Rust
//!
//! Current functionality will be to simply open the file
//! passed on invocation, and allow the user to execute print
//! commands. These commands will output specified lines from
//! the opened file.

// Bring in to namespace {{{
//extern crate clap;

use std::env;
use std::fs::{File, OpenOptions};
use::std::io::Error;
// }}}

// Define messages
// Some of these may be removed if builtin error descriptions work
const S_FOPEN: &'static str = "successfully opened file!";
//const E_FOPEN: &'static str = "unable to open file";

// Define file-mode flags, to be set by user options and commands
// I would prefer using 2.pow(x) but function calls not allowed in const def
//const F_EXEC: usize = 0b00000001;   // execute
const F_WRIT: u8 = 0b00000010;   // write
const F_READ: u8 = 0b00000100;   // read
const F_APPE: u8 = 0b00001000;   // append
const F_TRUN: u8 = 0b00010000;   // truncate
const F_CREA: u8 = 0b00100000;   // create
const F_CNEW: u8 = 0b01000000;   // create new
//const F_RESE: usize = 0b10000000;   // reserved

fn main() {
    // take as direct arg; will later be arg to flag
//    let file_name: &str = env::args[1].clone();

    // quick'n''dirty - will process one by one later
    let args: Vec<String> = env::args().collect();

    let file_mode: u8 = F_READ;

    let file_opened: File;

    match file_opener( &args[1], file_mode ) {
        Ok(f) => {
            file_opened = f;
            println!( "{}", S_FOPEN );
            println!( "our file is: {:?}", file_opened );
        },
        Err(e) => {
            println!( "error: {}", e );

        },
    };
    
}

/// Opens file with user-specified name in user-specified mode
///
/// Uses global definitions of mode flags in this file
///
/// Returns direct result of call to OpenOptions::new()
/// Will return an error if the file could not be opened
/// Otherwise, returns a File object
fn file_opener( name: &str, mode: u8 ) -> Result<File, Error> {

    // let's introduce OpenOptions now, though we don't need it
    // until we introduce more functionality
    OpenOptions::new()
        .read(          ( mode | F_READ ) == F_READ )
        .write(         ( mode | F_WRIT ) == F_WRIT )
        .append(        ( mode | F_APPE ) == F_APPE )
        .truncate(      ( mode | F_TRUN ) == F_TRUN )
        .create(        ( mode | F_CREA ) == F_CREA )
        .create_new(    ( mode | F_CNEW ) == F_CNEW )
        .open( name )

}

