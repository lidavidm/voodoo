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
        println!("\x1B[?47l");
    }
}
