use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, WavGenerator};
use crate::SSTV::{FColourToFreq, ModulatorInfo};

//AVT is great because there aren't any sync pulses, it just fucking yeets

pub(crate) struct AVT;
impl SSTV::Modulator for AVT {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage){
        SSTV::generateVis(generator, self.Info().VIS);
        let proc: RgbImage = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest).to_rgb8();
        let mspp: f32 = 125.0f32 / proc.width() as f32;

        for y in 0..proc.height(){
            for x in 0..proc.width(){
                generator.tone(FColourToFreq(proc.get_pixel(x, y)[1]), mspp);
            }

            for x in 0..proc.width(){
                generator.tone(FColourToFreq(proc.get_pixel(x, y)[2]), mspp);
            }

            for x in 0..proc.width(){
                generator.tone(FColourToFreq(proc.get_pixel(x, y)[0]), mspp);
            }
        }
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Aliases: vec!["Amiga 90", "ATV90"],
            ResX: 320,
            ResY: 240,
            VIS: 0x44
        };
    }
}