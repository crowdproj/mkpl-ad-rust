FROM rust:alpine3.19 as builder
WORKDIR /app-build
COPY . .

# libc-dev
RUN apk add --no-cache openssl-dev musl-dev libretls-dev pkgconf
#RUN apk add --no-cache pkgconf openssl-dev musl-dev cmake make gcc g++ perl clang16 curl strace
ENV OPENSSL_DIR=/usr

RUN cargo install --path app-hyper
#RUN cargo build --release

CMD ["myapp"]

FROM alpine:3.19
RUN apk add --no-cache openssl libretls
COPY --from=builder /usr/local/cargo/bin/app-hyper /
#COPY ./target/release/app-hyper /app/
#WORKDIR /app
EXPOSE 8080/tcp
ENTRYPOINT ["/app-hyper"]
