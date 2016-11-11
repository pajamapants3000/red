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

use parse::*;
use buf::*;
//use error::*;
use io::*;
use command::Operations;

//use io::FileMode;

// }}}
// *** Constants *** {{{
const DEFAULT_MODE: EditorMode = EditorMode::Command;
const DEFAULT_HELP: bool = false;
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
pub struct EditorState {
    mode: EditorMode,
    help: bool,
}
pub enum EditorMode {
    Command,
    Insert,
//    Replace { line: usize },
//    View,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
fn main() {// {{{
    let mut buffer: Buffer;
    // initialize editor state
    let mut state = EditorState { mode: DEFAULT_MODE, help: DEFAULT_HELP };
    // Construct operations hashmap
    let operations = Operations::new();
    // quick'n''dirty - will process one by one later; clap?
    // No! Invocation for this program is very simple: manual processing
    let args: Vec<String> = env::args().collect();

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
    loop {                          // loop until user calls quit operation
        input.clear();
        input = get_input( input, &state );
        match state.mode {
            EditorMode::Command => {
                let command: Command;
                match parse_command( &input, &buffer, &state ) {
                    Ok(x) => {
                        command = x;
                    }
                    Err(e) => {
                        print_help( &state, &format!( "main: {:?}", e ));
                        continue;
                    },
                }
                let opchar = command.operation;
                match operations.execute( &mut buffer, &mut state, command ) {
                    Ok( () ) => {},
                    Err(_) => {
                        print_help( &state,
                            &format!( "operation failed: {}", opchar ));
                    },
                }
            },
            EditorMode::Insert => {
                if input == ".".to_string() {
                    state.mode = EditorMode::Command;
                } else {
                    buffer.insert_here( &input );
                    state.mode = EditorMode::Insert;
                }
            },
        }
    }
}// }}}

/// Print help, warnings, other output depending on setting
pub fn print_help( state: &EditorState, output: &str ) {// {{{
    if state.help {
        println!( "{}", output );
    }
}// }}}

// ^^^ Functions ^^^ }}}
#[cfg(test)]
mod tests {

}

