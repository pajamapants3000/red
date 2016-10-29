/*
 * File   : buf.rs
 * Purpose: read/write buffer during user interaction
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016
 */

// *** Bring in to namespace *** {{{
extern crate chrono;

// Use LineWriter instead of, or in addition to, BufWriter?
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::collections::LinkedList;
use std::collections::linked_list::Iter;

use self::chrono::*;

use io::*;
use error::*;

// ^^^ Bring in to namespace ^^^ }}}
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
// *** Constants *** {{{
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
/// Type of input for starting buffer
///
/// File - read from existing file
/// Command - read output of specified command
/// None - no input
enum BufferInput {
    File(Path),         // box it?
    Command(String),    // OsString? box it?
    None,
}
/// Single assignment of a marker to a line
///
struct Marker {
    label: char,
    line: usize,
}
/// Stores collection of lines containing current working text
///
pub struct Buffer {
    /// the current working buffer content as a list of lines
    lines: LinkedList<String>,
    /// the optional path of file being worked on
    ///
    /// Is None if no exising file was loaded and not yet saved
    file: Option<Path>,
    /// timestamped path of file where buffer is stored regularly
    buffer_file: Option<Path>,  // convert to Path later
    /// collection of markers set for lines in lines
    markers: Vec<Marker>,
    /// line number of "cursor"
    ///
    /// By default, searches start from here, inserts go here, etc.
    current_line: usize,
    /// current total number of lines in lines
    total_lines: usize,
    /// buffer has read in contents of working file or command output
    is_initialized: bool,
    /// Date and time of last read of source file
    last_update: DateTime<UTC>,
    /// Date and time of last write to disk under temporary file name
    last_temp_write: DateTime<UTC>,
    /// Date and time of last write to disk under permanent file name
    last_write: DateTime<UTC>,
}
impl Buffer {
    pub fn new( content: BufferInput, output_file: Option<Path> ) -> Buffer {
        let mut _lines = Buffer::get_lines( content );
        let _total_lines = _lines.len();
        Buffer {
            lines: _lines,
            file: match content {
                BufferInput::File(x) => Some(x),
                _ => None,
            },
            buffer_file: output_file,
            markers: Vec::new(),
            current_line: _total_lines,     // usize; should be Copy
            total_lines: _total_lines,
            last_update: match content {
                BufferInput::File(x) => DateTime::now(),
                _ => get_null_time(),
            },
            last_temp_write: match content {
                BufferInput::File(x) => DateTime::now(),
                _ => get_null_time(),
            },
            last_write: get_null_time(),
        }
    }
    pub fn get_file_name( &self ) -> String {
    }
    pub fn set_file_name( &self, file_name: &str ) -> Result<(), RedError> {
    }
    pub fn get_line_content( &self, line: usize ) -> Result<&str, RedError> {
    }
    pub fn set_line_content( &self, line: usize ) -> Result<&str, RedError> {
    }
    pub fn line_iterator( &self ) -> Iter<String> {
    }
    pub fn mut_line_iterator( &self ) -> Iter<String> {
    }
    pub fn get_current_line_number( &self ) -> usize {
    }
    pub fn does_line_match_regex( &self, line: usize, regex: &str ) -> bool {
    }
    pub fn get_marker( &self, label: char ) -> Option<usize> {
    }
    pub fn set_marker( &self, line: usize, label: char ) -> Result<(), RedError> {
    }
    pub fn list_markers( &self ) -> Vec<(char, usize)> {
    }
    pub fn write_to_disk( &self ) -> Result<(), RedError> {
    }
    pub fn num_lines( &self ) -> usize {
        self.total_lines
    }
    fn store_buffer( &self ) -> Result<(), RedError> {
    }
    // later, change approach to homogenize file/stdout source
    // generate iterator over BufRead object, either file, stdout, or empty
    fn get_lines( content: BufferInput ) -> LinkedList<String> {
        let mut result: LinkedList<String>;
        match content {
            BufferInput::File( file_path ) => {
                let _file = file_path;
                let file_mode = FileMode{ f_read: true, ..Default::default() };
                if !_file.exists() {
                    file_mode.f_write = true;
                    file_mode.f_create = true;
                }
                let file_opened: File = try!(file_opener( _file, file_mode ) );
                let mut reader = BufReader::new( file_opened );
                // reader.lines() returns an iterator (Lines type) over
                // io::Result<String>
                // We map that to the String (at least that's my intention!)
                // it seems like this happens on the iterator level, never
                // needing to do the iteration - should be efficient
                // (again... that's my intention!)
                LinkedList::from_iter( reader.lines()
                                  .map( |result| result.unwrap() ));
            },
            BufferInput::Command(command) => {
                result = LinkedList::new();
                result.push_back(
                    "Command buffer not yet implemented, but thank you
                    for trying!"
                    );
                result
            },
            BufferInput::None => {
                result = LinkedList::new();
                result.push_back( "No input provided" );
                result
            },
        }
    }
}
// want to be able to create a new buffer without any information provided
//impl Default for Buffer {
//}

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{

/* DELETE?
/// Read stream or file into buffer and return buffer
///
/// Buffer cursor on last line
///
read_into_buffer<T: Read>( buffer: Buffer, content: <T> ) -> Buffer {
    let mut _content: Vec<u8> = Vec::new();
    // TODO: error-checking
    content.read_to_end( &mut _content );
    buffer.BufWriter( _content );
}

/// Open buffer file for buffered read/write
///
/// May take file to copy into buffer, else creates empty buffer
///
init_buffer( file_opened: Option<File> ) -> Buffer {
    match file_opened {
        Some(x) => {            // opening existing file to edit
        },
        None => {               // new file or buffer to collect some output
        }
    }
}

fn get_line_offset( buffer: Buffer,  line: usize ) -> usize {
    let current_position: usize = buffer.reader( 
}

*/ //DELETE?

/*
fn get_timestamp() -> String {
    let dt = chrono::Local::now();

    dt.format("%Y%m%d%H%M%S").to_string()

}*/
fn get_null_time() -> chrono::datetime::DateTime<UTC> {
    DateTime::from_utc( NaiveDateTime::from_timestamp(0, 0), UTC::Offset )
}

// ^^^ Functions ^^^ }}}

