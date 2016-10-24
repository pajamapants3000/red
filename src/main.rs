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
use::std::io::prelude::*;
// Use LineWriter instead of, or in addition to, BufWriter?
use::std::io::{self,BufReader,BufWriter,Error};

// }}}

// *** Constants *** {{{
// Define messages
// Some of these may be removed if builtin error descriptions work
const S_FOPEN: &'static str = "successfully opened file!";
//const E_FOPEN: &'static str = "unable to open file";

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
            println!( "{}", S_FOPEN );
            println!( "our file is: {:?}", file_opened );
        },
        Err(e) => {
            println!( "error: {}", e );

        },
    };

    //let mut file_buffer = BufReader::new(file_opened);
    //let mut file_writer = LineWriter::new(file_opened);
    let mut cli_reader = BufReader::new(io::stdin());
    let mut cli_writer = BufWriter::new(io::stdout());
    let mut cmd_input = String::new();
    let mut prompt = PROMPT.to_string();

    cli_writer.write(format!("{}", prompt).as_bytes()).unwrap();
    cli_writer.flush().unwrap();
    // Main interaction loop {{{
    loop {
        cli_reader.read_line(&mut cmd_input).unwrap();

        if cmd_input.ends_with(LINE_CONT) {  // continue
            prompt = PROMPT_CONT.to_string();
        } else {
            {                                            // Execute command {{{
                let command: Command = parse_command( &cmd_input );
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

            }                                           // Done executing }}}
            // ready for a new command
            cmd_input.clear();
            // in case of continuation, return prompt to standard
            prompt = PROMPT.to_string();
        }

        // prompt for the next round
        cli_writer.write(format!("{}", prompt).as_bytes()).unwrap();
        // put it all to the screen
        cli_writer.flush().unwrap();
    }
    //}}}
    
}
//}}}

/// Opens file with user-specified name and mode {{{
///
/// Uses global definitions of mode flags in this file
///
/// Returns direct result of call to OpenOptions::new()
/// This is of type Result<File, Error>
fn file_opener( name: &str, mode: FileMode ) -> Result<File, Error> {

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
fn parse_command( _cmd_input: &str ) -> Command {
    Command {
        address_initial: 1,
        address_final: 1,
        operation: 'p',
        parameters: _cmd_input,
    }
}
//]]]
