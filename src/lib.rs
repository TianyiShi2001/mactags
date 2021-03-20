#[derive(Debug)]
pub struct Tags {
    pub data: Vec<u8>,
}

impl Tags {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    pub fn from_hex(hex_str: &str) -> Self {
        Self::new(
            hex::decode(
                hex_str
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>(),
            )
            .unwrap(),
        )
        // TODO: handle error
    }
    pub fn parse(&self) -> Vec<String> {
        let n = self.data[8] as usize - 0xA0;
        let mut i = 9 + n;
        let mut is_utf16 = false;
        (0..n)
            .map(|_| {
                let mut m = self.data[i];
                if m < 0x60 {
                    is_utf16 = false;
                    if m == 0x5F {
                        i += 2;
                        m = self.data[i];
                    } else {
                        m -= 0x50;
                    }
                } else {
                    is_utf16 = true;
                    if m == 0x6F {
                        i += 2;
                        m = self.data[i];
                    } else {
                        m -= 0x60
                    }
                }
                let m = m as usize;
                i += 1;
                if is_utf16 {
                    let j = i + 2 * m;
                    let slc = &self.data[i..j];
                    let iter = (0..m).map(|i| u16::from_be_bytes([slc[2 * i], slc[2 * i + 1]]));
                    i = j;
                    String::from_utf16(&iter.collect::<Vec<u16>>()).unwrap()
                } else {
                    let slc = &self.data[i..i + m];
                    i += m;
                    String::from_utf8(slc.to_owned()).unwrap()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chinese() {
        let tags = Tags::from_hex(
            &"62 70 6C 69 73 74 30 30 A1 01 6F 10 0F 4E 00 4E
        8C 4E 09 56 DB 4E 94 51 6D 4E 03 51 6B 4E 5D 53
        41 4E 00 4E 8C 4E 09 56 DB 4E 94 08 0A 00 00 00
        00 00 00 01 01 00 00 00 00 00 00 00 02 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 2B",
        );
        assert_eq!(tags.parse(), vec!["一二三四五六七八九十一二三四五"]);
    }
}
