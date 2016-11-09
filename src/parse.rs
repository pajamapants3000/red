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
use std::str::Bytes;
use std::panic::catch_unwind;

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
pub fn parse_command<'a>( _cmd_input: &'a str, buffer: &Buffer )// {{{
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
            _parameters = &y[1..];
        },
    }
    let ( _address_initial, _address_final ) = try!(
            get_address_range( addrs, buffer ) );

    Ok( Command {
            address_initial: _address_initial,
            address_final: _address_final,
            operation: _operation,
            parameters: _parameters,
        }
    )
}// }}}
//}}}

/// Identify address range {{{
///
/// What do we want to do if string ends in ',' or ';'?
/// e.g. :13,25,p
/// ?
/// Should this print lines 13-25 or ignore 13 and just print 25?
/// Probably want to ignore. or error?
fn get_address_range( address_string: &str, buffer: &Buffer )// {{{
            -> Result<(usize, usize), RedError> {
    let (left, right) = parse_address_list( address_string );

    Ok( ((try!(parse_address_field( left, buffer ))).unwrap(),
            (try!(parse_address_field( right, buffer ))).unwrap()) )
    
}// }}}
//}}}
/// Test whether char is an address separator// {{{
fn is_address_separator( ch: char ) -> bool {// {{{
    ch == ',' || ch == ';'
}// }}}
// }}}
/// Turn address list into two address expressions// {{{
///
/// Returns the latter-two non-empty elements in a list
/// elements are separated according to is_address_separator()
/// separators inside /.../ or ?...? are ignored using is_in_regex()
fn parse_address_list( address_string: &str ) -> (&str, &str) {// {{{
    let mut right: &str = address_string;
    let mut left: &str = "";

    loop {
        match right.find( is_address_separator ) {
            Some(indx) => {
                if !is_in_regex( address_string, indx ) {
                    match right.split_at( indx ) {
                        (x, y) => {
                            if x.len() > 0 {
                                left = x;
                            }
                            right = &y[ 1 .. ];
                        }
                    }
                }
            }
            None => {
                if left.len() == 0 {
                    left = right.clone();
                }
                return ( left, right );
            },
        }
    }
}// }}}
// }}}
/// Parse address field; convert regex or integer into line number// {{{
fn parse_address_field( address: &str, buffer: &Buffer )// {{{
            -> Result<Option<usize>, RedError> {
    if address.len() > 0 {
        match &address[0..1] {
            "/" => {
                if &address[ address.len() - 1 .. ] != "/" {
                    return Err( RedError::AddressSyntax );
                }
                Ok( buffer.find_match(
                        &address[1 .. address.len() - 1 ] ) )
            }
            "?" => {
                if &address[ address.len() - 1 .. ] != "?" {
                    return Err( RedError::AddressSyntax );
                }
                Ok( buffer.find_match_reverse(
                        &address[1 .. address.len()-1 ] ) )
            }
            _ => {
                match address.parse() {
                    Ok(x) => {
                        if x == 0 {
                            Ok( Some( 1 ) )
                        } else if x < buffer.num_lines() {
                            Ok( Some(x) )
                        } else {
                            Ok( Some( buffer.num_lines() ) )
                        }
                    },
                    _ => Ok( Some( buffer.get_current_line_number() ) ),
                }
            }
        }
    } else {
        Ok( Some( buffer.get_current_line_number() ) )
    }
}// }}}
// }}}
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
/// Return true if index is contained in regex// {{{
///
/// Is regex if wrapped in /.../ or ?...? within larger string
/// In some functions, we need to know this so we know how to treat the
/// character
fn is_in_regex( text: &str, indx: usize ) -> bool {// {{{
    let regex: Vec<u8> = vec!(b'/', b'?');
    let mut c_regex: Vec<bool> = vec!( false; regex.len() );
    let mut c_indx: usize = 0;
    let mut escaped: bool = false;
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
        false
    } else {
        true
    }
}// }}}
// }}}
#[cfg(test)]
mod tests {
    use super::{get_opchar_index, is_in_regex, parse_address_field, parse_address_list, get_address_range, is_address_separator};
    use buf::*;

    const COMMAND_CONTENT_LINE_1: &'static str = "testcmd";
    const COMMAND_CONTENT_LINE_2: &'static str = "testcmda testcmdb";
    const COMMAND_FILE_SUFFIX: &'static str = ".cmd";
    const TEST_FILE: &'static str = "red_filetest";

