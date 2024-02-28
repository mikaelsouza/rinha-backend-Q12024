FROM rust AS builder
WORKDIR /root/rinha/
ADD . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /root/rinha/target/release/rinha /usr/local/bin/rinha
CMD [ "rinha" ]