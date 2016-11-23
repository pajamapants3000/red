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
use std::fmt::{Debug, Display};

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
const DEFAULT_MESSAGES: bool = true;
const DEFAULT_PROMPT: &'static str = "%";
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
/// Contain state values for the program during execution
///
/// TODO: include buffer and command structures?
#[derive(Clone)]
pub struct EditorState {
    mode: EditorMode,
    help: bool,
    messages: bool,
    prompt: String,
    buffer: Buffer,
    source: String,
}
#[derive(Clone)]
pub enum EditorMode {
    Command,
    Insert,
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
fn main() {// {{{
    // initialize buffer
    let buffer = Buffer::new( BufferInput::None )
        .expect( "main: failed to create initial empty buffer" );
    // initialize editor state
    let mut state = EditorState { mode: DEFAULT_MODE, help: DEFAULT_HELP,
            messages: DEFAULT_MESSAGES, prompt: DEFAULT_PROMPT.to_string(),
            buffer: buffer, source: String::new() };
    // Construct operations hashmap
    let operations = Operations::new();
    // Collect invocation arguments
    let args: Vec<String> = env::args().collect();
    parse_invocation( args, &mut state );
    if state.source.len() > 0 {
        // generate and execute edit operation for requested file or command
        let command = Command{ address_initial: 0, address_final: 0,
                operation: 'e', parameters: &state.source.clone() };
        operations.execute( &mut state, command )
            .expect( "main: failed to initialize buffer" );
    } else {
        state.buffer.set_file_name( "untitled" )
            .expect("main: failed to set file name");
    }
    let mut input: String = String::new();
    loop {                          // loop until user calls quit operation
        input.clear();
        /*
        input = get_input( input, &state );
        */
        match get_input( input, &state ) {
            Ok(  _input ) => input = _input,
            Err( _error ) => {
                print_help_debug( &state, _error );
                input = String::new();
                continue;
            },
        }
        match state.mode {
            EditorMode::Command => {
                if input == "" {
                    continue;   // set default command, e.g. print cur addr?
                }
                let command: Command;
                match parse_command( &input, &state ) {
                    Ok(x) => {
                        command = x;
                    }
                    Err(e) => {
                        print_help( &state, &format!( "main: {:?}", e ));
                        continue;
                    },
                }
                let opchar = command.operation;
                match operations.execute( &mut state, command ) {
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
                    state.buffer.append_here( &input );
                    state.mode = EditorMode::Insert;
                }
            },
        }
    }
}// }}}

/// Print standard messages
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_msg<T: Display>( state: &EditorState, output: T ) {// {{{
    if state.messages {
        println!( "{}", output );
    }
}// }}}

/// Print help, warnings, other output depending on setting
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_help<T: Display>( state: &EditorState, output: T ) {// {{{
    if state.help {
        println!( "{}", output );
    } else {
        println!( "?" );
    }
}// }}}

/// Print help, warnings, other output depending on setting
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_help_debug<T: Debug>( state: &EditorState, output: T ) {// {{{
    if state.help {
        println!( "{:?}", output );
    } else {
        println!( "?" );
    }
}// }}}

// ^^^ Functions ^^^ }}}
#[cfg(test)]
mod tests {

}

