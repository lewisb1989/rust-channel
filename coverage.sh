cargo llvm-cov --html --ignore-filename-regex="(_test.rs|main.rs|initializer.rs|strategy/random_trading.rs|types/binance.rs|types/bybit.rs|types/kraken.rs|types/config.rs|types/mod.rs|test_helpers.rs)" -- --test-threads=1
open target/llvm-cov/html/index.html
