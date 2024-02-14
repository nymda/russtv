use image::{DynamicImage, GrayImage};
use image::imageops::FilterType;
use crate::{SSTV, WavGenerator};
use crate::SSTV::ModulatorInfo;

pub fn encodeBW(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: GrayImage = img.to_luma8();
    let mspp: f32 = lineMS / proc.width() as f32;
    for y in 0..proc.height(){
        generator.tone(1200u16, 6f32);
        generator.tone(1500u16, 2f32);
        for x in 0..proc.width(){
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[0] as u16), mspp as f32);
        }
    }
}

pub(crate) struct BW8;
impl SSTV::Modulator for BW8 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeBW(generator, resize, 58.89709f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Robot BW8",
            SName: "Robot8",
            ResX: 160,
            ResY: 120,
            VIS: 0x82
        };
    }
}

pub(crate) struct BW12;
impl SSTV::Modulator for BW12 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeBW(generator, resize, 92f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Robot BW12",
            SName: "Robot12",
            ResX: 160,
            ResY: 120,
            VIS: 0x86
        };
    }
}