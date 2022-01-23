FROM rust:1.56.0 as builder
RUN USER=root cargo new --bin collaborative-draw 
WORKDIR ./collaborative-draw
RUN rm src/*.rs

ADD . ./
RUN rm ./target/release/collaborative-draw/deps/collaborative-draw*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

ENV TZ=Etc/UTC \
    APP_USER=app

RUN groupadd $APP_USER \
    && useradd $APP_USER $APP_USER \
    && mkdir -p $APP

COPY --from=builder /collaborative-draw/target/release/collaborative-draw $APP/collaborative-draw

USER $APP_USER
WORKDIR ${APP}

CMD ["./collaborative-draw"]