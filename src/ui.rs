use crate::*;

struct UiState {
    // btw these are representations of:
    // RGB
    levels: [u32; 3],
    // sets the refresh rate of the light
    frame_rate: u64,
}

impl UiState {
    /// show:
    /// used to display the state of each RGB element:
    /// with values of R -> 0..15, G -> 0..15, B -> 0..15
    /// used to also display the frame rate of the ui for calibration metrics and analysis
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
    /// default:
    /// function used to establish a default frame rate and RGB value per UiState
    /// upon creation this is the default impl for the UiState object creation
    ///
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// Ui struct to represent the user interface of the project:
/// knobs buttons and also the current UIState for later reference.
pub struct Ui {
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

/// Ui implementation from the struct 
impl Ui {
    /// constructor implementation for a freshly created/loaded ui
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// async function which is joined to the main ui async methods to modify and run the states of 
    /// the main application. run takes the state of the ui into context to allow for the appropriate dialing 
    /// and modifications to the RBG values to be made as desired from the user.
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
                self.state.show();
            }

            Timer::after_millis(50).await;
        }
    }
}
