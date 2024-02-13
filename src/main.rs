#![allow(non_snake_case)] //fuck you

mod WAV;
mod SSTV;
mod M_BWX;
mod M_R36;
mod YUV;
mod M_R24;
mod M_AVT;
mod M_MAX;
mod M_MRX;
mod M_PDX;
mod M_SCX;
mod M_R72;

use std::env;
use std::process::exit;
use std::io::{Result, Write};
use crate::WAV::WavGenerator;
use image::io::Reader as ImageReader;
use crate::SSTV::Modulator;

const PI: f32 = std::f32::consts::PI;

// ID  : Name       : Resolution
// 0   : Robot BW8  : [160, 120]
// 1   : Robot BW12 : [160, 120]
// 2   : Robot 36   : [320, 240]
// 3   : Robot 24   : [160, 120]
// 4   : Robot 72   : [320, 240]
// 5   : Scottie 1  : [320, 250]
// 6   : Scottie 2  : [320, 250]
// 7   : Scottie DX : [320, 250]
// 8   : Martin 1   : [320, 256]
// 9   : Martin 2   : [320, 256]
// 10  : ATV 90     : [320, 240]
// 11  : PD 50      : [320, 256]
// 12  : PD 90      : [320, 256]
// 13  : PD 120     : [640, 496]
// 14  : PD 160     : [512, 400]
// 15  : PD 180     : [640, 496]
// 16  : PD 240     : [640, 496]
// 17  : PD 290     : [800, 616]

fn hardFail(err: &str){
    //this doesnt have throw()
    println!("{}", err);
    exit(-1);
}

fn showUsage(){
    println!("Russtv : Cross platform SSTV modulator\n");

    println!("Usage:\
                \n    russtv -H\
                \n    russtv -L\
                \n    russtv -M 0 -I image.png -O audio.wav\n");

    println!("Options:\
                \n    -H Show this screen\
                \n    -V Enable verbosity\
                \n    -L List available modulators\
                \n    -M Set modulator (Required, see -LM)\
                \n    -I Set input file (Required)\
                \n    -O Set output file (Required)");
    exit(0);
}

fn showModulators(Modulators: &Vec<&dyn Modulator>){
    let mut I: i8 = 0;
    println!("{0: <3} : {1: <10} : {2: <10}", "ID", "Name", "Resolution");
    for M in Modulators {
        println!("{0: <3} : {1: <10} : [{2: <3}, {3: <3}]", I, M.Info().Name, M.Info().ResX, M.Info().ResY);
        I+=1;
    }
    exit(0);
}

fn main() {
    //build all modulators for later use
    let modulators: Vec<&dyn Modulator> = SSTV::buildModulators();

    //strings or something
    let mut moStr: String = String::from("");
    let mut iFile: String = String::from("");
    let mut oFile: String = String::from("");
    let mut verbose: bool = false;

    //argument handling
    for i in 0..env::args().len() {
        if let Some(cstr) = env::args().nth(i) {
            match cstr.to_lowercase().as_str() {
                ("-h")=> {
                    showUsage();
                }

                ("-l")=> {
                    showModulators(&modulators);
                }

                ("-v")=> {
                    verbose = true;
                }

                ("-m")=>
                    if let Some(val) = env::args().nth(i+1) {
                        moStr = val;
                    } else {
                        showUsage();
                    },

                ("-i")=>
                    if let Some(val) = env::args().nth(i+1) {
                        iFile = val;
                    } else {
                        showUsage();
                    },

                ("-o")=>
                    if let Some(val) = env::args().nth(i+1) {
                        oFile = val;
                    } else {
                        showUsage();
                    },

                _ => ()
            }
        }
    }

    if iFile == "" || oFile == "" || moStr == "" { showUsage(); }

    let mut generator: WavGenerator = WavGenerator::new(8000);
    let mut modIndex = -1;

    if let Ok(ok) = moStr.parse::<i32>(){ modIndex = ok }
    else { hardFail("[E] Invalid modulator") }
    if modIndex > (modulators.len() - 1) as i32 {  hardFail("[E] Invalid modulator"); }

    let mut img = Default::default();
    if let Ok(x) = ImageReader::open(iFile){ img = x.decode().unwrap() }
    else { hardFail("[E] Failed to open image file") }

    generator.tone(0u16, 500f32); //500ms header, not required but neat
    SSTV::generateVox(&mut generator); //vox tone
    modulators[modIndex as usize].ModulateSSTV(&mut generator, img);
    generator.tone(0u16, 500f32);

    let mut writtenBytes = 0;
    if let Ok(bytes) = generator.save(oFile.as_str()) { writtenBytes = bytes; }
    else { hardFail("[E] Failed to save WAV file") }

    println!("[I] Finished encoding");
    if verbose {
        println!("[V] Modulator   : {}", modulators[modIndex as usize].Info().Name);
        println!("[V] Samples     : {}", generator.totalSamples);
        println!("[V] Expected MS : {}", generator.expectedMs);
        println!("[V] Actual MS   : {}", generator.actualMs);
        println!("[V] Added {}, Skipped {}", generator.addedSamples, generator.removedSamples);
    }
    println!("[I] Wrote {} bytes to {}", writtenBytes, oFile);
}
