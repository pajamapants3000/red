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
use std::io::{BufReader, BufWriter, stdout};
use std::fs::{self, File, rename};
use std::path::{Path, PathBuf};
use std::collections::LinkedList;
use std::collections::linked_list::{self, Iter};
use std::iter::{IntoIterator, FromIterator, Iterator, Take, Skip};
use std::{thread, time};
use std::ffi::{OsStr,OsString};

use ::chrono::*;
use ::regex::{Regex, Captures};
use ::rand::{thread_rng, Rng};

use io::*;
use error::*;
use parse::*;

// ^^^ Bring in to namespace ^^^ }}}
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}

// *** Constants *** {{{
pub const NUM_LC: usize = 26;
const SAVE_RETRIES: usize = 3;
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
    file: Option<OsString>,
    /// timestamped path of file where buffer is stored regularly
    buffer_file: OsString,  // convert to Path later
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
    //last_update: DateTime<UTC>,
    /// Date and time of last write to disk under temporary file name
    last_temp_write: DateTime<UTC>,
    /// Date and time of last write to disk under permanent file name
    last_write: DateTime<UTC>,
}   //}}}
// }}}
impl Buffer {   //{{{
    /// Initialize new Buffer instance// {{{
    pub fn new( content: BufferInput ) -> Result<Buffer, RedError> {//{{{
        let mut _lines = Buffer::init_lines( &content );
        let _total_lines = _lines.len();
        let mut result = Buffer {
            lines: _lines,
            buffer_file: match &content {
                &BufferInput::File( ref file_name ) =>
                    temp_file_name( Some( file_name.as_str() )),
                _ => temp_file_name( None::<&str> ),
            },
            markers: vec!( 0; NUM_LC ),
            current_line: _total_lines,     // usize; should be Copy
            total_lines: _total_lines,
            _is_modified: false,
            /*  Comment until it becomes relevant
            last_update: match &content {
                &BufferInput::File(_) => UTC::now(),
                _ => get_null_time(),
            },
            */
            last_temp_write: match &content {
                &BufferInput::File(_) => UTC::now(),
                _ => get_null_time(),
            },
            last_write: get_null_time(),
            file: None,
        };

        match content {
            BufferInput::File( file_name ) =>
                match result.set_file( &file_name ) {
                    Ok( () ) => {},
                    Err(_) => {},
                },
            BufferInput::Command( command ) =>
                match result.set_file( &command.split_whitespace().next()
                                    .unwrap_or("command_stdout") ) {
                    Ok( () ) => {},
                    Err(_) => {},
                },
            _ => {},
        };
        let half_second: time::Duration = time::Duration::from_millis(500);
        for attempt in 1 .. ( SAVE_RETRIES + 1 ) {
            match result.store_buffer() {
                Ok(_) => {
                    return Ok( result );
                },
                Err(e) => {
                    println!( "Attempt {}/{}: unable to store buffer",
                            attempt, SAVE_RETRIES );
                    let _stdout = stdout();
                    let mut handle = _stdout.lock();
                    thread::sleep(half_second);
                    handle.write( b"." )
                        .expect("buf::Buffer::new: fail to write to stdout");
                    handle.flush()
                        .expect("buf::Buffer::new: fail to write to stdout");
                    thread::sleep(half_second);
                    handle.write( b"." )
                        .expect("buf::Buffer::new: fail to write to stdout");
                    handle.flush()
                        .expect("buf::Buffer::new: fail to write to stdout");
                    thread::sleep(half_second);
                    handle.write( b"." )
                        .expect("buf::Buffer::new: fail to write to stdout");
                    handle.flush()
                        .expect("buf::Buffer::new: fail to write to stdout");
                    thread::sleep(half_second);
                    handle.write( b".\n" )
                        .expect("buf::Buffer::new: fail to write to stdout");
                    handle.flush()
                        .expect("buf::Buffer::new: fail to write to stdout");
                    if attempt == SAVE_RETRIES {
                        return Err( e );
                    }
                },
            }
        }
        // not reached due to `if attempt == (SAVE_RETRIES - 1)`
        Err( RedError::CriticalError( "reached unreachable line".to_string() ))
    }//}}}
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
    /// TODO: This is a terrible approach - you have direct
    /// access to the private members, don't waste a function
    /// call! Plus the iteration... should be able to find a
    /// better way.
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
    /// Return iterator over range of lines in buffer// {{{
    ///
    pub fn range_iterator<'a>( &'a self, address_initial: usize,// {{{
                           address_final: usize ) -> Take<
            Skip<linked_list::Iter<'a, String> >> {
        // Make sure caller did their job!
        assert_addresses( address_initial, address_final, self.total_lines );
        // now let's do ours...
        let lines_ref: &'a LinkedList<String> = &self.lines;
        lines_ref.into_iter()
            .skip( address_initial - 1 )
            .take(( address_final + 1 ) - address_initial )
    }// }}}
