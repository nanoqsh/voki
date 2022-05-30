# Voki
Chat app written in Rust

## Build
To build entire application make:
```
cargo run
```

To build application with `release` profile make:
```
cargo run release
```

That runs a build script which creates `dock` directory with a ready to start `Dockerfile`. Build a container with:
```
docker build -t voki ./dock
```

## Run
todo!()
