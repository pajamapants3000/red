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

// TODO: Change all instances of `unwrap` to proper error handling

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
use std::str::Chars;
use::std::io::prelude::*;
// Use LineWriter instead of, or in addition to, BufWriter?
use::std::io::{self,BufReader,BufWriter};

// }}}

// Crate Attributes {{{
//}}}

// *** Constants *** {{{
// Define messages
// Some of these may be removed if builtin error descriptions work
const S_FOPEN_MSG: &'static str = "successfully opened file!";
//const E_FOPEN_MSG: &'static str = "unable to open file";

// Additional string constants
const LINE_CONT: &'static str = "\\\n";
const PROMPT: &'static str = "%";
const PROMPT_CONT: &'static str = ">";

// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
#[derive(Default)]
struct FileMode {
    f_write:        bool,
    f_read:         bool,
    f_append:       bool,
    f_truncate:     bool,
    f_create:       bool,
    f_create_new:   bool,
}

struct Command<'a> {
    address_initial: u32,
    address_final: u32,
    operation: char,
    parameters: &'a str,
}

enum RedError {
    FileOpen,
    FileClose,
}

// ^^^ Data Structures ^^^ }}}

// Main {{{
fn main() {
    // quick'n''dirty - will process one by one later; clap?
    let args: Vec<String> = env::args().collect();

    // take as direct arg; will later be arg to flag
    let file_name: &str = &args[0];
    let file_mode = FileMode { f_read: true, ..Default::default() };
    let file_opened: File;

    match file_opener( file_name, file_mode ) {
        Ok(f) => {
            file_opened = f;
            println!( "{}", S_FOPEN_MSG );
            println!( "our file is: {:?}", file_opened );
        },
        Err(e) => {
            println!( "error: {}", e );
            std::process::exit( error_code( RedError::FileOpen ) as i32 );

        },
    };

    //let mut file_buffer = BufReader::new(file_opened);
    //let mut file_writer = LineWriter::new(file_opened);
    let mut cli_reader = BufReader::new(io::stdin());
    let mut cli_writer = BufWriter::new(io::stdout());
    let mut cmd_input = String::new();
    let mut prompt = PROMPT.to_string();
    let mut user_quit: bool = false;

    cli_writer.write(format!("{}", prompt).as_bytes()).unwrap();
    cli_writer.flush().unwrap();
    // Main interaction loop {{{
    loop {
        cli_reader.read_line(&mut cmd_input).unwrap();

        if cmd_input.ends_with(LINE_CONT) {  // continue
            prompt = PROMPT_CONT.to_string();
        } else {
            {                                            // Execute command {{{
                let command: Command =
                        parse_command( &cmd_input, &file_opened );
                // just some test output
                cli_writer.write(command.parameters.as_bytes()).unwrap();
                cli_writer.write(command.address_initial.to_string()
                                 .as_bytes()).unwrap();
                cli_writer.write(b"\n").unwrap();
                cli_writer.write(command.address_final.to_string()
                                 .as_bytes()).unwrap();
                cli_writer.write(b"\n").unwrap();
                cli_writer.write(command.operation.to_string()
                                 .as_bytes()).unwrap();
                cli_writer.write(b"\n").unwrap();

                match command.operation {
                    'q' => user_quit = true,
                    _ => ()
                }

            }                                           // Done executing }}}
            // ready for a new command
            cmd_input.clear();
            // in case of continuation, return prompt to standard
            prompt = PROMPT.to_string();
        }

        if user_quit { break }

        // prompt for the next round
        cli_writer.write(format!("{}", prompt).as_bytes()).unwrap();
        // put it all to the screen
        cli_writer.flush().unwrap();
    }
    //}}}
    
    std::process::exit( error_code( RedError::FileClose ) as i32 );
    
}
//}}}

/// Opens file with user-specified name and mode {{{
///
/// Uses global definitions of mode flags in this file
///
/// Returns direct result of call to OpenOptions::new()
/// This is of type Result<File, io::Error>
fn file_opener( name: &str, mode: FileMode ) -> Result<File, io::Error> {

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

/// Parses command-mode input {{{
///
fn parse_command<'a>( _cmd_input: &'a str, file_opened: &File ) -> Command<'a> {
    // MUST initialize?
    let mut _address_initial: u32 = 1;
    let mut _address_final: u32 = 1;
    let mut _operation: char = 'p';
    let mut _parameters: &str = _cmd_input;

    let op_indx: usize;
    op_indx = get_opchar_index( _cmd_input ).unwrap_or( 0usize );
    _address_final = op_indx as u32;

    match get_address( "", file_opened ) {
        ( x, y ) => {
            _address_initial = x;
            _address_final = y;
        }
    }

    Command {
        address_initial: _address_initial,
        address_final: _address_final,
        operation: _operation,
        parameters: _parameters,
    }
}
//}}}

