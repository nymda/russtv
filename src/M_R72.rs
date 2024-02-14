use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::SSTV::ModulatorInfo;

pub(crate) struct R72;
impl SSTV::Modulator for R72 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage){
        SSTV::generateVis(generator, self.Info().VIS);

        let proc: RgbImage = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest).to_rgb8();

        let mspp_Y: f32 = 138.0f32 / proc.width() as f32;
        let mspp_UV: f32 = 69.0f32 / proc.width() as f32;

        let hSyncMs: f32 = 9.0f32;
        let syncPorchMs: f32 = 3.0f32;
        let separatorMs: f32 = 4.5f32;
        let separatorPorchMs: f32 = 1.5f32;

        for y in 0..proc.height() {

            generator.tone(1200u16, hSyncMs);
            generator.tone(1500u16, syncPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), mspp_Y);
            }

            generator.tone(1500u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::V(px[0], px[1], px[2]) as u16), mspp_UV);
            }

            generator.tone(2300u16, separatorMs);
            generator.tone(1900u16, separatorPorchMs);

            for x in 0..proc.width(){
                let px = proc.get_pixel(x,y);
                generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::U(px[0], px[1], px[2]) as u16), mspp_UV);
            }
        }
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "Robot 72",
            SName: "Robot72",
            ResX: 320,
            ResY: 240,
            VIS: 0x0C
        };
    }
}