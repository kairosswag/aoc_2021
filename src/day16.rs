use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Packet {
    packet_version: u8,
    content: Content,
}

#[derive(Clone, Debug)]
enum TypeId {
    Literal,
    Other,
}

impl From<u8> for TypeId {
    fn from(byte: u8) -> Self {
        match byte {
            4 => TypeId::Literal,
            _ => TypeId::Other,
        }
    }
}

#[derive(Clone, Debug)]
enum Content {
    Literal(Vec<u8>),
    Other(Vec<Packet>),
}

pub fn generator(input: &str) -> Packet {
    let input: Vec<u8> = input.chars().tuples().map(&from_hex).collect();
    parse_packet(&input, 0, &mut 0)
}

fn parse_packet(val: &[u8], offset: u8, global_offset: &mut u64) -> Packet {
    // println!("{:#24b}", ((val[0] as u32) << 16) + ((val[1] as u32) << 8) + val[2] as u32);
    let aligned_header = if offset == 0 {
        val[0]
    } else {
        let mut aligned_header = to_u16(val[..2].try_into().expect("eh"));
        aligned_header <<= offset;
        aligned_header >>= 8;
        aligned_header as u8
    };
    println!("aligned header:\t{:08b}", aligned_header);
    let packet_version = aligned_header >> 5;
    // println!("type_id: {:#08b}",  (aligned_header << 3) >> 5);
    let packet_type = TypeId::from((aligned_header << 3) >> 5);
    // println!("type: {:?}", packet_type);

    let offset = offset + 6;
    *global_offset += 6;
    let bit_offset = offset % 8;
    let byte_offset = (offset / 8) as usize;

    let content = match packet_type {
        TypeId::Literal => Content::Literal(parse_literal(
            &val[byte_offset..],
            bit_offset,
            global_offset,
        )),
        TypeId::Other => {
            Content::Other(parse_other(&val[byte_offset..], bit_offset, global_offset))
        }
    };

    Packet {
        packet_version,
        content,
    }
}

#[test]
fn test_parse_other() {
    let packet = generator(&"EE00D40C823060");
    println!("{:?}", packet);

    //0101001000100100
    //01010010

    //1000100100
    //10001001

}

fn parse_other(val: &[u8], offset: u8, global_offset: &mut u64) -> Vec<Packet> {
    // println!("other: {:#24b}", ((val[0] as u32) << 16) + ((val[1] as u32) << 8) + val[2] as u32);
    // println!("bit offset: {offset}");
    // println!("global offset: {global_offset}");
    let start_offset = *global_offset;
    let pck = if offset == 0 {
        to_u16(val[..2].try_into().expect("eh"))
    } else {
        let with_next = to_u32(val[..4].try_into().expect("eh"));
        ((with_next << offset) >> 16) as u16
    };
    println!("other pck:\t{:016b}", pck);

    let mut offset_bits;
    let mut offset_bytes;
    let mut packets = Vec::new();

    if pck >> 15 == 0 {
        *global_offset += 16;
        let total_len_of_bits = (pck << 1) >> 1;
        println!("total len:\t{}", total_len_of_bits);
        // // println!("pck whole: {:#016b}", pck);
        let finished = total_len_of_bits as u64 + *global_offset;
        while (finished - 4) > *global_offset {
            println!("fin: {}, global: {}", finished, global_offset);
            let curr_offset = offset as u64 + *global_offset - start_offset;
            offset_bits = (curr_offset % 8) as u8;
            offset_bytes = (curr_offset / 8) as usize;
            // println!("pshing a with global offset at {} and finished at {}", global_offset, finished);
            // println!("curr: {}, offset_bits {} and offset_bytes {}", curr_offset, offset_bits, offset_bytes);
            packets.push(parse_packet(
                &val[offset_bytes..],
                offset_bits,
                global_offset,
            ));
        }
    } else {
        *global_offset += 12;
        let num_sub_pcks = (pck << 1) >> 5;
        println!("sub_pks: {num_sub_pcks}");
        let mut packets = Vec::new();
        for _ in 0..num_sub_pcks {
            let curr_offset = offset as u64 + *global_offset - start_offset;
            offset_bits = (curr_offset % 8) as u8;
            offset_bytes = (curr_offset / 8) as usize;
            // println!("pshing b");
            packets.push(parse_packet(
                &val[offset_bytes..],
                offset_bits,
                global_offset,
            ))
        }
    }
    packets
}

fn parse_literal(val: &[u8], offset: u8, global_offset: &mut u64) -> Vec<u8> {
    let pck = if offset < 3 {
        val[0] << offset
    } else {
        let with_next = to_u16(val[..2].try_into().expect("eh"));
        ((with_next << offset) >> 8) as u8
    };
    println!("literal pck:\t{:08b}", pck);
    let payload = ((pck << 1) >> 4) as u8;
    let mut res_vec = vec![payload];
    *global_offset += 5;
    if pck >> 7 == 1 {
        // not last
        let offset_next = offset + 5;
        let bit_offset = offset_next % 8;
        let byte_offset = (offset_next / 8) as usize;

        let mut child_res = parse_literal(&val[byte_offset..], bit_offset, global_offset);
        res_vec.append(&mut child_res);
    }

    res_vec
}

fn to_u32(val: &[u8; 4]) -> u32 {
    ((val[0] as u32) << 24) + ((val[1] as u32) << 16) + ((val[2] as u32) << 8) + (val[3] as u32)
}

fn to_u16(val: &[u8; 2]) -> u16 {
    ((val[0] as u16) << 8) as u16 + (val[1] as u16)
}

pub fn part_1(value: &Packet) -> u32 {
    get_version_sum(value)
}

fn get_version_sum(packet: &Packet) -> u32 {
    let sum = packet.packet_version as u32;
    let ver_sum = match &packet.content {
        Content::Literal(_) => 0,
        Content::Other(others) =>  others.iter().map(|p| get_version_sum(p)).sum(),
    };
    sum + ver_sum
}

fn from_hex((a, b): (char, char)) -> u8 {
    (hex_val(a) << 4) + hex_val(b)
}

fn hex_val(hex_val: char) -> u8 {
    match hex_val {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => unreachable!(),
    }
}

#[test]
fn test_bitshifts() {
    let val: Vec<u8> = vec![56];
    let packet_version = val[0] >> 5;
    let packet_type = (val[0] << 3) >> 5;

    assert_eq!(1, packet_version);
    assert_eq!(6, packet_type);
}

#[test]
fn test_parse_literal() {
    let packet = generator(&"D2FE28");
    println!("{:?}", packet);
}


#[test]
fn test() {

    println!("val: {:?}", &generator(&"8A004A801A8002F478"));
    assert_eq!(16, part_1(&generator(&"8A004A801A8002F478")));
    assert_eq!(12, part_1(&generator(&"620080001611562C8802118E34")));
    assert_eq!(23, part_1(&generator(&"C0015000016115A2E0802F182340")));
    assert_eq!(31, part_1(&generator(&"A0016C880162017C3686B18A3D4780")));
}