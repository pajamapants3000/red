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

mod io;
//mod parse;
mod error;
mod buf;
#[cfg(test)]
mod tests;

use std::env;

use buf::{Buffer, BufferInput};
//use io::FileMode;

// }}}

// Crate Attributes {{{
//}}}

// *** Constants *** {{{

// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
/*
/// Current state - global variables, etc.
struct State {

} */

// ^^^ Data Structures ^^^ }}}

// Main {{{
fn main() {
    // quick'n''dirty - will process one by one later; clap?
    let args: Vec<String> = env::args().collect();

    // take as direct arg; will later be arg to flag
    let file_name = args[1].to_string();
    let buffer = Buffer::new( BufferInput::File( file_name ), None );
    for line in 0 .. buffer.num_lines() {
        println!("{}", buffer.get_line_content( line + 1 ).unwrap_or(""));
    }
    // confirm buffer is still valid
    let mut line_iterator = buffer.line_iterator();
    loop {
        match &line_iterator.next() {
            &Some( ref line ) => {
                println!("{}", line );
            },
            &None => break,
        }
    }
    println!("file: {}", buffer.get_file_name().unwrap_or("nofile") );
    /*
    let file_mode = FileMode { f_read: true, ..Default::default() };
    let file_opened: File;



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
                let command: parse::Command =
                        parse::parse_command( &cmd_input, &file_opened );
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
    */
    std::process::exit(
            error::error_code( error::RedError::FileClose ) as i32 );
    
}
//}}}

// *** Functions *** {{{

// ^^^ Functions ^^^ }}}

