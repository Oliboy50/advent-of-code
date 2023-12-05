## How to run program for each day

> [!WARNING]
> Use --release because some days take too much time to run in debug mode.

> [!TIP]
> We can use "time" to monitor how many seconds the program takes to run.

```shell
# run the day03 exercise
time cargo run --release --features day03

# ...

# run the day22 exercise
time cargo run --release --features day22
```

## How to run tests for each day

```shell
# run the day03 exercise
cargo test --features day03

# ...

# run the day22 exercise
cargo test --features day22
```
