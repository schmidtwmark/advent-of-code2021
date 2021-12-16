use std::{env, fs};
use itertools::Itertools;

#[derive(Debug)]
enum PacketType{
    Sum(Vec<Packet>),
    Product(Vec<Packet>),
    Minimum(Vec<Packet>),
    Maximum(Vec<Packet>),
    Literal(usize),
    GreaterThan(Vec<Packet>),
    LessThan(Vec<Packet>),
    EqualTo(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}
#[derive(Debug)]
enum LengthType{
    Bits(usize),
    Subpackets(usize)
}

fn read_bytes_and_incremenent(bits: &mut &str, count: usize) -> usize{
    let out = usize::from_str_radix(&bits[..count], 2).unwrap();
    *bits = &bits[count..];
    out
}

fn read_subpackets(bits: &mut &str) -> Vec<Packet>{
    let length_type_id = read_bytes_and_incremenent(bits, 1);
    let length_type = match length_type_id{
        0 => LengthType::Bits(read_bytes_and_incremenent(bits, 15)),
        1 => LengthType::Subpackets(read_bytes_and_incremenent(bits, 11)),
        _ => unreachable!()
    };

    println!("Length Type: {:?}", length_type);
    match length_type {
        LengthType::Subpackets(subpackets) => {
            (0..subpackets).map(|i| {
                println!("Parsing subpacket #{}", i);
                let (packet, new_bits) = Packet::from_binary(bits);
                *bits = new_bits;
                packet
            }).collect_vec()
        },
        LengthType::Bits(num_bits) => {
            let mut sub_bits = &bits[..num_bits];
            let mut subpackets = Vec::new();
            println!("Parsing bits '{}'", sub_bits);
            while sub_bits.len() > 6 { // this is a gross hack
                let (packet, new_bits) = Packet::from_binary(sub_bits);
                sub_bits = new_bits;
                subpackets.push(packet);
            }
            *bits = &bits[num_bits..];

            subpackets
        }
    }

}

impl Packet{
    fn from_binary(binary: &str) -> (Packet, &str){
        let mut bits = &binary[..];
        let version= read_bytes_and_incremenent(&mut bits, 3) as u8;
        println!("Version is {}", version);
        let type_id = read_bytes_and_incremenent(&mut bits, 3);
        println!("Type id is {}", type_id);
        let packet_type= match type_id {
            0 => {
                PacketType::Sum(read_subpackets(&mut bits))
            },
            1 => {
                PacketType::Product(read_subpackets(&mut bits))
            },
            2 => {
                PacketType::Minimum(read_subpackets(&mut bits))
            },
            3=> {
                PacketType::Maximum(read_subpackets(&mut bits))
            },
            4 => {
                // Literal, read the VLQ
                let mut literal_binary_rep = String::new();
                let mut more = true;
                while more {
                    more = bits.starts_with('1');
                    let next_bits = &bits[1..5];
                    literal_binary_rep += next_bits;

                    bits = &bits[5..];
                }
                PacketType::Literal(usize::from_str_radix(&literal_binary_rep, 2).unwrap())
            },
            5 => {
                PacketType::GreaterThan(read_subpackets(&mut bits))
            },
            6 => {
                PacketType::LessThan(read_subpackets(&mut bits))
            },
            7 => {
                PacketType::EqualTo(read_subpackets(&mut bits))
            },
            _ => unreachable!()
        };
        let packet = Packet {
            version,
            packet_type
        };
        println!("Parsed packet: {:?}", packet);
        (packet, bits)
    }

    fn version_sum(self: &Self) -> usize {
        let version = self.version as usize;
        match &self.packet_type {
            PacketType::Literal(_) => version,
            PacketType::Sum(subpackets) | 
            PacketType::Product(subpackets) | 
            PacketType::Maximum(subpackets) | 
            PacketType::Minimum(subpackets) | 
            PacketType::GreaterThan(subpackets) | 
            PacketType::LessThan(subpackets) | 
            PacketType::EqualTo(subpackets)
            => version + subpackets.iter().fold(0, |acc, packet| acc + packet.version_sum())
        }
    }

    fn process(self: &Self) -> usize {
        match &self.packet_type {
            PacketType::Literal(value) => *value,
            PacketType::Sum(subpackets) => subpackets.iter().fold(0, |acc, p| acc + p.process()),
            PacketType::Product(subpackets) => subpackets.iter().fold(1, |acc, p| acc * p.process()),
            PacketType::Minimum(subpackets) => subpackets.iter().map(|p| p.process()).min().unwrap(),
            PacketType::Maximum(subpackets) => subpackets.iter().map(|p| p.process()).max().unwrap(),
            PacketType::GreaterThan(subpackets) => if subpackets[0].process() > subpackets[1].process() { 1 } else { 0 },
            PacketType::LessThan(subpackets) => if subpackets[0].process() < subpackets[1].process() { 1 } else { 0 },
            PacketType::EqualTo(subpackets) => if subpackets[0].process() == subpackets[1].process() { 1 } else { 0 },

        }
    }
}


fn main() {
    let (filename, _sample_param)= if env::args().nth(1).map_or(false, |s| s == "-s") {
        ("sample.txt", 0)
    } else {
        ("input.txt", 0)
    };
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", filename));
    let hex_input = input.split('\n').next().unwrap();
    let binary_input = hex_input.chars().fold(String::new(), |acc, c| {
        let hex = c.to_digit(16).unwrap();
        let out = format!("{:04b}", hex);
        acc + &out
    });

    println!("Hex: {:?}\nBinary: {:?}", hex_input, binary_input);
    let (packet , _leftovers)= Packet::from_binary(&binary_input);
    println!("Packet: {:?}", packet);
    println!("Version sum: {}", packet.version_sum());
    println!("Packet total: {}", packet.process());

}

