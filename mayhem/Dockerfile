# ARGS
ARG project=parol

# Setup Fuzz-Rust Builder
FROM ghcr.io/nottirb/fuzz-rust:latest as builder
ADD . /${project}
WORKDIR /${project}

# Inject a lib.rs file, and build the fuzz targets
RUN mv mayhem/inject/lib_json.rs examples/json_parser/src/lib.rs && \
    cd ./fuzz && cargo +nightly hfuzz build

# Setup Fuzz-Env
FROM ubuntu:20.04
COPY --from=builder ${project}/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/parse_json /
