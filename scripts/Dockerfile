FROM rust:1.71-slim as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
        git \
        openssh-client

RUN rm -rf /var/lib/apt/lists/*

# Only works if you build with: docker build --ssh default
RUN mkdir -p /root/.ssh && chmod 0700 /root/.ssh
RUN ssh-keyscan github.com >> /root/.ssh/known_hosts

RUN git clone git@github.com:hbons/auroras.git .
RUN cargo build --release


FROM debian:bullseye-slim

WORKDIR /app
COPY --from=builder /app/target/release/auroras .

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

RUN useradd -u 1000 appuser
USER 1000

CMD ["./auroras --mirror --interval=60"]
