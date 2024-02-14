use image::{DynamicImage, RgbImage};
use image::imageops::FilterType;
use crate::{SSTV, YUV, WavGenerator};
use crate::SSTV::ModulatorInfo;

pub fn encodePD(generator: &mut WavGenerator, img: DynamicImage, lineMS: f32){
    let proc: RgbImage  = img.to_rgb8();
    let mspp: f32 = lineMS / proc.width() as f32;

    let hSyncMs: f32 = 20.0f32;
    let separatorMS: f32 = 2.08f32;

    for y in (0..proc.height()).step_by(2){
        generator.tone(1200u16, hSyncMs);
        generator.tone(1500u16, separatorMS);

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), mspp);
        }

        for x in 0..proc.width(){
            let px1 = proc.get_pixel(x,y);
            let px2 = proc.get_pixel(x,y+1);
            let avg = (YUV::V(px1[0], px1[1], px1[2]) as u16 + YUV::V(px2[0], px2[1], px2[2]) as u16) / 2u16;
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * avg), mspp);
        }

        for x in 0..proc.width(){
            let px1 = proc.get_pixel(x,y);
            let px2 = proc.get_pixel(x,y+1);
            let avg = (YUV::U(px1[0], px1[1], px1[2]) as u16 + YUV::U(px2[0], px2[1], px2[2]) as u16) / 2u16;
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * avg), mspp);
        }

        for x in 0..proc.width(){
            let px = proc.get_pixel(x,y+1);
            generator.tone(1500 + (SSTV::CFMULTIPLIER as u16 * YUV::Y(px[0], px[1], px[2]) as u16), mspp);
        }
    }
}

pub(crate) struct PD50;
impl SSTV::Modulator for PD50 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 91.520f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 50",
            SName: "PD50",
            ResX: 320,
            ResY: 256,
            VIS: 0xDD
        };
    }
}

pub(crate) struct PD90;
impl SSTV::Modulator for crate::M_PDX::PD90 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 170.240f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 90",
            SName: "PD90",
            ResX: 320,
            ResY: 256,
            VIS: 0x63
        };
    }
}

pub(crate) struct PD120;
impl SSTV::Modulator for crate::M_PDX::PD120 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 121.600f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 120",
            SName: "PD120",
            ResX: 640,
            ResY: 496,
            VIS: 0x5F
        };
    }
}

pub(crate) struct PD160;
impl SSTV::Modulator for crate::M_PDX::PD160 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 195.584f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 160",
            SName: "PD160",
            ResX: 512,
            ResY: 400,
            VIS: 0xE2
        };
    }
}

pub(crate) struct PD180;
impl SSTV::Modulator for crate::M_PDX::PD180 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 183.040f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 180",
            SName: "PD180",
            ResX: 640,
            ResY: 496,
            VIS: 0x60
        };
    }
}

pub(crate) struct PD240;
impl SSTV::Modulator for crate::M_PDX::PD240 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 244.480f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 240",
            SName: "PD240",
            ResX: 640,
            ResY: 496,
            VIS: 0xE1
        };
    }
}

pub(crate) struct PD290;
impl SSTV::Modulator for crate::M_PDX::PD290 {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage) {
        let resize = img.resize_exact(self.Info().ResX, self.Info().ResY, FilterType::Nearest);
        SSTV::generateVis(generator, self.Info().VIS);
        encodePD(generator, resize, 228.800f32);
    }
    fn Info(&self) -> ModulatorInfo {
        return ModulatorInfo{
            Name: "PD 290",
            SName: "PD290",
            ResX: 800,
            ResY: 616,
            VIS: 0xDE
        };
    }
}