use crate::wav::WavGenerator;

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