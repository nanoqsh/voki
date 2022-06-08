mod tools;

use self::tools::*;
use std::{
    env,
    env::consts::EXE_SUFFIX,
    fs, io,
    path::Path,
    process::{Command, ExitCode},
};

fn update(from: &str, to: &str) -> io::Result<()> {
    fn update_dir(from: &Path, to: &Path) -> io::Result<()> {
        loop {
            match fs::metadata(to) {
                Ok(meta) if !meta.file_type().is_dir() => {
                    fs::remove_file(to)?;
                }
                Ok(_) => break,
                Err(_) => break fs::create_dir_all(to)?,
            }
        }

        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let path = entry.path();

            if file_type.is_dir() {
                let name = path.file_name().expect("is a dir");
                update_dir(&path, &to.join(name))?;
            } else if file_type.is_file() {
                let name = path.file_name().expect("is a file");
                let to = to.join(name);
                if let Err(_) = update_file(&path, &to) {
                    println!("error: from {path:?} to {to:?}");
                }
            }
        }

        Ok(())
    }

    struct FileNotFound;

    fn update_file(from: &Path, to: &Path) -> Result<(), FileNotFound> {
        let from_last = fs::metadata(from)
            .map(|meta| meta.modified().expect("should support"))
            .map_err(|_| FileNotFound)?;

        fs::metadata(to)
            .map(|meta| meta.modified().expect("should support"))
            .ok()
            .map(|to_last| to_last < from_last)
            .unwrap_or(true)
            .then(|| fs::copy(from, to))
            .transpose()
            .map_err(|_| FileNotFound)?;

        Ok(())
    }

    let from = from.as_ref();
    let to = to.as_ref();

    if fs::metadata(from)?.file_type().is_file() {
        if let Err(_) = update_file(from, to) {
            println!("error: from {from:?} to {to:?}");
        }

        Ok(())
    } else {
        update_dir(from, to)
    }
}

fn main() -> ExitCode {
    let mut release = false;
    for arg in env::args() {
        if arg == "release" {
            release = true;
        }
    }

    let deps = [
        Dependency {
            name: "http",
            tool: Tool::Cargo,
        },
        Dependency {
            name: "server",
            tool: Tool::Cargo,
        },
        Dependency {
            name: "web",
            tool: Tool::Wasm,
        },
    ];

    for Dependency { name, tool } in deps {
        println!("Building: {}", name);

        let status = Command::new(tool)
            .args(tool.args())
            .args(release.then(|| "--release"))
            .current_dir(name)
            .spawn()
            .expect("build")
            .wait()
            .expect("wait");

        if !status.success() {
            eprintln!("error: building failed");
            return ExitCode::FAILURE;
        }
    }

    let sub = release.then(|| "release").unwrap_or("debug");
    let server_target_dir = format!("./server/target/{sub}/server{EXE_SUFFIX}");
    let http_target_dir = format!("./http/target/{sub}/http{EXE_SUFFIX}");
    let dirs = [
        ("./web/static", "./dock/voki/static"),
        ("./web/pkg", "./dock/voki/static/pkg"),
        (&server_target_dir, "./dock/voki/server"),
        (&http_target_dir, "./dock/voki/http"),
        ("./http/Rocket.toml", "./dock/voki/Rocket.toml"),
    ];

    for (from, to) in dirs {
        println!("Update: {}", to);

        if let Err(err) = update(from, to) {
            eprintln!("error: {err:?}");
            return ExitCode::FAILURE;
        }
    }

    println!("Done: Docker container is ready at ./dock");
    ExitCode::SUCCESS
}
