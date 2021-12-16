use crate::Day;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Result as IoResult;

enum PacketRead {
    Length(usize),
    Count(usize),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    typ: usize,
    data: PacketData,
}

#[derive(Debug, PartialEq)]
enum PacketData {
    SubPackets(Vec<Packet>),
    Literal(usize),
}

impl PacketData {
    fn subs(&self) -> &[Packet] {
        match self {
            Self::SubPackets(v) => &v,
            _ => panic!(),
        }
    }
    fn lit(&self) -> usize {
        match self {
            Self::Literal(v) => *v,
            _ => panic!(),
        }
    }
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version
            + match &self.data {
                PacketData::SubPackets(v) => v.iter().map(|sub| sub.version_sum()).sum(),
                _ => 0,
            }
    }

    fn value(&self) -> usize {
        match self.typ {
            0 => self.data.subs().iter().map(|s| s.value()).sum::<usize>(),
            1 => self
                .data
                .subs()
                .iter()
                .map(|s| s.value())
                .fold(1, |mut mul, v| {
                    mul *= v;
                    mul
                }),
            2 => self.data.subs().iter().map(|s| s.value()).min().unwrap(),
            3 => self.data.subs().iter().map(|s| s.value()).max().unwrap(),
            4 => self.data.lit(),
            5 => {
                if self.data.subs()[0].value() > self.data.subs()[1].value() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if self.data.subs()[0].value() < self.data.subs()[1].value() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if self.data.subs()[0].value() == self.data.subs()[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Unknown typ"),
        }
    }
}

fn parse_subpackets(mut bits: &[char], read: PacketRead) -> (&[char], Vec<Packet>) {
    let subs = match read {
        PacketRead::Length(_l) => {
            let mut subs = vec![];
            loop {
                let (new_bits, sub) = parse_packet(bits);
                bits = new_bits;
                if let Some(s) = sub {
                    subs.push(s);
                } else {
                    break;
                }
            }
            subs
        }
        PacketRead::Count(c) => (0..c)
            .map(|_| {
                let sub = parse_packet(bits);
                bits = sub.0;
                sub.1.unwrap()
            })
            .collect(),
    };

    (bits, subs)
}

fn parse_packet(mut bits: &[char]) -> (&[char], Option<Packet>) {
    if bits.is_empty() {
        return (bits, None);
    }
    let version = usize::from_str_radix(&bits[..3].iter().collect::<String>(), 2).unwrap();
    bits = &bits[3..];
    let typ = usize::from_str_radix(&bits[..3].iter().collect::<String>(), 2).unwrap();
    bits = &bits[3..];

    //println!("{:?} {:?}", version, typ);
    let data = match typ {
        4 => {
            let mut val: Vec<char> = vec![];
            loop {
                let c = bits[0];
                bits = &bits[1..];
                val.extend(&bits[..4]);
                bits = &bits[4..];
                if c == '0' {
                    break;
                }
            }
            PacketData::Literal(usize::from_str_radix(&val.iter().collect::<String>(), 2).unwrap())
        }
        _ => {
            /* operator packet */
            let length_type_id = bits[0];
            bits = &bits[1..];
            let sub = if length_type_id == '0' {
                let length =
                    usize::from_str_radix(&bits[..15].iter().collect::<String>(), 2).unwrap();
                bits = &bits[15..];
                let sub = parse_subpackets(&bits[..length], PacketRead::Length(length));
                bits = &bits[length..];
                sub.1
            } else {
                let subpackets =
                    usize::from_str_radix(&bits[..11].iter().collect::<String>(), 2).unwrap();
                bits = &bits[11..];
                let (new_bits, subs) = parse_subpackets(bits, PacketRead::Count(subpackets));
                bits = new_bits;
                subs
            };
            PacketData::SubPackets(sub)
        }
    };
    (bits, Some(Packet { version, data, typ }))
}

fn get_bits(v: &str) -> Vec<char> {
    v.chars()
        .flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect()
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 16;
    type Input = String;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input<R>(r: R) -> IoResult<Self::Input>
    where
        R: std::io::BufRead,
    {
        Ok(r.lines().next().unwrap().unwrap().to_string())
    }
    fn p1(v: &Self::Input) -> Self::Sol1 {
        let bits = get_bits(v);
        let p = parse_packet(&bits).1.unwrap();
        //println!("{:?}", p);
        p.version_sum()
    }
    fn p2(v: &Self::Input) -> Self::Sol2 {
        let bits = get_bits(v);
        let p = parse_packet(&bits).1.unwrap();
        //println!("{:?}", p);
        p.value()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let input = "8A004A801A8002F478";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        let bits = get_bits(&input);
        assert_eq!(
            parse_packet(&bits).1,
            Some(Packet {
                version: 4,
                typ: 2,
                data: PacketData::SubPackets(vec![Packet {
                    version: 1,
                    typ: 2,
                    data: PacketData::SubPackets(vec![Packet {
                        version: 5,
                        typ: 2,
                        data: PacketData::SubPackets(vec![Packet {
                            version: 6,
                            typ: 4,
                            data: PacketData::Literal(15)
                        }])
                    }])
                }])
            })
        );
        let input = "8A004A801A8002F478";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 16);

        let input = "620080001611562C8802118E34";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 12);

        let input = "C0015000016115A2E0802F182340";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 23);

        let input = "A0016C880162017C3686B18A3D4780";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p1(&input), 31);

        let input = "C200B40A82";
        let input = Solution::process_input(std::io::BufReader::new(input.as_bytes())).unwrap();
        assert_eq!(Solution::p2(&input), 3);
        //assert_eq!(Solution::p2(&input), 26984457539);
        //unimplemented!()
    }
}
