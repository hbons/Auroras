//   Auroras, space weather forecast
//   Copyright (C) 2025  Hylke Bons (hello@planetpeanut.studio)
//
//   This program is free software: you can redistribute it and/or modify it under
//   the terms of the GNU Affero General Public License v3 or any later version.


use std::error::Error;
use std::iter::repeat_n;


pub fn runlength_encode(input: &Vec<u8>) -> Vec<u16> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut output = Vec::new();
    let mut prev = input[0];
    let mut count: u16 = 1;

    for &val in &input[1..] {
        if val == prev {
            count += 1;
        } else {
            output.push(prev as u16);
            output.push(count);
            prev = val;
            count = 1;
        }
    }

    output.push(prev as u16);
    output.push(count);
    output
}


pub fn runlength_decode(input: &Vec<u16>) -> Result<Vec<u8>, Box<dyn Error>> {
    if input.is_empty() || !input.len().is_multiple_of(2) {
        return Err("Input length must be even and not zero".into());
    }

    let mut output = Vec::new();

    let iter = input.chunks_exact(2);
    for pair in iter {
        let value = pair[0];
        let count = pair[1];

        if value > u8::MAX as u16 {
            return Err("Value exceeds u8 range".into());
        }

        output.extend(repeat_n(value as u8, count as usize));
    }

    Ok(output)
}


#[test]
#[cfg_attr(test, allow(clippy::unwrap_used))]
fn test_runlength() {
    let original = vec![1u8, 1, 1, 2, 2, 3];
    let encoded = runlength_encode(&original);
    let decoded = runlength_decode(&encoded).unwrap();

    assert_eq!(encoded, [1u16, 3, 2, 2, 3, 1]);
    assert_eq!(original, decoded);


    let encoded = runlength_encode(&vec![]);
    assert!(encoded.is_empty());


    let odds = vec![0, 0, 0];
    let odds_result = runlength_decode(&odds);
    let empty_result = runlength_decode(&vec![]);

    assert!(odds_result.is_err());
    assert!(empty_result.is_err());
}
