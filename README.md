# rgbcal: RGB LED calibration tool

Greg Witt


## What Was Done

In this project I wired up my breadboard to the required specifications to achieve a potentiometer configuration that configures **4 States** that will affect an **RGB LED** wired into our system. 

**The 4 States Required**

1. When No Buttons are Held *(A, or B)*: the potentiometer adjusts the `frame_rate` of the `UiState` ensuring the LED is set to a range of `10..160`

2. When A Button is Held *(Only A button)*: the potentiometer adjusts the `blue` level of the RGB LED to ensure that it sets a state of `0..15` for the value of Blue within the LED

3. When B Button is Held *(Only B button)*: the potentiometer adjusts the `green` level of the RGB LED to ensure that it sets a state of `0..15` for the value of Green within the LED.

4. When A Button and B Button are Held *(Both Buttons)*: the potentiometer adjusts the `red` level of the RGB LED to ensure that it sets a state of `0..15` for the value of RED within the LED

I followed the supplied wiring specifications as mentioned in the project to achieve a baseline light color which is `White` ish to start the project.

### Wiring For Project

![](./img/PHOTO.JPG) 

### Connections Made

**Bottom Left**

Connect the RGB LED to the MB2 on the bottom Left

[LED Wiring Diagram](https://docs.sunfounder.com/projects/sf-components/en/latest/component_rgb_led.html#:~:text=We%20use%20the%20common%20cathode%20one.&text=An%20RGB%20LED%20has%204,%2C%20GND%2C%20Green%20and%20Blue)


Red to P9 (GPIO1)
Green to P8 (GPIO2)
Blue to P16 (GPIO3)
Gnd to Gnd (From Both Sides of the Dragon Tail)


**Bottom Right**

Connect the potentiometer (knob) to the MB2 as follows: 

Pin 1 to Gnd
Pin 2 to P2
Pin 3 to +3.3V

### Code Comments 

I've documented the code to the best of my understanding for each of the four components used. I will provide a brief high level understand of each below in a few sentences.

### `main.rs`

The `main.rs` sets the ground work for our knob code to convert analog to digital signal sources and calibrates our knob to register interrupts with embassy when the modifications to the dial are made.

Apart from all of this the main connects our Mutex values which represent states of the running application and reflect our code changes to the `frame_rate` and the `rgb_levels` these levels are crucial to dialing in the white tone we are looking for. the threads are also joined for the `rbg` and the `ui` code to work in sync and share data accurately as we send interrupts to modify the code between the knob and the ui.

### `ui.rs`

the ui is extremely important to the fine configurations we will need in order to get the unique balances of `white` we desire. we leverage the `UiState` and the `Ui` structs here within this code. 

**Ui**

the `Ui` holds all of the board peripherals and is the main way users will interact with the board. the knob, buttons and state are all nested within the `Ui` struct and hold the modification attributes we will leverage and access to dial in the colors we desire. 

**UiState**

the `UiState`'s `show` method takes track of the main elements we care about holding the access to the `frame_rate` and the `levels` which are the current state of an array of 3 `u32` elements which represent the state of the `Red`, `Green` or `Blue` in terms of an integer within `0-15` this struct's `show` method is crucial for us. apart from the `knob` code which dials in all of the signal configurations.

### `rgb.rs`





### `knob.rs`

probably the most detailed, important but also simple struct throughout this code base. the `knob` is crucial for all measurements conducted by the `ui` from the user to calibrate their led to white with minute details the knob code calibrates each movement when being created with the constructor's `new` method and the crate magic of the `ADC` Analog to Digital Conversion which will take the position of the knob and translate the position into a digital signal that is measure able with the `measure method`


## Measurements:




## How Development Went




## Other Observations

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.
