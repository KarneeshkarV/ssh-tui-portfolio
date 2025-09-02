#!/bin/bash

# SSH-TUI Installation Script
# This script is used by GitHub Actions to install/update the application

set -e

APP_DIR="/opt/ssh-tui"
BINARY_NAME="ssh-tui"
SERVICE_NAME="ssh-tui"

echo "Installing SSH-TUI..."

# Stop service if running
if systemctl is-active --quiet $SERVICE_NAME; then
    echo "Stopping $SERVICE_NAME service..."
    sudo systemctl stop $SERVICE_NAME
fi

# Backup existing binary
if [ -f "$APP_DIR/$BINARY_NAME" ]; then
    echo "Backing up existing binary..."
    sudo cp "$APP_DIR/$BINARY_NAME" "$APP_DIR/$BINARY_NAME.backup.$(date +%Y%m%d_%H%M%S)"
fi

# Set permissions
echo "Setting permissions..."
sudo chown sshtui:sshtui "$APP_DIR/$BINARY_NAME"
sudo chmod +x "$APP_DIR/$BINARY_NAME"

# Start and enable service
echo "Starting $SERVICE_NAME service..."
sudo systemctl start $SERVICE_NAME
sudo systemctl enable $SERVICE_NAME

# Check service status
echo "Checking service status..."
sudo systemctl status $SERVICE_NAME --no-pager

echo "Installation complete!"