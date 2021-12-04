pub fn bytes_to_hex(raw: &[u8]) -> String {
    raw.iter()
        .map(|v| format!("{:x?}", v))
        .collect::<Vec<String>>()
        .join("")
}

pub fn bytes_to_hex2(raw: &[u8]) -> String {
    raw.iter().map(|&v| v as char).collect::<String>()
}

pub fn bytes_to_usize(raw: &[u8]) -> usize {
    let mut res = 0;
    raw.iter().for_each(|&v| {
        res = res * 10 + (v as usize - 48);
    });

    res
}

#[cfg(test)]
mod tests {
    use crate::utils::bytes::bytes_to_hex2;

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
        let s = bytes_to_hex2(raw);
        println!("{}", s);
    }

    #[test]
    fn t2() {
        let raw = &[50, 52, 55]; // 247
        let size = bytes_to_usize(raw);
        assert_eq!(size, 247);
    }
}
