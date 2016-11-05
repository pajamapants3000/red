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
    pub address_initial: usize,
    pub address_final: usize,
    pub operation: char,
    pub parameters: &'a str,
}
// ^^^ Data Structures ^^^ }}}

/// Parses command-mode input {{{
///
/// This is the public interface to the parse module
///
pub fn parse_command<'a>( _cmd_input: &'a str, file_opened: &File )
        -> Result<Command<'a>, RedError> {
    // MUST initialize?
    let mut _address_initial: usize = 1;
    let mut _address_final: usize = 1;
    let mut _operation: char = 'p';
    let _parameters: &str;
    let addrs: &str;

    let ( op_indx, _operation ) = get_opchar_index( _cmd_input )
            .expect( "parse_command: unable to determine opchar" );

    match _cmd_input.split_at( op_indx ) {
        (x, y) => {
            addrs = x;
            _parameters = &y[2..];
        },
    }
    match get_address( addrs, file_opened ) {
        ( x, y ) => {
            _address_initial = x;
            _address_final = y;
        }
    }

    Ok( Command {
            address_initial: _address_initial,
            address_final: _address_final,
            operation: _operation,
            parameters: _parameters,
        }
    )
}
//}}}

/// Identify address or address range {{{
///
#[allow(unused_variables)]
fn get_address( address_string: &str, file_opened: &File ) -> (usize, usize) {
    // start with simple auto-return of ( 1u32, 2u32 )
    ( 1_usize, 2_usize )
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
/// All examples repeated in tests module below
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
fn get_opchar_index( _cmd_input: &str ) -> Result<(usize, char), RedError> {
    let mut result_indx: Option<usize> = None;
    let mut result_char: char = '\0';

    struct Interpret {
        index: usize,       // index of character in string
        pattern_char: char, // stores '/' or '?' while reading pattern
        escape: bool,       // last character read was '\'
        expect: bool,       // we expect next char to be ',', ';', or alpha
    }
    let mut current_state: Interpret = Interpret {
        index: 0_usize,
        // TODO: change pattern_char to pair of bool switches for // and ??
        pattern_char: '\0',
        escape: false,
        expect: false,  // expect subsequent , or ;
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
                                result_indx = Some( current_state.index );
                                result_char = x;
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
                            result_indx = Some( current_state.index );
                            result_char = x;
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
    match result_indx {
        Some(x) => {
            println!( "opchar result {:?}, {:?}", result_indx, result_char );
            Result::Ok( (x, result_char) )
        },
        None => Result::Err( RedError::OpCharIndex ),
    }
}

#[cfg(test)]
mod tests {
    use super::get_opchar_index;

// Tests for parse::get_opchar_index
/// No address given
#[test]
fn get_opchar_index_test_1() {
       let _in: &str = "e myfile.txt";
       assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
               (0, 'e') );
}

/// No address given, with spaces
#[test]
fn get_opchar_index_test_2() {
    let _in: &str = "       e myfile.txt";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (0, 'e') );
}

/// No address given, with spaces and tabs
#[test]
fn get_opchar_index_test_3() {
    let _in: &str = "  		  	e myfile.txt";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (0, 'e') );
}

/// Most basic address value types
#[test]
fn get_opchar_index_test_4() {
    let _in: &str = ".a";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (1, 'a') );
}

#[test]
fn get_opchar_index_test_5() {
    let _in: &str = ".,.p";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (3, 'p') );
}

/// Slightly more complicated
#[test]
fn get_opchar_index_test_6() {
    let _in: &str = ".-2,.+2p";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (7, 'p') );
}

/// Regular expression match line search forward
#[test]
fn get_opchar_index_test_7() {
    let _in: &str = "/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (37, 's') );
}

/// Regular expression match line search forward with spaces and tabs
#[test]
fn get_opchar_index_test_8() {
    let _in: &str =
    "		  	/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (37, 's') );
}

/// Regular expression match line search backward
#[test]
fn get_opchar_index_test_9() {
    let _in: &str = "?^Beginning with.*$?,?.* at the end$?s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or( (9999, '\0') ),
            (37, 's') );
}

}
