/*
 * File   : main.rs
 * Purpose: reimplementation of the classic `ed` in Rust
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/16/2016
 */

//! A re-implementation of the classic `ed` program in Rust
//!
//! Current functionality will be to simply open the file
//! passed on invocation, and allow the user to execute print
//! commands. These commands will output specified lines from
//! the opened file.

// Bring in to namespace {{{
//extern crate clap;
extern crate chrono;
extern crate regex;
#[macro_use]
extern crate lazy_static;

mod io;
//mod parse;
mod error;
mod buf;

use std::env;
use buf::{Buffer, BufferInput};

//use io::FileMode;

// }}}
// *** Constants *** {{{

// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{

fn main() {// {{{
    // quick'n''dirty - will process one by one later; clap?
    let args: Vec<String> = env::args().collect();

    // take as direct arg; will later be arg to flag
    let file_name = args[1].to_string();
    let mut buffer = Buffer::new(BufferInput::File(file_name));

    println!("file_name: {}", buffer.get_file_name().unwrap_or("") );

    quit( &mut buffer )

}// }}}

/// Exit program
///
/// Make sure all buffers have been saved
/// Delete all temprary storage
fn quit( buffer: &mut Buffer ) {
    if buffer.is_modified() {
        println!("file changed since last write");
    }
    buffer.destruct().expect("Failed to deconstruct buffer");
    std::process::exit( error::error_code(
            error::RedError::SetLineOutOfBounds ) as i32);
}

// ^^^ Functions ^^^ }}}

#[cfg(test)]
mod tests {

}

