#!/bin/bash
cargo build-bpf
solana program deploy target/deploy/dispatch.so
