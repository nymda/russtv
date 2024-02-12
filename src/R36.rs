use image::{DynamicImage, GrayImage, imageops::resize, RgbImage};
use image::imageops::FilterType;
use crate::{sstv, YUV};
use crate::wav::WavGenerator;

const CFMULTIPLIER: f32 = 3.1372549f32;

pub fn encodeR36(generator: &mut WavGenerator, img: DynamicImage){

    sstv::generateVis(generator, 0x88);

    let proc: RgbImage = img.resize(320, 240, FilterType::Nearest).to_rgb8();

    let mspp_Y: f32 = 88f32 / proc.width() as f32;
    let mspp_UV: f32 = 44f32 / proc.width() as f32;

    let hSyncMs: f32 = 9f32;
    let syncPorchMs: f32 = 3f32;
    let separatorMs: f32 = 4.5f32;
    let separatorPorchMs: f32 = 1.5f32;

    for y in 0..proc.height(){
        generator.tone(1200u16, hSyncMs);
        generator.tone(1500u16, syncPorchMs);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), mspp_Y);
        }

        if y % 2 == 0 {
            generator.tone(1500u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px1 = proc.get_pixel(x,y);
                let px2 = proc.get_pixel(x,y+1);
                let avg = (YUV::V(px1[0], px1[1], px1[2]) as u16 + YUV::V(px2[0], px2[1], px2[2]) as u16) / 2u16;
                generator.tone(1500 + (CFMULTIPLIER as u16 * avg), mspp_UV);
            }
        }
        else{
            generator.tone(2300u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px1 = proc.get_pixel(x,y);
                let px2 = proc.get_pixel(x,y-1);
                let avg = (YUV::U(px1[0], px1[1], px1[2]) as u16 + YUV::U(px2[0], px2[1], px2[2]) as u16) / 2u16;
                generator.tone(1500 + (CFMULTIPLIER as u16 * avg), mspp_UV);
            }
        }
    }
}

