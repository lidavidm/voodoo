use std::io::{stdout, stdin};

use termion;
use termion::color::Color;
use termion::input::{MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    pub stdin: ::std::io::Stdin,
    pub stdout: MouseTerminal<RawTerminal<::std::io::Stdout>>,
}

pub enum Mode {
    Enabled,
    Disabled,
}

use self::Mode::*;

impl Terminal {
    pub fn new() -> Terminal {
        Terminal {
            stdin: stdin(),
            stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
        }
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
        self.clear();
        print!("{}", termion::color::Bg(bg));
        let (_, rows) = termion::terminal_size().unwrap();
        for _ in 1..rows {
            print!("{}", termion::clear::All);
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.cursor(Mode::Enabled);
    }
}
