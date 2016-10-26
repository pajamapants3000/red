/*
 * File   : parse.rs
 * Purpose: routines for parsing using input into useful data
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 10/26/2016
 */

// Bring in to namespace {{{
use error::*;

use std::fs::File;
use std::str::Chars;
// }}}

// *** Data Structures *** {{{
pub struct Command<'a> {
    pub address_initial: u32,
    pub address_final: u32,
    pub operation: char,
    pub parameters: &'a str,
}
// ^^^ Data Structures ^^^ }}}

/// Parses command-mode input {{{
///
/// This is the public interface to the parse module
///
pub fn parse_command<'a>( _cmd_input: &'a str, file_opened: &File ) -> Command<'a> {
    // MUST initialize?
    let mut _address_initial: u32 = 1;
    let mut _address_final: u32 = 1;
    let mut _operation: char = 'p';
    let mut _parameters: &str = _cmd_input;

    let op_indx: usize;
    op_indx = get_opchar_index( _cmd_input ).unwrap_or( 0usize );
    _address_final = op_indx as u32;

    match get_address( "", file_opened ) {
        ( x, y ) => {
            _address_initial = x;
            _address_final = y;
        }
    }

    Command {
        address_initial: _address_initial,
        address_final: _address_final,
        operation: _operation,
        parameters: _parameters,
    }
}
//}}}

/// Identify address or address range {{{
///
#[allow(unused_variables)]
fn get_address( address_string: &str, file_opened: &File ) -> ( u32, u32 ) {
    // start with simple auto-return of ( 1u32, 2u32 )
    ( 1u32, 2u32 )
}
//}}}

/// Find index of operation code in string
///
/// Parses full command and finds the index of the operation character;
/// The trick here is that the operation character may appear any number
/// of times as part of a regular expression used to specify an address
/// range that matches;
/// What this function does is simply locates the first alphabetic character
/// that is not wrapped in either /.../ or ?...?
///
/// Trims white space on the left as well - does not count these characters
///
/// # Examples
///
/// No address given
/// ```rust
/// let _in: &str = "e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// No address given, with spaces
/// ```rust
/// let _in: &str = "       e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// No address given, with spaces and tabs
/// ```rust
/// let _in: &str = "  		  	e myfile.txt";
/// assert_eq!( get_opchar_index( _in ), 0 );
/// ```
///
/// Most basic address value types
/// ```rust
/// let _in: &str = ".a";
/// assert_eq!( get_opchar_index( _in ), 1 );
/// ```
///
/// ```rust
/// let _in: &str = ".,.p";
/// assert_eq!( get_opchar_index( _in ), 3 ); // test
/// ```
///
/// Slightly more complicated
/// ```rust
/// let _in: &str = ".-2,.+2p";
/// assert_eq!( get_opchar_index( _in ), 3 ); //test
/// ```
///
/// Regular expression match line search forward
/// ```rust
/// let _in: &str = "/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
/// Regular expression match line search forward with spaces and tabs
/// ```rust
/// let _in: &str =
/// "		  	/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
/// Regular expression match line search backward
/// ```rust
/// let _in: &str = "?^Beginning with.*$?,?.* at the end$?s_mytest_yourtest_g";
/// assert_eq!( get_opchar_index( _in ), 37 );
/// ```
///
pub fn get_opchar_index( _cmd_input: &str ) -> Result<usize, RedError> {
    let mut result: i32 = -1;

    struct Interpret {
        index: u16,         // index of character in string
        pattern_char: char, // stores '/' or '?' while reading pattern
        escape: bool,       // last character read was '\'
        expect: bool,       // we expect next char to be ',', ';', or alpha
    }
    let mut current_state: Interpret = Interpret {
        index: 0u16,
        pattern_char: '\0',
        escape: false,
        expect: false,
    };

    let mut ch_iter: Chars = _cmd_input.trim_left().chars();

    loop {
        match ch_iter.next() {
            None => break,
            Some( x ) => {
                if current_state.pattern_char == '\0' {
                    if current_state.expect {
                        match x {
                            'a'...'z' | 'A'...'Z' => {
                                result = current_state.index as i32;
                                break;
                            },
                            ',' | ';' => {
                                current_state.escape= false;
                                current_state.expect = false
                            },
                            '\\' => current_state.escape = true,
                            _ => break,
                        }
                    }
                    match x {
                        '/' | '?' => {
                            if !current_state.escape {
                                current_state.pattern_char = x;
                            }
                        }
                        'a'...'z' | 'A'...'Z' => {
                            result = current_state.index as i32;
                            break;
                        },
                        '\\' => current_state.escape = true,
                        _ => {
                            current_state.escape= false;
                            current_state.expect = false;
                        },
                    }
                } else {
                    match x {
                        '/' => {
                            if '/' == current_state.pattern_char {
                                if !current_state.escape {
                                    current_state.pattern_char = '\0';
                                    current_state.expect = true;
                                }
                            }
                        }
                        '?' => {
                            if '?' == current_state.pattern_char {
                                if !current_state.escape {
                                    current_state.pattern_char = '\0';
                                    current_state.expect = true;
                                }
                            }
                        }
                        '\\' => current_state.escape = true,
                        _ => {},
                    }
                }
            }
        }
        current_state.index += 1;
    }
    if result < 0 {
        Result::Err( RedError::FileClose )
    } else {
        println!("result {:?}", result);
        Result::Ok( result as usize )   // result will be positive here
    }
}
