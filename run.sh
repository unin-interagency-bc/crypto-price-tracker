#!/usr/bin/env bash
cargo build

while :
do
    ./target/debug/crypto-price-tracker --user postgres --password postgres --schema public
    echo Restarting in 15 seconds ...
    sleep 15
    echo Restarting now
done
