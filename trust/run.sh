#!/bin/bash

cargo b --release
retcode=$?
#sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/trust
./target/release/trust &
pid=$!
sleep 1
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" SIGINT SIGTERM
wait $pid

