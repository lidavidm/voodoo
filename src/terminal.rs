use termion;

use termion::input::{MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{stdout, stdin};

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
}

impl Drop for Terminal {
    fn drop(&mut self) {
        self.cursor(Mode::Enabled);
    }
}
