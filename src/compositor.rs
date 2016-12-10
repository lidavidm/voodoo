use std::cell::Cell;
use std::io::{Stdout, Write};

use termion::color::{Bg, Fg};
use termion::cursor::Goto;

use color::ColorValue;
use window::{Point, TermCell};

pub struct Compositor {
    width: u16,
    height: u16,
    dirty: Vec<TermCell>,
    presented: Vec<Cell<TermCell>>,
}

impl Compositor {
    pub fn new(width: u16, height: u16) -> Compositor {
        let size = width as usize * height as usize;
        let dirty = vec![' '.into(); size];
        let presented = vec![Cell::new(' '.into()); size];

        Compositor {
            width: width,
            height: height,
            dirty: dirty,
            presented: presented,
        }
    }

    pub fn blit(&mut self, pos: Point, width: u16, height: u16, buffer: &[TermCell]) {

    }

    pub fn refresh(&mut self, stdout: &mut Stdout) {
        // TODO: need to diff colors as well
        for (idx, (dirty, presented)) in self.dirty.iter().zip(self.presented.iter()).enumerate() {
            let idx = idx as u16;
            if *dirty != presented.get() {
                let row = idx / self.width;
                let col = idx - row * self.width;
                let g = Goto(col + 1, row + 1);
                // TODO: this is rather inefficient
                write!(stdout, "{}{}", g, Bg(ColorValue::Reset)).unwrap();
                match (dirty.bg, dirty.fg) {
                    (Some(bg), Some(fg)) => write!(stdout, "{}{}{}{}", g, Bg(bg), Fg(fg), dirty.c),
                    (Some(bg), None) => write!(stdout, "{}{}{}", g, Bg(bg), dirty.c),
                    (None, Some(fg)) => write!(stdout, "{}{}{}", g, Fg(fg), dirty.c),
                    (None, None) => write!(stdout, "{}{}", g, dirty.c),
                }.unwrap();
                presented.set(*dirty);
            }
        }
        stdout.flush().unwrap();
    }
}
