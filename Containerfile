FROM registry.access.redhat.com/ubi9/ubi:latest AS builder
RUN dnf install -y gcc gcc-c++ make openssl-devel
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup target add wasm32-unknown-unknown
RUN curl -L https://github.com/trunk-rs/trunk/releases/download/v0.20.1/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf - -C /usr/local/bin

WORKDIR /app
COPY shared-assets /app/shared-assets
COPY publish /app/publish
WORKDIR /app/publish

RUN trunk build --release
RUN cargo build --release

FROM registry.access.redhat.com/ubi9/ubi-minimal:latest
WORKDIR /app
COPY --from=builder /app/publish/target/release/publish /app/server
COPY --from=builder /app/publish/dist /app/dist
ENV BIND_ADDR="0.0.0.0:4405"
EXPOSE 4405
CMD ["/app/server"]
