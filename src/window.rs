use ::std::cell::Cell;
use ::std::io::{Write};
use termion;
use termion::cursor::Goto;

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
    c: char,
}

impl Into<TermCell> for char {
    fn into(self) -> TermCell {
        TermCell {
            c: self,
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

    pub fn refresh(&mut self, stdout: &mut ::std::io::Stdout) {
        for (idx, (dirty, presented)) in self.dirty.iter().zip(self.presented.iter()).enumerate() {
            let idx = idx as u16;
            if *dirty != presented.get() {
                let row = idx / self.width;
                let col = idx - row * self.width;
                write!(stdout, "{}{}", Goto(self.position.x + col + 1, self.position.y + row + 1), dirty.c).unwrap();
                presented.set(*dirty);
            }
        }
        stdout.flush().unwrap();
    }
}
