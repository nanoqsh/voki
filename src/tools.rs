use std::ffi;

#[derive(Clone, Copy)]
pub enum Tool {
    Cargo,
    Wasm,
}

impl Tool {
    pub fn args(&self) -> impl Iterator<Item = &ffi::OsStr> {
        let args: &[&str] = match self {
            Self::Cargo => &["build"],
            Self::Wasm => &["build", "--no-typescript", "--target", "web"],
        };

        args.iter().map(AsRef::as_ref)
    }
}

impl AsRef<ffi::OsStr> for Tool {
    fn as_ref(&self) -> &ffi::OsStr {
        match self {
            Self::Cargo => "cargo".as_ref(),
            Self::Wasm => "wasm-pack".as_ref(),
        }
    }
}

pub struct Dependency<'a> {
    pub name: &'a str,
    pub tool: Tool,
}
