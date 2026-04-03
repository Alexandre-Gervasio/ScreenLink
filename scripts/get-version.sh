#!/bin/bash
# Helper script to get version from Cargo.toml

get_version() {
    grep '^version' core/Cargo.toml | head -1 | sed 's/version = "\([^"]*\)".*/\1/'
}

get_app_name() {
    grep '^name' core/Cargo.toml | head -1 | sed 's/name = "\([^"]*\)".*/\1/'
}

case "$1" in
    --version)
        get_version
        ;;
    --name)
        get_app_name
        ;;
    --full)
        echo "$(get_app_name) v$(get_version)"
        ;;
    *)
        echo "Usage: $0 [--version|--name|--full]"
        exit 1
        ;;
esac
