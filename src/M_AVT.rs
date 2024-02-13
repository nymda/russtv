use image::{DynamicImage, GrayImage, imageops::resize, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::SSTV::ModulatorInfo;

//AVT is great because there aren't any sync pulses, it just fucking yeets

pub(crate) struct AVT;
impl SSTV::Modulator for AVT {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage){
        SSTV::generateVis(generator, self.Info().VIS);
        let proc: RgbImage = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest).to_rgb8();
        let mspp: f32 = 125.0f32 / proc.width() as f32;

        for y in 0..proc.height(){
            for x in 0..proc.width(){
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[1] as u16), mspp);
            }

            for x in 0..proc.width(){
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[2] as u16), mspp);
            }

            for x in 0..proc.width(){
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * proc.get_pixel(x, y)[0] as u16), mspp);
            }
        }
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "ATV 90",
            ResX: 320,
            ResY: 240,
            VIS: 0x44
        };
    }
}