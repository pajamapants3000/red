/*)
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
mod command;

use std::env;
use std::collections::hash_map::{HashMap, Entry};

use parse::*;
use buf::*;
use error::*;
use io::*;
use command::Operations;

//use io::FileMode;

// }}}
// *** Constants *** {{{
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
enum EditorMode {
    Command,
    Insert  { address: usize },
//    Replace { line: usize },
//    View,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
fn main() {// {{{
    let mut buffer: Buffer;
    let mut mode: EditorMode = EditorMode::Command;
    // quick'n''dirty - will process one by one later; clap?
    // No! Invocation for this program is very simple: manual processing
    let args: Vec<String> = env::args().collect();

    // Construct operations hashmap
    let operations = Operations::new();

    // take as direct arg; will later be arg to flag
    if args.len() > 1 {
        let content = args[1].to_string();
        if &content[0..1] == "@" {  // process command
            buffer = Buffer::new(BufferInput::Command(content[1..].to_string()));
        } else {                    // process file
            buffer = Buffer::new(BufferInput::File(content));
        }
    } else {    // new file
        buffer = Buffer::new(BufferInput::None);
        buffer.set_file_name( "untitled" );
    }
    /* Print buffer content for testing
    {
        let lines = buffer.lines_iterator();
        for line in lines {
            println!( "{:?}", line );
        }
    }
    */

    let mut input: String = String::new();
    loop {
        input = get_input( input );
        match mode {
            EditorMode::Command => {
                let command = parse_command( &input, &buffer )
                        .expect("main: failed to parse command");
                operations.execute( &mut buffer, command );
            }// {{{
            EditorMode::Insert { address: x } => {
            }
        }
        input.clear();
    }

    let quit_command = Command { address_initial: 0, address_final: 0,
            operation: 'q', parameters: "" };
    operations.execute( &mut buffer, quit_command );

}// }}}

// ^^^ Functions ^^^ }}}
#[cfg(test)]
mod tests {

}

