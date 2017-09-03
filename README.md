```
λ cargo build -p a && ./target/debug/a
   Compiling foo v0.1.0 (file:///home/matklad/trash/workspace/foo)
   Compiling a v0.1.0 (file:///home/matklad/trash/workspace/a)
    Finished dev [unoptimized + debuginfo] target(s) in 0.38 secs
a
```

```
λ cargo build --all && ./target/debug/a
   Compiling foo v0.1.0 (file:///home/matklad/trash/workspace/foo)
   Compiling b v0.1.0 (file:///home/matklad/trash/workspace/b)
   Compiling a v0.1.0 (file:///home/matklad/trash/workspace/a)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42 secs
a
b
```