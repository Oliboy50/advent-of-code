## How to run programs for a day

> [!WARNING]
> Use --release because some exercises take too much time to run in debug mode.

> [!TIP]
> We can use "time" to monitor how many seconds the program takes to run.

```shell
time cargo run --release --features day01
```

## How to run tests for a day

```shell
cargo test --features day01
```

## How to run benchmarks for a day

```shell
cargo bench --features day01
```
