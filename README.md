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

That runs a build script which creates `dock` directory with a ready to start `Dockerfile`. Build and run a container with:
```
docker-compose up -d --build
```

Visit [localhost](http://localhost/) to open the application. Currently only http support, so ensure the browser opens a page with `http://` prefix.

To show the server log make:
```
docker-compose logs
```
