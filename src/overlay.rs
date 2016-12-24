use super::window::{Point, TermCell};

pub struct Overlay {
    pub position: Point,
    pub width: u16,
    pub height: u16,
    contents: Vec<Option<TermCell>>,
}

// TODO: document that this is ZERO BASED COORDINATES
impl Overlay {
    pub fn new(position: Point, width: u16, height: u16) -> Overlay {
        let size = width as usize * height as usize;
        let contents = vec![None; size];
        Overlay {
            position: position,
            width: width,
            height: height,
            contents: contents,
        }
    }

    pub fn clear(&mut self) {
        for cell in self.contents.iter_mut() {
            *cell = None;
        }
    }

    pub fn put_at<C: Into<TermCell>>(&mut self, Point { x, y }: Point, c: C) {
        let x = x;
        let y = y;
        if x > self.width || y > self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        self.contents[idx] = Some(c.into());
    }

    pub fn refresh(&self, compositor: &mut ::compositor::Compositor) {
        for (index, cell) in self.contents.iter().enumerate() {
            let index = index as u16;
            if let Some(tc) = *cell {
                let y_offset = index / self.width;
                let x_offset = index % self.width;
                let point = Point::new(self.position.x + x_offset, self.position.y + y_offset);
                compositor.poke(point, tc);
            }
        }
    }
}
