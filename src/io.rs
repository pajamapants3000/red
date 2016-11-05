/*
 * File   : io.rs
 * Purpose: 
 * Program: red
 * About  : What does this program do?
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016
 */

// *** Bring in to namespace *** {{{
use std::fs::{File, OpenOptions};
use std::process::Command;

use error::*;

// ^^^ Bring in to namespace ^^^ }}}
//
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
//
// *** Constants *** {{{
//const LINE_CONT: &'static str = "\\\n";
//const PROMPT: &'static str = "%";
//const PROMPT_CONT: &'static str = ">";
// ^^^ Constants ^^^ }}}
//
// *** Data Structures *** {{{
#[derive(Default)]
pub struct FileMode {// {{{
    pub f_write:        bool,
    pub f_read:         bool,
    pub f_append:       bool,
    pub f_truncate:     bool,
    pub f_create:       bool,
    pub f_create_new:   bool,
}// }}}

/*
struct FileCoordinate {
    line: usize,
    col: usize,
}
*/
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/*
/// Return next occurrence of regular expression
regex_search( needle: &str, from: FileCoordinate ) -> FileCoordinate {
} */

/// Opens file with user-specified name and mode 
///
/// Uses global definitions of mode flags in this file
///
/// Returns direct result of call to OpenOptions::new()
/// This is of type Result<File, io::Error>
pub fn file_opener( name: &str, mode: FileMode ) -> Result<File, RedError> {// {{{

    // let's introduce OpenOptions now, though we don't need it
    // until we introduce more functionality
    OpenOptions::new()
        .read(mode.f_read)
        .write(mode.f_write)
        .append(mode.f_append)
        .truncate(mode.f_truncate)
        .create(mode.f_create)
        .create_new(mode.f_create_new)
        .open( name ).map_err(|err| RedError::FileOpen( err ) )
}// }}}
/// The public interface - turn command input into output string
pub fn command_output( _full_stdin: &str ) -> String {// {{{
    let command: String;
    let arguments: Vec<String>;
    match compose_command( _full_stdin ) {
        ( cmd, args ) => {
            command = cmd;
            arguments = args;
        },
    }
    let output = Command::new( &command ).args( &arguments )
            .output().expect("command failed");
    let output_stdout = output.stdout;
    // convert to RedError type
    String::from_utf8( output_stdout )
                        .expect("Failed to get output")
}// }}}
/// Turn command-line input into std::process::Command object
fn compose_command( _full_stdin: &str ) -> ( String, Vec<String> ) {// {{{
    let arguments: Vec<String>;

    match split_cmd_args( _full_stdin ) {
        ( cmd, arg ) => {
            arguments = split_args( &arg );
            ( cmd, arguments )
        },
    }
}// }}}
/// Split-off executed program/command from beginning of input
///
/// Splits input into "<command> <arguments>" string
/// returns <command> and <arguments> as separate strings for
/// further processing
fn split_cmd_args( _full_stdin: &str ) -> ( String, String ) {// {{{
    let input = _full_stdin.trim();
    let mut arguments = String::new();
    let command: String;
    let first_space = input.find( char::is_whitespace );
    // TODO handle possible quoting? e.g.for paths with spaces
    match first_space {
        Some(x) => {
            match input.split_at( x ) {
                (zi, zf) => {
                    command = zi.trim().to_string();
                    arguments = zf.trim().to_string();
                },
            }
        },
        None => {
            command = input.trim().to_string();
        },
    }
    ( command, arguments )
}// }}}
/// Convert string of arguments into vector
fn split_args( stringed: &str ) -> Vec<String> {// {{{
    let mut input = stringed.trim();
    let mut argument = String::new();
    let mut arguments: Vec<String> = Vec::new();
    loop {
        let next_space = input.trim().find( char::is_whitespace );
        //let next_space = input.trim().find( " " );
        match next_space {
            Some(x) => {
                match input.split_at( x ) {
                    (zi, zf) => {
                            input = zf.trim();
                            argument = argument + zi.trim();
                        if !is_quoted( stringed, x ) {
                            arguments.push( argument );
                            argument = String::new();
                        }
                    },
                }
            },
            None => {
                assert!( argument.is_empty(),
                        "command_output: unterminated quote" );
                arguments.push( input.to_string() );
                break;
            },
        }
    }
    arguments
}// }}}
/// return true if character is quoted according to quot, bra, and ket
///
/// quoted if preceded by odd number of "|'|`, or IMMEDIATELY preceded by
/// odd number of backslashes; or preceded by unclosed, unquoted brackets;
/// either (, [, or {.
/// in some cases, we may want to include <>, but this should do it for
/// the most part I think
/// XXX: What if the quoted string is not space-separated from the rest?
/// TODO: Implementation is slow, inelegant, brute-force approach
/// XXX: Do we really care about the right side? That's more a question of
/// whether the user properly closed their quotes
/// XXX: We ignore all parens if quoted, otherwise
/// include them even if backslash-escaped
/// TODO: ? add escaped brackets as separate brackets, e.g. "(|[|{|\(|\[|\{"
/// TODO: define bra, ket, and quot as global string or something and
/// convert to vector in function?
pub fn is_quoted( text: &str, indx: usize ) -> bool {// {{{
    let bra:  Vec<char> = vec!('(', '[', '{');
    let ket:  Vec<char> = vec!(')', ']', '}');
    let quot: Vec<char> = vec!('"', '\'', '`');
    let mut c_braket:  Vec<isize> = vec!( 0; bra.len() );
    let mut c_quote:   Vec<isize> = vec!( 0; quot.len() );
    let mut escaped: bool = false;
    let mut move_on: bool;  // avoid unnecessary tests in mess below
    //
    let (left, _) = text.split_at( indx );
    //
    for ch in left.chars() {
        move_on = false;
        if ch == '\\' {
            escaped = !escaped;
            continue
        }
        for i in 0 .. quot.len() {
            if ch == quot[i] {
                if !escaped {
                    c_quote[i] = 1 - c_quote[i];    // switch on/off
                }
                move_on = true;
                escaped = false;
            }
        }
        if move_on {
            continue
        }
        for i in 0 .. bra.len() {
            if ch == bra[i] {
                if c_quote == vec!( 0; c_quote.len() ) {
                    if !escaped {
                        c_braket[i] += 1;
                    }
                    move_on = true;
                    escaped = false;
                }
            }
        }
        if move_on {
            continue
        }
        for i in 0 .. ket.len() {
            if ch == ket[i] {
                if c_quote == vec!( 0; c_quote.len() ) {
                    if !escaped {
                        c_braket[i] -= 1;
                    }
                }
            }
        }
        escaped = false;
    }
    // sanity check
    for sum in &c_braket {
        assert!( *sum >= 0, "is_quoted: too many closing brackets" );
    }
    if c_quote == vec!( 0; c_quote.len() ) &&
            c_braket == vec!( 0; c_braket.len() ) {
        false
    } else {
        true
    }
}// }}}
// ^^^ Functions ^^^ }}}

