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

    pub fn blend_at<C: Into<TermCell>>(&mut self, Point { x, y }: Point, c: C) {
        // TODO: use a full 'style overlay' system not this crap
        let x = x;
        let y = y;
        if x > self.width || y > self.height {
            return;
        }
        let idx = (y * self.width + x) as usize;
        let mut tc = c.into();
        if let Some(old) = self.contents[idx] {
            tc.bg = tc.bg.or(old.bg);
            tc.fg = tc.fg.or(old.fg);
        }
        self.contents[idx] = Some(tc);
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
