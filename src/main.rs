use std::{thread, time};
use rand::prelude::*;
use traffic::TrafficLightWrapper;

#[derive(Debug, Eq, PartialEq)]
struct Intersection {
    traffic_light: TrafficLightWrapper,
    button_pressed: bool,
}

impl Intersection {
    fn new() -> Self {
        Self {
            traffic_light: TrafficLightWrapper::new(),
            button_pressed: false,
        }
    }

    fn press_button(&mut self) {
        self.button_pressed = true
    }
}

fn main() {
    const NS_GREEN_DURATION: u32 = 60;
    const EW_GREEN_DURATION: u32 = 30;
    const YELLOW_DURATION: u32 = 5;

    let mut ns_green_time = NS_GREEN_DURATION;
    let mut ew_green_time = EW_GREEN_DURATION;
    let mut yellow_time = YELLOW_DURATION;

    let mut intersection = Intersection::new();
    loop {
        match &intersection.traffic_light {
            TrafficLightWrapper::G1R2(_) => {
                println!(
                    "Light state: {:?} | Timer: {:?} | Button pressed: {:?}",
                    &intersection.traffic_light, &ns_green_time, &intersection.button_pressed
                );
                
                // Just throwing in some random button presses to simulate the pushing of the button
                if !intersection.button_pressed {
                    let mut rng = rand::thread_rng();
                    let x: u8 = rng.gen();
                    if x <= 25 {  // press the button roughly 10% of the time
                        println!("Button pressed!");
                        intersection.press_button();
                    }
                }

                ns_green_time = ns_green_time - 1;
                if intersection.button_pressed && ns_green_time < 30 {
                    intersection.traffic_light = intersection.traffic_light.step();
                    intersection.button_pressed = false;
                    ns_green_time = NS_GREEN_DURATION;
                }

                if ns_green_time == 0 {
                    intersection.traffic_light = intersection.traffic_light.step();
                    ns_green_time = NS_GREEN_DURATION;
                }
            }
            TrafficLightWrapper::Y1R2(_) | TrafficLightWrapper::R1Y2(_) => {
                println!(
                    "Light state: {:?} | Timer: {:?} | Button pressed: {:?}",
                    &intersection.traffic_light, &yellow_time, &intersection.button_pressed
                );
                yellow_time = yellow_time - 1;
                if yellow_time == 0 {
                    intersection.traffic_light = intersection.traffic_light.step();
                    yellow_time = YELLOW_DURATION;
                }
            }
            TrafficLightWrapper::R1G2(_) => {
                println!(
                    "Light state: {:?} | Timer: {:?} | Button pressed: {:?}",
                    &intersection.traffic_light, &ew_green_time, &intersection.button_pressed
                );
                ew_green_time = ew_green_time - 1;
                if ew_green_time == 0 {
                    intersection.traffic_light = intersection.traffic_light.step();
                    ew_green_time = EW_GREEN_DURATION;
                }
            }
        }

        let one_sec = time::Duration::from_secs(1);
        thread::sleep(one_sec);
    }
}
