use enum_map::{enum_map, Enum, EnumMap};

use anyhow::{anyhow, Result};
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Enum)]
enum DisplayedDigit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl DisplayedDigit {
    fn segments(&self) -> &'static [Segment] {
        match self {
            DisplayedDigit::One => &[Segment::C, Segment::F],
            DisplayedDigit::Four => &[Segment::B, Segment::C, Segment::D, Segment::F],
            DisplayedDigit::Seven => &[Segment::A, Segment::C, Segment::F],
            DisplayedDigit::Zero => &[
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            DisplayedDigit::Two => &[Segment::A, Segment::C, Segment::D, Segment::E, Segment::G],
            DisplayedDigit::Three => &[Segment::A, Segment::C, Segment::D, Segment::F, Segment::G],
            DisplayedDigit::Five => &[Segment::A, Segment::B, Segment::D, Segment::F, Segment::G],
            DisplayedDigit::Six => &[
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            DisplayedDigit::Eight => &[
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            DisplayedDigit::Nine => &[
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Enum)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
impl Segment {
    fn every() -> [Segment; 7] {
        [
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ]
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Enum)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
impl Wire {
    fn every() -> [Wire; 7] {
        [
            Wire::A,
            Wire::B,
            Wire::C,
            Wire::D,
            Wire::E,
            Wire::F,
            Wire::G,
        ]
    }
}

impl TryFrom<u8> for Wire {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            b'a' => Ok(Wire::A),
            b'b' => Ok(Wire::B),
            b'c' => Ok(Wire::C),
            b'd' => Ok(Wire::D),
            b'e' => Ok(Wire::E),
            b'f' => Ok(Wire::F),
            b'g' => Ok(Wire::G),
            _ => Err(anyhow!("Invalid wire identifier {}", value)),
        }
    }
}

struct SegmentMapping {
    mapping: EnumMap<Wire, SmallVec<[Segment; 7]>>,
}
impl SegmentMapping {
    fn new() -> Self {
        let mapping = enum_map! {
            Wire::A => SmallVec::from(Segment::every()),
            Wire::B => SmallVec::from(Segment::every()),
            Wire::C => SmallVec::from(Segment::every()),
            Wire::D => SmallVec::from(Segment::every()),
            Wire::E => SmallVec::from(Segment::every()),
            Wire::F => SmallVec::from(Segment::every()),
            Wire::G => SmallVec::from(Segment::every()),
        };
        SegmentMapping { mapping }
    }

    fn constrain(&mut self, wire: Wire, options: &[Segment]) {
        self.mapping[wire].retain(|segment| options.contains(segment));
    }
}

struct Problem {}
impl Problem {
    fn initial() -> Self {
        Self {}
    }

    fn analyze_signals(line: &str) -> Result<()> {
        let (wire_patterns, _message) = line
            .split_once(" | ")
            .ok_or_else(|| anyhow!("Line missing | character: {:?}", line))?;
        let mut _problem = Self::initial();
        let wire_patterns = wire_patterns
            .split(" ")
            .map(|wire_pattern| {
                wire_pattern
                    .as_bytes()
                    .iter()
                    .map(|&v| v.try_into())
                    .collect::<Result<Vec<Wire>>>()
            })
            .collect::<Result<Vec<Vec<Wire>>>>()?;
        let mut mapping = SegmentMapping::new();
        let one_pattern = wire_patterns
            .iter()
            .filter(|wire_pattern| wire_pattern.len() == 2)
            .next();
        let seven_pattern = wire_patterns.iter().filter(|w| w.len() == 3).next();
        let four_pattern = wire_patterns.iter().filter(|w| w.len() == 4).next();
        if let Some(one_pattern) = one_pattern {
            for &wire in one_pattern {
                mapping.constrain(wire, DisplayedDigit::One.segments());
            }
        }
        if let Some(seven_pattern) = seven_pattern {
            for &wire in seven_pattern {
                mapping.constrain(wire, DisplayedDigit::Seven.segments());
            }
            if let Some(one_pattern) = one_pattern {
                let a_wire = seven_pattern
                    .iter()
                    .filter(|w| !one_pattern.contains(w))
                    .next()
                    .unwrap();
                mapping.constrain(*a_wire, &[Segment::A]);
            }
        }
        if let Some(four_pattern) = four_pattern {
            for &wire in four_pattern {
                mapping.constrain(wire, DisplayedDigit::Four.segments());
            }
            if let Some(one_pattern) = one_pattern {
                let bd_wires = four_pattern
                    .iter()
                    .filter(|w| !one_pattern.contains(w))
                    .collect::<Vec<&Wire>>();
                for &&wire in bd_wires.iter() {
                    mapping.constrain(wire, &[Segment::B, Segment::D]);
                }
            }
        }

        Ok(())
    }
}

#[test]
fn test_part_1() {
    let easy =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    Problem::analyze_signals(easy).unwrap();
}
