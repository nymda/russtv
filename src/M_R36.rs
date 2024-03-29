use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::SSTV::{FColourToFreq, ModulatorInfo};

pub(crate) struct R36;
impl SSTV::Modulator for R36 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage){
        SSTV::generateVis(generator, self.Info().VIS);

        let proc: RgbImage = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest).to_rgb8();

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
                generator.tone(FColourToFreq(YUV::Y(px[0], px[1], px[2])), mspp_Y);
            }

            if y % 2 == 0 {
                generator.tone(1500u16, separatorMs);
                generator.tone(1900u16, separatorPorchMs);

                for x in 0..proc.width(){
                    let px1 = proc.get_pixel(x,y);
                    let px2 = proc.get_pixel(x,y+1);
                    let avg: u16 = (YUV::V(px1[0], px1[1], px1[2]) as u16 + YUV::V(px2[0], px2[1], px2[2]) as u16) / 2u16;
                    generator.tone(FColourToFreq(avg as u8), mspp_UV);
                }
            }
            else{
                generator.tone(2300u16, separatorMs);
                generator.tone(1900u16, separatorPorchMs);

                for x in 0..proc.width(){
                    let px1 = proc.get_pixel(x,y);
                    let px2 = proc.get_pixel(x,y-1);
                    let avg: u16 = (YUV::U(px1[0], px1[1], px1[2]) as u16 + YUV::U(px2[0], px2[1], px2[2]) as u16) / 2u16;
                    generator.tone(FColourToFreq(avg as u8), mspp_UV);
                }
            }
        }
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Aliases: vec!["Robot 36", "Robot36", "R36"],
            ResX: 320,
            ResY: 240,
            VIS: 0x88
        };
    }
}