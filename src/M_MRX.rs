use image::{DynamicImage, GrayImage, imageops::resize, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::M_R36::R36;
use crate::SSTV::ModulatorInfo;

pub fn encodeMR(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: RgbImage = img.to_rgb8();

    let mspp_Y: f32 = lineMS / proc.width() as f32;
    let mspp_UV: f32 = (lineMS / proc.width() as f32) / 2.0f32;

    let hSyncMs: f32 = 9.0f32;
    let syncPorchMs: f32 = 1.0f32;

    for y in 0..proc.height(){

        generator.tone(1200u16, hSyncMs);
        generator.tone(1500u16, syncPorchMs);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), mspp_Y);
        }
        let px = proc.get_pixel(proc.width(),y);
        generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), 0.1f32);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::V(px[0], px[1], px[2]) as u16), mspp_UV);
        }
        let px = proc.get_pixel(proc.width(),y);
        generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::V(px[0], px[1], px[2]) as u16), 0.1f32);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::U(px[0], px[1], px[2]) as u16), mspp_UV);
        }
        let px = proc.get_pixel(proc.width(),y);
        generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::U(px[0], px[1], px[2]) as u16), 0.1f32);
    }
}