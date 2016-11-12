/*
 * File   : buf.rs
 * Purpose: read/write buffer during user interaction
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016 */
// *** Bring in to namespace *** {{{

// Use LineWriter instead of, or in addition to, BufWriter?
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, stdin};
use std::fs::{self, File, copy, rename};
use std::path::Path;
use std::collections::LinkedList;
use std::collections::linked_list::{Iter, IterMut};
use std::iter::{IntoIterator, FromIterator, Iterator};

use ::chrono::*;
use ::regex::Regex;
use ::rand::{thread_rng, Rng};

use io::*;
use error::*;
use ::{EditorState, print_help};

// ^^^ Bring in to namespace ^^^ }}}
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}

// *** Constants *** {{{
const NUM_LC: usize = 26;
// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
/// Type of input for starting buffer// {{{
///
/// File - read from existing file
/// Command - read output of specified command
/// None - no input
pub enum BufferInput {// {{{
    File(String),         // box it?
    Command(String),    // OsString? box it?
    None,
}// }}}
// }}}
/// Stores collection of lines containing current working text// {{{
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
    markers: Vec<usize>,
    /// line number of "cursor"
    ///
    /// By default, searches start from here, inserts go here, etc.
    current_line: usize,
    /// current total number of lines in lines
    total_lines: usize,
    /// true if file has been modified since last write
    _is_modified: bool,
    /// Date and time of last read of source file
    last_update: DateTime<UTC>,
    /// Date and time of last write to disk under temporary file name
    last_temp_write: DateTime<UTC>,
    /// Date and time of last write to disk under permanent file name
    last_write: DateTime<UTC>,
}   //}}}
// }}}
impl Buffer {   //{{{
    /// Initialize new Buffer instance// {{{
    pub fn new( content: BufferInput )     //{{{
            -> Buffer {
        let mut _lines = Buffer::init_lines( &content );
        let _total_lines = _lines.len();
        let mut result = Buffer {
            lines: _lines,
            buffer_file: match &content {
                &BufferInput::File( ref file_name ) =>
                    temp_file_name( Some( file_name.as_str() ) ),
                _ => temp_file_name( None ),
            },
            markers: vec!( 0; NUM_LC ),
            current_line: _total_lines,     // usize; should be Copy
            total_lines: _total_lines,
            _is_modified: false,
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
                BufferInput::Command( command ) => Some(
                                        command.split_whitespace()
                                        .next()
                                        .unwrap_or("command_stdout")
                                        .to_string() ),
                _ => None,
            },
        };
        // TODO: ?
        match result.store_buffer() {
            Ok(_) => {},
            Err(_) => {
                println!("Unable to store buffer");
            },
        }
        result
    }   //}}}
// }}}
    /// Return total number of lines in buffer// {{{
    pub fn num_lines( &self ) -> usize {// {{{
        self.total_lines
    }// }}}
// }}}
    /// Return true if buffer modified since last write// {{{
    pub fn is_modified( &self ) -> bool {// {{{
        self._is_modified
    }// }}}
// }}}
    // later, change approach to homogenize file/stdout source
    // generate iterator over BufRead object, either file, stdout, or empty
    /// Return the linked-list of lines to store in buffer// {{{
    fn init_lines( content: &BufferInput ) -> LinkedList<String> {// {{{
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
                LinkedList::from_iter( command_output( command ).lines()
                                         .map(|x| x.to_string() ) )
            },
            BufferInput::None => {
                LinkedList::from_iter( "".to_string().lines()
                                         .map(|x| x.to_string() ) )
            },
        }
    }// }}}
// }}}
    /// Return single line// {{{
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
// }}}
    /// Return iterator over lines in buffer// {{{
    ///
    /// works in reverse with next_back?
    pub fn lines_iterator( &self ) -> Iter<String> {// {{{
        let lines_ref: &LinkedList<String> = &self.lines;
        lines_ref.into_iter()
    }// }}}
// }}}
    /// Return reference to working file name string// {{{
    pub fn get_file_name( &self ) -> Option<&str> {// {{{
        match &self.file {
            &Some( ref file_name ) => Some( &file_name ),
            &None => None,
        }
    }// }}}
