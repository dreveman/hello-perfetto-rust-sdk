// Copyright (C) 2025 David Reveman.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(
    feature = "intrinsics",
    allow(internal_features),
    feature(core_intrinsics)
)]

use perfetto_sdk::{
    producer::*, scoped_track_event, track_event::*, track_event_categories, track_event_instant,
};
use perfetto_sdk_derive::tracefn;
use std::error::Error;

track_event_categories! {
    pub mod hello_world_te_ns {
        ( "cat1", "Test category 1", [ "tag1" ] ),
        ( "cat2", "Test category 2", [ "tag2", "tag3" ] ),
    }
}

use hello_world_te_ns as perfetto_te_ns;

#[derive(Debug)]
struct HelloData {
    field_int32: Option<i32>,
    field_string: Option<String>,
}

#[tracefn("cat2", flush = true)]
fn hello_function(struct_arg: &HelloData) {
    assert_eq!(
        struct_arg.field_int32,
        struct_arg
            .field_string
            .clone()
            .map(|v| v.parse::<i32>().unwrap())
    );
    std::thread::sleep(std::time::Duration::from_secs(1));
}

fn main() -> Result<(), Box<dyn Error>> {
    let producer_args = ProducerInitArgsBuilder::new().backends(Backends::SYSTEM);
    Producer::init(producer_args.build());
    TrackEvent::init();
    perfetto_te_ns::register()?;

    let mut counter: i32 = 1;
    loop {
        track_event_instant!("cat1", "instant_hello", |ctx: &mut EventContext| {
            ctx.add_debug_arg("from", TrackEventDebugArg::String("perfetto"));
            ctx.add_debug_arg("sdk", TrackEventDebugArg::String("rust"));
        });
        {
            scoped_track_event!("cat1", "scoped_hello", |ctx: &mut EventContext| {
                ctx.add_debug_arg("what", TrackEventDebugArg::String("sleep"));
                ctx.add_debug_arg("ms", TrackEventDebugArg::Int64(1000));
            });
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        hello_function(&HelloData {
            field_int32: Some(counter),
            field_string: Some(counter.to_string()),
        });
        std::thread::sleep(std::time::Duration::from_millis(1000));
        counter += 1;
    }
}
