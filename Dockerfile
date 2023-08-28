# Use a Rust base image
FROM rust:slim-buster as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy over your manifest
COPY Cargo.toml Cargo.lock ./

# Cache dependencies - this step will only re-run if your manifest changes
RUN cargo fetch

# Copy your source tree
COPY src ./src

# Build for release
RUN cargo build --release

# Start a new build stage
FROM debian:12-slim

# Install Node.js, npm, and required dependencies for Puppeteer
RUN apt-get update && \
    apt-get install -y wget gnupg ca-certificates && \
    wget -qO - https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    apt-get install -y libx11-xcb1 libxcomposite1 libxdamage1 libxi6 libxext6 libxtst6 libnss3 libcups2 libxss1 libxrandr2 libasound2 libpangocairo-1.0-0 libatk1.0-0 libatk-bridge2.0-0 libgtk-3-0 && \
    apt-get clean && rm -rf /var/lib/apt/lists/* 

# Install sitetopdf globally
RUN npm install -g sitetopdf

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/page2doc /usr/local/bin/

# Set the command to run your application
CMD ["page2doc"]
