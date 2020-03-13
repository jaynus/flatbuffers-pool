//! flatbuffer builder pool benchmark
//!
//! # Examples
//!
//! ```sh
//! ```
#![feature(test)]
extern crate test;

use test::Bencher;

use flatbuffers::FlatBufferBuilder;
use flatbuffers_pool;
use parking_lot::Mutex;

mod pool;
use pool::{v1, v2, v3};

mod monster {
    #![allow(unused_imports)]
    include!(concat!(env!("OUT_DIR"), "/monster_generated.rs"));
}
use monster::my_game::sample::{Monster, MonsterArgs};

const INIT_POOL_SIZE: usize = 4_096;
const MAX_POOL_SIZE: usize = 8_192;
const BUFFER_CAPACITY: usize = 1_024;

#[bench]
fn pool_monster_stack(b: &mut Bencher) {
    b.iter(|| {
        let mut b = FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY);
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_mutex(b: &mut Bencher) {
    let builder = Mutex::new(FlatBufferBuilder::new_with_capacity(BUFFER_CAPACITY));
    b.iter(|| {
        let mut b = &mut *builder.lock();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global(b: &mut Bencher) {
    flatbuffers_pool::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    flatbuffers_pool::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    flatbuffers_pool::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = flatbuffers_pool::FlatBufferBuilderPool::get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v1(b: &mut Bencher) {
    v1::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v1::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v1::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v1::FlatBufferBuilderPool::get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v2(b: &mut Bencher) {
    v2::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v2::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v2::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v2::FlatBufferBuilderPool::get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_global_v3(b: &mut Bencher) {
    v3::FlatBufferBuilderPool::init_global_pool_size(INIT_POOL_SIZE);
    v3::FlatBufferBuilderPool::max_global_pool_size(MAX_POOL_SIZE);
    v3::FlatBufferBuilderPool::global_buffer_capacity(BUFFER_CAPACITY);
    b.iter(|| {
        let mut b = v3::FlatBufferBuilderPool::get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local(b: &mut Bencher) {
    let pool = flatbuffers_pool::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v1(b: &mut Bencher) {
    let pool = v1::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v2(b: &mut Bencher) {
    let pool = v2::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}

#[bench]
fn pool_monster_local_v3(b: &mut Bencher) {
    let pool = v3::FlatBufferBuilderPool::new()
        .init_pool_size(INIT_POOL_SIZE)
        .max_pool_size(MAX_POOL_SIZE)
        .buffer_capacity(BUFFER_CAPACITY)
        .build();
    b.iter(|| {
        let mut b = pool.get();
        let name = Some(b.create_string("monster"));
        let monster = Monster::create(&mut b, &MonsterArgs {
            name,
            ..MonsterArgs::default()
        });
        b.finish(monster, None);
    });
}
