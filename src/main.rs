mod info;
mod tools;

use self::{info::info, tools::*};
use std::{env, process::Command};

fn main() {
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
        info!("Building", name);

        Command::new(tool)
            .args(tool.args())
            .args(release.then(|| "--release"))
            .current_dir(name)
            .spawn()
            .expect("build")
            .wait()
            .expect("wait");
    }
}
