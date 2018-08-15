#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate rawloader;
use rawloader::RawImage;

mod buffer;
use self::buffer::*;
mod hasher;
mod ops;
mod opbasics;
mod pipeline;
pub use self::pipeline::*;
pub use self::ops::*;

use std::sync::Arc;

fn simple_decode_full(img: &RawImage, maxwidth: usize, maxheight: usize, linear: bool) -> OpBuffer {
  let buf = {
    let mut pipeline = Pipeline::new(img, maxwidth, maxheight, linear);
    // FIXME: turn these into tests
    //
    // --- Check if serialization roundtrips
    // let serial = pipeline.to_serial();
    // println!("Settings are: {}", serial);
    // pipeline = Pipeline::new_from_serial(img, maxwidth, maxheight, linear, serial);
    //
    // --- Check that the pipeline caches buffers and does not recalculate
    // pipeline.run();
    pipeline.run()
  };

  // Since we've kept the pipeline to ourselves unwraping always works
  Arc::try_unwrap(buf).unwrap()
}

pub fn simple_decode(img: &RawImage, maxwidth: usize, maxheight: usize) -> OpBuffer {
  simple_decode_full(img, maxwidth, maxheight, false)
}

pub fn simple_decode_linear(img: &RawImage, maxwidth: usize, maxheight: usize) -> OpBuffer {
  simple_decode_full(img, maxwidth, maxheight, true)
}
