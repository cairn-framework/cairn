# syntax=docker/dockerfile:1
#
# The guided-console prototype, packaged for the maintainer's own server.
#
# Build context is the repository root, because the image builds `cairn` from
# this checkout (so the prototype always tracks local cairn, never a published
# release) and serves `docs/design-system/` directly (so a token change lands
# in the console without an edit here).
#
#   docker compose -f studio/prototype/compose.yaml up -d --build

# ---- cairn, from this checkout ------------------------------------------
FROM rust:1-bookworm AS cairn
WORKDIR /src
COPY . .
RUN cargo install --path . --root /out --locked --bin cairn

# ---- the console ---------------------------------------------------------
FROM node:22-bookworm-slim AS console

# git and ripgrep are here for the harness, which reads and edits the target
# project with its own tools. curl and unzip are only for installing bun.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl git ripgrep unzip \
    && rm -rf /var/lib/apt/lists/*

# The harness. This is the maintainer's own, and the console's event reader
# expects its JSONL shape, so this argument exists to pin the version rather
# than to promise that another harness would work.
ARG HARNESS_PACKAGE="@oh-my-pi/pi-coding-agent"
# Deliberately not under /root: compose mounts a volume there for the harness's
# sign-in, which would otherwise shadow the install.
ENV BUN_INSTALL=/opt/bun
ENV PATH="/opt/bun/bin:${PATH}"
RUN curl -fsSL https://bun.sh/install | bash \
    && bun add -g "${HARNESS_PACKAGE}"

COPY --from=cairn /out/bin/cairn /usr/local/bin/cairn

WORKDIR /app
COPY studio/prototype/server.mjs ./server.mjs
COPY studio/prototype/lib ./lib
COPY studio/prototype/ui ./ui
COPY docs/design-system ./design-system

ENV CAIRN_PROTOTYPE_DESIGN_SYSTEM=/app/design-system
ENV CAIRN_PROTOTYPE_DATA=/data
ENV PORT=4400

EXPOSE 4400
CMD ["node", "server.mjs"]
