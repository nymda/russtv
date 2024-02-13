use std::fs::File;
use std::mem::size_of_val;
use crate::{PI, WAV};
use std::io::{Result, Write};

#[repr(C)]
pub struct WavHeader {
    riff: [u8; 4],
    fileSize: i32,
    wave: [u8; 4],
    fmt: [u8; 4],
    headerSize: i32,
    format: i16,
    channels: i16,
    sampleRate: i32,
    sbc: i32,
    bc: i16,
    bps: i16,
    data: [u8; 4],
    dataSize: i32,
}

pub struct WavGenerator {
    header: WavHeader,
    wav: Vec<i16>,
    pub(crate) expectedMs: f64,
    pub(crate) actualMs: f64,
    pub(crate) addedSamples: i32,
    pub(crate) removedSamples: i32,
    pub(crate) totalSamples: i64,
    angle: f64,
    pub(crate) calls: i64
}

impl WavGenerator {
    pub fn new(sRate: i32) -> Self {
        WavGenerator{
            header: WavHeader {
                riff: ['R' as u8, 'I' as u8, 'F' as u8, 'F' as u8],
                fileSize: 44,
                wave: ['W' as u8, 'A' as u8, 'V' as u8, 'E' as u8],
                fmt: ['f' as u8, 'm' as u8, 't' as u8, 0x20],
                headerSize: 16,
                format: 1,
                channels: 1,
                sampleRate: sRate,
                sbc: 96000,
                bc: 2,
                bps: 16,
                data: ['d' as u8, 'a' as u8, 't' as u8, 'a' as u8],
                dataSize: 0,
            },
            wav: Vec::new(),
            expectedMs: 0f64,
            actualMs: 0f64,
            addedSamples: 0i32,
            removedSamples: 0i32,
            totalSamples: 0i64,
            angle: 0f64,
            calls: 0i64
        }
    }

    pub fn tone(&mut self, freq: u16, time: f32) {
        let mut sampleCount: i32 = ((self.header.sampleRate as f32) * (time / 1000f32)).round() as i32;
        let mut sampleLoop: i32 = 0;
        self.calls+=1;

        self.expectedMs += time as f64;
        self.actualMs += (sampleCount as f64 / self.header.sampleRate as f64) * 1000f64;
        let msPerSample: f32 = 1000f32 / (self.header.sampleRate as f32);

        loop {
            sampleLoop+=1; //balancing modifies iterator within loop, this isn't possible with a for loop in rust.
            self.wav.push((30000f64 * self.angle.sin()) as i16);
            self.totalSamples += 1;
            self.angle += (2f64 * PI as f64 * freq as f64) / (self.header.sampleRate as f64);

            let diff: f64 = self.actualMs - self.expectedMs;
            if diff > msPerSample as f64 {
                sampleCount -= 1;
                self.removedSamples += 1;
                self.actualMs -= msPerSample as f64;
            }
            if diff < -msPerSample as f64 {
                sampleCount += 1;
                self.addedSamples += 1;
                self.actualMs += msPerSample as f64;
            }

            if sampleLoop >= sampleCount {
                break;
            }
        }

        //println!("Total samples created: {}", self.dbg);
        self.header.dataSize += sampleCount * 2; //2: sizeof(u16)
        self.header.fileSize = self.header.dataSize + size_of_val(&self.header) as i32;
        while self.angle > 2f64 * PI as f64 { self.angle -= 2f64 * PI as f64; }
    }

    pub fn getData(&self) -> &Vec<i16> {
        return &self.wav;
    }

    pub fn getHeader(&self) -> &WavHeader {
        return &self.header;
    }

    pub fn save(&self, path: &str) -> std::io::Result<(usize)> {
        let mut file = File::create(path)?;

        let header_bytes = unsafe {
            std::slice::from_raw_parts(&self.header as *const WAV::WavHeader as *const u8, std::mem::size_of::<WAV::WavHeader>())
        };
        file.write_all(header_bytes)?;

        let data_bytes: &[u8] = unsafe {
            std::slice::from_raw_parts(self.wav.as_ptr() as *const u8, self.wav.len() * std::mem::size_of::<i16>())
        };
        file.write_all(data_bytes)?;

        Ok(header_bytes.len() + data_bytes.len())
    }
}