// }}}
    /// Return iterator over all lines in buffer// {{{
    ///
    pub fn lines_iterator( &self ) -> Iter<String> {
        let lines_ref: &LinkedList<String> = &self.lines;
        lines_ref.into_iter()
    }// }}}
// }}}
    /// Return reference to working file name string// {{{
    pub fn get_file_name( &self ) -> Option<&OsStr> {// {{{
        match self.file {
            Some( ref _file ) => {
                let file_path = Path::new( _file );
                Some( file_path.file_name()
                      .expect("path does not have a file name")
                    )
            },
            None => None,
        }
    }// }}}
// }}}
    /// Return reference to working file name string// {{{
    #[cfg(test)]
    pub fn get_file_path( &self ) -> Option<&OsStr> {// {{{
        match &self.file {
            &Some( ref file_path ) => Some( file_path ),
            &None => None,
        }
    }// }}}
// }}}
    /// Set new working file name; remove file at old name// {{{
    #[cfg(test)]
    pub fn move_file( &mut self, path: &str )
            -> Result<(), RedError> {// {{{
        match &self.file {
            &Some(ref f) => { let _ = fs::remove_file(f).is_ok(); },
            &None => {},
        };
        try!( self.set_file( path ) );
        Ok( () )
    }//}}}
//}}}

    /// Set new working file name// {{{
    ///
    /// At some point, need to test for existing file and ask user if overwrite
    pub fn set_file<S: AsRef<OsStr> + ?Sized>( &mut self, path: &S )
            -> Result<(), RedError> {// {{{
        let result: Option<OsString>;
        let file_path = Path::new( path );
        // abort if no valid file name provided
        let file_name = match file_path.file_name() {
            Some(f) => f,
            None => return Err( RedError::ParameterSyntax{ parameter:
                "file_name: ".to_string() +
                    path.as_ref().to_str().unwrap_or("<invalid UTF-8>") } ),
        };
        result = match file_path.canonicalize() {
            Ok(pb) => Some( pb.into_os_string() ),
            Err(_) => {
                let mut file_pathbuf = file_path.to_path_buf();
                file_pathbuf.pop();
                let parent = file_pathbuf.as_path();
                match parent.canonicalize() {
                    Ok(mut pb) => {
                       pb.push(file_name);
                       Some( pb.into_os_string() )
                    },
                    Err(_) => Some( file_name.to_os_string() ),
                }
            },
        };
        self.file = result.clone();
        let _ = fs::remove_file( &self.buffer_file ).is_ok();
        self.buffer_file = temp_file_name( Some( &result.unwrap() ));
        try!( self.store_buffer() );
        Ok( () )
    }// }}}
// }}}
    /// Delete line// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    ///     actually, I think I should remove error handling here; only error
    ///     would be line doesn't exist, in which case we can just do nothing.
    pub fn delete_line( &mut self, address: usize )
            -> Result<(), RedError> {// {{{
        if address > self.lines.len() {
            return Err( RedError::GetLineOutOfBounds{ address: address } );
        }
        let mut back = self.lines.split_off( address );
        match self.lines.pop_back() {
            Some(_) => {},
            None => {
                return Err(
                    RedError::GetLineOutOfBounds{ address: address } );
            },
        }
        self.lines.append( &mut back );
        self.delete_update_markers( address );
        self.total_lines -= 1; // previous tests preclude underflow here?
        self._is_modified = true;
        Ok( () )
    }// }}}
