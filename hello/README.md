#  Hello in RUST

RUST `Hello World` on Linux AMD64 

## Compile and execute `Hello` using Cargo
```console
cd ~/src
cargo new hello
cd hello
cargo build
cargo run
```

## Compile and execute `Hello' using rustc
```console
cd src
./rrun.sh main
```

## Build and Run `Hello` in a container
```console
# Build on the platform
docker build -t erraticmotion:hello .

# Displayes the runtime size.
docker images erraticmotion:hello

# Runs a container based on the created image
docker run erraticmotion:hello

# Removes all containers.
docker rm $(docker ps -a -q)
```