// }}}
    /// Set new working file name// {{{
    ///
    /// At some point, need to test for existing file and ask user if overwrite
    pub fn set_file_name( &mut self, file_name: &str ) {// {{{
        self.file = Some(file_name.to_string());
    }// }}}
// }}}
    /// Delete line// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    pub fn delete_line( &mut self, line_num: usize ) {// {{{
        let mut back = self.lines.split_off( line_num );
        back.pop_front();
        self.lines.append( &mut back );
        self.delete_update_markers( line_num );
    }// }}}
// }}}
    /// Insert new line at current position// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    pub fn insert_here( &mut self, new_line: &str ) {// {{{
        let line_num = self.current_line;
        self.insert_line( line_num, new_line );
        self.insert_update_markers( line_num );
    }// }}}
// }}}
    /// Insert new line// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    pub fn insert_line( &mut self, line_num: usize, new_line: &str ) {// {{{
        let mut back = self.lines.split_off( line_num );
        self.lines.push_back( new_line.to_string() );
        self.lines.append( &mut back );
        self.set_current_line_number( line_num + 1 );
        self.insert_update_markers( line_num );
    }// }}}
// }}}
    /// Replace line with new string// {{{
    ///
    /// TODO: Add error handling; panics if line_num > len
    pub fn set_line_content( &mut self, line_num: usize, new_line: &str )// {{{
            -> Result<(), RedError> {
        if line_num > self.lines.len() {
            return Err( RedError::SetLineOutOfBounds );
        }
        let mut back_list = self.lines.split_off( line_num - 1 );
        let _ = back_list.pop_front();
        self.lines.push_back( new_line.to_string() );
        self.lines.append( &mut back_list );
        Ok( () )
    }// }}}
// }}}
    /// Return mutable iterator over lines in buffer// {{{
    pub fn mut_lines_iterator( &mut self ) -> IterMut<String> {// {{{
        let mut lines_ref: &mut LinkedList<String> = &mut self.lines;
        lines_ref.into_iter()
    }// }}}
// }}}
    /// Return current working line number// {{{
    pub fn get_current_line_number( &self ) -> usize {// {{{
        self.current_line
    }// }}}
// }}}
    /// Move "cursor" to new line// {{{
    pub fn set_current_line_number( &mut self, line_number: usize ) {// {{{
        if line_number < self.total_lines {
            self.current_line = line_number;
        } else {
            self.current_line = self.total_lines;
        }
    }// }}}
// }}}
    /// Return number of line with a specified mark set// {{{
    pub fn get_marked_line( &self, label: char ) -> usize {// {{{
        self.markers[ (( label as u8 ) - ( 'a' as u8 )) as usize ]
    }// }}}
// }}}
    /// Add new line marker// {{{
    ///
    pub fn set_marker( &mut self, line: usize, label: char ) {// {{{
        self.markers[ (( label as u8 ) - ( 'a' as u8 )) as usize ] = line;
    }// }}}
// }}}
    /// Return immutable slice over all markers// {{{
    pub fn list_markers( &self ) {// {{{
        let mut indx: u8 = 0;
        for marker in &self.markers {
            if *marker != 0 {
                println!("{}: {}", (( 'a' as u8 ) + indx) as char, marker );
            }
            indx += 1;
        }
    }// }}}
// }}}
    /// Write buffer contents to temp file// {{{
    ///
    /// TODO: Delete on buffer destruct or at least on program exit
    pub fn store_buffer( &mut self ) -> Result<(), RedError> {// {{{
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
                        try!( writer.write( x.as_bytes() )
                                .map_err( |e| RedError::FileWrite(e) ) );
                    },
                    None => break,
                }
                try!( writer.write( b"\n" )
                      .map_err( |e| RedError::FileWrite(e) ));
            }
        }
        try!( writer.flush().map_err( |e| RedError::FileWrite(e) ));
        let new_buffer_file = match &self.file {
            &Some(ref x) => temp_file_name( Some( x.as_str() ) ),
            &None => temp_file_name( None ),
        };
        try!( rename( &self.buffer_file, &new_buffer_file )
              .map_err(|e| RedError::FileRename(e) )
            );
        self.buffer_file = new_buffer_file;
        Ok( () )
    }// }}}
