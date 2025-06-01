# Stage 1: Build the application using the Rust image
FROM rust:1.83 AS builder
WORKDIR /usr/src/finance_scraper

# Copy the workspace manifest and lock files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Copy the entire source code (all crates)
COPY ./scraper ./scraper
COPY ./csv_processor ./csv_processor
COPY ./python ./python

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    python3-dev \
    libssl-dev \
    pkg-config \
    libcurl4-openssl-dev \
    libclang-dev \
    libpq-dev \
    libffi-dev \
    && rm -rf /var/lib/apt/lists/*

# Build the main binary crate
RUN cargo build --release -p scraper

# Stage 2: Final runtime image
FROM python:3.10-slim AS runtime
WORKDIR /usr/src/finance_scraper 

# Set environment variables
ENV GECKODRIVER_VERSION=0.35.0
ENV FIREFOX_VERSION=115.0
ENV PYTHONUNBUFFERED=1
ENV DISPLAY=:99
ENV PYTHONPATH=":/usr/local/lib/python3.10/site-packages"


# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    wget \
    curl \
    unzip \
    gnupg \
    ca-certificates \
    libx11-dev \
    libxrandr-dev \
    libxcomposite-dev \
    libxdamage-dev \
    libglu1-mesa \
    libfontconfig1 \
    libnss3 \
    libgdk-pixbuf2.0-0 \
    libgtk-3-0 \
    libasound2 \
    libappindicator3-1 \
    libdbus-1-3 \
    libnspr4 \
    libxss1 \
    libpython3.11 \
    firefox-esr \
    && rm -rf /var/lib/apt/lists/*

# Install GeckoDriver
RUN wget https://github.com/mozilla/geckodriver/releases/download/v${GECKODRIVER_VERSION}/geckodriver-v${GECKODRIVER_VERSION}-linux64.tar.gz \
    && tar -zxf geckodriver-v${GECKODRIVER_VERSION}-linux64.tar.gz -C /usr/local/bin \
    && rm geckodriver-v${GECKODRIVER_VERSION}-linux64.tar.gz \
    && chmod +x /usr/local/bin/geckodriver


# Install Python dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy the compiled binary and Python scripts from builder
COPY --from=builder /usr/src/finance_scraper/target/release/scraper /usr/local/bin/finance-scraper
COPY --from=builder /usr/src/finance_scraper/python /usr/src/finance_scraper/python

# Create and set permissions for site-packages
RUN mkdir -p /usr/local/lib/python3.10/site-packages && \
    chmod -R 755 /usr/local/lib/python3.10/site-packages


# Set the entrypoint
ENTRYPOINT ["finance-scraper"]
# CMD ["2023", "4"]  
