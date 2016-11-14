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

use regex::Regex;

use error::*;
use io::*;
use buf::*;
use ::{EditorState, EditorMode, print_help};

// ^^^ Bring in to namespace ^^^ }}}

// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
// *** Constants *** {{{
const ADDR_REGEX_FWDSEARCH: &'static str = r#"/([^/]*)/"#;
const ADDR_REGEX_REVSEARCH: &'static str = r#"\?([^\?]*)\?"#;
const ADDR_REGEX_RITHMETIC: &'static str = r#"(\d*|.|$)(((\+|-)(\d*))+)"#;
const ADDR_REGEX_ADDORSUBT: &'static str = r#"((\+|-)(\d*))(((\+|-)(\d*))*)"#;

// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
pub struct Command<'a> {
    pub address_initial: usize,
    pub address_final: usize,
    pub operation: char,
    pub parameters: &'a str,
}

// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Parses command-mode input {{{
///
/// This is the public interface to the parse module
///
pub fn parse_command<'a>( _cmd_input: &'a str, buffer: &Buffer,// {{{
                          state: &EditorState )
        -> Result<Command<'a>, RedError> {
    // MUST initialize?
    let mut _address_initial: usize = 1;
    let mut _address_final: usize = 1;
    let mut _operation: char = 'p';
    let _parameters: &str;
    let addrs: &str;

    match state.mode {
        EditorMode::Insert => {
            Err( RedError::CriticalError(
                    "parse_command: executing command while in input mode!"
                    .to_string() ))
        },
        EditorMode::Command => {
            let ( op_indx, _operation ) =
                    match get_opchar_index( _cmd_input ) {
                        Ok( x ) => x,
                        Err( e ) => return Err(e),
                    };

            match _cmd_input.split_at( op_indx ) {
                (x, y) => {
                    addrs = x;
                    _parameters = &y[1..].trim();
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
        }
    }
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
    let (left, right) = match address_string {
        "%" => ( "0", "$" ),
        "," => ( "0", "$" ),
        ";" => ( ".", "$" ),
        _ => parse_address_list( address_string ),
    };

    let result_right = try!(parse_address_field( right, buffer )).unwrap();
    let result_left = match left.len() {
        0 => result_right,
        _ => try!(parse_address_field( left, buffer )).unwrap(),
    };
    Ok( (result_left, result_right) )

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
                                left = x.trim();
                            }
                            right = &y[ 1 .. ].trim();
                        }
                    }
                }
            }
            None => {
                return ( left, right );
            },
        }
    }
}// }}}
// }}}
/// Ensure line number is in buffer range// {{{
fn normalize_line_num( buffer: &Buffer, line_num: usize ) -> usize {// {{{
    if line_num > buffer.num_lines() {
        buffer.num_lines()
    } else if line_num < 1 {
        1
    } else {
        line_num
    }
    // not reached
}// }}}
// }}}
/// Calculates address from arithmetic expression// {{{
fn calc_address_field( address: &str, buffer: &Buffer )// {{{
            -> Result<Option<usize>, RedError> {
    let re_rithmetic: Regex = Regex::new( ADDR_REGEX_RITHMETIC ).unwrap();
    let re_addorsubt: Regex = Regex::new( ADDR_REGEX_ADDORSUBT ).unwrap();
    let rithmetic_captures = re_rithmetic.captures( address ).unwrap();
    let operand_lstr: &str = rithmetic_captures.at( 1 ).unwrap();
    let mut operand_adds: usize = 0;
    let mut operand_subs: usize = 0;
    let mut operation_str: &str;
    let mut operand_str: &str;
    let mut next_step_str: &str;
    // Parse left operand value
    if operand_lstr == "." || operand_lstr == "" {
        operand_adds = buffer.get_current_line_number();
    } else if operand_lstr == "$" {
        operand_adds = buffer.num_lines();
    } else {
        operand_adds = try!( operand_lstr.parse()
                                .map_err(|_| RedError::AddressSyntax{
                                address: address.to_string() } ));
    }

    next_step_str = rithmetic_captures.at(2).expect(
            "parse::calc_address_field: regex capture missing" );
    let mut next_step_caps = re_addorsubt.captures( next_step_str ).unwrap();
    loop {
        operation_str = next_step_caps.at( 2 ).expect(
                "parse::calc_address_field: regex capture missing" );
        operand_str = next_step_caps.at( 3 ).expect(
                "parse::calc_address_field: regex capture missing" );

        match operation_str {
            "+" => {
                operand_adds += match operand_str {
                    "" => 1,
                    _ => try!( operand_str.parse()
                                .map_err(|_| RedError::AddressSyntax{
                                address: address.to_string() } )),
                };
            },
            "-" => {
                operand_subs += match operand_str {
                    "" => 1,
                    _ => try!( operand_str.parse()
                                .map_err(|_| RedError::AddressSyntax{
                                address: address.to_string() } )),
                };
            },
            _ => {
                return Err( RedError::AddressSyntax{
                    address: address.to_string() });
            },
        }

        match next_step_caps.at(4) {
            Some( "" ) => break,
            Some( x )  => next_step_str = x,
            None => return Err( RedError::AddressSyntax{
                    address: address.to_string() }),
        }
        next_step_caps = re_addorsubt.captures( next_step_str ).unwrap();
    }

    if operand_subs > operand_adds {
        Ok( Some( 1 ))
    } else {
        Ok( Some( normalize_line_num( &buffer, operand_adds - operand_subs ) ))
    }
}// }}}
// }}}
/// Parse address field; convert regex or integer into line number// {{{
fn parse_address_field( address: &str, buffer: &Buffer )// {{{
            -> Result<Option<usize>, RedError> {
    let re_fwdsearch: Regex = Regex::new( ADDR_REGEX_FWDSEARCH ).unwrap();
    let re_revsearch: Regex = Regex::new( ADDR_REGEX_REVSEARCH ).unwrap();
    let re_rithmetic: Regex = Regex::new( ADDR_REGEX_RITHMETIC ).unwrap();
    if address.len() == 0 {
        Ok( Some( buffer.get_current_line_number() ))
    } else if re_fwdsearch.is_match( address ) {
        Ok( buffer.find_match( re_fwdsearch.captures( address )
                               .unwrap().at(1).unwrap() ))
    } else if re_revsearch.is_match( address ) {
        Ok( buffer.find_match_reverse( re_revsearch.captures( address )
                               .unwrap().at(1).unwrap() ))
    //TODO: markers in arithmetic expression
    } else if re_rithmetic.is_match( address ) {
        calc_address_field( address, &buffer )
    } else if address.len() == 1 {
        match address {
            "." => {
                Ok( Some( buffer.get_current_line_number() ))
            },
            "$" => {
                Ok( Some( buffer.num_lines() ))
            },
            _ => {
                match address.parse() {
                Ok(x) => Ok( Some( normalize_line_num( &buffer, x ) )),
                Err(_) => Err( RedError::AddressSyntax {
                    address: address.to_string() } ),
                }
            },
        }
    } else if address.len() == 2 && &address[0..1] == "\'" {
        Ok( Some(
                buffer.get_marked_line( *&address[1..2]
                                        .chars().next().unwrap() ) ))
    } else {
        match address.parse() {
        Ok(x) => Ok( Some( normalize_line_num( &buffer, x ) )),
        Err(e) => Err( RedError::AddressSyntax { address: address.to_string() } )
        }
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
// ^^^ Functions ^^^ }}}
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
        let expected: usize = 8;            // num_lines
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
    #[test]
    fn parse_address_field_test_5() {
        // set contstants
        let test_num: u8 = 5;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "?testcmd4?";
        let expected: usize = 4;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_6() {
        // set contstants
        let test_num: u8 = 6;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "/cmda te/";     // no match
        let expected: Option<usize> = None;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_7() {
        // set contstants
        let test_num: u8 = 7;
        let command_line_version: u8 = 2;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "/cmda te/";
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
    fn parse_address_field_test_8() {
        // set contstants
        let test_num: u8 = 8;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = ".-3";
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
    fn parse_address_field_test_9() {
        // set contstants
        let test_num: u8 = 9;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "+";
        let expected: usize = 2;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_10() {
        // set contstants
        let test_num: u8 = 10;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "5-3";
        let expected: usize = 2;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_11() {
        // set contstants
        let test_num: u8 = 11;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = ".+1";
        let expected: usize = 2;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_12() {
        // set contstants
        let test_num: u8 = 12;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "7--1-3";
        let expected: usize = 2;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_13() {
        // set contstants
        let test_num: u8 = 13;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = ".+1---+5";
        let expected: usize = 4;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_14() {
        // set contstants
        let test_num: u8 = 14;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "$-3";
        let expected: usize = 5;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
    #[test]
    fn parse_address_field_test_15() {
        // set contstants
        let test_num: u8 = 15;
        let command_line_version: u8 = 1;
        //
        let mut buffer = open_command_buffer_test( test_num,
                                                   command_line_version );
        let address_string: &str = "-5";
        let expected: usize = 1;
        //
        let result = parse_address_field( address_string, &buffer )
            .unwrap()
            .unwrap();
        assert_eq!( result, expected );
        // Common test close routine
        close_command_buffer_test( &mut buffer );

    }
}