// }}}
    /// Insert new line at current position// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    ///     I don't think that will be necessary
    pub fn append_here( &mut self, new_line: &str ) {// {{{
        let address = self.current_line;
        self.append_line( address, new_line );
    }// }}}
// }}}
    /// Insert new line// {{{
    ///
    /// TODO: Add error handling, Result<> return?
    ///     I don't think that will be necessary
    pub fn append_line( &mut self, address: usize, new_line: &str ) {// {{{
        let mut back = self.lines.split_off( address );
        self.lines.push_back( new_line.to_string() );
        self.lines.append( &mut back );
        self.current_line = address + 1;   // next line
        self.insert_update_markers( address );
        self._is_modified = true;
        self.total_lines += 1;
    }// }}}
// }}}
    /// Replace line with new string// {{{
    ///
    /// TODO: Add error handling; panics if address > len
    ///     I don't think that will be necessary
    pub fn set_line_content( &mut self, address: usize, new_line: &str )// {{{
            -> Result<(), RedError> {
        if address > self.lines.len() {
            return Err( RedError::SetLineOutOfBounds{ address: address } );
        }
        let mut back_list = self.lines.split_off( address - 1 );
        let _ = back_list.pop_front();
        self.lines.push_back( new_line.to_string() );
        self.lines.append( &mut back_list );
        self._is_modified = true;
        Ok( () )
    }// }}}
// }}}
    /// Return current working line number// {{{
    pub fn get_current_address( &self ) -> usize {// {{{
        self.current_line
    }// }}}
// }}}
    /// Move "cursor" to new line// {{{
    pub fn set_current_address( &mut self, address: usize ) {// {{{
        if address < self.total_lines {
            self.current_line = address;
        } else {
            self.current_line = self.total_lines;
        }
    }// }}}
// }}}
    /// Return number of line with a specified mark set// {{{
    pub fn get_marked_line( &self, label: char ) -> usize {// {{{
        if 'a' <= label && label <= 'z' {
            self.markers[ (( label as u8 ) - ( 'a' as u8 )) as usize ]
        } else {
            0_usize
        }
    }// }}}
// }}}
    /// Add new line marker// {{{
    ///
    pub fn set_marker( &mut self, label: char, line: usize ) {// {{{
        self.markers[ (( label as u8 ) - ( 'a' as u8 )) as usize ] = line;
    }// }}}
// }}}
    /// Return immutable slice over all markers// {{{
    ///
    /// Will add operation for this at some point; until then, commented out
    /*
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
    */
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
            &Some(ref x) => temp_file_name( Some(x) ),
            &None => temp_file_name( None::<&OsString> ),
        };
        try!( rename( &self.buffer_file, &new_buffer_file )
              .map_err(|e| RedError::FileRename(e) )
            );
        self.buffer_file = new_buffer_file;
        self.last_temp_write = UTC::now();
        Ok( () )
    }// }}}
// }}}
    /// Save work to permanent file; behavior depends on do_append argument// {{{
    ///
    /// TODO: move to io.rs? I don't think so, it's a part of the
    /// functionality of the buffer
    /// TODO: set up default filename?
    /// # Panics
    /// # Errors
    /// # Safety
    /// # Examples
    pub fn write_to_disk<S: AsRef<OsStr> + ?Sized>( &mut self,// {{{
                file_name: &S, do_append: bool, address_initial: usize,
                address_final: usize ) -> Result<(), RedError> {
        // Make sure caller did their job!
        assert_addresses( address_initial, address_final, self.total_lines );
        // now let's do ours...
        let file_mode = FileMode{ f_write: !do_append, f_truncate: !do_append,
                f_append: do_append, f_create: true, ..Default::default() };
        // set as default file if one provided but not previously set
        if !file_name.as_ref().is_empty() && self.file == None {
            self.file = Some( file_name.as_ref().to_os_string() );
        }
        // use provided file, or default if one not provided
        let file_to_use: &OsStr = match file_name.as_ref().is_empty() {
            true => {
                match &self.file {
                    &Some(ref f) => {
                        f
                    },
                    &None => {
                        println!("No file name chosen for save");
                        return Err(
                            RedError::ParameterSyntax{
                                parameter: "".to_string() });
                    },
                }
            },
            false => file_name.as_ref(),
        };
        let mut file_opened = try!( file_opener( file_to_use, file_mode ));
        for line in self.range_iterator( address_initial, address_final ) {
            try!( file_opened.write( line.as_bytes() )
                  .map_err(|e| RedError::FileWrite(e) ));
            try!( file_opened.write( &[b'\n'] )
                  .map_err(|e| RedError::FileWrite(e) ));
        }
        if file_name.as_ref().is_empty() && address_initial == 1 &&
                address_final == self.total_lines {
            self.last_write = UTC::now();
            self._is_modified = false;
        }
        Ok( () )

    }// }}}
