use ::std::cell::Cell;
use ::std::io::{Write};
use termion;
use termion::color::{Bg, Fg};
use termion::cursor::Goto;

use color::ColorValue;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn from_global_frame(&self, p: Point) -> Option<Point> {
        if p.x < self.x || p.y < self.y {
            None
        }
        else {
            Some(Point::new(p.x - self.x, p.y - self.y))
        }
    }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct TermCell {
    pub c: char,
    pub bg: Option<ColorValue>,
    pub fg: Option<ColorValue>,
}

pub struct FormattedString<'a> {
    pub s: &'a str,
    pub bg: Option<ColorValue>,
}

impl TermCell {
    pub fn new(c: char) -> TermCell {
        c.into()
    }

    pub fn new_with_bg(c: char, bg: ColorValue) -> TermCell {
        TermCell {
            c: c,
            bg: Some(bg),
            fg: None,
        }
    }
}

impl Into<TermCell> for char {
    fn into(self) -> TermCell {
        TermCell {
            c: self,
            bg: None,
            fg: None,
        }
    }
}

impl<'a> Into<FormattedString<'a>> for &'a str {
    fn into(self) -> FormattedString<'a> {
        FormattedString {
            s: self,
            bg: None,
        }
    }
}

impl<'a> Into<FormattedString<'a>> for &'a String {
    fn into(self) -> FormattedString<'a> {
        FormattedString {
            s: self,
            bg: None,
        }
    }
}

pub struct Window {
    pub position: Point,
    pub width: u16,
    pub height: u16,
    dirty: Vec<TermCell>,
    presented: Vec<Cell<TermCell>>,
}

impl Window {
    pub fn new(position: Point, width: u16, height: u16) -> Window {
        let size = width as usize * height as usize;
        let dirty = vec![' '.into(); size];
        let presented = vec![Cell::new(' '.into()); size];
        Window {
            position: position,
            width: width,
            height: height,
            dirty: dirty,
            presented: presented,
        }
    }

    pub fn clear(&mut self) {
        for cell in self.dirty.iter_mut() {
            *cell = ' '.into();
        }
    }

    pub fn border(&mut self) {
        let width = self.width;
        let height = self.height;
        for y in 2..self.height {
            self.put_at(Point::new(1, y), '│');
            self.put_at(Point::new(width, y), '│');
        }

        for w in 2..self.width {
            self.put_at(Point::new(w, 1), '─');
            self.put_at(Point::new(w, height), '─');
        }

        self.put_at(Point::new(1, 1), '┌');
        self.put_at(Point::new(width, 1), '┐');
        self.put_at(Point::new(1, height), '└');
        self.put_at(Point::new(width, height), '┘');
    }

    pub fn put_at<C: Into<TermCell>>(&mut self, Point { x, y }: Point, c: C) {
        // TODO: event itself should probably be translated
        let x = x - 1;
        let y = y - 1;
        if x > self.width || y > self.height {
            self.dirty[0] = 'A'.into();
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.dirty[idx] = c.into();
    }

    pub fn print_at<'a, F: Into<FormattedString<'a>>>(&mut self, Point { x, y }: Point, s: F) {
        let f = s.into();
        for (offset, c) in f.s.chars().enumerate() {
            let mut t: TermCell = c.into();
            t.bg = f.bg;
            self.put_at(Point { x: x + offset as u16, y: y }, t);
        }
    }

    pub fn refresh(&mut self, stdout: &mut ::std::io::Stdout) {
        // TODO: need to diff colors as well
        for (idx, (dirty, presented)) in self.dirty.iter().zip(self.presented.iter()).enumerate() {
            let idx = idx as u16;
            if *dirty != presented.get() {
                let row = idx / self.width;
                let col = idx - row * self.width;
                let g = Goto(self.position.x + col + 1, self.position.y + row + 1);
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
