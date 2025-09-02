#!/bin/bash

# SSH-TUI VPS Setup Script
# Run this script on your Digital Ocean VPS to set up the environment

set -e

echo "Setting up SSH-TUI on Digital Ocean VPS..."

# Create application user if it doesn't exist
if ! id -u sshtui >/dev/null 2>&1; then
    echo "Creating sshtui user..."
    sudo useradd -r -s /bin/false -d /opt/ssh-tui sshtui
fi

# Create application directory
echo "Creating application directory..."
sudo mkdir -p /opt/ssh-tui
sudo chown sshtui:sshtui /opt/ssh-tui

# Create systemd service file
echo "Creating systemd service..."
sudo tee /etc/systemd/system/ssh-tui.service > /dev/null << 'EOF'
[Unit]
Description=SSH-TUI Portfolio Service
After=network.target

[Service]
Type=simple
User=sshtui
WorkingDirectory=/opt/ssh-tui
ExecStart=/opt/ssh-tui/ssh-tui
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/ssh-tui

[Install]
WantedBy=multi-user.target
EOF

# Reload systemd daemon
echo "Reloading systemd daemon..."
sudo systemctl daemon-reload

# Configure SSH to allow the application
echo "Setting up SSH configuration..."
sudo tee -a /etc/ssh/sshd_config > /dev/null << 'EOF'

# SSH-TUI Portfolio Configuration
Match User portfolio
    ForceCommand /opt/ssh-tui/ssh-tui
    PermitTTY yes
    X11Forwarding no
    AllowAgentForwarding no
    AllowTcpForwarding no
    PermitTunnel no
EOF

# Create portfolio user for SSH access
if ! id -u portfolio >/dev/null 2>&1; then
    echo "Creating portfolio user..."
    sudo useradd -r -m -s /bin/bash -d /home/portfolio portfolio
    sudo mkdir -p /home/portfolio/.ssh
    sudo chown portfolio:portfolio /home/portfolio/.ssh
    sudo chmod 700 /home/portfolio/.ssh
    
    echo "Add your public key to /home/portfolio/.ssh/authorized_keys"
    echo "sudo -u portfolio nano /home/portfolio/.ssh/authorized_keys"
    echo "Then run: sudo chmod 600 /home/portfolio/.ssh/authorized_keys"
fi

# Restart SSH service
echo "Restarting SSH service..."
sudo systemctl restart sshd

echo "Setup complete!"
echo ""
echo "Next steps:"
echo "1. Add your public SSH key to /home/portfolio/.ssh/authorized_keys"
echo "2. Deploy your binary to /opt/ssh-tui/ssh-tui"
echo "3. Start the service with: sudo systemctl start ssh-tui"
echo "4. Test SSH connection: ssh portfolio@your-server-ip"