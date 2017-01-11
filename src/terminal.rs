// Copyright (C) 2016-2017 David Li

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::io::{stdout, stdin};

use termion;
use termion::color::Color;
use termion::input::MouseTerminal;
use termion::raw::{IntoRawMode, RawTerminal};

pub type Stdin = ::std::io::Stdin;
pub type Stdout = MouseTerminal<RawTerminal<::std::io::Stdout>>;

pub struct Terminal;

pub enum Mode {
    Enabled,
    Disabled,
}

use self::Mode::*;

impl Terminal {
    pub fn new() -> (Terminal, Stdin, Stdout) {
        // Save/restore screen
        println!("\x1B[?47h");
        // Enable all mouse events
        // TODO: figure out how to get Termion to report them
        println!("\x1B[?1003h");
        let stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
        (Terminal, stdin(), stdout)
    }

    pub fn cursor(&self, mode: Mode) {
        match mode {
            Enabled => print!("{}", termion::cursor::Show),
            Disabled => print!("{}", termion::cursor::Hide),
        };
    }

    pub fn clear(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn clear_color<C: Color>(&self, bg: C) {
        print!("{}", termion::color::Bg(bg));
        self.clear();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.clear_color(termion::color::Reset);
        self.cursor(Mode::Enabled);
        // Disable mouse events
        println!("\x1B[?1003l");
        // Restore the screen
        println!("\x1B[?47l");
    }
}
