use std::str;
use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");
    let start = Instant::now();
    let binary = &to_binary_string(input.trim());
    let (packet, rest) = parse_input(binary);
    println!("packet {} rest {}", packet.version_sum(), &rest);
    println!("count: {}", packet.calculate());
    let duration = start.elapsed();
    println!("Time elapsed in task is: {:?}", duration);
}
enum PacketBody {
    Operator(Vec<Packet>), // not Literal
    Literal(usize),        // type id == 4
}

struct Packet {
    version: u8,
    packet_type: PacketBody,
    type_id: u8,
    len: usize,
}

impl Packet {
    fn calculate(&self) -> usize {
        match self.type_id {
            0 => self.children().unwrap().iter().map(|p| p.calculate()).sum(),
            1 => self
                .children()
                .unwrap()
                .iter()
                .fold(1, |acc, a| acc * a.calculate()),
            2 => self
                .children()
                .unwrap()
                .iter()
                .fold(usize::MAX, |acc, b| usize::min(acc, b.calculate())),
            3 => self
                .children()
                .unwrap()
                .iter()
                .fold(0, |acc, b| usize::max(acc, b.calculate())),
            4 => match &self.packet_type {
                PacketBody::Literal(nums) => *nums,
                _ => panic!("nope"),
            },
            5 => {
                let mut children = self.children().unwrap().iter();
                let (first, second) = (children.next().unwrap(), children.next().unwrap());
                if first.calculate() > second.calculate() {
                    1
                } else {
                    0
                }
            }
            6 => {
                let mut children = self.children().unwrap().iter();
                let (first, second) = (children.next().unwrap(), children.next().unwrap());
                if first.calculate() < second.calculate() {
                    1
                } else {
                    0
                }
            }
            7 => {
                let mut children = self.children().unwrap().iter();
                let (first, second) = (children.next().unwrap(), children.next().unwrap());
                if first.calculate() == second.calculate() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("nope"),
        }
    }

    fn children(&self) -> Option<&Vec<Packet>> {
        match self.packet_type {
            PacketBody::Operator(ref children) => Some(children),
            _ => None,
        }
    }

    fn version_sum(&self) -> usize {
        match &self.packet_type {
            PacketBody::Literal(_) => self.version as usize,
            PacketBody::Operator(packets) => {
                self.version as usize + packets.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }
}

fn to_binary_string(s: &str) -> String {
    s.bytes().fold(String::new(), |acc, byte| {
        acc + &format!(
            "{:04b}",
            u8::from_str_radix(str::from_utf8(&[byte]).unwrap(), 16).unwrap()
        )
    })
}
const INIT_SEGMENT: usize = 3;
const BINARY: u32 = 2;
const SEGMENT_LEN: usize = 5;

fn parse_input(s: &str) -> (Packet, &str) {
    let version = u8::from_str_radix(&s[..INIT_SEGMENT], BINARY).unwrap();
    let type_id = u8::from_str_radix(&s[INIT_SEGMENT..INIT_SEGMENT * 2], BINARY).unwrap();
    let start_index = 2 * INIT_SEGMENT;
    match type_id {
        4 => {
            let mut vec = String::new();
            let mut i = 0;
            loop {
                let curr_index = start_index + SEGMENT_LEN * i;
                vec.push_str(&s[curr_index + 1..curr_index + SEGMENT_LEN]);
                i += 1;
                if &s[curr_index..=curr_index] == "0" {
                    break;
                }
            }
            let len = INIT_SEGMENT * 2 + i * SEGMENT_LEN;
            (
                Packet {
                    version,
                    packet_type: PacketBody::Literal(
                        usize::from_str_radix(vec.as_str(), 2).unwrap(),
                    ),
                    len,
                    type_id,
                },
                &s[len..],
            )
        }
        type_id => {
            let length_type_id = &s[6..=6];
            match length_type_id {
                "0" => {
                    let end_of_len_bits = 7 + 15;
                    let bits = u32::from_str_radix(&s[7..end_of_len_bits], 2).unwrap();
                    let mut subpackets = vec![];
                    let mut chars_eaten = 0;
                    while chars_eaten < bits as usize {
                        let (packet, _) = parse_input(&s[end_of_len_bits + chars_eaten..]);
                        chars_eaten += packet.len;
                        subpackets.push(packet);
                    }
                    (
                        Packet {
                            version,
                            packet_type: PacketBody::Operator(subpackets),
                            len: end_of_len_bits + chars_eaten,
                            type_id,
                        },
                        &s[end_of_len_bits + chars_eaten..],
                    )
                }
                "1" => {
                    let end_of_len_bits = 7 + 11;
                    let subpackets_num = u32::from_str_radix(&s[7..end_of_len_bits], 2).unwrap();
                    let mut subpackets = vec![];
                    let mut chars_eaten = 0;
                    while subpackets.len() < subpackets_num as usize {
                        let next = parse_input(&s[end_of_len_bits + chars_eaten..]);
                        let (packet, _) = next;
                        chars_eaten += packet.len;
                        subpackets.push(packet);
                    }
                    (
                        Packet {
                            version,
                            packet_type: PacketBody::Operator(subpackets),
                            len: end_of_len_bits + chars_eaten,
                            type_id,
                        },
                        &s[end_of_len_bits + chars_eaten..],
                    )
                }
                _ => panic!("what 2"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_binary_str() {
        assert_eq!(&to_binary_string("1"), "0001");
        assert_eq!(&to_binary_string("F"), "1111");
        assert_eq!(
            &to_binary_string("38006F45291200"),
            "00111000000000000110111101000101001010010001001000000000"
        );
    }

    #[test]
    fn test_input() {
        let packet = &to_binary_string("D2FE28");
        let (packet, rest) = parse_input(packet);
        assert_eq!(rest, "");
        assert_eq!(packet.version_sum(), 6);
        assert_eq!(packet.len, "110100101111111000101000".len());
    }

    #[test]
    fn test_input_operator() {
        let packet = &to_binary_string("38006F45291200");
        let (packet, _) = parse_input(packet);
        assert_eq!(packet.version_sum(), 9);
    }

    #[test]
    fn test_input_operator_2() {
        let packet = &to_binary_string("EE00D40C823060");
        let (packet, _) = parse_input(packet);
        assert_eq!(packet.version_sum(), 14);
    }

    #[test]
    fn test_task2() {
        let packet = to_binary_string("C200B40A82");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 3);

        let packet = to_binary_string("04005AC33890");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 54);

        //product 6 * 9
        let packet = to_binary_string("880086C3E88112");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 7);

        // max 7 8 9
        let packet = to_binary_string("CE00C43D881120");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 9);

        // 5 < 15
        let packet = to_binary_string("D8005AC2A8F0");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 1);

        // 5 < 15
        let packet = to_binary_string("F600BC2D8F");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 0);

        // 5 != 15
        let packet = to_binary_string("9C005AC2F8F0");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 0);

        // 1 + 3 = 2 * 2
        let packet = to_binary_string("9C0141080250320F1802104A08");
        let (packet, _) = parse_input(&packet);
        assert_eq!(packet.calculate(), 1);
    }
}
