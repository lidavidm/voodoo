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

use std::fmt::Formatter;
use termion::color::Color;

macro_rules! ansi_csi {
    ($($v:expr),*) => {
        concat!("\x1B[", $($v),*)
    };
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum ColorValue {
    // AnsiValue(pub u8),
    Black,
    Blue,
    Cyan,
    Green,
    // LightBlack,
    // LightBlue,
    // LightCyan,
    // LightGreen,
    // LightMagenta,
    // LightRed,
    // LightWhite,
    // LightYellow,
    Magenta,
    Red,
    Reset,
    // Rgb(pub u8, pub u8, pub u8),
    White,
    Yellow,
}

use self::ColorValue::*;

impl Color for ColorValue {
    fn write_fg(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            // AnsiValue(pub u8),
            Black => write!(f, ansi_csi!("30m")),
            Blue => write!(f, ansi_csi!("34m")),
            Cyan => write!(f, ansi_csi!("36m")),
            Green => write!(f, ansi_csi!("32m")),
            // LightBlack,
            // LightBlue,
            // LightCyan,
            // LightGreen,
            // LightMagenta,
            // LightRed,
            // LightWhite,
            // LightYellow,
            Magenta => write!(f, ansi_csi!("35m")),
            Red => write!(f, ansi_csi!("31m")),
            Reset => write!(f, ansi_csi!("39m")),
            // Rgb(pub u8, pub u8, pub u8),
            White => write!(f, ansi_csi!("37m")),
            Yellow => write!(f, ansi_csi!("33m")),
        }
    }

    fn write_bg(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            // AnsiValue(pub u8),
            Black => write!(f, ansi_csi!("40m")),
            Blue => write!(f, ansi_csi!("44m")),
            Cyan => write!(f, ansi_csi!("46m")),
            Green => write!(f, ansi_csi!("42m")),
            // LightBlack,
            // LightBlue,
            // LightCyan,
            // LightGreen,
            // LightMagenta,
            // LightRed,
            // LightWhite,
            // LightYellow,
            Magenta => write!(f, ansi_csi!("45m")),
            Red => write!(f, ansi_csi!("41m")),
            Reset => write!(f, ansi_csi!("40m")),
            // Rgb(pub u8, pub u8, pub u8),
            White => write!(f, ansi_csi!("47m")),
            Yellow => write!(f, ansi_csi!("43m")),
        }
    }
}
