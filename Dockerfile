FROM alpine:latest

WORKDIR /app
EXPOSE 8080

COPY /target/x86_64-unknown-linux-musl/release/webcrust .
COPY /app app

CMD ["./webcrust"]