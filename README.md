This is my attempt to quantify the runtime overhead of using tokio tasks for CPU bound work


In my laptop when you run this progam

```shell
cargo run --release
```

Produces:
```
Setting up a1...
Setting up a2...
Begin non async...
ran 10000000 runs in 8.849184698s
average time per run: 884ns
Begin async...
ran 10000000 runs in 8.559310341s
average time per run: 855ns
Begin async with multi-threads...
ran 10000000 runs in 8.428401692s
average time per run: 842ns
```

The same thing on a server class gcp machine is:



My interpretation of this result is the overhead is low enough that it is well within measurement noise.
