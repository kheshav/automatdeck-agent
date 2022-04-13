#!/bin/bash
set -e

MY_PATH=$(dirname "$0")
DESTINATION=/etc/ad-agent
mkdir -p $DESTINATION

echo "Installing ad-agent"
cp -rv $MY_PATH/* $DESTINATION

if [[ "$1" == "--configure-systemctl" ]];then
    echo "Configuring systemctl service"
    cat > /etc/systemd/system/ad-agent.service << EOF
[Unit]
Description=ad-agent
After=network.target

[Service]
WorkingDirectory=/etc/ad-agent
ExecStart=/etc/ad-agent/ad-agent launch
User=root
Group=root

[Install]
WantedBy=multi-user.target
EOF
    echo "Systemctl file /etc/systemd/system/ad-agent configured"
    systemctl daemon-reload
    echo "Systemctl daemon reloaded"
    systemctl enable ad-agent
    echo "Systemctl ad-agent enabled"
fi


# Cleanup
echo "Cleaning up"
rm /etc/ad-agent/install.sh
