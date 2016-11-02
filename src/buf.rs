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
#![allow(dead_code)]
// *** Bring in to namespace *** {{{

// Use LineWriter instead of, or in addition to, BufWriter?
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Error};
use std::fs::{File, copy, rename};
use std::path::Path;
use std::collections::LinkedList;
use std::collections::linked_list::{Iter, IterMut};
use std::iter::{IntoIterator, FromIterator, Iterator};
use std::str::Lines;

use ::chrono::*;
use ::regex::Regex;

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
pub enum BufferInput {// {{{
    File(String),         // box it?
    Command(String),    // OsString? box it?
    None,
}// }}}
/// Single assignment of a marker to a line
///
pub struct Marker {// {{{
    label: char,
    line: usize,
}// }}}
/// Stores collection of lines containing current working text
///
pub struct Buffer {     //{{{
    /// the current working buffer content as a list of lines
    lines: LinkedList<String>,
    /// the optional path of file being worked on
    ///
    /// Is None if no exising file was loaded and not yet saved
    file: Option<String>,
    /// timestamped path of file where buffer is stored regularly
    buffer_file: String,  // convert to Path later
    /// collection of markers set for lines in lines
    markers: Vec<Marker>,
    /// line number of "cursor"
    ///
    /// By default, searches start from here, inserts go here, etc.
    current_line: usize,
    /// current total number of lines in lines
    total_lines: usize,
    /// Date and time of last read of source file
    last_update: DateTime<UTC>,
    /// Date and time of last write to disk under temporary file name
    last_temp_write: DateTime<UTC>,
    /// Date and time of last write to disk under permanent file name
    last_write: DateTime<UTC>,
}   //}}}
impl Buffer {   //{{{
    /// Initialize new Buffer instance
    pub fn new( content: BufferInput )     //{{{
            -> Buffer {
        let mut _lines = Buffer::init_lines( &content );
        let _total_lines = _lines.len();
        Buffer {
            lines: _lines,
            buffer_file: match &content {
                &BufferInput::File( ref file_name ) =>
                    temp_file_name( Some( file_name.as_str() ) ),
                _ => temp_file_name( None ),
            },
            markers: Vec::new(),
            current_line: _total_lines,     // usize; should be Copy
            total_lines: _total_lines,
            last_update: match &content {
                &BufferInput::File(_) => UTC::now(),
                _ => get_null_time(),
            },
            last_temp_write: match &content {
                &BufferInput::File(_) => UTC::now(),
                _ => get_null_time(),
            },
            last_write: get_null_time(),
            file: match content {
                BufferInput::File( file_name ) => Some( file_name ),
                _ => None,
            },
        }
    }   //}}}
    /// Return total number of lines in buffer
    pub fn num_lines( &self ) -> usize {// {{{
        self.total_lines
    }// }}}
    // later, change approach to homogenize file/stdout source
    // generate iterator over BufRead object, either file, stdout, or empty
    /// Return the linked-list of lines to store in buffer
    fn init_lines( content: &BufferInput ) -> LinkedList<String> {// {{{
        let mut result: LinkedList<String>;
        match *content {
            BufferInput::File( ref file_name ) => {
                let file_path = Path::new( &file_name );
                let _file = file_path;
                let file_mode = FileMode{ f_read: true, ..Default::default() };
                if !_file.exists() {
                    return Buffer::init_lines( &BufferInput::None );
                }
                let file_opened: File;
                match file_opener( file_name, file_mode ) {
                    Ok( _file_opened ) => {
                        file_opened = _file_opened;
                    },
                    Err(_) => {
                        return Buffer::init_lines( &BufferInput::None );
                    },
                }
                let reader = BufReader::new( file_opened );
                LinkedList::from_iter( reader.lines()
                                             .map(|result| result.unwrap() ) )
                // reader.lines() returns an iterator (Lines type) over
                // io::Result<String>
                // We map that to the String (at least that's my intention!)
                // it seems like this happens on the iterator level, never
                // needing to do the iteration - should be efficient
                // (again... that's my intention!)
            },
            BufferInput::Command(ref command) => {
                LinkedList::from_iter( command_output_lines( command )
                                         .map(|x| x.to_string() ) )
            },
            BufferInput::None => {
                LinkedList::from_iter( "".to_string().lines()
                                         .map(|x| x.to_string() ) )
            },
        }
    }// }}}
    /// Return single line
    ///
    /// change to Result instead of Option?
    pub fn get_line_content( &self, line: usize ) -> Option<&str> {// {{{
        let mut lines_iter = self.lines_iterator();
        let mut _line: usize = 1;
        let mut result: &str = "";
        while _line <= line {
            match lines_iter.next() {
                Some( content ) => {
                    if _line == line {
                        result = content;
                    }
                },
                None => {
                    return None;
                },
            }
            _line += 1;
        }
        Some(result)
    }// }}}
    /// Return iterator over lines in buffer
    pub fn lines_iterator( &self ) -> Iter<String> {// {{{
        let lines_ref: &LinkedList<String> = &self.lines;
        lines_ref.into_iter()
    }// }}}
    /// Return reference to working file name string
    pub fn get_file_name( &self ) -> Option<&str> {// {{{
        match &self.file {
            &Some( ref file_name ) => Some( &file_name ),
            &None => None,
        }
    }// }}}
    /// Set new working file name
    ///
    /// At some point, need to test for existing file and ask user if overwrite
    pub fn set_file_name( &mut self, file_name: &str ) {// {{{
        self.file = Some(file_name.to_string());
    }// }}}
    /// Replace line with new string
    ///
    /// TODO: Add error handling; panics if line_num > len
    pub fn set_line_content( &mut self, line_num: usize, new_line: String )// {{{
            -> Result<(), RedError> {
        if line_num > self.lines.len() {
            return Err(RedError::SetLineOutOfBounds);
        }
        let mut back_list = self.lines.split_off( line_num - 1 );
        let _ = back_list.pop_front();
        self.lines.push_back( new_line );
        self.lines.append( &mut back_list );
        Ok( () )
    }// }}}
    /// Return mutable iterator over lines in buffer
    pub fn mut_lines_iterator( &mut self ) -> IterMut<String> {// {{{
        let mut lines_ref: &mut LinkedList<String> = &mut self.lines;
        lines_ref.into_iter()
    }// }}}
    pub fn get_current_line_number( &self ) -> usize {// {{{
        self.current_line
    }// }}}
    pub fn get_marked_line( &self, label: char ) -> Option<usize> {// {{{
        for i in 0 .. self.markers.len() {
            if self.markers[i].label == label {
                return Some( self.markers[i].line );
            }
        }
        None
    }// }}}
    /// Add new line marker
    ///
    /// TODO: need exception handling? What can happen? Just out of space I think
    pub fn set_marker( &mut self, _line: usize, _label: char ) {// {{{
        self.markers.push( Marker{ label: _label, line: _line } );
    }// }}}
    /// Return immutable slice over all markers
    pub fn list_markers( &self ) -> &[ Marker ] {// {{{
        self.markers.as_slice()
    }// }}}
    /// Return mutable slice over all markers
    pub fn list_markers_mut( &mut self ) -> &mut [ Marker ] {// {{{
        self.markers.as_mut_slice()
    }// }}}
    /// Write buffer contents to temp file
    ///
    /// TODO: Delete on buffer destruct
    fn store_buffer( &mut self ) -> Result<(), RedError> {// {{{
        let file_mode = FileMode { f_write: true, f_create: true,
                ..Default::default() };
        let temp_file_opened = try!( file_opener(
                &self.buffer_file, file_mode ) );
        let mut writer = BufWriter::new( temp_file_opened );
        {
            let mut _lines_iterator = self.lines_iterator();
            loop {
                match _lines_iterator.next() {
                    Some(x) => {
                        writer.write( x.as_bytes() )
                                .expect( "failed to write to disk" );
                    },
                    None => break,
                }
                writer.write( b"\n" ).expect( "failed to write to disk" );
            }
        }
        writer.flush().expect( "failed to write to disk" );
        let new_buffer_file = match &self.file {
            &Some(ref x) => temp_file_name( Some( x.as_str() ) ),
            &None => temp_file_name( None ),
        };
        try!( rename( &self.buffer_file, &new_buffer_file )
              .map_err(|err| RedError::FileRename( err ) )
            );
        self.buffer_file = new_buffer_file;
        Ok( () )
    }// }}}
    /// Save work to permanent file
    ///
    /// move to io.rs?
    pub fn write_to_disk( &mut self ) -> Result<(), Error> {// {{{
        self.store_buffer().expect( "failed to write to disk" );
        match &self.file {
            &Some(ref x) => {
                try!( copy( &self.buffer_file, x ) );
            },
            &None => {
                println!("No file name chosen for save");
            },
        }
        Ok( () )
    }// }}}
    /// Determine whether line matches regex
    ///
    /// Do NOT use for search over multiple lines - will be very inefficient!
    /// Use find_match instead
    pub fn does_line_match( &self, line: usize, regex: &str ) -> bool {// {{{
        let re = Regex::new( regex ).unwrap();
        let haystack = self.get_line_content( line );
        match haystack {
            Some( line ) => re.is_match( line ),
            None => false
        }
    }// }}}
    /// Return number of next matching line
    pub fn find_match( &self, regex: &str ) -> Option<usize> {// {{{
        let re = Regex::new( regex ).unwrap();
        let mut lines_iter = self.lines_iterator();
        for _ in 0 .. self.current_line {
            lines_iter.next();
        }
        let mut index: usize = self.current_line;
        loop {
            match lines_iter.next() {
                Some( line ) => {
                    if re.is_match( line.as_str() ) {
                        return Some( index );
                    }
                },
                None => return None,
            }
            index += 1;
        }
        // not reached
    }// }}}
}   //}}}

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{

/// Get timestamp to use for buffer filename
fn get_timestamp() -> String {// {{{
    let dt = UTC::now();

    dt.format("%Y%m%d%H%M%S").to_string()

}// }}}

/// Get DateTime to use as Null value
fn get_null_time() -> datetime::DateTime<UTC> {// {{{
    let utc_instance: UTC = UTC {};
    utc_instance.timestamp( 0, 0 )
}// }}}

/// Produce name for temporary buffer storage
fn temp_file_name( file_name: Option<&str> ) -> String {// {{{
    match file_name {
        Some(x) => ".red.".to_string() + x +
                "." + &get_timestamp(),
        None => ".red.".to_string() + &get_timestamp(),
    }
}// }}}

// ^^^ Functions ^^^ }}}

