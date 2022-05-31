macro_rules! info {
    ($l:expr, $e:expr) => {
        println!(
            "{}{}{:>12}{}{} {}",
            ::termion::color::Fg(::termion::color::Blue),
            ::termion::style::Bold,
            $l,
            ::termion::color::Fg(::termion::color::Reset),
            ::termion::style::Reset,
            $e
        );
    };
}

pub(crate) use info;

macro_rules! error {
    ($e:expr) => {
        println!(
            "{}{}error{}{}: {}",
            ::termion::color::Fg(::termion::color::Red),
            ::termion::style::Bold,
            ::termion::color::Fg(::termion::color::Reset),
            ::termion::style::Reset,
            $e
        );
    };
}

pub(crate) use error;
