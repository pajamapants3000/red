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
    /// last regex used in address search
    last_regex: String,
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
            last_regex: String::new(), undo: Undo::new(), }
    }// }}}
// }}}
    /// Store string in last_regex// {{{
    pub fn store_last_regex( &mut self, re_str: &str ) {// {{{
        self.last_regex = re_str.to_string()
    }// }}}
    // }}}
    /// Obtain string from last_regex// {{{
    pub fn get_last_regex( &mut self ) -> String {// {{{
        self.last_regex.clone()
    }// }}}
    // }}}
    /// Reset undo - only need if we can't state.undo.reset( &state.buffer )
    pub fn u_reset( &mut self ) {
        self.undo.reset( &self.buffer )
    }
    /// Record line number of added line// {{{
    pub fn u_added_line( &mut self, address: usize ) {// {{{
        self.undo.added_lines( address, address )
    }// }}}
    // }}}
    /// Record line number of added line// {{{
    pub fn u_added_current_line( &mut self ) {// {{{
        let current_address = self.buffer.get_current_address();
        self.undo.added_lines( current_address, current_address )
    }// }}}
    // }}}
    /// Record range of added lines// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [1,$] )
    pub fn u_added_lines( &mut self,// {{{
                                      _initial: usize, _final: usize ) {
        assert!( 0 < _initial && _initial <= self.buffer.num_lines() );
        assert!( 0 < _final && _final <= self.buffer.num_lines() );
        self.undo.added_lines( _initial, _final )
    }// }}}
    // }}}
    /// Store current line, to be removed, in Undo structure// {{{
    ///
    /// do we use this?
    pub fn u_deleting_current_line( &mut self ) {// {{{
        let current_address = self.buffer.get_current_address();
        self.undo.deleting_line( &self.buffer, current_address )
    }// }}}
    // }}}
    /// Store provided line, to be removed, in Undo structure// {{{
    pub fn u_deleting_line( &mut self, address: usize ) {// {{{
        self.undo.deleting_line( &self.buffer, address )
    }// }}}
    // }}}
    /// Store lines to be removed, with addresses, in Undo structure// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [0,$] )
    /// Confirmed address still returns None for buffer::get_line_content ( This
    ///     should never be logically possible! )
    pub fn u_deleting_lines( &mut self,// {{{
                                       _initial: usize, _final: usize ) {
        // can be 0: e.g. when inserting first line
        assert!( _initial <= self.buffer.num_lines() );
        assert!( _final <= self.buffer.num_lines() );
        self.undo.deleting_lines( &self.buffer, _initial, _final )
    }// }}}
    // }}}
    /// Get collection of markers//{{{
    pub fn u_get_markers( &self ) -> Vec<usize> {// {{{
        self.undo.get_markers()
    }// }}}
    // }}}
    /// Get collection of saved changes//{{{
    pub fn u_get_changes( &self ) -> Vec<Change> {// {{{
        self.undo.get_changes()
    }// }}}
    // }}}
    /// Lock undo structure//{{{
    pub fn u_lock( &mut self ) {// {{{
        self.undo.lock()
    }// }}}
    // }}}
    /// Unlock undo structure//{{{
    pub fn u_unlock( &mut self ) {// {{{
        self.undo.unlock()
    }// }}}
    // }}}
    /// Unlock undo structure//{{{
    pub fn u_get_wascurrent_address( &self ) -> usize {// {{{
        self.undo.get_wascurrent_address()
    }// }}}
    // }}}
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
    wascurrent_address: usize,
    /// markers before the change
    markers: Vec<usize>,
    /// lock the structure for complex operations
    is_locked: bool
}
impl Undo {
    pub fn reset( &mut self, buffer: &Buffer ) {
        if !self.is_locked {
            let mut _markers = vec!( 0; NUM_LC );
            for indx in ( 'a' as u8 ) .. ( 'z' as u8 ) + 1 {
                _markers[ ( indx as usize ) - ( 'a' as usize ) ] =
                    buffer.get_marked_line( indx as char );
            }
            self.changes = Vec::new();
            self.wascurrent_address = buffer.get_current_address();
            self.markers = _markers;
        }
    }
    pub fn lock( &mut self ) {
        self.is_locked = true;
    }
    pub fn unlock( &mut self ) {
        self.is_locked = false;
    }
    /// Record range of added lines// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [1,$] )
    pub fn added_lines( &mut self,// {{{
                                      _initial: usize, _final: usize ) {
        for _address in _initial .. _final + 1 {
            self.changes.push( Change::Add{ address: _address });
        }
    }// }}}
    // }}}
    /// Store provided line, to be removed, in Undo structure// {{{
    pub fn deleting_line( &mut self, buffer: &Buffer, address: usize ) {// {{{
        self.deleting_lines( buffer, address, address )
    }// }}}
    // }}}
    /// Store lines to be removed, with addresses, in Undo structure// {{{
    ///
    /// # Panics
    /// _initial and/or _final are not a valid address ( in range [0,$] )
    /// Confirmed address still returns None for buffer::get_line_content ( This
    ///     should never be logically possible! )
    pub fn deleting_lines( &mut self, buffer: &Buffer,// {{{
                                       _initial: usize, _final: usize ) {
        for _address in _initial .. _final + 1 {
        // to delete range, delete same address repeatedly on _initial
            self.changes.push( Change::Remove{ address: _initial,
        // however, not ACTUALLY deleting right now, so we have to increment
        // to get the correct content to save - use _address
                    content: buffer.get_line_content( _address )
                    .expect( &( "main::u_deleting_lines: ".to_string() +
                                "unexpected missing line" ))
                    .to_string() });
        }
    }// }}}
    // }}}
    /// Obtain stored address// {{{
    pub fn get_wascurrent_address( &self ) -> usize {// {{{
        self.wascurrent_address
    }// }}}
    //}}}
    /// Get collection of markers//{{{
    pub fn get_markers( &self ) -> Vec<usize> {// {{{
        self.markers.clone()
    }// }}}
    /// Get collection of saved changes//{{{
    pub fn get_changes( &self ) -> Vec<Change> {// {{{
        self.changes.clone()
    }// }}}
    //}}}
    fn new() -> Undo {
        Undo {
            changes: Vec::new(),
            wascurrent_address: 0_usize,
            markers: vec!( 0; NUM_LC ),
            is_locked: false,
        }
    }
}
#[derive(Clone)]
pub enum Change {
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
                    state.u_added_current_line();
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

