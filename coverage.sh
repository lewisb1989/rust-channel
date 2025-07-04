cargo llvm-cov --html --ignore-filename-regex="(_test.rs|main.rs)" -- --test-threads=1
open target/llvm-cov/html/index.html
