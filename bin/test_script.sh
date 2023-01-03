#!/usr/bin/env zsh

echo -e "Test: \e[1;31mLIB [TEST(UNLOG)]\e[0m"
cargo test --color always --lib -- --nocapture

echo -e "Test: \e[1;31mLIB [TEST(LOG)]\e[0m"
cargo test --color always --features logging --lib -- --nocapture
