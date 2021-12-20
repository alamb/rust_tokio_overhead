This is my attempt to quantify the runtime overhead of using tokio tasks for CPU bound work

TLDR is that it is at most 50ns, and likely much smaller (single nanosecond range); Let me know if you find anything else interesting

Using this command
```shell
cargo run --release
```

The performance of this program when run on a server class machine in GCP is noticeable more stable, and the overhead seems to be

```
Setting up a1...
Setting up a2...
Begin non async...
ran 10000000 runs in 4.016897171s
average time per run: 401ns
Begin async...
ran 10000000 runs in 4.127365172s
average time per run: 412ns
Begin async with multi-threads...
ran 10000000 runs in 4.491564919s
average time per run: 449ns
```

But when I reverse the order in which I run the tests, the deltas are reduced:


```
Setting up a1...
Setting up a2...
Begin async with multi-threads...
ran 10000000 runs in 4.389970424s
average time per run: 438ns
Begin async...
ran 10000000 runs in 4.406801101s
average time per run: 440ns
Begin non async...
ran 10000000 runs in 4.335061672s
average time per run: 433ns
```


Interestingly, on my laptop when you run the same command, the overhead is low enough that it is well within measurement noise


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