// }}}
    /// Save work to permanent file// {{{
    ///
    /// TODO: move to io.rs? I don't think so, it's a part of the
    /// functionality of the buffer
    /// TODO: set up default filename?
    /// # Panics
    /// # Errors
    /// # Safety
    /// # Examples
    pub fn write_to_disk( &mut self, file_name: &str )// {{{
            -> Result<(), RedError> {
        try!( self.store_buffer() );
        if file_name.len() == 0 {
            match &self.file {
                &Some(ref x) => {
                    try!( copy( &self.buffer_file, x )
                          .map_err( RedError::FileCopy ) );
                },
                &None => {
                    println!("No file name chosen for save");
                },
            }
        } else {
            try!( copy( &self.buffer_file, file_name )
                          .map_err( RedError::FileCopy ) );
        }
        Ok( () )
    }// }}}
// }}}
    /// Determine whether line matches regex// {{{
    ///
    /// Do NOT use for search over multiple lines - will be very inefficient!
    /// Use find_match instead
    pub fn does_line_match( &self, line: usize, regex: &str ) -> bool {// {{{
        let re: Regex = Regex::new( regex ).unwrap();
        let haystack = self.get_line_content( line );
        match haystack {
            Some( line ) => re.is_match( line ),
            None => false
        }
    }// }}}
// }}}
    /// Return number of next matching line// {{{
    pub fn find_match( &self, regex: &str ) -> Option<usize> {// {{{
        let re = Regex::new( regex ).unwrap();
        let mut lines_iter = self.lines_iterator();
        for _ in 1 .. self.current_line {
            lines_iter.next();              // start at current line
        }
        let mut index: usize = self.current_line;
        loop {
            match lines_iter.next() {
                Some( line ) => {
                    if re.is_match( line.as_str() ) {
                        return Some( index );
                    }
                },
                None => break,
            }
            index += 1;
        }
        index = 1;
        let mut lines_iter = self.lines_iterator();
        for _ in 0 .. self.current_line {
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
        None
    }// }}}
// }}}
    /// Return number of previous matching line// {{{
    pub fn find_match_reverse( &self, regex: &str ) -> Option<usize> {// {{{
        let re = Regex::new( regex ).unwrap();
        let mut lines_iter = self.lines_iterator();
        for _ in self.current_line .. ( self.total_lines + 1 ) {
            lines_iter.next_back();              // start at current line
        }
        let mut index: usize = self.current_line - 1;
        loop {
            match lines_iter.next_back() {
                Some( line ) => {
                    if re.is_match( line.as_str() ) {
                        return Some( index );
                    }
                },
                None => break,
            }
            index -= 1;
        }
        let mut lines_iter = self.lines_iterator();
        let mut index: usize = self.total_lines;
        for _ in self.current_line .. ( self.total_lines + 1 ) {
            match lines_iter.next_back() {
                Some( line ) => {
                    if re.is_match( line.as_str() ) {
                        return Some( index );
                    }
                },
                None => return None,
            }
            index -= 1;
        }
        None
    }// }}}
// }}}
    /// Deconstruct buffer// {{{
    pub fn destruct( &mut self ) -> Result<(), RedError> {// {{{
        let _stdin = stdin();
        if self.is_modified() {
            println!("Write file before closing?\n>");
            let mut _stdin_handle = _stdin.lock();
            let mut response: String = "".to_string();
            _stdin_handle.read_to_string( &mut response )
                .expect("Failed to read user input");
            match response.to_lowercase().as_str() {
                "y" | "yes" => { try!( self.write_to_disk( "" ) ); },
                _ => (),
            }
        }
        fs::remove_file( &self.buffer_file )
            .expect("Failed to delete buffer file");
        self.lines.clear();
        Ok( () )
    }// }}}
    /// Keep markers valid after inserting new line
    ///
    /// I can't think of any errors that might go here
    fn insert_update_markers( &mut self, line_num: usize ) {
        for marker in &mut self.markers {
            if *marker > line_num {
                *marker += 1;
            }
        }
    }
    /// Keep markers valid after deleting line;
    /// delete any markers to deleted line
    ///
    /// I can't think of any errors that might go here
    fn delete_update_markers( &mut self, line_num: usize ) {
        for marker in &mut self.markers {
            if *marker > line_num {
                *marker -= 1;
            } else if *marker == line_num {
                *marker = 0;
            }
        }
    }
// }}}
}   //}}}

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Get timestamp to use for buffer filename// {{{
///
/// # Panics
/// # Errors
/// # Safety
/// # Examples
fn get_timestamp() -> String {// {{{
    let dt = UTC::now();
    dt.format("%Y%m%d%H%M%S").to_string()

}// }}}
// }}}
/// Get DateTime to use as Null value// {{{
///
/// # Panics
/// # Errors
/// # Safety
/// # Examples
fn get_null_time() -> datetime::DateTime<UTC> {// {{{
    let utc_instance: UTC = UTC {};
    utc_instance.timestamp( 0, 0 )
}// }}}
// }}}
/// Produce name for temporary buffer storage// {{{
///
/// # Panics
/// # Errors
/// # Safety
/// # Examples
fn temp_file_name( file_name: Option<&str> ) -> String {// {{{
    // only way to conflict is by choosing the same eight alphanumeric
    // characters in less than a second!
    let random_string: String = thread_rng()
                                .gen_ascii_chars().take(8).collect();
    match file_name {
        Some(x) => ".red.".to_string() + x +
                random_string.as_str() + "." + &get_timestamp(),
        None => ".red.".to_string() + random_string.as_str() +
            "." + &get_timestamp(),
    }
}// }}}
// }}}
// ^^^ Functions ^^^ }}}

