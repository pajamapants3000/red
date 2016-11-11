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

// *** Bring in to namespace *** {{{
use std::collections::hash_map::HashMap;
use std::process::exit;

use buf::*;
use error::*;
use parse::*;
use ::{EditorState, EditorMode, print_help};
// ^^^ Bring in to namespace ^^^ }}}

// *** Attributes *** {{{
const NUM_OPERATIONS: usize = 32;
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
    buffer.set_current_line_number( command.address_final + 1 );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
fn change( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn delete( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn edit( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn edit_unsafe( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn filename( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn global( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn global_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                       command: Command ) -> Result<(), RedError> {
    placeholder( buffer, state, command )
}//}}}
fn help_recall( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
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
    buffer.set_current_line_number( command.address_final );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
fn join( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn mark( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn lines_list( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn move_lines( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn print_numbered( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    placeholder( buffer, state, command )
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
    {   // XXX: just some random use of parms for now
        if command.parameters == "heading" {
            println!( "here's a heading" );
            return Err( RedError::OpCharIndex );
        }
    }
    for indx in command.address_initial .. ( command.address_final + 1 ) {
        println!("{}", buffer.get_line_content( indx ).unwrap() );
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
    if command.parameters == "!" && buffer.is_modified() {
        println!("file changed since last write");
    }
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    match buffer.destruct() {
        Ok( _ ) => exit( error_code( RedError::Quit ) as i32),
        Err( _ ) => exit( error_code( RedError::NoDestruct ) as i32),
    }
}// }}}
//}}}
fn read( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn substitute( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn transfer( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn undo( buffer: &mut Buffer, state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    placeholder( buffer, state, command )
}//}}}
fn global_reverse( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    placeholder( buffer, state, command )
}//}}}
fn global_reverse_interactive( buffer: &mut Buffer, state: &mut EditorState,//{{{
                               command: Command ) -> Result<(), RedError> {
    placeholder( buffer, state, command )
}//}}}
/// Write buffer to file// {{{
fn write_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                  command: Command ) -> Result<(), RedError> {
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    buffer.write_to_disk( command.parameters )
}// }}}
// }}}
fn append_to_disk( buffer: &mut Buffer, state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    placeholder( buffer, state, command )
}//}}}

// ^^^ Functions ^^^ }}}


