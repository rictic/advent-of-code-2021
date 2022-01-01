use std::fmt::Debug;

use anyhow::{anyhow, Result};

struct BitstreamReader {
    nibbles: Vec<u8>,
    bits_remaining_in_head: u8,
    position: usize,
}

impl BitstreamReader {
    fn from_hex_str(s: &str) -> Self {
        Self {
            nibbles: s
                .trim()
                .chars()
                .rev()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .collect(),
            bits_remaining_in_head: 4,
            position: 0,
        }
    }

    fn next_bit(&mut self) -> Option<bool> {
        self.bits_remaining_in_head -= 1;
        self.position += 1;
        let nibble = self.nibbles.last_mut()?;
        let result = *nibble & 0b1000 == 0b1000;
        if self.bits_remaining_in_head == 0 {
            self.bits_remaining_in_head = 4;
            self.nibbles.pop();
        } else {
            *nibble <<= 1;
        }
        Some(result)
    }

    fn bits_into_u8(&mut self, mut n: u8) -> Option<u8> {
        if n > 8 {
            panic!("Can't read more than 8 bits into a u8, asked for {}", n);
        }
        let mut result = 0;
        while self.bits_remaining_in_head <= n {
            result <<= self.bits_remaining_in_head;
            n -= self.bits_remaining_in_head;
            let bits = (self.nibbles.pop()? & 0b1111) >> (4 - self.bits_remaining_in_head);
            result |= bits;
            self.position += self.bits_remaining_in_head as usize;
            self.bits_remaining_in_head = 4;
        }
        for _ in 0..n {
            result <<= 1;
            result |= self.next_bit()? as u8;
        }
        Some(result)
    }

    fn bits_into_u16(&mut self, mut n: u8) -> Option<u16> {
        if n > 16 {
            panic!("Can't read more than 16 bits into a u16, asked for {}", n);
        }
        let mut result = 0;
        if n > 8 {
            result |= self.bits_into_u8(8)? as u16;
            n -= 8;
        }
        for _ in 0..n {
            result <<= 1;
            result |= self.next_bit()? as u16;
        }
        Some(result)
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    content: PacketContent,
}
impl Packet {
    fn from_bitstream(reader: &mut BitstreamReader) -> Result<Self> {
        let version = reader.bits_into_u8(3).ok_or(anyhow!("No version"))?;
        let packet_type = reader.bits_into_u8(3).ok_or(anyhow!("No packet type"))?;
        let content = match packet_type {
            4 => PacketContent::LiteralValue(LiteralValuePacket::from_bitstream(reader)?),
            val => PacketContent::Operator(OperatorPacket::from_bitstream(
                OperatorType::from_u8(val).ok_or(anyhow!("Invalid operator type {}", val))?,
                reader,
            )?),
        };
        Ok(Self { version, content })
    }

    fn sum_versions(&self) -> u64 {
        let sum = self.version as u64;
        match &self.content {
            PacketContent::LiteralValue(_) => sum,
            PacketContent::Operator(OperatorPacket { children, .. }) => {
                sum + children.iter().map(|c| c.sum_versions()).sum::<u64>()
            }
        }
    }

    fn value(&self) -> u64 {
        self.content.value()
    }
}

#[derive(Debug)]
enum PacketContent {
    LiteralValue(LiteralValuePacket),
    Operator(OperatorPacket),
}
impl PacketContent {
    fn value(&self) -> u64 {
        match self {
            PacketContent::LiteralValue(p) => p.value,
            PacketContent::Operator(p) => p.value(),
        }
    }
}

#[derive(Debug)]
struct LiteralValuePacket {
    value: u64,
}
impl LiteralValuePacket {
    fn from_bitstream(reader: &mut BitstreamReader) -> Result<Self> {
        // read 5 bits at a time. if the high bit is 1, keep reading. only the bottom 4 bits are content
        let mut value = 0;
        loop {
            let high_bit = reader
                .next_bit()
                .ok_or(anyhow!("Premature end of literal value packet"))?;

            value <<= 4;
            value |= reader
                .bits_into_u8(4)
                .ok_or(anyhow!("Premature end of literal value packet"))?
                as u64;
            if !high_bit {
                break;
            }
        }
        Ok(Self { value })
    }
}

#[derive(Debug)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
impl OperatorType {
    fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Sum),
            1 => Some(Self::Product),
            2 => Some(Self::Minimum),
            3 => Some(Self::Maximum),
            5 => Some(Self::GreaterThan),
            6 => Some(Self::LessThan),
            7 => Some(Self::EqualTo),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct OperatorPacket {
    kind: OperatorType,
    children: Vec<Packet>,
}
impl OperatorPacket {
    fn from_bitstream(kind: OperatorType, reader: &mut BitstreamReader) -> Result<Self> {
        let length_type = reader
            .next_bit()
            .ok_or(anyhow!("Premature end of operator packet"))?;
        let children = if length_type {
            let num_packets = reader
                .bits_into_u16(11)
                .ok_or(anyhow!("Premature end of operator packet"))?;
            Self::read_n_packets(reader, num_packets)?
        } else {
            let num_bits = reader
                .bits_into_u16(15)
                .ok_or(anyhow!("Premature end of operator packet"))?;
            Self::read_n_bits_of_packets(reader, num_bits)?
        };
        Ok(Self { kind, children })
    }

