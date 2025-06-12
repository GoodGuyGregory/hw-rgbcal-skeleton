use crate::*;
// Analog to Digital Conversion Import
pub type Adc = saadc::Saadc<'static, 1>;

pub struct Knob(Adc);

/// Knob:
/// 
/// new() function to create a knob instance
/// 
/// measure() function to determine the current position state of the knob
/// to ensure the knob is calibrated correctly, ensuring there is a scaled buffer
/// for target resolution and a rounded result for the dial values 
impl Knob {
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
