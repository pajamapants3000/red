/*
 * File   : command.rs
 * Purpose: defines functions which carry out possible user commands
 * Program: red
 * About  : command-line text editor
 * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
 * License: MIT; See LICENSE!
 * Notes  : Notes on successful compilation
 * Created: 11/05/2016
 */

// *** Bring in to namespace *** {{{
// ^^^ Bring in to namespace ^^^ }}}

// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}

// *** Constants *** {{{
// ^^^ Constants ^^^ }}}

// *** Data Structures *** {{{
// ^^^ Data Structures ^^^ }}}

// *** Functions *** {{{
/// Display range of lines of buffer in terminal 
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
print( buffer: &Buffer, address_i: usize, address_f: usize )
        -> Result<(), RedError> {
    for indx in address_i .. address_f {
        println!("{}", buffer.get_line_content( indx ).unwrap() );
    }
}
// ^^^ Functions ^^^ }}}