#[cfg(test)]
mod tests {// {{{
    //  ***     ***      Bring into namespace   ***     *** //// {{{
    use std::process::Command;
    use std::fs;
    use std::io::Write;
    use std::default::Default;

    use super::*;
    use error::*;
    use io::*;
    //  ^^^     ^^^     Bring into namespace    ^^^     ^^^ //// }}}
    //  ***     ***     Constants   ***     ***     //// {{{
    const TEST_FILE: &'static str = "red_filetest";
    const FILE_CONTENT_LINE: &'static str = "testfile";
    const COMMAND_CONTENT_LINE_1: &'static str = "testcmd";
    const COMMAND_CONTENT_LINE_2: &'static str = "testcmda testcmdb";
    const FILE_FILE_SUFFIX: &'static str = ".file";
    const COMMAND_FILE_SUFFIX: &'static str = ".cmd";
    //  ^^^     ^^^     Constants   ^^^     ^^^     //// }}}
    // begin prep functions// {{{
    /// Generate and return string containing lines for testing// {{{
    ///
    /// Takes string to use as base for text on each line
    /// This string will have the line number appended
    /// Also takes a single u8 integer, the number of lines to generate
    fn test_lines( line_str: &str, num_lines: usize ) -> String {// {{{
        let mut file_content = "".to_string();
        let mut next: String;
        for i in 1 .. ( num_lines + 1 ) {
            next = line_str.to_string() + i.to_string().as_str();
            next = next + r"\n";
            file_content.push_str( &next );
        }
        file_content
    }// }}}
// }}}
    /// Prep and return buffer for use in "file buffer" test functions// {{{
    ///
    /// uses test_lines function to create file with which buffer
    /// is initialized
    fn open_file_buffer_test( test_num: u8 ) -> Buffer {// {{{
        let num_lines: usize = 5;   // number of lines to have in buffer
        // generate test file of known content
        let command = Command::new( "echo" )
                        .arg( "-e" )
                        .arg( &test_lines( FILE_CONTENT_LINE, num_lines ) )
                        .output()
                        .expect( "Failed to execute command" );
        let file_mode = FileMode{ f_write: true, f_create: true,
                ..Default::default() };
        let test_file: String = TEST_FILE.to_string() +
                FILE_FILE_SUFFIX + test_num.to_string().as_str();
        let mut file_opened = file_opener( &test_file, file_mode )
                .expect( "Failed to open test file" );
        file_opened.write( &command.stdout )
                .expect( "Failed to write to file" );
        // create new buffer from this file
        Buffer::new( BufferInput::File( test_file ) )
    }// }}}
// }}}
    /// Prep and return buffer for use in "command buffer" test functions// {{{
    ///
    /// uses test_lines function to create file with which buffer
    /// is initialized
    pub fn open_command_buffer_test( test_num: u8, command_line_version: u8 )// {{{
            -> Buffer {
        //
        let num_lines: usize = 7;   // number of lines to have in buffer
        let command_content_line = match command_line_version {
            1_u8 => COMMAND_CONTENT_LINE_1,
            2_u8 => COMMAND_CONTENT_LINE_2,
            _ => "",
        };
        let test_file: String = TEST_FILE.to_string() +
                COMMAND_FILE_SUFFIX + test_num.to_string().as_str();
        let test_command = "echo -e ".to_string() +
                                    &test_lines( command_content_line,
                                    num_lines );
        let mut buffer = Buffer::new( BufferInput::Command( test_command ));
        buffer.set_file_name( &test_file );
        buffer
    }// }}}
// }}}
    /// Prep and return buffer for use in "empty buffer" test functions// {{{
    fn open_empty_buffer_test() -> Buffer {// {{{
        Buffer::new( BufferInput::None )
    }// }}}
// }}}
    /// deconstruct buffer from "file buffer" test// {{{
    /// any other necessary closing actions
    fn close_file_buffer_test( buffer: &mut Buffer ) {// {{{
        match fs::remove_file( buffer.get_file_name()
                                     .unwrap_or( "" ) )
                                     .map_err( |x| RedError::FileRemove(x) ) {
                Err(_) => {
                    println!( "Failed to delete test file" );
                },
                Ok(_) => {},
            }
        buffer.destruct().unwrap();
    }// }}}
// }}}
    /// deconstruct buffer from "command buffer" test;// {{{
    /// any other necessary closing actions
    pub fn close_command_buffer_test( buffer: &mut Buffer ) {// {{{
        buffer.destruct().unwrap();
    }// }}}
// }}}
    // end prep functions// }}}
    // begin test functions {{{
    /// read line from buffer// {{{
    #[test]
    fn file_buffer_test_1() {// {{{
        // Common test start routine
        // set contstants
        let test_num: u8 = 1;
        let test_line: usize = 2;
        //
        let mut buffer = open_file_buffer_test( test_num );
        let expectation =
                FILE_CONTENT_LINE.to_string() + test_line.to_string().as_str();
        //

        // Apply actual test(s)
        assert_eq!( buffer.get_line_content( test_line ).unwrap(),
                &expectation );
        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test get_line_content() values// {{{
    #[test]
    fn file_buffer_test_2() {// {{{
        // set contstants
        let test_num: u8 = 2;
        //
        let mut buffer = open_file_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expectation value
            for test_line in 1 .. buffer.num_lines() {
                expectation = FILE_CONTENT_LINE.to_string() +
                    test_line.to_string().as_str();
                assert_eq!( *buffer.get_line_content( test_line ).unwrap(),
                        expectation );
            }
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test lines_iterator() values// {{{
    #[test]
    fn file_buffer_test_3() {// {{{
        // set contstants
        let test_num: u8 = 3;
        //
        let mut buffer = open_file_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            let mut lines_iter = buffer.lines_iterator();
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expectation value
            for test_line in 1 .. buffer.num_lines() {
                expectation = FILE_CONTENT_LINE.to_string() +
                    test_line.to_string().as_str();
                match lines_iter.next() {
                    Some( line ) => {
                        assert_eq!( *line, expectation );
                    },
                    None => break,
                }
            }
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test get_file_name() and set_file_name() functions// {{{
    #[test]
    fn file_buffer_test_4() {// {{{
        // set contstants
        let test_num: u8 = 4;
        let alt_file_name = "red_anothertest".to_string();
        //
        let mut buffer = open_file_buffer_test( test_num );
        //

        // Apply actual test(s)
        {
            assert_eq!( buffer.get_file_name().unwrap(),
                    TEST_FILE.to_string() +
                    FILE_FILE_SUFFIX + test_num.to_string().as_str() );
            buffer.set_file_name( &alt_file_name );
            assert_eq!( buffer.get_file_name().unwrap(), alt_file_name );
            buffer.set_file_name( &(TEST_FILE.to_string() +
                FILE_FILE_SUFFIX + test_num.to_string().as_str() ));
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test modifying buffer// {{{
    #[test]
    fn file_buffer_test_5() {// {{{
        // set contstants
        let test_num: u8 = 5;
        let test_line: usize = 2;
        let new_line_content: String = "This is the new line!".to_string();
        //
        let mut buffer = open_file_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            expectation = FILE_CONTENT_LINE.to_string() +
                test_line.to_string().as_str();
            assert_eq!( *buffer.get_line_content( test_line ).unwrap(),
                    expectation );
            buffer.set_line_content( test_line, &new_line_content ).unwrap();
            expectation = new_line_content;
            assert_eq!( *buffer.get_line_content( test_line ).unwrap(),
                    expectation );
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Read and compare/test a single line from "command buffer"// {{{
    #[test]
    fn command_buffer_test_1() {// {{{
        // Common test start routine
        // set contstants
        let test_num: u8 = 1;
        let test_line: usize = 2;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let expectation =
                COMMAND_CONTENT_LINE_1.to_string() +
                test_line.to_string().as_str();
        //

        // Apply actual test(s)
        assert_eq!( buffer.get_line_content( test_line ).unwrap(),
                    &expectation );
        //

        // Common test close routine
        close_command_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Read and compare/test a single line from "command buffer"// {{{
    #[test]
    fn command_buffer_test_2() {// {{{
        // Common test start routine
        // set contstants
        let test_num: u8 = 2;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let mut expectation: String;
        //

        // Apply actual test(s)
        for test_line in 1 .. buffer.num_lines() {
            expectation = COMMAND_CONTENT_LINE_1.to_string() +
                test_line.to_string().as_str();
            assert_eq!( *buffer.get_line_content( test_line ).unwrap(),
                    expectation );
        }
        //

        // Common test close routine
        close_command_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test lines_iterator() values// {{{
    #[test]
    fn command_buffer_test_3() {// {{{
        // set contstants
        let test_num: u8 = 3;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            let mut lines_iter = buffer.lines_iterator();
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expected value
            for test_line in 1 .. buffer.num_lines() {
                expectation = COMMAND_CONTENT_LINE_1.to_string() +
                    test_line.to_string().as_str();
                match lines_iter.next() {
                    Some( line ) => {
                        assert_eq!( *line, expectation );
                    },
                    None => break,
                }
            }
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test get_line_content() with spaced, quoted lines// {{{
    #[test]
    fn command_buffer_test_4() {// {{{
        // Common test start routine
        // set contstants
        let test_num: u8 = 4;
        let test_line: usize = 2;
        let command_line_version: u8 = 2;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let expectation =
                COMMAND_CONTENT_LINE_2.to_string() +
                test_line.to_string().as_str();
        //

        // Apply actual test(s)
        assert_eq!( buffer.get_line_content( test_line ).unwrap(),
                    &expectation );
        //

        // Common test close routine
        close_command_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Iterate over each line individually with spaced, quoted lines// {{{
    #[test]
    fn command_buffer_test_5() {// {{{
        // Common test start routine
        // set contstants
        let test_num: u8 = 5;
        let command_line_version: u8 = 2;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let mut expectation: String;
        //

        // Apply actual test(s)
        for test_line in 1 .. buffer.num_lines() {
            expectation = COMMAND_CONTENT_LINE_2.to_string() +
                test_line.to_string().as_str();
            assert_eq!( *buffer.get_line_content( test_line ).unwrap(),
                    expectation );
        }
        //

        // Common test close routine
        close_command_buffer_test( &mut buffer );
    }// }}}
// }}}
    /// Test lines_iterator() values with spaced, quoted lines// {{{
    #[test]
    fn command_buffer_test_6() {// {{{
        // set contstants
        let test_num: u8 = 6;
        let command_line_version: u8 = 2;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            let mut lines_iter = buffer.lines_iterator();
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expected value
            for test_line in 1 .. buffer.num_lines() {
                expectation = COMMAND_CONTENT_LINE_2.to_string() +
                    test_line.to_string().as_str();
                match lines_iter.next() {
                    Some( line ) => {
                        assert_eq!( *line, expectation );
                    },
                    None => break,
                }
            }
        }

        //

        // Common test close routine
        close_file_buffer_test( &mut buffer );
    }// }}}
// }}}
    /*
    #[test]
    fn empty_buffer_test() {// {{{
        let buffer = open_empty_buffer_test();
    }// }}}
    */
    // end test functions }}}
}// }}}

