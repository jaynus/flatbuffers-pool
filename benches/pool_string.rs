//! flatbuffer builder pool benchmark
//!
//! # Examples
//!
//! ```sh
//! $ cargo -q bench --bench pool_string
//! running 8 tests
//! test pool_global    ... bench:          87 ns/iter (+/- 1)
//! test pool_global_v1 ... bench:          80 ns/iter (+/- 1)
//! test pool_global_v2 ... bench:         107 ns/iter (+/- 5)
//! test pool_local     ... bench:         122 ns/iter (+/- 0)
//! test pool_local_v1  ... bench:         115 ns/iter (+/- 4)
//! test pool_local_v2  ... bench:         132 ns/iter (+/- 2)
//! test pool_mutex     ... bench:          43 ns/iter (+/- 4)
//! test pool_stack     ... bench:          61 ns/iter (+/- 1)
//!
//! test result: ok. 0 passed; 0 failed; 0 ignored; 8 measured; 0 filtered out
//! ```
#![feature(test)]
extern crate test;

use test::Bencher;

use flatbuffers::FlatBufferBuilder;
use flatbuffers_pool;
use parking_lot::Mutex;

mod pool;
use pool::{v1, v2, v3};

const INIT_POOL_SIZE: usize = 4_096;
const MAX_POOL_SIZE: usize = 8_192;
const BUFFER_CAPACITY: usize = 64;

#[bench]
fn pool_stack(b: &mut Bencher) {
    b.iter(|| {
        let mut b = FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY);
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_mutex(b: &mut Bencher) {
    let builder = Mutex::new(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY));
    b.iter(|| {
        let b = &mut *builder.lock();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global(b: &mut Bencher) {
    flatbuffers_pool::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    flatbuffers_pool::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    flatbuffers_pool::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = flatbuffers_pool::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v1(b: &mut Bencher) {
    v1::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v1::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v1::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v1::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v2(b: &mut Bencher) {
    v2::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v2::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v2::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v2::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_global_v3(b: &mut Bencher) {
    v3::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v3::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v3::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v3::FlatBufferBuilderPool::get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local(b: &mut Bencher) {
    let pool = flatbuffers_pool::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v1(b: &mut Bencher) {
    let pool = v1::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v2(b: &mut Bencher) {
    let pool = v2::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}

#[bench]
fn pool_local_v3(b: &mut Bencher) {
    let pool = v3::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let data = b.create_string("a");
        b.finish(data, None);
    });
}