// }}}
    /// Pattern match predicate // {{{
    ///
    /// Returns true if line has pattern match, false otherwise
    pub fn does_match( &self, regex: &str, address: usize ) -> bool {// {{{
        let re = Regex::new( regex ).unwrap();
        re.is_match( self.get_line_content( address ).unwrap_or("") )
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
    /// Prepare for closing buffer// {{{
    pub fn on_close( &mut self )
            -> Result<(), RedError> {// {{{
        if self.is_modified() {
            return Err( RedError::NoDestruct );
        }
        try!( fs::remove_file( &self.buffer_file ).map_err(|e|
                                                    RedError::FileRemove(e) ));
        Ok( () )
    }//}}}
// }}}
    /// Deconstruct buffer// {{{
    /// I'm not sure we need this function, since opening a new
    /// file or command just creates a new Buffer instance and the
    /// old one should be deleted automatically; use on_close to
    /// check for unsaved changes and ensure the buffer temp file
    /// is deleted.
    /// Until I can determine if this is needed, I will keep it and
    /// just comment it out.
    #[cfg(test)]
    pub fn destruct( &mut self ) {// {{{
        match self.get_file_path() {
            // RedError::FileRemove?
            Some(f) => { let _ = fs::remove_file(f).is_ok(); }
            None => {},

        }
        let _ = fs::remove_file( &self.buffer_file ).is_ok();
        // restore all values to defaults - necessary?
        self.lines.clear();
        self.file = None;
        self.buffer_file = OsStr::new( "" ).to_os_string();
        self.markers = Vec::new();
        self.current_line = 0;
        self.total_lines = 0;
        self._is_modified = false;
        //XXX: self.last_update = get_null_time();
        self.last_temp_write = get_null_time();
        self.last_write = get_null_time();
    }// }}}
    /// Keep markers valid after inserting new line
    ///
    /// I can't think of any errors that might go here
    fn insert_update_markers( &mut self, address: usize ) {
        for marker in &mut self.markers {
            if *marker > address {
                *marker += 1;
            }
        }
    }
    /// Keep markers valid after deleting line;
    /// delete any markers to deleted line
    ///
    /// I can't think of any errors that might go here
    fn delete_update_markers( &mut self, address: usize ) {
        for marker in &mut self.markers {
            if *marker > address {
                *marker -= 1;
            } else if *marker == address {
                *marker = 0;
            }
        }
    }
// }}}
    /// make substitution in range of lines// {{{
    pub fn substitute( &mut self, to_match: &str, to_sub: &str,// {{{
                       which: WhichMatch,
                       address_initial: usize, address_final: usize ) {
        let re: Regex = Regex::new( to_match ).unwrap();
        for line in address_initial .. address_final + 1 {
            self._substitute_line( line, &re, to_sub, &which );
        }
    }// }}}
// }}}
    fn _substitute_line( &mut self, address: usize, re_to_match: &Regex,// {{{
                     to_sub: &str, which: &WhichMatch ) {
        let mut new_line: String = String::new();
        {   // create wrapping namespace
            let line_content = self.get_line_content( address )
                .expect("Line outside range"); // XXX: shouldn't be possible here!
            let mut all_matches = re_to_match.find_iter( line_content );
            let mut all_captures = re_to_match.captures_iter( line_content );
            let mut _capture: Captures;
            let mut to_sub_w_backrefs: String;
            let mut count: usize = 0;
            let mut last_end: usize = 0;
            loop {
                count += 1;
                match all_matches.next() {
                    Some(( start, end )) => {
                        _capture = all_captures.next()
                            .expect("Fewer captures than matches ...?");
                        to_sub_w_backrefs = sub_captures( to_sub, _capture );
                        match which {
                            &WhichMatch::Number(n) => {
                                if n == count {
                                    new_line += &line_content[last_end..start];
                                    new_line += &to_sub_w_backrefs;
                                } else {
                                    new_line += &line_content[last_end..end];
                                }
                            },
                            &WhichMatch::Global => {
                                new_line += &line_content[last_end..start];
                                new_line += &to_sub_w_backrefs;
                            },
                        }
                        last_end = end;
                    },
                    None => {
                        new_line += &line_content[ last_end .. ];
                        break;
                    },
                }
            }
        }
        // Approach 1 - set line content regardless, sometimes to same
        self.set_line_content( address, &new_line )
            .expect("error setting line content");
        // Approach 2 - repeat the above match on sub_parms.which
    }// }}}
    pub fn join_lines( &mut self,
                       address_initial: usize, address_final: usize )
            -> Result<(), RedError> {
        let mut new_line = String::new();
        let mut line: Option<String>;
        for _ in address_initial .. address_final + 1 {
            line = self.get_line_content( address_initial )
                .map(|x| x.to_string() );
            match line {
                Some(x) => {
                    new_line.push_str( &x );
                    try!( self.delete_line( address_initial ));
                },
                None => break,
            }
        }
        if address_initial == 0 {
            self.append_line( 0, &new_line );
            self.current_line = 1;
        } else {
            self.append_line( address_initial - 1, &new_line );
            self.current_line = address_initial;
        }
        try!( self.store_buffer() );
        Ok( () )
    }
    pub fn move_lines( &mut self, address_initial: &usize,// {{{
                       address_final: &usize, destination: &usize )
            -> Result<(), RedError> {
        let mut _initial:usize = *address_initial;
        let mut _final:usize = *address_final;
        let mut _destination:usize = *destination;
        // no need to move anything if initial < dest < final
        if (_initial-1) <= _destination && _destination <= _final {
            return Ok( () );
        }
        let mut offset: usize = 0;
        let mut line: String;
        for address in _initial .. _final + 1 {
            line = self.get_line_content( address - offset )
                .unwrap_or("").to_string();
            try!( self.delete_line( address - offset ));
            if ( address - offset ) > _destination {
                self.append_line( _destination, &line );
            } else {
                offset += 1;
                self.append_line( _destination - offset, &line );
            }
            _destination += 1;
        }
        Ok( () )
    }// }}}
    pub fn copy_lines( &mut self, address_initial: usize,// {{{
                       address_final: usize, destination: usize )
            -> Result<(), RedError> {
        let mut line: String;
        for address in address_initial .. address_final + 1 {
            if address > destination {
                line = self.get_line_content( 2 * address - address_initial )
                        .unwrap_or("").to_string();
            } else {
                line = self.get_line_content( address )
                        .unwrap_or("").to_string();
            }
            self.append_line( destination + address - address_initial, &line );
        }
        self.current_line = destination + 1 + address_final - address_initial;
        Ok( () )
    }// }}}
}   //}}}
impl Clone for Buffer {
    fn clone( &self ) -> Self {
        let _lines = self.lines.clone();
        let _file = match &self.file {
            &Some(ref f) => Some(f.clone()),
            &None => None,
        };
        let mut _markers: Vec<usize> = Vec::new();
        for marker in &self.markers {
            _markers.push( *marker );
        }
        let _last_temp_write = self.last_temp_write.clone();
        let _last_write = self.last_write.clone();
        Buffer{
            lines: _lines,
            buffer_file: self.buffer_file.clone(),
            file: _file,
            markers: _markers,
            _is_modified: self._is_modified,
            current_line: self.current_line,
            total_lines: self.total_lines,
            last_write: _last_write,
            last_temp_write: _last_temp_write,
        }
    }
}

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
/// Produce random string of characters
pub fn random_string() -> String {
    let result: String = thread_rng().gen_ascii_chars().take(8).collect();
    result
}
/// Produce name for temporary buffer storage// {{{
///
/// # Panics
/// # Errors
/// We don't want any errors to be possible here - whatever the case,
/// return something to use as a temp file name for storing the buffer
/// # Safety
/// # Examples
fn temp_file_name<S: AsRef<OsStr> + ?Sized>( path_str: Option<&S> )
        -> OsString {//{{{
    // only way to conflict is by choosing the same eight alphanumeric
    // characters in less than a second!
    let _random_string = random_string();
    let mut path: PathBuf = Path::new( path_str.map( |s| s.as_ref() )
                                       .unwrap_or(OsStr::new("temp")) )
                                       .to_path_buf();
    let mut _temp_file_name = OsStr::new(".red.").to_os_string();
    _temp_file_name.push( path.file_name().unwrap_or(
            OsStr::new( ("temp.".to_string() + _random_string.as_str() + "." +
                       &get_timestamp()).as_str() )));

    path.pop();
    match path.canonicalize() {
        Ok(p) => {
            path = p;
            path.push( _temp_file_name );
        },
        Err(_) => path = Path::new( &_temp_file_name ).to_path_buf(),
    }
    path.into_os_string()
}// }}}
// }}}
// ^^^ Functions ^^^ }}}

