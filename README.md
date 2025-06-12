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

### `rgb.rs`

### `knob.rs`

## Measurements:




## How Development Went


## Other Observations

This tool is designed to find out a decent frame rate and
maximum RGB component values to produce a white-looking RGB
of reasonable brightness.

See below for UI.

**XXX This tool is *mostly* finished! Please wire your
hardware up (see below), finish it, comment it, and use it
to find good values. Then document those values in this
README.**

## Build and Run

Run with `cargo embed --release`. You'll need `cargo embed`, as
`cargo run` / `probe-rs run` does not reliably maintain a
connection for printing. See
https://github.com/probe-rs/probe-rs/issues/1235 for the
details.

## Wiring

Connect the RGB LED to the MB2 as follows:

* Red to P9 (GPIO1)
* Green to P8 (GPIO2)
* Blue to P16 (GPIO3)
* Gnd to Gnd

Connect the potentiometer (knob) to the MB2 as follows:

* Pin 1 to Gnd
* Pin 2 to P2
* Pin 3 to +3.3V

## UI

The knob controls the individual settings: frame rate and
color levels. Which parameter the knob controls should be
determined by which buttons are held. (Right now, the knob
jus always controls Blue. You should see the color change
from green to teal-blue as you turn the knob clockwise.)

* No buttons held: Change the frame rate in steps of 10
  frames per second from 10..160.
* A button held: Change the blue level from off to on over
  16 steps.
* B button held: Change the green level from off to on over
  16 steps.
* A+B buttons held: Change the red level from off to on over
  16 steps.

The "frame rate" (also known as the "refresh rate") is the
time to scan out all three colors. (See the scanout code.)
At 30 frames per second, every 1/30th of a second the LED
should scan out all three colors. If the frame rate is too
low, the LED will appear to "blink". If it is too high, it
will eat CPU for no reason.

I think the frame rate is probably set higher than it needs
to be right now: it can be tuned lower.

**LED Specifications**

[LED Wiring Diagram](https://docs.sunfounder.com/projects/sf-components/en/latest/component_rgb_led.html#:~:text=We%20use%20the%20common%20cathode%20one.&text=An%20RGB%20LED%20has%204,%2C%20GND%2C%20Green%20and%20Blue)
