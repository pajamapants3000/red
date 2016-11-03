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
#[macro_use]
extern crate lazy_static;

mod io;
//mod parse;
mod error;
mod buf;

use std::env;
use buf::{Buffer, BufferInput};

//use io::FileMode;

// }}}
// *** Constants *** {{{

// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{

fn main() {// {{{
    // quick'n''dirty - will process one by one later; clap?
    let args: Vec<String> = env::args().collect();

    // take as direct arg; will later be arg to flag
    let file_name = args[1].to_string();

    // Test routine
    let mut buffer1 = Buffer::new( BufferInput::File( file_name ) );
    let mut buffer = Buffer::new( BufferInput::Command( "ls".to_string() ) );
    for line in 0 .. buffer.num_lines() {
        println!("{}", buffer.get_line_content( line + 1 ).unwrap_or(""));
    }
    // confirm buffer is still valid
    {
        let mut lines_iterator = buffer.lines_iterator();
        loop {
            match &lines_iterator.next() {
                &Some( ref line ) => {
                    println!("{}", line );
                },
                &None => break,
            }
        }
    }
    // print file name
    println!("file: {}", buffer.get_file_name().unwrap_or("nofile") );
    // set new file name
    buffer.set_file_name( "myfile.txt" );
    // print new file name
    println!("file: {}", buffer.get_file_name().unwrap_or("nofile") );
    {
        let mut _line_2 = buffer.get_line_content( 2 ).unwrap_or("missing");
        println!("line 2: {}", _line_2);
    }
    buffer.set_line_content( 2, "this is the new line".to_string() ).unwrap();
    {
        let mut _line_2 = buffer.get_line_content( 2 ).unwrap_or("missing");
        println!("new line 2: {}", _line_2 );
    }
    println!("Let's print one last time...");
    for line in 0 .. buffer.num_lines() {
        println!("{}", buffer.get_line_content( line + 1 ).unwrap_or(""));
    }
    //buffer.write_to_disk().expect("unable to save file");
    buffer.store_buffer().expect("Failed to store buffer on disk");
    buffer1.store_buffer().expect("Failed to store buffer on disk");

    buffer1.destruct().expect("Failed to deconstruct buffer1");
    quit( &mut buffer )

    // end test routine
    
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

