
pub use crate::soxr::Soxr;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use libsoxr::Soxr;

fn MakeWavFromCurve(source_to_be: &[i32]) -> Soxr {
  let soxr = Soxr::create(1.0, 2.0, 1, None, None, None).unwrap();
  let source = source_to_be;
  soxr.process(Some(&source), &mut source_to_be).unwrap();
  soxr.process::<f32,_>(None, &mut source_to_be[0..]).unwrap();
  return soxr;
}
/*
const SAMPLE_RATE: u16 = 44100;
pub fn Audio(pixels: Vec<u8>)/* -> std::io::Result<()> */{
    static mut IN_FILE: *mut sox_format_t = ptr::null_mut();
    static mut OUT_FILE: *mut sox_format_t = ptr::null_mut();
    let mut file = File::create("currentsound.wav").unwrap();
    let mut inp_file = File::open(Path::new("data/sine.wav"))?;
    let (header, data) = wav::read(&mut inp_file)?;
    print!("{}", header);
}

*/
