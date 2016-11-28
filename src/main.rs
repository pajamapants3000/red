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
use io::*;
//use error::*;
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
    /// enum indicating current mode: Command (aka Normal) or Insert
    mode: EditorMode,
    /// whether to show or hide informational output
    show_messages: bool,
    /// whether to show or hide help, warnings, and error messages
    show_help: bool,
    prompt: String,
    /// structure containing all text and plenty of logic for manipulating it
    buffer: Buffer,
    /// file name or command from which our initial text originates
    source: String,
    /// most recent help, warning, or error message
    last_help: String,
    /// structure containing enough information to roll back latest change
    undo: Undo,
}
impl EditorState {
    /// Initialize new editor state// {{{
    ///
    /// This will be used when program first loads, and any time we open
    /// a new file or command using e.g. the edit operation
    pub fn new( _buffer: Buffer ) -> EditorState {// {{{
        EditorState { mode: DEFAULT_MODE, show_help: DEFAULT_HELP,
            show_messages: DEFAULT_MESSAGES, prompt: DEFAULT_PROMPT.to_string(),
            buffer: _buffer, source: String::new(), last_help: String::new(),
            undo: EditorState::u_new(), }
    }// }}}
// }}}

    pub fn u_reset( &mut self ) {
        if !self.undo.is_locked {
            let mut _markers = vec!( 0; NUM_LC );
            for indx in ( 'a' as u8 ) .. ( 'z' as u8 ) + 1 {
                _markers[ ( indx as usize ) - ( 'a' as usize ) ] =
                    self.buffer.get_marked_line( indx as char );
            }
            self.undo.changes = Vec::new();
            self.undo.address = self.buffer.get_current_address();
            self.undo.markers = _markers;
        }
    }
    pub fn u_lock( &mut self ) {
        self.undo.is_locked = true;
    }
    pub fn u_unlock( &mut self ) {
        self.undo.is_locked = false;
    }
    /// Record line number of added line// {{{
    pub fn u_address_added_line( &mut self, _address: usize ) {// {{{
        self.undo.changes.push( Change::Add{ address: _address });
    }// }}}
    // }}}
    /// Record line number of added line// {{{
    pub fn u_this_added_line( &mut self ) {// {{{
        self.undo.changes.push( Change::Add{
            address: self.buffer.get_current_address() });
    }// }}}
    // }}}
    /// Record range of added lines// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [1,$] )
    pub fn u_address_added_lines( &mut self,// {{{
                                      _initial: usize, _final: usize ) {
        assert!( 0 < _initial && _initial <= self.buffer.num_lines() );
        assert!( 0 < _final && _final <= self.buffer.num_lines() );
        for _address in _initial .. _final + 1 {
            self.undo.changes.push( Change::Add{ address: _address });
        }
    }// }}}
// }}}
    /// Record line number of added line// {{{
    pub fn u_address_delete_line( &mut self, _address: usize ) {// {{{
        self.u_address_delete_lines( _address, _address );
    }// }}}
    // }}}
    /// Record line number of added line// {{{
    pub fn u_this_delete_line( &mut self ) {// {{{
        let address = self.buffer.get_current_address();
        self.u_address_delete_line( address );
    }// }}}
    // }}}
    /// Store lines to be removed, with addresses, in Undo structure// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [0,$] )
    /// Confirmed address still returns None for buffer::get_line_content ( This
    ///     should never be logically possible! )
    pub fn u_address_delete_lines( &mut self,// {{{
                                       _initial: usize, _final: usize ) {
        // can be 0: e.g. when inserting first line
        assert!( _initial <= self.buffer.num_lines() );
        assert!( _final <= self.buffer.num_lines() );
        for _address in _initial .. _final + 1 {
        // to delete range, delete same address repeatedly on _initial
            self.undo.changes.push( Change::Remove{ address: _initial,
        // however, not ACTUALLY deleting right now, so we have to increment
        // to get the correct content to save - use _address
                    content: self.buffer.get_line_content( _address )
                    .expect( &( "main::u_address_delete_lines: ".to_string() +
                                "unexpected missing line" ))
                    .to_string() });
        }
    }// }}}
    // }}}
    /// Store current address in undo// {{{
    pub fn u_store_here_address( &mut self ) {// {{{
        self.undo.address = self.buffer.get_current_address();
    }// }}}
    // }}}
    /// Store provided address in undo// {{{
    pub fn u_store_address( &mut self, address: usize ) {// {{{
        self.undo.address = address;
    }// }}}
    // }}}
    /// Obtain stored address// {{{
    pub fn u_get_stored_address( &self ) -> usize {// {{{
        self.undo.address
    }// }}}
    //}}}
    /// Get address associated with certain marker character// {{{
    pub fn u_get_marked_address( &self, ch: char ) -> usize {// {{{
        self.undo.markers[ (( ch as u8 ) - ( 'a' as u8 )) as usize ]
    }// }}}
    //}}}
    fn u_new() -> Undo {
        Undo {
            changes: Vec::new(),
            address: 0_usize,
            markers: vec!( 0; NUM_LC ),
            is_locked: false,
        }
    }
}
#[derive(Clone)]
pub enum EditorMode {
    Command,
    Insert,
}
/// Lines to add, lines to remove, to be used by the undo operation
///
/// Perhaps not the most space-efficient approach, but probably faster and
/// definitely simple; Inspired by diff;
/// This could easily be extended to infinite undo by using a Vec of these.
#[derive(Clone)]
struct Undo {
    /// Vector of changes
    ///
    /// Each change is pushed onto the stack, then if we execute undo
    /// we pop the changes off one-by-one, rolling back step-by-step
    ///
    /// Each step is either an added line - just an address, or a
    /// removed line - an address and the full line content.
    /// When executing undo, all added lines are simply deleted, and
    /// removed lines are re-inserted.
    changes: Vec<Change>,
    /// current address before the change
    address: usize,
    /// markers before the change
    markers: Vec<usize>,
    /// lock the structure for complex operations
    is_locked: bool
}
#[derive(Clone)]
enum Change {
    Add { address: usize },
    Remove { address: usize, content: String },
}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
fn main() {// {{{
    // initialize buffer
    let _buffer = Buffer::new( BufferInput::None )
        .expect( "main: failed to create initial empty buffer" );
    // Construct operations hashmap
    let operations: Operations = Operations::new();
    // initialize editor state
    let mut state = EditorState::new( _buffer );
    // Collect invocation arguments
    let args: Vec<String> = env::args().collect();
    parse_invocation( args, &mut state );
    if state.source.len() > 0 {
        // generate and execute edit operation for requested file or command
        let command = Command{ address_initial: 0, address_final: 0,
                operation: 'e', parameters: &state.source.clone(),
                operations: &operations };
        operations.execute( &mut state, command )
            .expect( "main: failed to initialize buffer" );
    } else {
        state.buffer.set_file( "untitled" )
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
                    continue;   // set default command? e.g. print cur addr?
                }
                let command: Command;
                match parse_command( &input, &state, &operations ) {
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
                    Err(e) => {
                        print_help( &state,
                            &format!( "operation `{}` failed: {:?}",
                                                    opchar, e ));
                    },
                }
            },
            EditorMode::Insert => {
                if input == ".".to_string() {
                    state.mode = EditorMode::Command;
                } else {
                    state.buffer.append_here( &input );
                    state.u_this_added_line();
                    state.mode = EditorMode::Insert;
                }
            },
        }
        match state.mode {
            EditorMode::Command => state.u_unlock(),
            EditorMode::Insert => {},
        }
    }
}// }}}

/// Print standard messages
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_msg<T: Display>( state: &EditorState, output: T ) {// {{{
    if state.show_messages {
        println!( "{}", output );
    }
}// }}}

/// Print help, warnings, other output depending on setting
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_help<T: Display>( state: &EditorState, output: T ) {// {{{
    if state.show_help {
        println!( "{}", output );
    } else {
        println!( "?" );
    }
}// }}}

/// Print help, warnings, other output depending on setting
///
/// TODO: Change first arg to just boolean: state.help?
pub fn print_help_debug<T: Debug>( state: &EditorState, output: T ) {// {{{
    if state.show_help {
        println!( "{:?}", output );
    } else {
        println!( "?" );
    }
}// }}}

// ^^^ Functions ^^^ }}}
#[cfg(test)]
mod tests {

}

