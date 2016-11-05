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
use std::fs::File;
use std::str::Bytes;

use error::*;
use io::*;
use buf::*;

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
pub fn parse_command<'a>( _cmd_input: &'a str, buffer: &Buffer )
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
    match get_address_range( addrs, buffer ) {
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

/// Identify address range {{{
///
fn get_address_range( address_string: &str, buffer: &Buffer ) -> (usize, usize) {
    // start with simple auto-return of ( 1u32, 2u32 )
    ( 1_usize, 5_usize ) // for use with tests
    /*
    let address_initial: usize;
    let address_final: usize;
    */
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
    let mut current_indx: usize = 0;
    let mut bytes_iter: Bytes = _cmd_input.trim().bytes();
    loop {
        match bytes_iter.next() {
            Some( x ) => {
                if _cmd_input.is_char_boundary( current_indx ) {
                    if !is_in_regex( _cmd_input.trim(), current_indx ) {
                        match x {
                            b'a'...b'z' | b'A'...b'Z' => {
                                return Ok( (current_indx, x as char ) );
                            },
                            _ => {},
                        }
                    }
                }
            },
            None => break,
        }
        current_indx += 1;
    }
    Result::Err( RedError::OpCharIndex )
}
/// Return true if index is contained in regex
///
/// Is regex if wrapped in /.../ or ?...? within larger string
/// In some functions, we need to know this so we know how to treat the
/// character
fn is_in_regex( text: &str, indx: usize ) -> bool {// {{{
    let regex: Vec<u8> = vec!(b'/', b'?');
    let mut c_regex: Vec<bool> = vec!( false; regex.len() );
    let mut c_indx: usize = 0;
    let mut escaped: bool = false;
    let thechar = &text[indx..indx+1];  // debug
    //
    let (left, _) = text.split_at( indx );
    //
    for ch in left.bytes() {
        if left.is_char_boundary( c_indx ) {
            if ch == b'\\' {
                escaped = !escaped;
                c_indx += 1;
                continue
            }
            for i in 0 .. regex.len() {
                if ch == regex[i] {
                    if !escaped && !is_quoted( text, c_indx ) &&
                            c_regex[1-i] == false {     // can't have both
                        c_regex[i] = !c_regex[i];       // switch on/off
                    }
                }
            }
            escaped = false;
        }
        c_indx += 1;
    }
    if c_regex == vec!( false; c_regex.len() ) {
        println!("false");
        false
    } else {
        println!("true");
        true
    }
}// }}}
#[cfg(test)]
mod tests {
    use super::{get_opchar_index, is_in_regex};

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
        "	 	 /^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
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

    #[test]
    fn is_in_regex_test_1() {
        let haystack = "This is a / abc /string to search";
        let indx = haystack.find( "abc" ).unwrap();
        assert!( is_in_regex( haystack, indx ), "abc" );
        let indx = haystack.find( "is a" ).unwrap();
        assert!( !is_in_regex( haystack, indx ), "is a" );
        let indx = haystack.find( "search" ).unwrap();
        assert!( !is_in_regex( haystack, indx ), "search" );
    }
    #[test]
    fn is_in_regex_test_2() {
        let haystack = "This is a ? abc ?string to search";
        let indx = haystack.find( "abc" ).unwrap();
        assert!( is_in_regex( haystack, indx ) );
        let indx = haystack.find( "is a" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
        let indx = haystack.find( "string" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
    }
    #[test]
    fn is_in_regex_test_3() {
        let haystack = r#"?This? "is a / abc /string" to search"#;
        let indx = haystack.find( "abc" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
        let indx = haystack.find( "is a" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
        let indx = haystack.find( "string" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
        let indx = haystack.find( "This" ).unwrap();
        assert!( is_in_regex( haystack, indx ) );
    }
    #[test]
    fn is_in_regex_test_4() {
        let haystack: &str =
        "		  	/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
        let indx = haystack.find( "Beginning" ).unwrap();
        assert!( is_in_regex( haystack, indx ) );
        let indx = haystack.find( "the end" ).unwrap();
        assert!( is_in_regex( haystack, indx ) );
        let indx = haystack.find( "with" ).unwrap();
        assert!( is_in_regex( haystack, indx ) );
        let indx = haystack.find( "mytest" ).unwrap();
        assert!( !is_in_regex( haystack, indx ) );
    }

}
