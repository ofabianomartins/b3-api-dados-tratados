#!/bin/bash

# Inicie o Puma
cargo run &

tail -f /dev/null
