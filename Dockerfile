FROM debian:bookworm-slim
COPY target/release/rinha /usr/local/bin/rinha
CMD [ "rinha" ]