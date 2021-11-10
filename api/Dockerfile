FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/api /usr/local/app/server/
COPY config /usr/local/app/server/
COPY proto /usr/local/app/server/
WORKDIR /usr/local/app/server
ENV PORT=5341

CMD ["./api"]