#[derive(Debug, Eq, PartialEq)]
pub enum TrafficLightWrapper {
    G1R2(TrafficLight<Green, Red>),
    Y1R2(TrafficLight<Yellow, Red>),
    R1G2(TrafficLight<Red, Green>),
    R1Y2(TrafficLight<Red, Yellow>),
}

impl TrafficLightWrapper {
    pub fn step(self) -> Self {
        match self {
            TrafficLightWrapper::G1R2(val) => TrafficLightWrapper::Y1R2(val.into()),
            TrafficLightWrapper::Y1R2(val) => TrafficLightWrapper::R1G2(val.into()),
            TrafficLightWrapper::R1G2(val) => TrafficLightWrapper::R1Y2(val.into()),
            TrafficLightWrapper::R1Y2(val) => TrafficLightWrapper::G1R2(val.into()),
        }
    }

    pub fn new() -> Self {
        TrafficLightWrapper::G1R2(TrafficLight::new())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Green;

#[derive(Debug, Eq, PartialEq)]
pub struct Yellow;

#[derive(Debug, Eq, PartialEq)]
pub struct Red;

#[derive(Debug, Eq, PartialEq)]
pub struct TrafficLight<N, E> {
    ns_state: N,
    ew_state: E,
}

impl TrafficLight<Green, Red> {
    fn new() -> Self {
        Self {
            ns_state: Green,
            ew_state: Red,
        }
    }
}

impl From<TrafficLight<Green, Red>> for TrafficLight<Yellow, Red> {
    fn from(_val: TrafficLight<Green, Red>) -> TrafficLight<Yellow, Red> {
        TrafficLight {
            ns_state: Yellow,
            ew_state: Red,
        }
    }
}

impl From<TrafficLight<Yellow, Red>> for TrafficLight<Red, Green> {
    fn from(_val: TrafficLight<Yellow, Red>) -> TrafficLight<Red, Green> {
        TrafficLight {
            ns_state: Red,
            ew_state: Green,
        }
    }
}

impl From<TrafficLight<Red, Green>> for TrafficLight<Red, Yellow> {
    fn from(_val: TrafficLight<Red, Green>) -> TrafficLight<Red, Yellow> {
        TrafficLight {
            ns_state: Red,
            ew_state: Yellow,
        }
    }
}

impl From<TrafficLight<Red, Yellow>> for TrafficLight<Green, Red> {
    fn from(_val: TrafficLight<Red, Yellow>) -> TrafficLight<Green, Red> {
        TrafficLight {
            ns_state: Green,
            ew_state: Red,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_g1r2() {
        let traffic_light = TrafficLightWrapper::new();
        assert_eq!(
            traffic_light,
            TrafficLightWrapper::G1R2(TrafficLight {
                ns_state: Green,
                ew_state: Red
            })
        )
    }

    #[test]
    fn g1r2_steps_to_y1r2() {
        let mut traffic_light = TrafficLightWrapper::new();
        traffic_light = traffic_light.step();
        assert_eq!(
            traffic_light,
            TrafficLightWrapper::Y1R2(TrafficLight {
                ns_state: Yellow,
                ew_state: Red
            })
        )
    }

    #[test]
    fn y1r2_steps_to_r1g2() {
        let mut traffic_light = TrafficLightWrapper::Y1R2(TrafficLight {
            ns_state: Yellow,
            ew_state: Red,
        });
        traffic_light = traffic_light.step();
        assert_eq!(
            traffic_light,
            TrafficLightWrapper::R1G2(TrafficLight {
                ns_state: Red,
                ew_state: Green
            })
        )
    }

    #[test]
    fn r1g2_steps_to_r1y2() {
        let mut traffic_light = TrafficLightWrapper::R1G2(TrafficLight {
            ns_state: Red,
            ew_state: Green,
        });
        traffic_light = traffic_light.step();
        assert_eq!(
            traffic_light,
            TrafficLightWrapper::R1Y2(TrafficLight {
                ns_state: Red,
                ew_state: Yellow
            })
        )
    }

    #[test]
    fn r1y2_steps_to_g1r2() {
        let mut traffic_light = TrafficLightWrapper::R1Y2(TrafficLight {
            ns_state: Red,
            ew_state: Yellow,
        });
        traffic_light = traffic_light.step();
        assert_eq!(
            traffic_light,
            TrafficLightWrapper::G1R2(TrafficLight {
                ns_state: Green,
                ew_state: Red
            })
        )
    }
}
