name         := "slugify"
target       := "x86_64-unknown-linux-musl"
release      := join(parent_directory(justfile()), "target", target, "release", name)
install_dest := join(executable_directory(), name)


build:
    RUSTFLAGS='-C link-arg=-s' cargo build --release --target {{target}}

install: build
    cp -f {{release}} {{install_dest}}
