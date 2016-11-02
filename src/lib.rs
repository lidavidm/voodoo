extern crate ncurses;

pub mod input;
pub mod terminal;
pub mod window;

pub fn term_size() -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    ncurses::getmaxyx(ncurses::stdscr(), &mut y, &mut x);

    (x, y)
}

pub enum Event {
    Char(char),
    SpecialKey(i32),
    Mouse,
}

pub fn poll_event() -> Option<Event> {
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
                if v == ncurses::KEY_MOUSE {
                    Some(Event::Mouse)
                }
                else {
                    Some(Event::SpecialKey(v))
                }
            }
            else {
                std::char::from_u32(v as u32).map(|c| Event::Char(c))
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
