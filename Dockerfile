FROM rust:slim AS build
ENV SQLX_OFFLINE=true
WORKDIR /work
COPY . .

# Build
RUN cargo build --release --locked --target x86_64-unknown-linux-gnu


FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=build /work/target/x86_64-unknown-linux-gnu/release/manganotif-api .
COPY --from=busybox:1.35-glibc /bin/sh /bin/busybox /bin/

EXPOSE 7878
ENTRYPOINT ["sh", "-c", "/app/manganotif-api >> /app/manganotif.log"]
