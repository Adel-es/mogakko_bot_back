FROM rust:latest

RUN apt-get update \
    && apt-get install -y postgresql-client \
    && rustup toolchain install nightly-2022-04-12 --force \
    && rustup default nightly-2022-04-12 \
    && cargo install diesel_cli --no-default-features --features 'postgres'

# Rewrite DATABASE_URL
ENV DATABASE_URL=postgres://[USER_NAME]:[PASSWORD]@[IP]:[PORT]/[DB_NAME] \
    ROCKET_ADDRESS=0.0.0.0 \
    DISCORD_BOT_TOKEN=[DICOSRD_BOT_TOKEN] \
    DISCORD_GUILD_ID=[GUILD_ID]

WORKDIR /mogakko_bot_back
EXPOSE 8000
CMD cargo run
