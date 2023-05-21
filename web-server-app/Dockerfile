FROM rust:1.67 as builder
ADD ./ ./server-app
WORKDIR ./server-app
RUN cargo  build


FROM ubuntu
RUN apt update && apt install ca-certificates curl gnupg -y
ARG APP=/usr/src/app
COPY --from=builder ./server-app/target/debug/web-server-app ${APP}/web-server-app
WORKDIR ${APP}

EXPOSE 3000

CMD ["./web-server-app"]
