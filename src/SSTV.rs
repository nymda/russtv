use image::DynamicImage;
use crate::{M_AVT, M_BWX, M_MAX, M_PDX, M_R24, M_R36, M_R72, M_SCX};
use crate::WAV::WavGenerator;

pub struct ModulatorInfo {
    pub(crate) Aliases: Vec<&'static str>,
    pub(crate) ResX: u32,
    pub(crate) ResY: u32,
    pub(crate) VIS: u8,
}

pub trait Modulator {
    fn ModulateSSTV(&self, generator: &mut WavGenerator, img: DynamicImage);
    fn Info(&self) -> ModulatorInfo;
}

pub fn buildModulators() -> Vec<&'static dyn Modulator> {
    return vec![&M_BWX::BW8,
                &M_BWX::BW12,
                &M_R24::R24,
                &M_R36::R36,
                &M_R72::R72,
                &M_SCX::SC1,
                &M_SCX::SC2,
                &M_SCX::SCDX,
                &M_MAX::MA1,
                &M_MAX::MA2,
                &M_AVT::AVT,
                &M_PDX::PD50,
                &M_PDX::PD90,
                &M_PDX::PD120,
                &M_PDX::PD160,
                &M_PDX::PD180,
                &M_PDX::PD240,
                &M_PDX::PD290];
}
pub fn generateVox(generator: &mut WavGenerator){
    generator.tone(1900u16, 100f32);
    generator.tone(1500u16, 100f32);
    generator.tone(1900u16, 100f32);
    generator.tone(1500u16, 100f32);
    generator.tone(2300u16, 100f32);
    generator.tone(1500u16, 100f32);
    generator.tone(2300u16, 100f32);
    generator.tone(1500u16, 100f32);
}

pub fn generateVis(generator: &mut WavGenerator, vis: u8){
    generator.tone(1900u16, 300f32);
    generator.tone(1200u16, 10f32);
    generator.tone(1900u16, 300f32);
    generator.tone(1200u16, 30f32);

    for i in 0..8 {
        let bit: u8 = (vis >> i) & 1;
        if bit > 0 {
            generator.tone(1100u16, 30f32);
        }
        else {
            generator.tone(1300u16, 30f32);
        }
    }

    generator.tone(1200u16, 30f32);
}

// pub fn IColourToFreq(c: u8) -> u16 {
//     return (1500 + (c as i32 * (2300 - 1500) / 256)) as u16;
// }

pub fn FColourToFreq(c: u8) -> u16 {
    return (1500f32 + (3.125f32 * c as f32)) as u16;
}