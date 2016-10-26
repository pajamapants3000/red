/*
    * File   : test/mod.rs
    * Purpose: test module
    * Program: red
    * About  : command-line text editor
    * Authors: Tommy Lincoln <pajamapants3000@gmail.com>
    * License: MIT; See LICENSE!
    * Notes  : Notes on successful compilation
    * Created: 10/26/2016
    */

// *** Bring in to namespace *** {{{
use super::*;
use parse::*;
use error::*;

// ^^^ Bring in to namespace ^^^ }}}
// *** Attributes *** {{{
// ^^^ Attributes ^^^ }}}
// *** Constants *** {{{
// ^^^ Constants ^^^ }}}
// *** Data Structures *** {{{
// ^^^ Data Structures ^^^ }}}
// *** Functions *** {{{


#[cfg(not(test))]
fn main() {
       println!(
           "If you are reading this, the tests were neither compiled nor run!");
}

// Tests for parse::get_opchar_index
/// No address given
#[test]
fn get_opchar_index_test_1() {
       let _in: &str = "e myfile.txt";
       assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 0 );
}

/// No address given, with spaces
#[test]
fn get_opchar_index_test_2() {
    let _in: &str = "       e myfile.txt";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 0 );
}

/// No address given, with spaces and tabs
#[test]
fn get_opchar_index_test_3() {
    let _in: &str = "  		  	e myfile.txt";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 0 );
}

/// Most basic address value types
#[test]
fn get_opchar_index_test_4() {
    let _in: &str = ".a";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 1 );
}

#[test]
fn get_opchar_index_test_5() {
    let _in: &str = ".,.p";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 3 ); // test
}

/// Slightly more complicated
#[test]
fn get_opchar_index_test_6() {
    let _in: &str = ".-2,.+2p";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 7 ); //test
}

/// Regular expression match line search forward
#[test]
fn get_opchar_index_test_7() {
    let _in: &str = "/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 37 );
}

/// Regular expression match line search forward with spaces and tabs
#[test]
fn get_opchar_index_test_8() {
    let _in: &str =
    "		  	/^Beginning with.*$/;/.* at the end$/s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 37 );
}

/// Regular expression match line search backward
#[test]
fn get_opchar_index_test_9() {
    let _in: &str = "?^Beginning with.*$?,?.* at the end$?s_mytest_yourtest_g";
    assert_eq!( get_opchar_index( _in ).unwrap_or(9999), 37 );
}

// ^^^ Functions ^^^ }}}

