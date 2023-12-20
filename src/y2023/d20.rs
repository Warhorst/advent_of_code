use std::collections::HashMap;
use crate::y2023::d20::ModuleType::*;
use crate::y2023::d20::Pulse::*;

pub fn solve_20a(input: &str) -> usize {
    let mut configuration = Configuration::from(input);
    let (low, high) = configuration.push_button_times(1000);
    low * high
}

/// TODO naive and slow, improve!
pub fn solve_20b(input: &str) -> usize {
    let mut configuration = Configuration::from(input);
    configuration.count_times_to_reach_rx()
}

#[derive(Debug)]
struct Configuration {
    modules: HashMap<String, Module>,
}

impl Configuration {
    fn push_button_times(&mut self, times: usize) -> (usize, usize) {
        let mut lows = 0;
        let mut highs = 0;

        for _ in 0..times {
            self.push_button(&mut lows, &mut highs);
        }

        (lows, highs)
    }

    fn push_button(&mut self, lows: &mut usize, highs: &mut usize) {
        let mut current_pulses = self.send_pulse("button".to_string(), Low, "broadcaster".to_string());
        *lows += 1;

        while !current_pulses.is_empty() {
            current_pulses = current_pulses
                .into_iter()
                .inspect(|(_, pulse, _)| match pulse {
                    Low => *lows += 1,
                    High => *highs += 1
                })
                .flat_map(|(sender, pulse, destination)| self.send_pulse(sender, pulse, destination))
                .collect();
        }
    }

    fn count_times_to_reach_rx(&mut self) -> usize {
        let mut count = 1;

        while !self.rx_received_low_after_press() {
            count += 1
        }

        count
    }

    fn rx_received_low_after_press(&mut self) -> bool {
        let mut current_pulses = self.send_pulse("button".to_string(), Low, "broadcaster".to_string());
        let mut rx_received_low = false;

        while !current_pulses.is_empty() {
            current_pulses = current_pulses
                .into_iter()
                .inspect(|(_, pulse, destination)| {
                    if destination == "rx" && *pulse == Low {
                        rx_received_low = true
                    } else if destination == "rx" && *pulse == High {
                        rx_received_low = false
                    }
                })
                .flat_map(|(sender, pulse, destination)| self.send_pulse(sender, pulse, destination))
                .collect();
        }

        rx_received_low
    }

    fn send_pulse(
        &mut self,
        sender: String,
        input_pulse: Pulse,
        destination: String,
    ) -> Vec<(String, Pulse, String)> { // sender, pulse, destination
        // println!("{sender} -> {:?} -> {destination}", input_pulse);

        let module = match self.modules.get_mut(&destination) {
            Some(m) => m,
            None => return vec![]
        };

        match &mut module.module_type {
            Broadcast => module.destinations
                .iter()
                .map(|dest| (destination.clone(), input_pulse, dest.clone()))
                .collect(),
            FlipFlop { ref mut is_on } => match input_pulse {
                Low => {
                    *is_on = !*is_on;
                    module.destinations
                        .iter()
                        .map(|dest| match is_on {
                            true => (destination.clone(), High, dest.clone()),
                            false => (destination.clone(), Low, dest.clone()),
                        })
                        .collect()
                }
                High => vec![] // nothing happens
            }
            Conjunction { ref mut last_pulses } => {
                last_pulses.insert(sender, input_pulse);
                let all_high = last_pulses.values().all(|val| *val == High);

                module.destinations
                    .iter()
                    .map(|dest| match all_high {
                        true => (destination.clone(), Low, dest.clone()),
                        false => (destination.clone(), High, dest.clone()),
                    })
                    .collect()
            }
        }
    }
}

impl From<&str> for Configuration {
    fn from(value: &str) -> Self {
        let names_to_destinations = value
            .lines()
            .map(|line| {
                let mut split = line.split("->");
                let name = split.next().unwrap().trim();
                let destinations = split.next().unwrap().split(",").map(|s| s.trim().to_string()).collect::<Vec<_>>();

                (name, destinations)
            })
            .collect::<HashMap<_, _>>();

        let modules = names_to_destinations
            .iter()
            .map(|(name, destinations)| {
                let clean_name = |n: &str| n.replace("%", "").replace("&", "").to_string();

                let module_type = if name.contains("broadcaster") {
                    Broadcast
                } else if name.contains("%") {
                    FlipFlop { is_on: false }
                } else {
                    Conjunction {
                        last_pulses: names_to_destinations
                            .iter()
                            .filter_map(|(n, dests)| match dests.contains(&clean_name(name)) {
                                true => Some(clean_name(n)),
                                false => None
                            })
                            .map(|n| (n.to_string(), Low))
                            .collect()
                    }
                };

                (clean_name(name), Module {
                    module_type,
                    destinations: destinations.clone(),
                })
            })
            .collect();

        Configuration { modules }
    }
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>,
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcast,
    FlipFlop {
        is_on: bool
    },
    Conjunction {
        last_pulses: HashMap<String, Pulse>
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}