#![allow(non_snake_case)] //fuck you

mod WAV;
mod SSTV;
mod M_BWX;
mod M_R36;
mod YUV;
mod M_R24;
mod M_AVT;
mod M_MAX;
mod M_PDX;
mod M_SCX;
mod M_R72;

use std::env;
use std::process::exit;
use std::time::{Duration, Instant};
use image::DynamicImage;
use crate::WAV::WavGenerator;
use image::io::Reader as ImageReader;
use crate::SSTV::Modulator;

const PI: f32 = std::f32::consts::PI;

// ID  : Name       : Resolution
// 0   : Robot8     : 160x120
// 1   : Robot12    : 160x120
// 2   : Robot24    : 160x120
// 3   : Robot36    : 320x240
// 4   : Robot72    : 320x240
// 5   : Scottie1   : 320x256
// 6   : Scottie2   : 320x256
// 7   : ScottieDX  : 320x256
// 8   : Martin1    : 320x256
// 9   : Martin2    : 320x256
// 10  : ATV90      : 320x240
// 11  : PD50       : 320x256
// 12  : PD90       : 320x256
// 13  : PD120      : 640x496
// 14  : PD160      : 512x400
// 15  : PD180      : 640x496
// 16  : PD240      : 640x496
// 17  : PD290      : 800x616

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
                \n    -M Set modulator (Required, see -L)\
                \n    -I Set input file (Required)\
                \n    -O Set output file (Required)");
    exit(0);
}

fn showModulators(Modulators: &Vec<&dyn Modulator>){
    let mut I: i8 = 0;
    println!("{0: <3} : {1: <10} : {2: <10}", "ID", "Name", "Resolution");
    for M in Modulators {
        println!("{0: <3} : {1: <10} : {2: <3}x{3: <3}", I, M.Info().Aliases[1], M.Info().ResX, M.Info().ResY);
        I+=1;
    }
    exit(0);
}

fn getModulator(userInput: String, Modulators: &Vec<&dyn Modulator>) -> i32 {
    if let Ok(ok) = userInput.parse::<i32>(){
        if ok < (Modulators.len() - 1) as i32 {
            return ok;
        }
    }

    let mut i: i32 = 0;
    for m in Modulators {
        for a in m.Info().Aliases {
            if a.to_lowercase() == userInput.to_lowercase() {
                return i;
            }
        }
        i += 1;
    }

    return -1;
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
                "-h"=> {
                    showUsage();
                }

                "-l"=> {
                    showModulators(&modulators);
                }

                "-v"=> {
                    verbose = true;
                }

                "-m"=>
                    if let Some(val) = env::args().nth(i+1) {
                        moStr = val;
                    } else {
                        showUsage();
                    },

                "-i"=>
                    if let Some(val) = env::args().nth(i+1) {
                        iFile = val;
                    } else {
                        showUsage();
                    },

                "-o"=>
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
    let modIndex: i32 = getModulator(moStr, &modulators);
    if modIndex < 0 { hardFail("[E] Invalid modulator"); }

    let mut img:DynamicImage = Default::default();
    if let Ok(ok) = ImageReader::open(iFile){ img = ok.decode().unwrap() }
    else { hardFail("[E] Failed to open image file") }

    let now: Instant = Instant::now();
    generator.tone(0u16, 500f32); //500ms header, not required but neat
    SSTV::generateVox(&mut generator); //vox tone
    modulators[modIndex as usize].ModulateSSTV(&mut generator, img); //actual SSTV signal
    generator.tone(0u16, 500f32);  //500ms footer, also not required but neat
    let elapsed: Duration = now.elapsed();

    let mut writtenBytes: usize = 0;
    if let Ok(bytes) = generator.save(oFile.as_str()) { writtenBytes = bytes; }
    else { hardFail("[E] Failed to save WAV file") }

    println!("[I] Finished encoding ({} | {}ms)", modulators[modIndex as usize].Info().Aliases[0], elapsed.as_millis());
    if verbose {
        println!("[V] Samples     : {}", generator.totalSamples);
        println!("[V] Expected MS : {}", generator.expectedMs);
        println!("[V] Actual MS   : {}", generator.actualMs);
        println!("[V] Added {}, Skipped {}", generator.addedSamples, generator.removedSamples);
    }
    println!("[I] Wrote {} bytes to {}", writtenBytes, oFile);
}
