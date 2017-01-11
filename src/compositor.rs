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

use std::cell::Cell;
use std::io::Write;

use termion::color::{Bg, Fg, Reset};
use termion::cursor::Goto;
use termion::style;

use color::ColorValue;
use window::{Brightness, Point, TermCell};

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
        for row_offset in 0..height {
            let src_offset = (row_offset * width) as usize;
            let dst_offset = ((pos.y + row_offset) * self.width + pos.x) as usize;
            let src_slice = &buffer[src_offset..src_offset + width as usize];
            let dst_slice = &mut self.dirty[dst_offset..dst_offset + width as usize];
            dst_slice.copy_from_slice(src_slice);
        }
    }

    pub fn poke(&mut self, pos: Point, cell: TermCell) {
        let offset = (pos.y * self.width + pos.x) as usize;
        self.dirty[offset] = cell;
    }

    pub fn refresh(&mut self, compositor: &mut ::compositor::Compositor) {
        compositor.blit(Point::new(0, 0), self.width, self.height, &self.dirty);
    }

    pub fn display<W: Write>(&mut self, stdout: &mut W) {
        // TODO: need to diff colors as well
        let mut last_fg = None;
        let mut last_bg = None;

        for (idx, (dirty, presented)) in self.dirty.iter().zip(self.presented.iter()).enumerate() {
            let idx = idx as u16;
            if *dirty != presented.get() {
                let row = idx / self.width;
                let col = idx - row * self.width;
                let g = Goto(col + 1, row + 1);

                if dirty.brightness == Brightness::Faint {
                    write!(stdout, "{}", style::Faint).unwrap();
                }
                if dirty.bg != last_bg {
                    write!(stdout, "{}", if let Some(color) = dirty.bg {
                        Bg(color)
                    } else { Bg(ColorValue::Reset) }).unwrap();
                    last_bg = dirty.bg;
                }
                if dirty.fg != last_fg {
                    write!(stdout, "{}", if let Some(color) = dirty.fg {
                        Fg(color)
                    } else { Fg(ColorValue::Reset) }).unwrap();
                    last_fg = dirty.fg;
                }
                write!(stdout, "{}{}", g, dirty.c).unwrap();
                if dirty.brightness == Brightness::Faint {
                    write!(stdout, "{}", style::NoFaint).unwrap();
                }

                presented.set(*dirty);
            }
        }
        write!(stdout, "{}{}", Fg(ColorValue::Reset), Bg(ColorValue::Reset)).unwrap();
        stdout.flush().unwrap();
    }
}
