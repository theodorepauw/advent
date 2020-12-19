const INPUT: &str = include_str!("../inputs/14.txt"); // My solution's very sloppy, but I loved this one!

use std::{collections::HashMap, str::FromStr};
struct MaskV1 {
    zeros: u64,
    ones: u64,
}

#[derive(Clone)]
struct AddressV2(u64);

fn main() {
    let (mut memory_v1, mut memory_v2) = (HashMap::new(), HashMap::new());
    let (mut mask_v1, mut mask_v2) = (MaskV1 { zeros: 0, ones: 0 }, "");

    for l in INPUT.lines() {
        let s: Vec<&str> = l.split(" = ").collect();
        let (spec, val) = (s[0], s[1]);

        match &spec[..4] {
            "mask" => {
                mask_v1 = val.parse::<MaskV1>().expect("couldn't decode mask");
                mask_v2 = val;
            }
            "mem[" => {
                let address_v1 = spec[4..]
                    .split(']')
                    .next()
                    .expect("invalid mem")
                    .parse::<u64>()
                    .unwrap();
                let val = val.parse::<u64>().unwrap();
                memory_v1.insert(address_v1, mask_v1.apply(val));
                AddressV2::generate_all(mask_v2, address_v1)
                    .iter()
                    .for_each(|addr| {
                        memory_v2.insert(addr.0, val);
                    });
            }
            _ => panic!("Shucks. Unrecognised characters in your puzzle input!"),
        }
    }

    println!(
        "Day 14 Part 1: {}\nDay 14 Part 2: {}",
        memory_v1.values().sum::<u64>(),
        memory_v2.values().sum::<u64>()
    );
}

impl FromStr for MaskV1 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (zeros, ones) = s.chars().fold((0, 0), |(mut z, mut o), c| {
            z <<= 1;
            o <<= 1;
            match c {
                '0' => z |= 1,
                '1' => o |= 1,
                _ => {}
            }
            (z, o)
        });
        Ok(MaskV1 { zeros, ones })
    }
}

impl MaskV1 {
    fn apply(&self, val: u64) -> u64 {
        self.ones | val & !self.zeros
    }
}

impl AddressV2 {
    fn generate_all(mask: &str, address: u64) -> Vec<Self> {
        let address = format!("{:0>36b}", address);
        mask.chars()
            .zip(address.chars())
            .fold(vec![AddressV2(0)], |mut result, (mc, ac)| {
                result.iter_mut().for_each(|x| x.0 <<= 1);
                match (mc, ac) {
                    ('0', '0') => {}
                    ('0', '1') | ('1', _) => result.iter_mut().for_each(|x| x.0 |= 1),
                    ('X', _) => result.extend(result.clone().iter().map(|a| AddressV2(a.0 | 1))),
                    _ => panic!("unrecognised characters"),
                }
                result
            })
    }
}
