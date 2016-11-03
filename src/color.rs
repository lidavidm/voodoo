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
    // Cyan,
    // Green,
    // LightBlack,
    // LightBlue,
    // LightCyan,
    // LightGreen,
    // LightMagenta,
    // LightRed,
    // LightWhite,
    // LightYellow,
    // Magenta,
    // Red,
    Reset,
    // Rgb(pub u8, pub u8, pub u8),
    White,
    // Yellow,
}

use self::ColorValue::*;

impl Color for ColorValue {
    fn write_fg(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            // AnsiValue(pub u8),
            Black => write!(f, ansi_csi!("30m")),
            Blue => write!(f, ansi_csi!("40m")),
            // Cyan,
            // Green,
            // LightBlack,
            // LightBlue,
            // LightCyan,
            // LightGreen,
            // LightMagenta,
            // LightRed,
            // LightWhite,
            // LightYellow,
            // Magenta,
            // Red,
            Reset => write!(f, ansi_csi!("39m")),
            // Rgb(pub u8, pub u8, pub u8),
            White => write!(f, ansi_csi!("37m")),
            // Yellow,
        }
    }

    fn write_bg(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            // AnsiValue(pub u8),
            Black => write!(f, ansi_csi!("40m")),
            Blue => write!(f, ansi_csi!("44m")),
            // Cyan,
            // Green,
            // LightBlack,
            // LightBlue,
            // LightCyan,
            // LightGreen,
            // LightMagenta,
            // LightRed,
            // LightWhite,
            // LightYellow,
            // Magenta,
            // Red,
            Reset => write!(f, ansi_csi!("40m")),
            // Rgb(pub u8, pub u8, pub u8),
            White => write!(f, ansi_csi!("47m")),
            // Yellow,
        }
    }
}
