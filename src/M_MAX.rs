use image::{DynamicImage, GrayImage, imageops::resize, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::M_R36::R36;
use crate::SSTV::ModulatorInfo;

pub fn encodeMA(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: RgbImage  = img.to_rgb8();
    let mspp: f32 = lineMS / proc.width() as f32;

    let hSyncMs: f32 = 4.862f32;
    let separatorMS: f32 = 0.572f32;

    for y in 0..proc.height(){
        generator.tone(1200u16, hSyncMs);
        generator.tone(1500u16, separatorMS);

        for x in 0..proc.width(){
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[1] as u16), mspp);
        }
        generator.tone(1500u16, separatorMS);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[2] as u16), mspp);
        }
        generator.tone(1500u16, separatorMS);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[0] as u16), mspp);
        }
        generator.tone(1500u16, separatorMS);
    }
}

pub(crate) struct MA1;
impl SSTV::Modulator for MA1 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeMA(generator, resize, 146.432f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Martin 1",
            ResX: 320,
            ResY: 256,
            VIS: 0xAC
        };
    }
}

pub(crate) struct MA2;
impl SSTV::Modulator for MA2 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeMA(generator, resize, 73.216f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Martin 2",
            ResX: 320,
            ResY: 256,
            VIS: 0x28
        };
    }
}