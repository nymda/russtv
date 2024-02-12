use image::{DynamicImage, GrayImage, imageops::resize};
use image::imageops::FilterType;
use crate::sstv;
use crate::wav::WavGenerator;

const CFMULTIPLIER: f32 = 3.1372549f32;

pub fn encodeBW(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: GrayImage = img.to_luma8();
    let mspp: f32 = lineMS / proc.width() as f32;
    for y in 0..proc.height(){
        generator.tone(1200u16, 6f32);
        generator.tone(1500u16, 2f32);
        for x in 0..proc.width(){
            generator.tone(1500 + (CFMULTIPLIER as u16 * proc.get_pixel(x, y)[0] as u16), mspp as f32);
        }
    }
}

pub fn encodeBW8(generator: &mut WavGenerator, img: DynamicImage){
    let resize = img.resize(160, 120, FilterType::Nearest);
    sstv::generateVis(generator, 0x82);
    encodeBW(generator, resize, 58.89709f32);
}

pub fn encodeBW12(generator: &mut WavGenerator, img: DynamicImage){
    let resize = img.resize(160, 120, FilterType::Nearest);
    sstv::generateVis(generator, 0x86);
    encodeBW(generator, resize, 92f32);
}