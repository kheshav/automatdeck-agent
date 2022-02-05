#!/bin/bash

if [[ "$#" -eq 0 ]];then
    echo "No flag is specified"
    exit 127
fi

while test $# -gt 0; do
    case "$1" in
        --arch)
            shift
            arch=$1
            shift
            ;;
        --version)
            shift
            version=$1
            shift
            ;;
        *)
           echo "$1 is not a recognized flag!"
           exit 1;
           ;;
    esac
done

echo "Copying files..."
mkdir -p /tmp/ad-agent/{modules,config,log}
cp -rv ./config/* /tmp/ad-agent/config/
cp -rv ./modules /tmp/ad-agent/
cp -v install.sh /tmp/ad-agent/

echo "Regenerating settings file.."
echo """[main]
url = \"\" # Automatdeck entrypoint url
check_interval = 300    # Check interval in seconds (default: 300)
email = \"\" # Email of the account
access_key = \"\"
secret_key = \"\"
log_dir = \"/etc/ad-agent/log\" # Log dir path
log_level = \"INFO\"  # Allowed values: INFO, WARN, DEBUG (default: DEBUG)
max_thread = 4 # Max allowed active thread (default: 4)

[security]
enable_encryption = false # Encrypt data (default: false) NOTE: Feature not implemented
key_path = \"\"

[modules]
python_path = \"/usr/bin/python\" # Path of python binary
module_dir = \"/etc/ad-agent/modules\"
enabled_modules = [\"example.py\"] # Double quoted String separated by comma
""" > /tmp/ad-agent/config/settings.toml

echo "Copying binary"
cp -v ./target/$arch/release/ad-agent /tmp/ad-agent/
chmod +x /tmp/ad-agent/ad-agent

echo "Generating tar file"
tar -czvf /tmp/ad-agent_${version}_${arch}.tar.gz -C /tmp ad-agent

echo "Cleaning temp dir"
rm -rf /tmp/ad-agent
