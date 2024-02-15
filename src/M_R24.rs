use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::SSTV::{FColourToFreq, ModulatorInfo};

pub(crate) struct R24;
impl SSTV::Modulator for R24 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage){
        SSTV::generateVis(generator, self.Info().VIS);

        let proc: RgbImage = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest).to_rgb8();

        let mspp_Y: f32 = 92f32 / proc.width() as f32;
        let mspp_UV: f32 = 46f32 / proc.width() as f32;

        let hSyncMs: f32 = 6f32;
        let syncPorchMs: f32 = 2f32;
        let separatorMs: f32 = 3f32;
        let separatorPorchMs: f32 = 1f32;

        for y in 0..proc.height() {

            generator.tone(1200u16, hSyncMs);
            generator.tone(1500u16, syncPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(FColourToFreq(YUV::Y(px[0], px[1], px[2])), mspp_Y);
            }

            generator.tone(1500u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(FColourToFreq(YUV::V(px[0], px[1], px[2])), mspp_UV);
            }

            generator.tone(2900u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(FColourToFreq(YUV::U(px[0], px[1], px[2])), mspp_UV);
            }
        }
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Aliases: vec!["Robot 24", "Robot24", "R24"],
            ResX: 160,
            ResY: 120,
            VIS: 0x84
        };
    }
}