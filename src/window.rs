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
        if p.x <= self.x || p.y <= self.y {
            None
        }
        else {
            Some(Point::new(p.x - self.x, p.y - self.y))
        }
    }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Brightness {
    Bold,
    Faint,
    Normal,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub struct TermCell {
    pub c: char,
    pub bg: Option<ColorValue>,
    pub fg: Option<ColorValue>,
    pub brightness: Brightness,
}

pub struct FormattedString<'a> {
    pub s: &'a str,
    pub bg: Option<ColorValue>,
    pub fg: Option<ColorValue>,
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
            brightness: Brightness::Normal,
        }
    }

    pub fn with_fg(self, fg: ColorValue) -> TermCell {
        TermCell {
            c: self.c,
            bg: self.bg,
            fg: Some(fg),
            brightness: self.brightness,
        }
    }

    pub fn faint(self) -> TermCell {
         TermCell {
            c: self.c,
            bg: self.bg,
            fg: self.fg,
            brightness: Brightness::Faint,
        }
    }
}

impl Into<TermCell> for char {
    fn into(self) -> TermCell {
        TermCell {
            c: self,
            bg: None,
            fg: None,
            brightness: Brightness::Normal,
        }
    }
}

impl<'a> Into<FormattedString<'a>> for &'a str {
    fn into(self) -> FormattedString<'a> {
        FormattedString {
            s: self,
            bg: None,
            fg: None,
        }
    }
}

impl<'a> Into<FormattedString<'a>> for &'a String {
    fn into(self) -> FormattedString<'a> {
        FormattedString {
            s: self,
            bg: None,
            fg: None,
        }
    }
}

pub struct Window {
    pub position: Point,
    pub width: u16,
    pub height: u16,
    contents: Vec<TermCell>,
}

// TODO: document that this is ZERO BASED COORDINATES
impl Window {
    pub fn new(position: Point, width: u16, height: u16) -> Window {
        let size = width as usize * height as usize;
        let contents = vec![' '.into(); size];
        Window {
            position: position,
            width: width,
            height: height,
            contents: contents,
        }
    }

    pub fn clear(&mut self) {
        for cell in self.contents.iter_mut() {
            *cell = ' '.into();
        }
    }

    pub fn border(&mut self) {
        // TODO: need to convert coordinates
        let width = self.width;
        let height = self.height;
        for y in 1..self.height {
            self.put_at(Point::new(0, y), '│');
            self.put_at(Point::new(width - 1, y), '│');
        }

        for w in 1..self.width {
            self.put_at(Point::new(w, 0), '─');
            self.put_at(Point::new(w, height - 1), '─');
        }

        self.put_at(Point::new(0, 0), '┌');
        self.put_at(Point::new(width - 1, 0), '┐');
        self.put_at(Point::new(0, height - 1), '└');
        self.put_at(Point::new(width - 1, height - 1), '┘');
    }

    pub fn put_at<C: Into<TermCell>>(&mut self, Point { x, y }: Point, c: C) {
        // TODO: event itself should probably be translated
        let x = x;
        let y = y;
        if x > self.width || y > self.height {
            self.contents[0] = 'A'.into();
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.contents[idx] = c.into();
    }

    pub fn print_at<'a, F: Into<FormattedString<'a>>>(&mut self, Point { x, y }: Point, s: F) {
        let f = s.into();
        for (offset, c) in f.s.chars().enumerate() {
            let mut t: TermCell = c.into();
            t.bg = f.bg;
            t.fg = f.fg;
            self.put_at(Point { x: x + offset as u16, y: y }, t);
        }
    }

    pub fn refresh(&self, compositor: &mut ::compositor::Compositor) {
        compositor.blit(self.position, self.width, self.height, &self.contents);
    }
}
