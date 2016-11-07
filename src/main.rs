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
extern crate rand;
#[macro_use]
extern crate lazy_static;

mod io;
mod parse;
mod error;
mod buf;

use std::env;
use buf::{Buffer, BufferInput};

//use io::FileMode;

// }}}
// *** Constants *** {{{

// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
enum EditorMode {
    Command,
    Insert  { line: usize },
    Replace { line: usize },
    View,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{

fn main() {// {{{
    let mut buffer: Buffer;
    // quick'n''dirty - will process one by one later; clap?
    let args: Vec<String> = env::args().collect();

    // take as direct arg; will later be arg to flag
    if args.len() > 1 {
        let content = args[1].to_string();
        if &content[0..1] == "@" {
            buffer = Buffer::new(BufferInput::Command(content[1..].to_string()));
        } else {
            buffer = Buffer::new(BufferInput::File(content));
        }
    } else {
        buffer = Buffer::new(BufferInput::None);
        buffer.set_file_name( "untitled" );
    }
    // Print buffer content for testing
    {
        let lines = buffer.lines_iterator();
        for line in lines {
            println!( "{:?}", line );
        }
    }

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

