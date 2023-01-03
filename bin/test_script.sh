#!/usr/bin/env zsh

echo -e "Test: \e[1;31mLIB [TEST(JUST_ONE_TIME[INITIALIZE_BTREE])]\e[0m"
cargo test --color always --lib -- --nocapture --ignored --exact func::tests::test_just_one_time

echo -e "Test: \e[1;31mLIB [TEST(UNLOG)]\e[0m"
cargo test --color always --lib -- --nocapture

echo -e "Test: \e[1;31mLIB [TEST(LOG)]\e[0m"
cargo test --color always --features logging --lib -- --nocapture
