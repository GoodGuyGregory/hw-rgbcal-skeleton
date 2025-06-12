use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    /// 
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// step takes an LED to modfiy as RGB 
    /// this LED is then modified based on the time supplied for the refresh_rate and 
    /// displayed accordingly.
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// run takes nothing
    /// sets the tick_time field which is acquired from the other classes to 
    /// determine the length of time the light must be on according to our main ui context.
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            self.tick_time = Self::frame_tick_time(get_frame_rate().await);
            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
