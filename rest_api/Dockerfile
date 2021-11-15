FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/rest_api /usr/local/app/
COPY config /usr/local/app/config
WORKDIR /usr/local/app/
ENV PORT=80


RUN ["chmod", "+x", "./rest_api"]
CMD ["./rest_api"]