    fn read_n_packets(reader: &mut BitstreamReader, num_packets: u16) -> Result<Vec<Packet>> {
        let mut children = Vec::new();
        for _ in 0..num_packets {
            children.push(Packet::from_bitstream(reader)?);
        }
        Ok(children)
    }

    fn read_n_bits_of_packets(reader: &mut BitstreamReader, num_bits: u16) -> Result<Vec<Packet>> {
        let goal = reader.position + num_bits as usize;
        let mut children = Vec::new();
        while reader.position < goal {
            children.push(Packet::from_bitstream(reader)?);
        }
        Ok(children)
    }

    fn value(&self) -> u64 {
        match self.kind {
            OperatorType::Sum => self.children.iter().map(|c| c.value()).sum::<u64>(),
            OperatorType::Product => self.children.iter().map(|c| c.value()).product::<u64>(),
            OperatorType::Minimum => self
                .children
                .iter()
                .map(|c| c.value())
                .min()
                .unwrap_or_else(|| panic!("Attempted minimum over empty list")),
            OperatorType::Maximum => self
                .children
                .iter()
                .map(|c| c.value())
                .max()
                .unwrap_or_else(|| panic!("Attempted maximum over empty list")),
            OperatorType::GreaterThan => {
                if self.children.len() != 2 {
                    panic!(
                        "Attempted greater than with {} children",
                        self.children.len()
                    );
                }
                let a = self.children[0].value();
                let b = self.children[1].value();
                if a > b {
                    1
                } else {
                    0
                }
            }
            OperatorType::LessThan => {
                if self.children.len() != 2 {
                    panic!("Attempted less than with {} children", self.children.len());
                }
                let a = self.children[0].value();
                let b = self.children[1].value();
                if a < b {
                    1
                } else {
                    0
                }
            }
            OperatorType::EqualTo => {
                if self.children.len() != 2 {
                    panic!("Attempted equal to with {} children", self.children.len());
                }
                let a = self.children[0].value();
                let b = self.children[1].value();
                if a == b {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut reader = BitstreamReader::from_hex_str(input);
    let packet = Packet::from_bitstream(&mut reader)?;
    Ok(packet.sum_versions())
}

#[test]
fn test_part_1() {
    let hex = "38006F45291200";
    let mut reader = BitstreamReader::from_hex_str(hex);
    let version = reader.bits_into_u8(3).unwrap();
    let packet_type = reader.bits_into_u8(3).unwrap();
    let length_type: bool = reader.next_bit().unwrap();
    assert_eq!(version, 1);
    assert_eq!(packet_type, 6);
    assert_eq!(length_type, false);

    assert_eq!(part_1("8A004A801A8002F478").unwrap(), 16);
    assert_eq!(part_1("620080001611562C8802118E34").unwrap(), 12);
    assert_eq!(part_1("C0015000016115A2E0802F182340").unwrap(), 23);
    assert_eq!(part_1("A0016C880162017C3686B18A3D4780").unwrap(), 31);
    assert_eq!(part_1(include_str!("day16.txt")).unwrap(), 951);
}

fn part_2(input: &str) -> Result<u64> {
    let mut reader = BitstreamReader::from_hex_str(input);
    let packet = Packet::from_bitstream(&mut reader)?;
    Ok(packet.value())
}

#[test]
fn test_part_2() {
    assert_eq!(part_2("C200B40A82").unwrap(), 3);
    assert_eq!(part_2("04005AC33890").unwrap(), 54);
    assert_eq!(part_2("880086C3E88112").unwrap(), 7);
    assert_eq!(part_2("CE00C43D881120").unwrap(), 9);
    assert_eq!(part_2("D8005AC2A8F0").unwrap(), 1);
    assert_eq!(part_2("F600BC2D8F").unwrap(), 0);
    assert_eq!(part_2("9C005AC2F8F0").unwrap(), 0);
    assert_eq!(part_2("9C0141080250320F1802104A08").unwrap(), 1);
    assert_eq!(part_2(include_str!("day16.txt")).unwrap(), 902_198_718_880);
}
