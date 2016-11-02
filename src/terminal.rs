use ncurses;

#[derive(Debug)]
pub enum VoodooError {
    NCursesError,
}

pub struct Terminal {
}

pub enum Mode {
    Enabled,
    Disabled,
}

use self::Mode::*;

impl Terminal {
    pub fn new() -> Terminal {
        ncurses::initscr();
        Terminal {}
    }

    pub fn cbreak(&self, mode: Mode) -> Result<(), VoodooError> {
        let result = match mode {
            Enabled => ncurses::cbreak(),
            Disabled => ncurses::nocbreak(),
        };
        if result == ncurses::ERR {
            Err(VoodooError::NCursesError)
        }
        else {
            Ok(())
        }
    }

    pub fn echo(&self, mode: Mode) -> Result<(), VoodooError> {
        let result = match mode {
            Enabled => ncurses::echo(),
            Disabled => ncurses::noecho(),
        };
        if result == ncurses::ERR {
            Err(VoodooError::NCursesError)
        }
        else {
            Ok(())
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
