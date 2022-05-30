macro_rules! info {
    ($l:expr, $e:expr) => {{
        use termion::{color, style};

        println!(
            "{}{}{:>12}{}{} {}",
            color::Fg(color::Blue),
            style::Bold,
            $l,
            color::Fg(color::Reset),
            style::Reset,
            $e
        );
    }};
}

pub(crate) use info;

macro_rules! error {
    ($e:expr) => {
        use termion::{color, style};

        println!(
            "{}{}error{}{}: {}",
            color::Fg(color::Red),
            style::Bold,
            color::Fg(color::Reset),
            style::Reset,
            $e
        );
    };
}

pub(crate) use error;
