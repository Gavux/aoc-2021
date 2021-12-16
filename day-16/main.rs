fn main() {
    part_a();
}

fn part_a() {
    let input = include_str!("input.txt");

    let bits = input
        .chars()
        .map(|x| format!("{:04b}", x.to_digit(16).unwrap()))
        .collect::<Vec<_>>()
        .join("");
    let packet = Packet::from(bits);

    println!("{}", sum_versions(&packet));
    println!("{}", evaluate(&packet));
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: Option<u64>,
    sub_packets: Option<Vec<Packet>>,
    length: usize,
}

impl From<String> for Packet {
    fn from(inp: String) -> Packet {
        let version = to_dec(&inp[0..3]) as u8;
        let type_id = to_dec(&inp[3..6]) as u8;
        let mut value: Option<u64> = None;
        let mut sub_packets: Option<Vec<Packet>> = None;
        let mut length = 0;

        if type_id == 4 {
            let mut literal = String::new();
            for i in 0..inp[6..].len() / 5 {
                let slice = &inp[7 + 5 * i..11 + 5 * i];
                literal += slice;

                if inp.chars().nth(6 + 5 * i).unwrap() == '0' {
                    length = 11 + 5 * i;
                    break;
                }
            }

            value = Some(to_dec(&literal) as u64);
        } else {
            let len_type_id = inp.chars().nth(6).unwrap().to_digit(2).unwrap();
            if len_type_id == 1 {
                let packet_count = to_dec(&inp[7..=17]) as usize;
                let mut packets: Vec<Packet> = Vec::new();
                while packets.len() != packet_count {
                    packets.push(Packet::from(
                        inp[18 + packets.iter().map(|x| x.length).sum::<usize>()..].to_string(),
                    ));

                    length = 18 + packets.iter().map(|x| x.length).sum::<usize>();
                }

                sub_packets = Some(packets);
            } else if len_type_id == 0 {
                let packet_len = to_dec(&inp[7..=21]) as usize;
                let mut packets: Vec<Packet> = Vec::new();
                while packets.iter().map(|x| x.length).sum::<usize>() != packet_len {
                    packets.push(Packet::from(
                        inp[22 + packets.iter().map(|x| x.length).sum::<usize>()..].to_string(),
                    ));

                    length = 22 + packets.iter().map(|x| x.length).sum::<usize>();
                }

                sub_packets = Some(packets);
            }
        }

        Packet {
            version,
            type_id,
            value,
            sub_packets,
            length,
        }
    }
}

fn to_dec(inp: &str) -> isize {
    isize::from_str_radix(inp, 2).unwrap()
}

fn sum_versions(packet: &Packet) -> u32 {
    let mut sum = packet.version as u32;

    if !packet.sub_packets.is_none() {
        sum += packet
            .sub_packets
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| sum_versions(x))
            .sum::<u32>();
    }

    sum
}

fn evaluate(packet: &Packet) -> u64 {
    if !packet.value.is_none() {
        return packet.value.unwrap();
    }

    let sub = packet.sub_packets.as_ref().unwrap();
    let map = sub.iter().map(|x| evaluate(x));

    match packet.type_id {
        0 => map.sum(),
        1 => map.product(),
        2 => map.min().unwrap(),
        3 => map.max().unwrap(),
        5 => {
            if evaluate(&sub[0]) > evaluate(&sub[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if evaluate(&sub[0]) < evaluate(&sub[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if evaluate(&sub[0]) == evaluate(&sub[1]) {
                1
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}