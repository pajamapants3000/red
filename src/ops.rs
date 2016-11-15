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

use buf::*;
use error::*;
use parse::*;
use ::{EditorState, EditorMode, print_help};
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
    // append is the default/natural line-insert behavior
    buffer.set_current_line_number( command.address_final );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
/// Deletes address range and inserts text in its place// {{{
fn change( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'c', command.operation );
    let delete_command = Command{ address_initial: command.address_initial,
            address_final: command.address_final, operation: 'd',
            parameters: ""  };
    let insert_command = Command{ address_initial: command.address_initial,
            address_final: command.address_initial, operation: 'i',
            parameters: ""  };
    try!( delete( buffer, state, delete_command ) );
    insert( buffer, state, insert_command )
}//}}}
// }}}
fn delete( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'd', command.operation );
    for _ in command.address_initial .. ( command.address_final + 1 ) {
        // NOTE: lines move as you delete them - don't increment!
        try!( buffer.delete_line( command.address_initial ) );
    }
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
    placeholder( buffer, state, command )
}//}}}
fn global_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                       command: Command ) -> Result<(), RedError> {
    assert_eq!( 'G', command.operation );
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
    // append is the default/natural line-insert behavior
    buffer.set_current_line_number( command.address_final );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
fn join( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'j', command.operation );
    let mut new_line = String::new();
    for line in command.address_initial .. command.address_final + 1 {
        match buffer.get_line_content( line ) {
            Some(x) => {
                new_line.push_str( &x );
            },
            None => break,
        }
    }
    try!( delete( buffer, state, Command{
        address_initial: command.address_initial,
        address_final: command.address_final,
        operation: 'd', parameters: "" } ));
    buffer.set_current_line_number( command.address_initial );
    buffer.insert_here( &new_line );
    try!( buffer.store_buffer() );
    Ok( () )
}//}}}
fn mark( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'k', command.operation );
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
    buffer.set_marker( command.address_initial, mark_char );
    Ok( () )

}//}}}
fn lines_list( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'l', command.operation );
    placeholder( buffer, state, command )
}//}}}
fn move_lines( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'm', command.operation );
    let mut destination: usize = match command.parameters.parse() {
        Ok(x) => x,
        Err(_) => 0,
    };
    destination += 1;           // we want to append at this line
    let mut offset: usize = 0;
    let mut line: String;
    for line_num in command.address_initial .. command.address_final + 1 {
        line = buffer.get_line_content( line_num - offset )
            .unwrap_or("").to_string();
        try!( buffer.delete_line( line_num - offset ));
        if ( line_num - offset ) > destination {
            buffer.insert_line( destination, &line );
        } else {
            offset += 1;
            buffer.insert_line( destination - offset, &line );
        }
        destination += 1;
    }
    Ok( () )
}//}}}
fn print_numbered( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'n', command.operation );
    let mut indx: usize = 0;
    let num_lines_f: f64 = buffer.num_lines() as f64 + 1.0_f64;
    let _width = num_lines_f.log10().ceil() as usize;
    for _line in buffer.lines_iterator() {
        indx += 1;
        print!( "{:width$}|", indx, width = _width );
        println!("{}", _line );
    }
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
    {   // XXX: just some random use of parms for now
        if command.parameters == "heading" {
            println!( "here's a heading" );
            return Err( RedError::OpCharIndex );
        }
    }
    for indx in command.address_initial .. ( command.address_final + 1 ) {
        println!("{}", buffer.get_line_content( indx ).expect(
                "ops::print: called get_line_content on out-of-range line" ) );
    }
    buffer.set_current_line_number( command.address_final );
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
    placeholder( buffer, state, command )
}//}}}
fn substitute( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 's', command.operation );
    placeholder( buffer, state, command )
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
    placeholder( buffer, state, command )
}//}}}
fn global_reverse_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                               command: Command ) -> Result<(), RedError> {
    assert_eq!( 'V', command.operation );
    placeholder( buffer, state, command )
}//}}}
/// Write buffer to file// {{{
fn write_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                  command: Command ) -> Result<(), RedError> {
    assert_eq!( 'w', command.operation );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    buffer.write_to_disk( command.parameters )
}// }}}
// }}}
fn append_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'W', command.operation );
    placeholder( buffer, state, command )
}//}}}

// ^^^ Functions ^^^ }}}


