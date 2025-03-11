#!/usr/bin/env bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Check if there are any arguments
if [ $# -eq 0 ]; then
    echo "No arguments provided."
    echo "Usage:"
    echo "  $0 --scan"
    echo "  $0 -d /dev/{name}"
    exit 1
fi

# Handle arguments
if [ "$1" == "--scan" ]; then
    # Display scan_onlyseagate.txt
    cat "${SCRIPT_DIR}/scan_onlyseagate.txt"

elif [ "$1" == "-d" ]; then
    if [ -z "$2" ]; then
        echo "No device provided after -d."
        exit 1
    fi

    # Extract the device name (strip off /dev/)
    DEVICE_NAME=$(basename "$2")

    # Compose the filename
    FILE="${SCRIPT_DIR}/${DEVICE_NAME}_smart_attributes.txt"

    # Check if the file exists
    if [ -f "$FILE" ]; then
        cat "$FILE"
    else
        echo "File not found: $FILE"
        exit 1
    fi

else
    echo "Invalid argument: $1"
    echo "Usage:"
    echo "  $0 --scan"
    echo "  $0 -d /dev/{name}"
    exit 1
fi
