#![allow(non_snake_case)] //fuck you

mod wav;
mod sstv;
mod BWX;
mod R36;
mod YUV;

use std::env;
use std::process::exit;
use std::io::{Result, Write};
use crate::wav::WavGenerator;
use image::io::Reader as ImageReader;

const PI: f32 = std::f32::consts::PI;
fn hardFail(err: &str){
    //this doesnt have throw()
    println!("{}", err);
    exit(-1);
}

fn main() {
    //strings or something
    let mut eMeth: String = String::from("");
    let mut iFile: String = String::from("");
    let mut oFile: String = String::from("");

    //argument handling
    let argc: usize = env::args().len();
    for i in 0..argc {
        let current = env::args().nth(i);
        let next = env::args().nth(i+1);
        if let Some(cstr) = current {
            match cstr.as_str() {
                ("-M")=>
                    if let Some(val) = next {
                        eMeth = val;
                    } else {
                        hardFail("Malformed arguments");
                    },

                ("-I")=>
                    if let Some(val) = next {
                        iFile = val;
                    } else {
                        hardFail("Malformed arguments");
                    },

                ("-O")=>
                    if let Some(val) = next {
                        oFile = val;
                    } else {
                        hardFail("Malformed arguments");
                    },

                _ => ()
            }
        }
    }

    let mut generator: WavGenerator = WavGenerator::new(8000);
    generator.tone(0u16, 500f32); //500ms header, not required but neat
    sstv::generateVox(&mut generator); //vox tone

    let img;
    if let Ok(reader) = ImageReader::open(iFile) {
        img = reader.decode().unwrap(); // This assumes you want to immediately decode the image
        //R36::encodeR36(&mut generator, img);
        BWX::encodeBW8(&mut generator, img);

        generator.tone(0u16, 500f32);
        generator.save(oFile.as_str()).expect("Wav saving failed, check path.");
        println!("Calls    : {}", generator.calls);
        println!("Samples  : {}", generator.totalSamples);
        println!("Expected : {}", generator.expectedMs);
        println!("Actual   : {}", generator.actualMs);
        println!("Added {}, Skipped {}", generator.addedSamples, generator.removedSamples);
    } else {
        hardFail("Image file not found");
    }
}