    /// Prep and return buffer for use in "command buffer" test functions
    ///
    /// uses test_lines function to create file with which buffer
    /// is initialized
    pub fn open_command_buffer_test( test_num: u8, command_line_version: u8 )// {{{
            -> Buffer {
        //
        let num_lines: usize = 7;   // number of lines to have in buffer
        let command_content_line = match command_line_version {
            1_u8 => COMMAND_CONTENT_LINE_1,
            2_u8 => COMMAND_CONTENT_LINE_2,
            _ => "",
        };
        let test_file: String = TEST_FILE.to_string() +
                COMMAND_FILE_SUFFIX + test_num.to_string().as_str();
        let test_command = "echo -e ".to_string() +
                                    &test_lines( command_content_line,
                                    num_lines );
        let mut buffer = Buffer::new( BufferInput::Command( test_command ));
        buffer.set_file_name( &test_file );
        buffer.set_current_line_number( 1 );
        buffer
    }// }}}
    /// deconstruct buffer from "command buffer" test;
    /// any other necessary closing actions
    pub fn close_command_buffer_test( buffer: &mut Buffer ) {// {{{
        buffer.destruct().unwrap();
    }// }}}
    // begin prep functions
    /// Generate and return string containing lines for testing
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
    #[test]
    fn is_address_separator_test_1() {
        let ch = ';';
        assert!( is_address_separator( ch ) );
    }
    #[test]
    fn is_address_separator_test_2() {
        let ch = '.';
        assert!( !is_address_separator( ch ) );
    }
    #[test]
    fn is_address_separator_test_3() {
        let ch = 'r';
        assert!( !is_address_separator( ch ) );
    }
    #[test]
    fn is_address_separator_test_4() {
        let ch = ',';
        assert!( is_address_separator( ch ) );
    }
    #[test]
    fn get_address_range_test_1() {
        // set contstants
        let test_num: u8 = 1;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "1, 3";
        let ini_expected: usize = 1;
        let fin_expected: usize = 3;
        //
        let ( ini, fin ) = get_address_range( address_string, &buffer ).unwrap();
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn get_address_range_test_2() {
        // set contstants
        let test_num: u8 = 2;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "1, 56";
        let ini_expected: usize = 1;
        let fin_expected: usize = 8;
        //
        let ( ini, fin ) = get_address_range( address_string, &buffer ).unwrap();
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn get_address_range_test_3() {
        // set contstants
        let test_num: u8 = 3;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "0, 4";
        let ini_expected: usize = 1;
        let fin_expected: usize = 4;
        //
        let ( ini, fin ) = get_address_range( address_string, &buffer ).unwrap();
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn get_address_range_test_4() {
        // set contstants
        let test_num: u8 = 4;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "/testcmd1/, 5";
        let ini_expected: usize = 1;
        let fin_expected: usize = 5;
        //
        let ( ini, fin ) = get_address_range( address_string, &buffer ).unwrap();
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn parse_address_list_test_1() {
        // set contstants
        let address_string: &str = "1, 3";
        let ini_expected: &str = "1";
        let fin_expected: &str = "3";
        //
        let ( ini, fin ) = parse_address_list( address_string );
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        
    }
    #[test]
    fn parse_address_list_test_2() {
        // set contstants
        let address_string: &str = "1, 56";
        let ini_expected: &str = "1";
        let fin_expected: &str = "56";
        //
        let ( ini, fin ) = parse_address_list( address_string );
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        
    }
    #[test]
    fn parse_address_list_test_3() {
        // set contstants
        let address_string: &str = "0, 4";
        let ini_expected: &str = "0";
        let fin_expected: &str = "4";
        //
        let ( ini, fin ) = parse_address_list( address_string );
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        
    }
    #[test]
    fn parse_address_list_test_4() {
        // set contstants
        let address_string: &str = "/testcmd3/, 5";
        let ini_expected: &str = "/testcmd3/";
        let fin_expected: &str = "5";
        //
        let ( ini, fin ) = parse_address_list( address_string );
        assert_eq!( ini, ini_expected );
        assert_eq!( fin, fin_expected );
        
    }
    #[test]
    fn parse_address_field_test_1() {
        // set contstants
        let test_num: u8 = 1;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "1";
        let expected: usize = 1;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn parse_address_field_test_2() {
        // set contstants
        let test_num: u8 = 2;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "56";
        let expected: usize = 8;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn parse_address_field_test_3() {
        // set contstants
        let test_num: u8 = 3;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "0";
        let expected: usize = 1;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
    #[test]
    fn parse_address_field_test_4() {
        // set contstants
        let test_num: u8 = 4;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "/testcmd3/";
        let expected: usize = 3;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );
        
    }
}
