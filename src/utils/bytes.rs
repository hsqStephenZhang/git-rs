// transfer 35 like into 2*16 + 3, which is 23 in hexcode
// notice that we should fill the content with width 2, filled by '0' (48 in c char)
pub fn bytes_to_hex(raw: &[u8]) -> String {
    raw.iter()
        .map(|v| format!("{:02x?}", v))
        .collect::<Vec<String>>()
        .join("")
}

pub fn hex_to_bytes(hex: &[u8]) -> Vec<u8> {
    hex.windows(2)
        .step_by(2)
        .map(|v| {
            let a = v[0];
            let b = v[1];
            let v1 = if a >= b'a' {
                a - b'a' + 10
            } else if a >= b'A' {
                a - b'A' + 10
            } else {
                a - b'0'
            };

            let v2 = if b >= b'a' {
                b - b'a' + 10
            } else if b >= b'A' {
                b - b'A' + 10
            } else {
                b - b'0'
            };

            v1 * 16 + v2
        })
        .collect()
}

pub fn bytes_to_string(raw: &[u8]) -> String {
    raw.iter().map(|&v| v as char).collect::<String>()
}

// transfer 1234(0x31 0x32 0x33 0x34) into 1234
pub fn bytes_to_usize(raw: &[u8]) -> usize {
    let mut res = 0;
    raw.iter().for_each(|&v| {
        res = res * 10 + (v as usize - 0x30);
    });

    res
}

#[cfg(test)]
mod tests {
    use crate::utils::bytes::bytes_to_string;

    use super::{bytes_to_hex, bytes_to_usize};

    #[test]
    fn t1() {
        // let raw = &[
        //     35, 106, 6, 195, 235, 145, 129, 18, 35, 184, 173, 220, 40, 101, 120, 150, 201, 64, 128,
        //     173,
        // ];
        // let s = bytes_to_hex(raw);
        // assert_eq!(&s, "236a6c3eb91811223b8addc28657896c94080ad");

        let raw = &[
            55, 97, 100, 49, 51, 54, 101, 102, 97, 51, 51, 52, 98, 51, 52, 97, 102, 49, 56, 54, 97,
            54, 102, 55, 48, 52, 97, 100, 56, 50, 48, 56, 48, 48, 57, 49, 102, 55, 53, 101,
        ];
        // let s= raw.
        let s = bytes_to_hex(raw);
        println!("{}", s);
        let s = bytes_to_string(raw);
        println!("{}", s);
    }

    #[test]
    fn t2() {
        let raw = &[50, 52, 55]; // 247
        let size = bytes_to_usize(raw);
        assert_eq!(size, 247);
    }
}
