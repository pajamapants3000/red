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
use std::fs::{File, OpenOptions};
use std::fs::path::Path;
use std::collections::LinkedList;

use chrono::naive::datetime::NaiveDateTime;
use chrono::datetime::DateTime;

use io::*

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
    File(Path),
    Command(String),    // OsString?
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
    /// Date and time of last write to buffer file
    last_updated: DateTime<UTC>,
    /// Date and time of last write to disk under permanent file name
    last_written: DateTime<UTC>,
}
impl Buffer {
    pub fn new( content: BufferInput, output_file: Option<Path> ) -> Buffer {
        let mut _lines: LinkedList<String>;
        let mut _file: Path;                // need to initialize?
        // move to return?
        let mut _buffer_file = output_file;
        let mut _markers: Vec<Marker> = Vec::new();
        let mut _current_line: usize;
        let mut _total_lines: usize;
        let mut _is_initialized: bool = false;
        let mut _last_updated: DateTime<UTC> = DateTime::from_utc(
            NaiveDateTime::from_timestamp(0, 0), UTC::Offset );
        let mut _last_written: DateTime<UTC> = DateTime::from_utc(
            NaiveDateTime::from_timestamp(0, 0), UTC::Offset );
        // end 'move to return?'

        match content {
            BufferInput::File( file_path ) => {
                _file = file_path;
                let file_mode = FileMode{ f_read: true, ..Default::default() };
                if _file.exists() {
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
                _lines = from_iter<Lines>(
                        reader.lines()
                        .map( |result| result.unwrap() );
            },
            BufferInput::Command(command) => {
                _lines = LinkedList::new();
                _lines.push_back(
                    "Command buffer not yet implemented, but thank you
                    for trying!"
                    );
            },
            BufferInput::None => {
                _lines = LinkedList::new();
                _lines.push_back( "No input provided" );
            },
        }
        _total_lines = _lines.len();
        _current_line = _total_lines;       // usize; should be Copy
        Buffer {
            lines: _lines,
            file: _file,
            buffer_file: _buffer_file,
            markers: _markers,
            current_line: _current_line,
            total_lines: _total_lines,
            is_initialized: _is_initialized,
            last_updated: _last_updated,
            last_written: _last_written,
    }
    pub fn get_file_name( &self ) -> String {
    }
    pub fn set_file_name( &self, &str ) -> Result<()> {
    }
    pub fn get_line_content( &self, line: usize ) -> Result<&str> {
    }
    pub fn set_line_content( &self, line: usize ) -> Result<&str> {
    }
    pub fn get_current_line_number( &self ) -> usize {
    }
    pub fn does_line_match_regex( &self, line: usize, regex: &str ) -> bool {
    }
    pub fn get_marker( &self, label: char ) -> Option<usize> {
    }
    pub fn set_marker( &self, line: usize, label: char ) -> Result<()> {
    }
    pub fn list_markers( &self ) -> Vec<(char, usize)> {
    }
    pub fn write_to_disk( &self ) -> Result<()> {
    }
    fn store_buffer( &self ) -> Result<()> {
    }
    fn initialize_buffer( &self, content: BufferInput ) -> Result<()> {
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

}
*/

// ^^^ Functions ^^^ }}}

