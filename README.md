# jason-longshore

This is the source code for my personal website.

### End Of Life

This project has been superceded: https://github.com/longshorej/jasonlongshore.com

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
2) Commit your changes.
3) Login with `docker login`
4) Run `cargo make publish-docker`
