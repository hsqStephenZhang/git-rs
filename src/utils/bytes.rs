pub fn bytes_to_hex(raw: &[u8]) -> String {
    raw.iter()
        .map(|v| format!("{:x?}", v))
        .collect::<Vec<String>>()
        .join("")
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
    use super::{bytes_to_hex, bytes_to_usize};

    #[test]
    fn t1() {
        let raw = &[
            35, 106, 6, 195, 235, 145, 129, 18, 35, 184, 173, 220, 40, 101, 120, 150, 201, 64, 128,
            173,
        ];
        let s = bytes_to_hex(raw);
        assert_eq!(&s, "236a6c3eb91811223b8addc28657896c94080ad");
    }

    #[test]
    fn t2() {
        let raw = &[50, 52, 55]; // 247
        let size = bytes_to_usize(raw);
        assert_eq!(size, 247);
    }
}
