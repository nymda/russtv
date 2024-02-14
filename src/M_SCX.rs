use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, WavGenerator};
use crate::SSTV::ModulatorInfo;

pub fn encodeSC(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: RgbImage = img.to_rgb8();
    let mspp: f32 = lineMS / proc.width() as f32;

    generator.tone(1200u16, 9.0f32);
    for y in 0..proc.height(){

        generator.tone(1500u16, 1.5f32);
        for x in 0..proc.width(){
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[1] as u16), mspp as f32);
        }

        generator.tone(1500u16, 1.5f32);
        for x in 0..proc.width(){
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[2] as u16), mspp as f32);
        }

        generator.tone(1200u16, 9.0f32);
        generator.tone(1500u16, 1.5f32);
        for x in 0..proc.width(){
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[0] as u16), mspp as f32);
        }
    }
}

pub(crate) struct SC1;
impl SSTV::Modulator for SC1 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeSC(generator, resize, 138.240f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Scottie 1",
            SName: "Scottie1",
            ResX: 320,
            ResY: 256,
            VIS: 0x3C
        };
    }
}

pub(crate) struct SC2;
impl SSTV::Modulator for SC2 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeSC(generator, resize, 88.064f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Scottie 2",
            SName: "Scottie2",
            ResX: 320,
            ResY: 256,
            VIS: 0xB8
        };
    }
}

pub(crate) struct SCDX;
impl SSTV::Modulator for crate::M_SCX::SCDX {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodeSC(generator, resize, 345.600f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Scottie DX",
            SName: "ScottieDX",
            ResX: 320,
            ResY: 256,
            VIS: 0xCC
        };
    }
}