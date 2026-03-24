# Default Argos Dockerfile

FROM rust:latest

# Install system dependencies for eframe (egui) and other requirements
RUN apt-get update && apt-get install -y \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libssl-dev \
    libfontconfig1-dev \
    libx11-dev \
    libxcb1-dev \
    pkg-config

# Set up a working directory
WORKDIR /app

# The actual command (cargo ...) is passed via docker run
# Argos handles mounting and permissions automatically.
