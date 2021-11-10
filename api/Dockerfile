FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/api /usr/local/app/
COPY config /usr/local/app/config
COPY proto /usr/local/app/proto
WORKDIR /usr/local/app/
ENV PORT=5341


RUN ["chmod", "+x", "./api"]
CMD ["./api"]