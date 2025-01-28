#!/bin/bash

trap "trap - SIGTERM && kill 0" SIGINT SIGTERM EXIT

expo_build() {
    cd frontend
    pnpm web:build
    cd ..
}

expo_start() {
    cd frontend
    pnpm start
    cd ..
}

cargo_run_debug_proxy() {
    cargo run -- --reverse-proxy
}

cargo_run_release() {
    cargo run --release
}

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo " -h, --help      Display this help message."
    echo " -d, --dev       Run in development mode."
    echo " -r, --release   Run in release mode."
}

handle_options() {
    while [ $# -gt 0 ]; do
        case $1 in
            -h | --help)
                usage
                exit 0
                ;;
            -d | --dev)
                cargo_run_debug_proxy &
                expo_start 
                ;;
            -r | --release)
                expo_build
                cargo_run_release
                ;;
            *)
                echo "Invalid option: $1" >&2
                usage
                exit 1
                ;;
        esac
        shift 1
    done
}
if [ "$#" -eq 0 ];
then
    usage
else
    handle_options "$@"
fi
