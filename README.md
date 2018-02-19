# jason-longshore

This is the source code for my personal website.

### Build

#### Static Binary (Linux)

```bash
cargo make linux
```

#### Docker Image (Linux)

```bash
cargo make docker
```

### Publish

This project is build and published to [DockerHub](https://hub.docker.com/r/longshorej/jason-longshore/). To make a new release, follow the steps below.

1) Edit `Cargo.toml` and increment the release version.
2) `cargo clean; cargo test; cargo build`
3) Commit all changes, including Cargo.lock
4) Login with `docker login`
5) Run `cargo make publish-docker`
