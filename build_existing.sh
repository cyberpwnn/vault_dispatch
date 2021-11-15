#!/bin/bash
cargo build-bpf
solana program deploy --program-id target/deploy/dispatch-keypair.json target/deploy/dispatch.so
