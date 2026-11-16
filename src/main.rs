// rustses, lilith's own curses-like API written in Rust.
// copyright (C) 2026 ghlilith1k

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

// DEPENDENCIES
use inputbot::{KeySequence, KeybdKey::*};
use clearscreen;
// only for testing, comment out this
use std::io::{self, Read};

pub fn scroll() {

    // please include this file in your project /src folder, and rename it as rustses.src
    println!("this is a test, put this file in your project /src folder and rename it rustses.rs");
    Numrow1Key.bind(|| 
        println!("
            
        ")
    );
    inputbot::handle_input_events();

}

pub fn add_entry() {
    
    let mut entries = String::new();
    io::stdin().read_line(&mut entries).expect("failed to read string");
    for entry in entries.lines() {
        let (_, entry) = entries.split_once(' ').unwrap();
        println!("{}", entry.to_string());
    }

}

fn main() {
    // only for testing, this is not needed
    add_entry();
}