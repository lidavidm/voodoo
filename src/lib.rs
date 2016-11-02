extern crate ncurses;

pub mod input;
pub mod window;

pub fn term_size() -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    ncurses::getmaxyx(ncurses::stdscr(), &mut y, &mut x);

    (x, y)
}

pub fn poll_event() -> Option<ncurses::WchResult> {
    // Can't poll non-root screen for key events - it doesn't work
    // anymore (dead link:
    // http://blog.chris.tylers.info/index.php?/archives/212-Using-the-Mouse-with-ncurses.html)
    // Need to call refresh() or getch() will call it for us, clearing
    // the screen
    // TODO: use wget_ch (behind flag?)
    match ncurses::getch() {
        ncurses::ERR => {
            None
        }
        v => {
            if v >= ncurses::KEY_MIN {
                Some(ncurses::WchResult::KeyCode(v))
            }
            else {
                Some(ncurses::WchResult::Char(v as u32))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
