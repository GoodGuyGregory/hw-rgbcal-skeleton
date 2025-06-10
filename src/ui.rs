use crate::*;

struct UiState {
    // btw these are r, g, and b
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    pub async fn run(&mut self) -> ! {
        self.state.levels[2] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            if self._button_a.is_low() && !self._button_b.is_low() {
                rprintln!("A button held modify");
                rprintln!("modify BLUE LED");
                let level = self.knob.measure().await;
                if level != self.state.levels[2] {
                    self.state.levels[2] = level;
                    self.state.show();
                    set_rgb_levels(|rgb| {
                        *rgb = self.state.levels;
                    })
                    .await;
                }
            }

            else if !self._button_a.is_low() && self._button_b.is_low() {
                rprintln!("B button held modify");
                rprintln!("modify GREEN LED");
                let level = self.knob.measure().await;
                if level != self.state.levels[1] {
                    self.state.levels[1] = level;
                    self.state.show();
                    set_rgb_levels(|rgb| {
                        *rgb = self.state.levels;
                    })
                    .await;
                }
            }
            // when both are held 
            else if self._button_a.is_low() && self._button_b.is_low() {
                rprintln!("A & B button held");
                rprintln!("modify RED LED");
                let level = self.knob.measure().await;
                if level != self.state.levels[0] {
                    self.state.levels[0] = level;
                    self.state.show();
                    set_rgb_levels(|rgb| {
                        *rgb = self.state.levels;
                    })
                    .await;
                }
            }

            else {
                rprintln!("adjust the frame-rate based on the position of the knob");
                let mut frame_rate = self.knob.measure().await * 10;
                if frame_rate == 0 {
                    frame_rate = 10;
                }
                set_frame_rate(u64::from(frame_rate)).await;
                self.state.frame_rate = u64::from(frame_rate);
            }

            Timer::after_millis(50).await;
        }
    }
}