/// Identify address or address range {{{
///
#[allow(unused_variables)]
fn get_address( address_string: &str, file_opened: &File ) -> ( u32, u32 ) {
    // start with simple auto-return of ( 1u32, 2u32 )
    ( 1u32, 2u32 )
}
//}}}

/// Return error code for given error type {{{
///
fn error_code( _error: RedError ) -> u32 {
    match _error {
        RedError::FileOpen => 280,
        RedError::FileClose => 281,
    }
}
//}}}

/// Find index of operation code in string
///
/// Parses full command and finds the index of the operation character;
/// The trick here is that the operation character may appear any number
/// of times as part of a regular expression used to specify an address
/// range that matches;
/// What this function does is simply locates the first alphabetic character
/// that is not wrapped in either /.../ or ?...?
///
/// Trims white space on the left as well - does not count these characters
///
/// # Examples
///
/// No address given
/// ```rust
/// let _in: &str = "e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// No address given, with spaces
/// ```rust
/// let _in: &str = "       e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// No address given, with spaces and tabs
/// ```rust
/// let _in: &str = "  		  	e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// Most basic address value types
/// ```rust
/// let _in: &str = ".a";
/// assert_eq!( get_opchar_index( _in ), 1 );
/// ```
///
/// ```rust
/// let _in: &str = ".,.p";
/// assert_eq!( get_opchar_index( _in ), 3 );
/// ```
///
/// Slightly more complicated
/// ```rust
/// let _in: &str = ".-2,.+2p";
/// assert_eq!( get_opchar_index( _in ), 3 );
/// ```
///
/// Regular expression match line search forward
/// ```rust
/// let _in: &str = "/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
/// Regular expression match line search forward with spaces and tabs
/// ```rust
/// let _in: &str =
/// "		  	/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
/// Regular expression match line search backward
/// ```rust
/// let _in: &str = "?^Beginning with.*$?,?.* at the end$?s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
fn get_opchar_index( _cmd_input: &str ) -> Result<usize, RedError> {
    let mut result: i32 = -1;

    struct Interpret {
        index: u16,         // index of character in string
        pattern_char: char, // stores '/' or '?' while reading pattern
        escape: bool,       // last character read was '\'
        expect: bool,       // we expect next char to be ',', ';', or alpha
    }
    let mut current_state: Interpret = Interpret {
        index: 0u16,
        pattern_char: '\0',
        escape: false,
        expect: false,
    };

    let mut ch_iter: Chars = _cmd_input.trim_left().chars();

    loop {
        match ch_iter.next() {
            None => break,
            Some( x ) => {
                if current_state.pattern_char == '\0' {
                    if current_state.expect {
                        match x {
                            'a'...'z' | 'A'...'Z' => {
                                result = current_state.index as i32;
                                break;
                            },
                            ',' | ';' => {
                                current_state.escape= false;
                                current_state.expect = false
                            },
                            '\\' => current_state.escape = true,
                            _ => break,
                        }
                    }
                    match x {
                        '/' | '?' => {
                            if !current_state.escape {
                                current_state.pattern_char = x;
                            }
                        }
                        'a'...'z' | 'A'...'Z' => {
                            result = current_state.index as i32;
                            break;
                        },
                        '\\' => current_state.escape = true,
                        _ => {
                            current_state.escape= false;
                            current_state.expect = false;
                        },
                    }
                } else {
                    match x {
                        '/' => {
                            if '/' == current_state.pattern_char {
                                if !current_state.escape {
                                    current_state.pattern_char = '\0';
                                    current_state.expect = true;
                                }
                            }
                        }
                        '?' => {
                            if '?' == current_state.pattern_char {
                                if !current_state.escape {
                                    current_state.pattern_char = '\0';
                                    current_state.expect = true;
                                }
                            }
                        }
                        '\\' => current_state.escape = true,
                        _ => {},
                    }
                }
            }
        }
        current_state.index += 1;
    }
    if result < 0 {
        Result::Err( RedError::FileClose )
    } else {
        println!("result {:?}", result);
        Result::Ok( result as usize )   // result will be positive here
    }
}
