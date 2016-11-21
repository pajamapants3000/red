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

//! A re-implementation, in Rust, of the classic `ed` program
//!
// Bring in to namespace {{{
//extern crate clap;
extern crate chrono;
extern crate regex;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate term_size;

mod io;
mod parse;
mod error;
mod buf;
mod ops;

use std::env;

use parse::*;
use buf::*;
//use error::*;
use io::*;
use ops::Operations;

//use io::FileMode;

// }}}
// *** Constants *** {{{
const DEFAULT_MODE: EditorMode = EditorMode::Command;
const DEFAULT_HELP: bool = true;
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
#[derive(Clone)]
/// Contain state values for the program during execution
///
/// TODO: include buffer and command structures?
pub struct EditorState {
    mode: EditorMode,
    help: bool,
}
#[derive(Clone)]
pub enum EditorMode {
    Command,
    Insert,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
fn main() {// {{{
    // initialize editor state
    let mut state = EditorState { mode: DEFAULT_MODE, help: DEFAULT_HELP };
    // initialize buffer
    let mut buffer = Buffer::new( BufferInput::None, &state )
        .expect( "Failed to create initial empty buffer" );
    buffer.set_file_name( "untitled" ).expect("main: failed to set file name");
    // Construct operations hashmap
    let operations = Operations::new();
    // Collect invocation arguments
    let args: Vec<String> = env::args().collect();
    // take as direct arg; will later check for additional -s, -p flags
    if args.len() > 1 {
        // generate and execute edit operation for requested file
        let command = Command{ address_initial: 0, address_final: 0,
                operation: 'e', parameters: &args[1] };
        let _ = operations.execute( &mut buffer, &mut state, command );
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
                if input == "" {
                    continue;
                }
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
                    buffer.append_here( &input );
                    state.mode = EditorMode::Insert;
                }
            },
        }
    }
}// }}}

/// Print help, warnings, other output depending on setting
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_help( state: &EditorState, output: &str ) {// {{{
    if state.help {
        println!( "{}", output );
    } else {
        println!( "?" );
    }
}// }}}

// ^^^ Functions ^^^ }}}
#[cfg(test)]
mod tests {

}

