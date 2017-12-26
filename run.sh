#!/usr/bin/env bash
while :
do
    cargo run
    echo Restarting in 15 seconds ...
    sleep 15
    echo Restarting now
done
