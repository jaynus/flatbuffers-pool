# flatbuffers-pool

[![crate]](https://lib.rs/flatbuffers-pool)
[![docs]](https://docs.rs/flatbuffers-pool)

[crate]: https://img.shields.io/crates/v/flatbuffers-pool.svg
[docs]: https://docs.rs/flatbuffers-pool/badge.svg

RAII based FlatBufferBuilder pool.

## Examples

Global pool:

```rust
use flatbuffers_pool::FlatBufferBuilderPool;

let mut b = FlatBufferBuilderPool::get();
let name = b.create_string("something fun");
b.finish(name, None);
```

Local pool

```rust
use flatbuffers_pool::FlatBufferBuilderPool;

let mut pool = FlatBufferBuilderPool::new().build();
let mut b = pool.get();
let name = b.create_string("something fun");
b.finish(name, None);
```
