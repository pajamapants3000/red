/*
 * File   : command.rs
 * Purpose: defines functions which carry out possible user commands
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : All operation functions have same signature
 * Created: 11/05/2016
 */

//! Operations callable by the user during program execution
//!
//! All operations take as arguments the current buffer, editor state, and
//! command (these may eventually be condensed into editor state) and return
//! Result<(), RedError>
//!
//! All operations assume that the addresses passed through the command have
//! already been checked for validity and range. This is handled when the
//! command is parsed, and so is a safe assumption.
// *** Bring in to namespace *** {{{
use std::collections::hash_map::HashMap;
use std::process::exit;
use std::io::{Write, BufRead, BufWriter, stdout, StdoutLock, stdin};

use regex::{Regex, Captures};

use buf::*;
use error::*;
use parse::*;
use ::{EditorState, EditorMode, print_help, term_size};
// ^^^ Bring in to namespace ^^^ }}}

// *** Attributes *** {{{
const NUM_OPERATIONS: usize = 26;
const COMMAND_PREFIX: &'static str = "@";
// ^^^ Attributes ^^^ }}}

// *** Constants *** {{{
// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
pub struct Operations {
    operation_map: HashMap<char, Box<Fn( &mut Buffer, &mut EditorState,
                                         Command ) -> Result<(), RedError>> >,
}
impl Operations {// {{{
    /// Creates Operations HashMap// {{{
    pub fn new() -> Operations {// {{{
        let mut _operation_map: HashMap<char, Box<Fn( &mut Buffer,
              &mut EditorState, Command ) -> Result<(), RedError>> > =
            HashMap::with_capacity( NUM_OPERATIONS );
        _operation_map.insert( 'a', Box::new(append) );
        _operation_map.insert( 'c', Box::new(change) );
        _operation_map.insert( 'd', Box::new(delete) );
        _operation_map.insert( 'e', Box::new(edit) );
        _operation_map.insert( 'E', Box::new(edit_unsafe) );
        _operation_map.insert( 'f', Box::new(filename) );
        _operation_map.insert( 'g', Box::new(global) );
        _operation_map.insert( 'G', Box::new(global_interactive) );
        _operation_map.insert( 'h', Box::new(help_recall) );
        _operation_map.insert( 'H', Box::new(help_tgl) );
        _operation_map.insert( 'i', Box::new(insert) );
        _operation_map.insert( 'j', Box::new(join) );
        _operation_map.insert( 'k', Box::new(mark) );
        _operation_map.insert( 'l', Box::new(lines_list) );
        _operation_map.insert( 'm', Box::new(move_lines) );
        _operation_map.insert( 'n', Box::new(print_numbered) );
        _operation_map.insert( 'p', Box::new(print) );
        _operation_map.insert( 'q', Box::new(quit) );
        _operation_map.insert( 'r', Box::new(read) );
        _operation_map.insert( 's', Box::new(substitute) );
        _operation_map.insert( 't', Box::new(transfer) );
        _operation_map.insert( 'u', Box::new(undo) );
        _operation_map.insert( 'v', Box::new(global_reverse)  );
        _operation_map.insert( 'V', Box::new(global_reverse_interactive) );
        _operation_map.insert( 'w', Box::new(write_to_disk) );
        _operation_map.insert( 'W', Box::new(append_to_disk) );

        Operations { operation_map: _operation_map }
    }// }}}
// }}}
    /// Execute command// {{{
    pub fn execute( &self, buffer: &mut Buffer, state: &mut EditorState,//{{{
                    command: Command ) -> Result<(), RedError> {
        match self.operation_map.contains_key( &command.operation ) {
            true => {
                let op_to_execute = self.operation_map
                    .get( &command.operation ).unwrap();
                op_to_execute( buffer, state, command )
            },
            false => {
                Err( RedError::InvalidOperation{ operation: command.operation } )
            },
        }
    }// }}}
// }}}
}// }}}
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Avoid `unused` warnings for functions that don't modify mode// {{{
fn mode_noop( mode: &mut EditorMode ) -> EditorMode {// {{{
    match mode {
        &mut EditorMode::Command => EditorMode::Command,
        &mut EditorMode::Insert => EditorMode::Insert,
    }
}// }}}
// }}}
/// Avoid `unused` warnings for functions that don't modify buffer// {{{
fn buffer_noop( buffer: &mut Buffer ) -> &mut Buffer {// {{{
    let temp = buffer.get_current_line_number();
    buffer.set_current_line_number( temp );
    buffer
}// }}}
// }}}
/// A simple placeholder function for unimplemented features// {{{
fn placeholder( buffer: &mut Buffer, state: &mut EditorState,//{{{
                command: Command) -> Result<(), RedError> {
    print_help( state, &format!(
            "Operation not yet implemented: {}", command.operation ));
    state.mode = mode_noop( &mut state.mode );
    match buffer.get_file_name() {
        Some( file_name ) => {
            print_help( state, &format!(
                    "Continuing work on {}", file_name ));
            return Err(
                RedError::InvalidOperation{ operation: command.operation } );
        }
        None => {
            return Err(
                RedError::InvalidOperation{ operation: command.operation } );
        }
    }
}// }}}
// }}}
fn append( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'a', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    buffer.set_current_line_number( _final );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
/// Deletes address range and inserts text in its place// {{{
fn change( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'c', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    let delete_command = Command{ address_initial: _initial,
            address_final: _final, operation: 'd', parameters: ""  };
    let insert_command = Command{ address_initial: _initial,
            address_final: _initial, operation: 'i', parameters: ""  };
    try!( delete( buffer, state, delete_command ) );
    insert( buffer, state, insert_command )
}//}}}
// }}}
fn delete( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'd', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    for _ in _initial .. ( _final + 1 ) {
        // NOTE: lines move as you delete them - don't increment!
        try!( buffer.delete_line( _initial ) );
    }
    buffer.set_current_line_number( _initial - 1 );
    Ok( () )
}//}}}
fn edit( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'e', command.operation );
    let _ = try!( buffer.on_close( state ));
    edit_unsafe( buffer, state, Command{
        address_initial: command.address_initial,
        address_final: command.address_final,
        operation: 'E', parameters: command.parameters })
}//}}}
fn edit_unsafe( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'E', command.operation );
    let content = command.parameters;
    if &content[0..1] == COMMAND_PREFIX {  // process command
        match Buffer::new( BufferInput::Command(content[1..].to_string() ),
                state ) {
            Ok( _buffer ) => {
                *buffer = _buffer;
            },
            Err(e) => {
                return Err(e);
            },
        };
        print_help( &state, &format!( "Now editing output of command: {}",
                                     buffer.get_file_name()
                                     .unwrap_or( "<untitled>" ) ));
    } else {                    // process file
        match Buffer::new(BufferInput::File( content.to_string() ), state ) {
            Ok( _buffer ) => {
                *buffer = _buffer;
            },
            Err(e) => {
                return Err(e);
            },
        };
        print_help( &state, &format!( "Now editing file: {}",
                                     buffer.get_file_name()
                                     .unwrap_or( "<untitled>" ) ));
    }
    Ok( () )
}//}}}
fn filename( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'f', command.operation );
    if command.parameters == "" {
        match buffer.get_file_name() {
            Some(f) => println!( "filename: {}", f ),
            None => println!( "no filename currently set" ),
        }
    } else {
        try!( buffer.set_file_name( command.parameters ));
    }
    Ok( () )
}//}}}
fn global( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'g', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    placeholder( buffer, state, command )
}//}}}
fn global_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                       command: Command ) -> Result<(), RedError> {
    assert_eq!( 'G', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    placeholder( buffer, state, command )
}//}}}
fn help_recall( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'h', command.operation );
    placeholder( buffer, state, command )
}//}}}
fn help_tgl( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'H', command.operation );
    state.help = !state.help;
    println!("help output set to {:?}", match state.help {
        true => "on",
        false => "off", });
    Ok( () )
}//}}}
fn insert( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'i', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    buffer.set_current_line_number( _final - 1 );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
fn join( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'j', command.operation );
    let mut new_line = String::new();
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                          buffer.get_current_line_number() + 1,
                                            );
    for line in _initial .. _final + 1 {
        match buffer.get_line_content( line ) {
            Some(x) => {
                new_line.push_str( &x );
            },
            None => break,
        }
    }
    try!( delete( buffer, state, Command{
        address_initial: _initial,
        address_final: _final,
        operation: 'd', parameters: "" } ));
    buffer.insert_line( _initial, &new_line );
    buffer.set_current_line_number( _initial );
    try!( buffer.store_buffer() );
    Ok( () )
}//}}}
fn mark( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'k', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    // make sure we have been provided a single character for the mark
    let param_len = command.parameters.len();
    if param_len > 1 || param_len == 0 {
        print_help( state, "mark must be a single character" );
        return Err( RedError::ParameterSyntax{
            parameter: command.parameters.to_string() });
    }
    let mark_char = match command.parameters.chars().next() {
        Some(x) => {
            if !( 'a' <= x && x <= 'z' ) {
                print_help( state,
                        "mark must be a lowercase latin character ('a'..'z')" );
                return Err( RedError::ParameterSyntax{
                        parameter: command.parameters.to_string() });
            } else {
                x
            }
        },
        None => {
            print_help( state, "failed to parse mark character" );
            return Err( RedError::ParameterSyntax{
                    parameter: command.parameters.to_string() });
        },
    };
    // if given a section of lines, mark the beginning
    buffer.set_marker( _final, mark_char );
    Ok( () )

}//}}}
fn lines_list( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'l', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    let stdout = stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new( handle );
    let mut ch_written: usize = 0;
    let line_prefix: &str;  // ! to indicate unknown screen size
    let term_width: usize;
    let term_height: usize;
    if let Some((w, h)) = term_size::dimensions() {
        line_prefix = "";
        term_width = w;
        term_height = h;
    } else {
        line_prefix = "!";
        term_width = 0;
        term_height = 0;
    }
    for line_num in _initial .. _final + 1 {
        let line = buffer.get_line_content( line_num ).unwrap_or("");
        try!( writer.write( line_prefix.as_bytes() )
              .map_err(|_| RedError::Stdout));
        ch_written += line_prefix.len();
        for ch in line.chars() {
            for _ch in ch.escape_default() {
                try!( writer.write( &[_ch as u8] ).map_err(|_| RedError::Stdout));
                ch_written += 1;
                if line_prefix.len() == 0 && ch_written ==
                    ( term_width - line_prefix.len() ) * ( term_height - 1 ) {
                    try!( writer.flush().map_err(|_| RedError::Stdout));
                    prompt_for_more( &mut writer );
                }
            }
        }
        try!( writer.write( "$\n".as_bytes() ).map_err(|_| RedError::Stdout));
        ch_written += 1;
        ch_written += term_width - ( ch_written % term_width );
        try!( writer.flush().map_err(|_| RedError::Stdout));
    }
    Ok( () )
}//}}}
/// Prompts the user to press enter and waits until they do// {{{
fn prompt_for_more( stdout_writer: &mut BufWriter<StdoutLock> ) {// {{{
    stdout_writer.write( "--<press enter to continue>--".as_bytes() )
        .expect( "prompt_for_more: error writing to stdout" );
    stdout_writer.flush().expect( "prompt_for_more: error writing to stdout" );
    let stdin = stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_line( &mut buf ).expect( "error reading keystroke" );
    stdout_writer.write( &['\n' as u8] )
        .expect( "prompt_for_more: error writing to stdout" );
}// }}}
// }}}
fn move_lines( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'm', command.operation );
    let mut destination: usize;
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    if command.parameters == "0" {
        destination = 0;
    } else {
    destination = try!(parse_address_field( command.parameters, buffer ))
            .unwrap_or(buffer.get_current_line_number() );
    }
    try!( buffer.move_lines( &_initial, &_final, &destination ));
    if (_initial-1) <= destination && destination <= _final {
        destination = _initial - 1;
    }
    buffer.set_current_line_number( destination + 1 + ( _final - _initial ) );
    Ok( () )
}//}}}
fn print_numbered( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'n', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number()
                                                );
    let num_lines_f: f64 = buffer.num_lines() as f64 + 1.0_f64;
    let _width = num_lines_f.log10().ceil() as usize;
    for ( _num, _line ) in buffer.lines_iterator().enumerate()
        .skip( _initial - 1 )
        .take(( _final + 1 ) - _initial )
        .map( |(x, y)| ( x+1, y )) {
        print!( "{:width$}|", _num, width = _width );
        println!("{}", _line );
    }
    buffer.set_current_line_number( _final );
    Ok( () )
}//}}}
/// Display range of lines of buffer in terminal // {{{
///
/// Caller will choose the start and finish addresses to fit
/// the range of the buffer; For example, if the user tries
/// to print beyond the end of the buffer, address_final will
/// be the address of the last line of the buffer (in other
/// words, the lines number of the last line)
///
/// # Panics
/// if println! panics, which happens if it fails to write
/// to io::stdout()
///
fn print( buffer: &mut Buffer, state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
    assert_eq!( 'p', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number()
                                                );
    for indx in _initial .. ( _final + 1 ) {
        println!("{}", buffer.get_line_content( indx ).expect(
                "ops::print: called get_line_content on out-of-range line" ) );
    }
    buffer.set_current_line_number( _final );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    Ok( () )
}// }}}
// }}}
/// Exit program// {{{
///
/// Make sure all buffers have been saved
///
/// Delete all temprary storage
fn quit( buffer: &mut Buffer, state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
    assert_eq!( 'q', command.operation );
    match buffer.on_close( state ) {
        Ok( _ ) => exit( error_code( RedError::Quit ) as i32),
        Err( _ ) => exit( error_code( RedError::NoDestruct ) as i32),
    }
}// }}}
//}}}
fn read( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'r', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    placeholder( buffer, state, command )
}//}}}
fn substitute( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 's', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              buffer.get_current_line_number(),
                                              buffer.get_current_line_number(),
                                            );
    let sub_parms: Substitution = try!( parse_substitution_parameter(
            command.parameters ));
    buffer.substitute( &sub_parms.to_match, &sub_parms.to_sub, sub_parms.which,
                       state, _initial, _final );
    Ok( () )
}//}}}
fn transfer( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 't', command.operation );
    placeholder( buffer, state, command )
}//}}}
fn undo( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'u', command.operation );
    placeholder( buffer, state, command )
}//}}}
fn global_reverse( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'v', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    placeholder( buffer, state, command )
}//}}}
fn global_reverse_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                               command: Command ) -> Result<(), RedError> {
    assert_eq!( 'V', command.operation );
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    placeholder( buffer, state, command )
}//}}}
/// Write buffer to file// {{{
fn write_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                  command: Command ) -> Result<(), RedError> {
    assert_eq!( 'w', command.operation );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    buffer.write_to_disk( command.parameters, false, _initial, _final )
}// }}}
// }}}
fn append_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'W', command.operation );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    let ( _initial, _final ) = default_addrs( command.address_initial,
                                              command.address_final,
                                              1, buffer.num_lines() );
    buffer.write_to_disk( command.parameters, true, _initial, _final )
}//}}}

/// Return pair of lines, either original or the specified defaults
///
/// Defaults are used if both original integers are 0;
/// Also fixes lower address to be 1 instead of zero if an otherwise
/// suitable range is provided;
fn default_addrs( _initial: usize, _final: usize,
                  default_i: usize, default_f: usize ) -> ( usize, usize ) {
    if _initial == 0 {
        if _final == 0 {
            ( default_i, default_f )
        } else {
            ( 1, _final )
        }
    } else {
        ( _initial, _final )
    }
}

// ^^^ Functions ^^^ }}}

