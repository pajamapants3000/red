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
use std::io::{BufReader,BufWriter};
use std::fs::{File, OpenOptions};

use io::*

// ^^^ Bring in to namespace ^^^ }}}
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
// *** Constants *** {{{
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
pub struct Buffer {
    pub reader: BufReader,
    pub writer: BufWriter,
    buffer_name: String,
    buffer_opened: File,
    current_line: usize,
}
impl Buffer {
    fn new( _buffer_name: String, _buffer_opened: File ) -> Buffer {
        let mut reader = BufReader::new( _buffer_opened );
        let mut writer = BufWriter::new( _buffer_opened );

        Buffer {
            buffer_name: _buffer_name,
            buffer_opened: _buffer_opened,
            current_line: 
        }
    }
    fn get_file_name( &self ) -> String {
        self.file_name
    }
    fn get_current_line( &self ) -> usize {
        self.cursor
    }
}
// ^^^ Data Structures ^^^ }}}
// *** Functions *** {{{

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

fn get_timestamp() -> String {
    let dt = chrono::Local::now();

    dt.format("%Y%m%d%H%M%S").to_string()

}

fn get_line_offset( buffer: Buffer,  line: usize ) -> usize {
    let current_position: usize = buffer.reader( 
}

// ^^^ Functions ^^^ }}}