#[cfg(test)]
mod tests {// {{{
    //  ***     ***      Bring into namespace   ***     *** //// {{{
    use std::process;
    use std::io::Write;
    use std::ffi::OsStr;
    use std::default::Default;

    use super::*;
    use io::*;
    use parse::*;
    //  ^^^     ^^^     Bring into namespace    ^^^     ^^^ //// }}}
    //  ***     ***     Constants   ***     ***     //// {{{
    const TEST_FILE: &'static str = "red_filetest";
    const FILE_CONTENT_LINE: &'static str = "testfile line number";
    const COMMAND_CONTENT_LINE: &'static str = "testcmd line number";
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
    fn open_file_buffer_test_gen( test_num: u8, file_content_line: &str )
            -> Buffer {// {{{
        let num_lines: usize = 21;   // number of lines to have in buffer
        // generate test file of known content
        let command = process::Command::new( "echo" )
                        .arg( "-e" )
                        .arg( &test_lines( file_content_line, num_lines ) )
                        .output()
                        .expect( "Failed to execute command" );
        let file_mode = FileMode{ f_write: true, f_create: true,
                ..Default::default() };
        let test_file: String = TEST_FILE.to_string() + "." +
                random_string().as_str() +
                FILE_FILE_SUFFIX + test_num.to_string().as_str();
        let mut file_opened = file_opener( &test_file, file_mode )
                .expect( "Failed to open test file" );
        file_opened.write( &command.stdout )
                .expect( "Failed to write to file" );
        // create new buffer from this file
        Buffer::new( BufferInput::File( test_file )).unwrap()
    }// }}}
// }}}
    /// Call file test buffer generator//{{{
    fn open_file_buffer_test( test_num: u8 ) -> Buffer {// {{{
        open_file_buffer_test_gen( test_num, FILE_CONTENT_LINE )
    }// }}}
// }}}
    /// Prep and return buffer for use in "command buffer" test functions// {{{
    ///
    /// uses test_lines function to create file with which buffer
    /// is initialized
    pub fn open_command_buffer_test_gen( test_num: u8, cmd_content_line: &str )// {{{
            -> Buffer {
        //
        let num_lines: usize = 21;   // number of lines to have in buffer
        let test_file: String = TEST_FILE.to_string() + "." +
                random_string().as_str() +
                COMMAND_FILE_SUFFIX + test_num.to_string().as_str();
        let test_command = "echo -e ".to_string() +
                                    &test_lines( cmd_content_line,
                                    num_lines );
        let mut buffer = Buffer::new( BufferInput::Command( test_command ))
            .unwrap();
        buffer.move_file( &test_file ).unwrap();
        buffer
    }// }}}
// }}}
    /// Call command test buffer generator//{{{
    pub fn open_command_buffer_test( test_num: u8 )// {{{
            -> Buffer {
        open_command_buffer_test_gen( test_num, COMMAND_CONTENT_LINE )
    }// }}}
// }}}
    /// Prep and return buffer for use in "empty buffer" test functions// {{{
    /*
    fn open_empty_buffer_test() -> Buffer {// {{{
        Buffer::new( BufferInput::None ).unwrap()
    }// }}}
// }}}
*/
    /// deconstruct buffer from "file buffer" test// {{{
    /// any other necessary closing actions
    fn close_file_buffer_test( buffer: &mut Buffer ) {// {{{
        buffer.destruct();
    }// }}}
// }}}
    /// deconstruct buffer from "command buffer" test;// {{{
    /// any other necessary closing actions
    pub fn close_command_buffer_test( buffer: &mut Buffer ) {// {{{
        buffer.destruct();
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
    /// Test get_file_path() and set_file() functions// {{{
    #[test]
    fn file_buffer_test_4() {// {{{
        // set contstants
        let test_num: u8 = 4;
        let alt_file_name = "red_anothertest".to_string();
        let mut buffer = open_file_buffer_test( test_num );
        buffer.move_file( &(TEST_FILE.to_string() + FILE_FILE_SUFFIX +
                test_num.to_string().as_str() )).unwrap();
        //

        // Apply actual test(s)
        {
            assert_eq!( buffer.get_file_name().unwrap_or(OsStr::new("")),
                    &OsStr::new( &(TEST_FILE.to_string() + FILE_FILE_SUFFIX +
                    test_num.to_string().as_str()) ).to_os_string() );
            buffer.set_file( &alt_file_name ).unwrap();
            assert_eq!( buffer.get_file_path().unwrap_or(OsStr::new("")),
                    OsStr::new( &alt_file_name ) );
            buffer.set_file( &(TEST_FILE.to_string() +
                FILE_FILE_SUFFIX + test_num.to_string().as_str() )).unwrap();
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
        let mut buffer = open_command_buffer_test( test_num );
        let expectation =
                COMMAND_CONTENT_LINE.to_string() +
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
        let mut buffer = open_command_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        for test_line in 1 .. buffer.num_lines() {
            expectation = COMMAND_CONTENT_LINE.to_string() +
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
        let mut buffer = open_command_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            let mut lines_iter = buffer.lines_iterator();
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expected value
            for test_line in 1 .. buffer.num_lines() {
                expectation = COMMAND_CONTENT_LINE.to_string() +
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
        let mut buffer = open_command_buffer_test( test_num );
        let expectation =
                COMMAND_CONTENT_LINE.to_string() +
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
        let mut buffer = open_command_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        for test_line in 1 .. buffer.num_lines() {
            expectation = COMMAND_CONTENT_LINE.to_string() +
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
        let mut buffer = open_command_buffer_test( test_num );
        let mut expectation: String;
        //

        // Apply actual test(s)
        {
            let mut lines_iter = buffer.lines_iterator();
            // NOTE: We don't iterate to num_lines+1 because the last line
            // is blank and won't match the expected value
            for test_line in 1 .. buffer.num_lines() {
                expectation = COMMAND_CONTENT_LINE.to_string() +
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
    #[test]
    fn substitute_test_1() {// {{{
        let test_num: u8 = 1;
        let mut buffer = open_file_buffer_test( test_num );
        let num_lines = buffer.num_lines() - 1;
        let expectation: String = "line testfile number".to_string();
        let mut _expectation: String;
        let regex_str: &str = r#"(testfile) (line)"#;
        let to_sub: &str = r#"\2 \1"#;

        // Apply actual test(s)
        buffer.substitute(regex_str, to_sub, WhichMatch::Global, 1, num_lines);
        let mut count = 0_usize;
        for line in buffer.lines_iterator() {
            count += 1;
            if count > num_lines {
                break;
            }
            _expectation = expectation.clone();
            _expectation.push_str( &count.to_string() );
            assert_eq!( _expectation, *line );
        }
        close_file_buffer_test( &mut buffer );
    }// }}}
    #[test]
    fn substitute_test_2() {// {{{
        let test_num: u8 = 2;
        let mut buffer = open_file_buffer_test_gen( test_num,
                    "one two three four five -" );
        let num_lines = buffer.num_lines() - 1;
        let expectation: String = "five four three two one -".to_string();
        let mut _expectation: String;
        let regex_str: &str = r#"(one) (two) (three) (four) (five)"#;
        let to_sub: &str = r#"\5 \4 \3 \2 \1"#;

        // Apply actual test(s)
        buffer.substitute(regex_str, to_sub, WhichMatch::Global, 1, num_lines);
        let mut count = 0_usize;
        for line in buffer.lines_iterator() {
            count += 1;
            if count > num_lines {
                break;
            }
            _expectation = expectation.clone();
            _expectation.push_str( &count.to_string() );
            assert_eq!( _expectation, *line );
        }
        close_file_buffer_test( &mut buffer );
    }// }}}
    #[test]
    fn substitute_test_3() {// {{{
        let test_num: u8 = 3;
        let mut buffer = open_file_buffer_test( test_num );
        let num_lines = buffer.num_lines() - 1;
        let mut _expectation: String;
        let regex_str: &str = r#"e"#;
        let to_sub: &str = r#"x"#;
        let expectation: String = "txstfile line number".to_string();

        // Apply actual test(s)
        buffer.substitute( regex_str, to_sub, WhichMatch::Number(1),
                           1, num_lines );
        let mut count = 0_usize;
        for line in buffer.lines_iterator() {
            count += 1;
            if count > num_lines {
                break;
            }
            _expectation = expectation.clone();
            _expectation.push_str( &count.to_string() );
            assert_eq!( _expectation, *line );
        }
        close_file_buffer_test( &mut buffer );
    }// }}}
    #[test]
    fn substitute_test_4() {// {{{
        let test_num: u8 = 4;
        let mut buffer = open_file_buffer_test( test_num );
        let num_lines = buffer.num_lines() - 1;
        let mut _expectation: String;
        let regex_str: &str = r#"e"#;
        let to_sub: &str = r#"x"#;
        let expectation: String = "testfile linx number".to_string();

        // Apply actual test(s)
        buffer.substitute( regex_str, to_sub, WhichMatch::Number(3),
                           1, num_lines );
        let mut count = 0_usize;
        for line in buffer.lines_iterator() {
            count += 1;
            if count > num_lines {
                break;
            }
            _expectation = expectation.clone();
            _expectation.push_str( &count.to_string() );
            assert_eq!( _expectation, *line );
        }
        close_file_buffer_test( &mut buffer );
    }// }}}
    #[test]
    fn substitute_test_5() {// {{{
        let test_num: u8 = 5;
        let mut buffer = open_file_buffer_test( test_num );
        let num_lines = buffer.num_lines() - 1;
        let mut _expectation: String;
        let regex_str: &str = r#"e"#;
        let to_sub: &str = r#"x"#;
        let expectation: String = "txstfilx linx numbxr".to_string();

        // Apply actual test(s)
        buffer.substitute( regex_str, to_sub, WhichMatch::Global,
                           1, num_lines );
        let mut count = 0_usize;
        for line in buffer.lines_iterator() {
            count += 1;
            if count > num_lines {
                break;
            }
            _expectation = expectation.clone();
            _expectation.push_str( &count.to_string() );
            assert_eq!( _expectation, *line );
        }
        close_file_buffer_test( &mut buffer );
    }// }}}
    #[test]
    fn substitute_test_6() {// {{{
        let test_num: u8 = 6;
        let mut buffer = open_file_buffer_test( test_num );
        let regex_str: &str = r#"e"#;
        let to_sub: &str = r#"x"#;
        let expectation: String = "txstfilx linx numbxr8".to_string();

        // Apply actual test(s)
        buffer.substitute( regex_str, to_sub, WhichMatch::Global, 8, 8 );
        assert_eq!( expectation, buffer.get_line_content(8).unwrap() );
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

