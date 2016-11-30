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
//! All operations take as arguments the current state.buffer, editor state, and
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
use std::ffi::OsStr;

use buf::*;
use error::*;
use parse::*;
use io::get_input;
use ::{EditorState, EditorMode, print_help, print_msg, term_size, Change};
use self::NotableLine::*;
// ^^^ Bring in to namespace ^^^ }}}

// *** Attributes *** {{{
const NUM_OPERATIONS: usize = 27;
const COMMAND_PREFIX: &'static str = "@";
// ^^^ Attributes ^^^ }}}

// *** Constants *** {{{
// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
enum NotableLine {
    FirstLine,
    LastLine,
    CurrentLine,
    CurrentPlusOneLine,
    LineNotApplicable
}
pub struct Operations {
    operation_map: HashMap<char, OpData>,
}
struct OpData {
    function: Box<Fn( &mut EditorState, Command ) -> Result<(), RedError>>,
    default_initial_address: NotableLine,
    default_final_address: NotableLine,
}
impl Operations {// {{{
    /// Creates Operations HashMap// {{{
    pub fn new() -> Operations {// {{{
        let mut _operation_map: HashMap<char, OpData> =
            HashMap::with_capacity( NUM_OPERATIONS );
        // Insert operations and data   //{{{
        _operation_map.insert( 'a',// {{{
                                OpData{ function: Box::new(append),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'c',// {{{
                                OpData{ function: Box::new(change),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'd',// {{{
                                OpData{ function: Box::new(delete),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'e',// {{{
                                OpData{ function: Box::new(edit),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'E',// {{{
                                OpData{ function: Box::new(edit_unsafe),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'f',// {{{
                                OpData{ function: Box::new(filename),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'g',// {{{
                                OpData{ function: Box::new(global),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 'G',// {{{
                                OpData{ function: Box::new(global_interactive),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 'h',// {{{
                                OpData{ function: Box::new(help_recall),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'H',// {{{
                                OpData{ function: Box::new(help_tgl),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'i',// {{{
                                OpData{ function: Box::new(insert),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'j',// {{{
                                OpData{ function: Box::new(join),
                                        default_initial_address: CurrentLine,
                                        default_final_address:
                                            CurrentPlusOneLine,
                                }
        );// }}}
        _operation_map.insert( 'k',// {{{
                                OpData{ function: Box::new(mark),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'l',// {{{
                                OpData{ function: Box::new(lines_list),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'm',// {{{
                                OpData{ function: Box::new(move_lines),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'n',// {{{
                                OpData{ function: Box::new(print_numbered),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'p',// {{{
                                OpData{ function: Box::new(print),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'P',// {{{
                                OpData{ function: Box::new(prompt),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'q',// {{{
                                OpData{ function: Box::new(quit),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'r',// {{{
                                OpData{ function: Box::new(read),
                                        default_initial_address: LastLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 's',// {{{
                                OpData{ function: Box::new(substitute),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 't',// {{{
                                OpData{ function: Box::new(transfer),
                                        default_initial_address: CurrentLine,
                                        default_final_address: CurrentLine,
                                }
        );// }}}
        _operation_map.insert( 'u',// {{{
                                OpData{ function: Box::new(undo),
                                        default_initial_address:
                                            LineNotApplicable,
                                        default_final_address:
                                            LineNotApplicable,
                                }
        );// }}}
        _operation_map.insert( 'v',// {{{
                                OpData{ function: Box::new(global_inverse),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 'V',// {{{
                                OpData{ function:
                                    Box::new(global_inverse_interactive),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 'w',// {{{
                                OpData{ function: Box::new(write_to_disk),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        _operation_map.insert( 'W',// {{{
                                OpData{ function: Box::new(append_to_disk),
                                        default_initial_address: FirstLine,
                                        default_final_address: LastLine,
                                }
        );// }}}
        //}}}
        Operations { operation_map: _operation_map }
    }// }}}
// }}}
    /// Execute command// {{{
    pub fn execute( &self, state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
        match self.operation_map.contains_key( &command.operation ) {
            true => {
                let ref op_to_execute = self.operation_map
                    .get( &command.operation ).unwrap().function;
                op_to_execute( state, command )
            },
            false => {
                Err(RedError::InvalidOperation{ operation: command.operation })
            },
        }
    }// }}}
    // }}}
    /// Execute list of commands at address// {{{
    fn execute_list( &self, state: &mut EditorState,// {{{
                commands: &str, address: usize ) -> Result<(), RedError> {
        for cmd in commands.lines() {
            let mut _command: Command;
            if cmd.trim().is_empty() {
                _command = Command {
                                address_initial: address,
                                address_final: address,
                                operation: 'p',
                                parameters: "",
                                operations: &self,
                            };
            } else {
                _command = try!( parse_command( cmd, state, &self ));
                _command.address_initial = address;
                _command.address_final   = address;
            }
            try!( self.execute( state, _command ));
            state.buffer.set_current_address( address );
        }
        Ok( () )
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
/// A simple placeholder function for unimplemented features// {{{
fn placeholder( state: &mut EditorState, command: Command)//{{{
        -> Result<(), RedError> {
    print_msg( state, &format!(
            "Operation not yet implemented: {}", command.operation ));
    state.mode = mode_noop( &mut state.mode );
    match state.buffer.get_file_name() {
        Some( file_name ) => {
            print_msg( state, &format!(
                    "Continuing work on {}", file_name.to_str()
                            .unwrap_or("<invalid UTF-8>")) );
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
fn append( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'a', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    state.buffer.set_current_address( _final );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
/// Deletes address range and inserts text in its place// {{{
fn change( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'c', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    let delete_command = Command{ address_initial: _initial,
            address_final: _final, operation: 'd', parameters: "",
            operations: command.operations };
    let insert_command = Command{ address_initial: _initial,
            address_final: _initial, operation: 'i', parameters: "",
            operations: command.operations };
    try!( delete( state, delete_command ) );
    try!( insert( state, insert_command ) );
    Ok( () )
}//}}}
// }}}
fn delete( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'd', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    state.u_deleting_lines( _initial, _final );
    for _ in _initial .. ( _final + 1 ) {
        // NOTE: lines move as you delete them - don't increment!
        try!( state.buffer.delete_line( _initial ) );
    }
    state.buffer.set_current_address( _initial - 1 );
    Ok( () )
}//}}}
fn edit( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'e', command.operation );
    let _ = try!( state.buffer.on_close() );
    edit_unsafe( state, Command{ address_initial: command.address_initial,
        address_final: command.address_final, operation: 'E',
        parameters: command.parameters, operations: command.operations, })
}//}}}
fn edit_unsafe( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'E', command.operation );

    let content = command.parameters;
    if &content[0..1] == COMMAND_PREFIX {  // process command
        match Buffer::new( BufferInput::Command(content[1..].to_string() )) {
            Ok( _buffer ) => {
                EditorState::new( _buffer );
            },
            Err(e) => {
                return Err(e);
            },
        };
        print_msg( &state, &format!( "Now editing output of command: {}",
                                     state.buffer.get_file_name()
                                     .unwrap_or( OsStr::new("<untitled>") )
                                     .to_str()
                                     .unwrap_or( "<invalid UTF-8>" ) ));
    } else {                    // process file
        match Buffer::new(BufferInput::File( content.to_string() )) {
            Ok( _buffer ) => {
                state.buffer = _buffer;
            },
            Err(e) => {
                return Err(e);
            },
        };
        print_msg( &state, &format!( "Now editing file: {}",
                                     state.buffer.get_file_name()
                                     .unwrap_or( OsStr::new("<untitled>") )
                                     .to_str()
                                     .unwrap_or( "<invalid UTF-8>" ) ));
    }
    Ok( () )
}//}}}
fn filename( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'f', command.operation );
    if command.parameters == "" {
        match state.buffer.get_file_name() {
            Some(f) => println!( "filename: {}", f.to_str()
                            .unwrap_or("<invalid UTF-8>") ),
            None => println!( "no filename currently set" ),
        }
    } else {
        try!( state.buffer.set_file( command.parameters ));
    }
    Ok( () )
}//}}}
/// Execute a set of commands on lines matching pattern
///
/// TODO:
/// * current address handling is a bit simplified for now
/// * creating new operations object may be costly (?)
fn global( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'g', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    let ( pattern, commands ) = try!(parse_global_op(command.parameters));
    for address in _initial .. _final + 1 {
        if state.buffer.does_match( pattern, address ) {
            try!( command.operations.execute_list( state, commands, address ));
        }
    }
    Ok( () )
}//}}}
fn global_interactive( state: &mut EditorState,//{{{
                       command: Command ) -> Result<(), RedError> {
    assert_eq!( 'G', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    let mut input: String = String::new();
    let mut last_input: String = String::new();
    let ( pattern, commands ) = try!(parse_global_op(command.parameters));
    // make sure no additional text after /re/
    if !commands.is_empty() {
        return Err( RedError::ParameterSyntax{
            parameter: "global_interactive: ".to_string() +
            command.parameters });
    }
    let prompt_save = state.prompt.clone();
    state.prompt = "(G)%".to_string();
    for address in _initial .. _final + 1 {
        if state.buffer.does_match( pattern, address ) {
            state.buffer.set_current_address( address );
            print_numbered( state, Command{
                                    address_initial: address,
                                    address_final: address,
                                    operation: 'n',
                                    operations: command.operations,
                                    parameters: "" }).expect(
                    "global_interactive: line matching regex doesn't exist!" );
            input = try!( get_input( input, state ));
            if input.trim() == "&" {
                input = last_input;
            }
            try!( command.operations.execute_list( state, &input, address ));
            last_input = input;
            input = String::new();
        }
    }
    state.prompt = prompt_save;
    Ok( () )
}//}}}
fn help_recall( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'h', command.operation );
    placeholder( state, command )
}//}}}
fn help_tgl( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'H', command.operation );
    state.show_help = !state.show_help;
    println!("help output set to {:?}", match state.show_help {
        true => "on",
        false => "off", });
    Ok( () )
}//}}}
fn insert( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'i', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    state.buffer.set_current_address( _final - 1 );
    state.mode = EditorMode::Insert;
    Ok( () )
}//}}}
fn join( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'j', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    state.u_deleting_lines( _initial, _final );
    try!( state.buffer.join_lines( _initial, _final ));
    state.u_added_current_line();
    Ok( () )
}//}}}
fn mark( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'k', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
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
    state.buffer.set_marker( mark_char, _final );
    Ok( () )

}//}}}
// XXX: write built-in implementation in Buffer?
fn lines_list( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'l', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
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
    for address in _initial .. _final + 1 {
        let line = state.buffer.get_line_content( address ).unwrap_or("");
        try!( writer.write( line_prefix.as_bytes() )
              .map_err(|_| RedError::Stdout));
        ch_written += line_prefix.len();
        for ch in line.chars() {
            for _ch in ch.escape_default() {
                try!( writer.write( &[_ch as u8] )
                          .map_err(|_| RedError::Stdout));
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
fn move_lines( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'm', command.operation );
    state.u_reset();
    state.u_lock();
    let mut destination: usize;
    let ( _initial, _final ) = default_addrs( state, &command );
    state.u_deleting_lines( _initial, _final );
    if command.parameters == "0" {
        destination = 0;
    } else {
    destination = try!(parse_address_field(command.parameters, &state.buffer))
            .unwrap_or(state.buffer.get_current_address() );
    }
    try!( state.buffer.move_lines( &_initial, &_final, &destination ));
    // destination is the address to which lines are appended,
    // so it is one less than the first moved line
    // if it is anywhere between the moved lines, it ends up just before
    // initial
    // XXX: wait... doesn't that mean it should be a no-op?
    // save that for next update - this works for now.
    if (_initial-1) <= destination && destination <= _final {
        destination = _initial - 1;
    }
    // we adjust dest. not by the diff, but num lines (hence the extra 1)
    if _final < destination && destination <= state.buffer.num_lines() {
        destination = destination - ( 1 + _final - _initial );
    }
    state.buffer.set_current_address( destination + 1 + ( _final - _initial ));
    state.u_added_lines( destination + 1,
                                 destination + 1 + ( _final - _initial ));
    Ok( () )
}//}}}
// XXX: write built-in implementation in Buffer?
fn print_numbered( state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'n', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    let num_lines_f: f64 = state.buffer.num_lines() as f64 + 1.0_f64;
    let _width = num_lines_f.log10().ceil() as usize;
    for ( _num, _line ) in state.buffer.lines_iterator().enumerate()
        .skip( _initial - 1 )
        .take(( _final + 1 ) - _initial )
        .map( |(x, y)| ( x+1, y )) {
        print!( "{:width$}|", _num, width = _width );
        println!("{}", _line );
    }
    state.buffer.set_current_address( _final );
    Ok( () )
}//}}}
/// Display range of lines of state.buffer in terminal // {{{
///
/// Caller will choose the start and finish addresses to fit
/// the range of the state.buffer; For example, if the user tries
/// to print beyond the end of the state.buffer, address_final will
/// be the address of the last line of the state.buffer (in other
/// words, the lines number of the last line)
///
/// # Panics
/// if println! panics, which happens if it fails to write
/// to io::stdout()
///
fn print( state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
    assert_eq!( 'p', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    for indx in _initial .. ( _final + 1 ) {
        println!("{}", state.buffer.get_line_content( indx ).expect(
                "ops::print: called get_line_content on out-of-range line" ) );
    }
    state.buffer.set_current_address( _final );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    Ok( () )
}// }}}
// }}}
/// Toggles (sets?) commant prompt
fn prompt( state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
    assert_eq!( 'p', command.operation );
    placeholder( state, command )
}// }}}
// }}}
/// Exit program// {{{
///
/// Make sure all state.buffers have been saved
///
/// Delete all temprary storage
fn quit( state: &mut EditorState, command: Command )//{{{
            -> Result<(), RedError> {
    assert_eq!( 'q', command.operation );
    match state.buffer.on_close() {
        Ok( _ ) => exit( error_code( RedError::Quit ) as i32),
        Err( _ ) => exit( error_code( RedError::NoDestruct ) as i32),
    }
}// }}}
//}}}
fn read( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'r', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    placeholder( state, command )
}//}}}
fn substitute( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 's', command.operation );
    state.u_reset();
    state.u_lock();
    let ( _initial, _final ) = default_addrs( state, &command );
    state.u_deleting_lines( _initial, _final );
    let sub_parms: Substitution = try!( parse_substitution_parameter(
            command.parameters ));
    state.buffer.substitute( &sub_parms.to_match, &sub_parms.to_sub,
                             sub_parms.which, _initial, _final );
    state.u_added_lines( _initial, _final );
    Ok( () )
}//}}}
fn transfer( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 't', command.operation );
    state.u_reset();
    state.u_lock();
    let destination: usize;
    let ( _initial, _final ) = default_addrs( state, &command );
    if command.parameters == "0" {
        destination = 0;
    } else {
    destination = try!(parse_address_field(command.parameters, &state.buffer))
            .unwrap_or(state.buffer.get_current_address() );
    }
    try!( state.buffer.copy_lines( _initial, _final, destination ));
    state.u_added_lines( destination + 1,
                                 destination + 1 + ( _final - _initial ));
    Ok( () )
}//}}}
fn undo( state: &mut EditorState, command: Command )
        -> Result<(), RedError> {// {{{
    assert_eq!( 'u', command.operation );
    let address = state.u_get_wascurrent_address();
    let markers = state.u_get_markers();
    let mut changes = state.u_get_changes();
    state.u_unlock();
    state.u_reset();
    state.u_lock();
    loop {
        match changes.pop() {
            Some( Change::Add{ address: _address }) => {
                state.u_deleting_line( _address );
                try!( state.buffer.delete_line( _address ) );
            },
            Some( Change::Remove{ address: _address, content: _content }) => {
                state.u_added_line( _address );
                state.buffer.append_line( _address - 1, &_content );
            },
            None => break,
        }
    }
    state.buffer.set_current_address( address );
    for indx in ( 'a' as u8 ) .. ( 'z' as u8 ) + 1 {
        state.buffer.set_marker( indx as char,
            markers[ ( indx as usize ) - ( 'a' as usize ) ] );
    }
    Ok( () )
}//}}}
fn global_inverse( state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'v', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    let ( pattern, commands ) = try!(parse_global_op(command.parameters));
    for address in _initial .. _final + 1 {
        if !state.buffer.does_match( pattern, address ) {
            try!( command.operations.execute_list( state, commands, address ));
        }
    }
    Ok( () )
}//}}}
fn global_inverse_interactive( state: &mut EditorState,//{{{
                               command: Command ) -> Result<(), RedError> {
    assert_eq!( 'V', command.operation );
    let ( _initial, _final ) = default_addrs( state, &command );
    let mut input: String = String::new();
    let mut last_input: String = String::new();
    let ( pattern, commands ) = try!(parse_global_op(command.parameters));
    // make sure no additional text after /re/
    if !commands.is_empty() {
        return Err( RedError::ParameterSyntax{
            parameter: "global_interactive: ".to_string() +
            command.parameters });
    }
    let prompt_save = state.prompt.clone();
    state.prompt = "(G)%".to_string();
    for address in _initial .. _final + 1 {
        if !state.buffer.does_match( pattern, address ) {
            state.buffer.set_current_address( address );
            print_numbered( state, Command{
                                    address_initial: address,
                                    address_final: address,
                                    operation: 'n',
                                    operations: command.operations,
                                    parameters: "" }).expect(
                    "global_interactive: line matching regex doesn't exist!" );
            input = try!( get_input( input, state ));
            if input.trim() == "&" {
                input = last_input;
            }
            try!( command.operations.execute_list( state, &input, address ));
            last_input = input;
            input = String::new();
        }
    }
    state.prompt = prompt_save;
    Ok( () )
}//}}}
/// Write state.buffer to file// {{{
fn write_to_disk( state: &mut EditorState,//{{{
                  command: Command ) -> Result<(), RedError> {
    assert_eq!( 'w', command.operation );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    let ( _initial, _final ) = default_addrs( state, &command );
    state.buffer.write_to_disk( command.parameters, false, _initial, _final )
}// }}}
// }}}
fn append_to_disk( state: &mut EditorState,//{{{
                   command: Command ) -> Result<(), RedError> {
    assert_eq!( 'W', command.operation );
    // TODO: Drop this? Or Keep to avoid unused warnings?
    state.mode = EditorMode::Command;
    let ( _initial, _final ) = default_addrs( state, &command );
    state.buffer.write_to_disk( command.parameters, true, _initial, _final )
}//}}}

/// Return pair of lines, either original or the specified defaults// {{{
///
/// Defaults are used if both original integers are 0;
/// Also fixes lower address to be 1 instead of zero if an otherwise
/// suitable range is provided;
fn default_addrs( state: &EditorState, command: &Command ) -> (usize, usize) {// {{{
    let default_i = match command.operations.operation_map
                .get( &command.operation ).unwrap().default_initial_address {
        FirstLine => 1,
        LastLine => state.buffer.num_lines(),
        CurrentLine => state.buffer.get_current_address(),
        CurrentPlusOneLine => state.buffer.get_current_address() + 1,
        LineNotApplicable => 1,
    };
    let default_f = match command.operations.operation_map
                .get( &command.operation ).unwrap().default_final_address {
        FirstLine => 1,
        LastLine => state.buffer.num_lines(),
        CurrentLine => state.buffer.get_current_address(),
        CurrentPlusOneLine => state.buffer.get_current_address() + 1,
        LineNotApplicable => 1,
    };
    if command.address_initial == 0 {
        if command.address_final == 0 {
            ( default_i, default_f )
        } else {
            ( 1, command.address_final )
        }
    } else {
        ( command.address_initial, command.address_final )
    }
}// }}}
// }}}
// ^^^ Functions ^^^ }}}

