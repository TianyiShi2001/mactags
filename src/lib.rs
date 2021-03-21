use std::error::Error;
use std::path::Path;

pub const TAG_ATTR_KEY: &'static str = "com.apple.metadata:_kMDItemUserTags";

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
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        match xattr::get(path, TAG_ATTR_KEY) {
            Ok(tags) => {
                if let Some(tags) = tags {
                    return Self::new(tags);
                } else {
                    panic!("The attribute {} is empty.", TAG_ATTR_KEY);
                }
            }
            Err(e) => {
                eprintln!("Failed to access the attribute {}.", TAG_ATTR_KEY);
                panic!("{:?}", e);
            }
        }
    }
    pub fn from_tags(tags: Vec<String>) -> Self {
        let mut data = b"bplist00".to_vec();
        let n = tags.len();
        if n < 0xF {
            data.push(0xA0 + n as u8);
        } else {
            data.push(0xAF);
            data.push(0x10);
            data.push(n as u8);
        }
        for i in 1..=n as u8 {
            data.push(i);
        }
        for tag in tags {
            let l = tag.len();
            let mut is_utf16 = false;
            for c in tag.chars() {
                if !c.is_ascii() {
                    is_utf16 = true;
                    break;
                }
            }
            if is_utf16 {
                if l < 0xF {
                    data.push(0x60 + l as u8);
                } else {
                    data.push(0x6F);
                    data.push(0x10);
                    data.push(l as u8);
                }
                for [a, b] in tag.encode_utf16().map(|c| c.to_be_bytes()) {
                    data.push(a);
                    data.push(b);
                }
            } else {
                if l < 0xF {
                    data.push(0x50 + l as u8);
                } else {
                    data.push(0x5F);
                    data.push(0x10);
                    data.push(l as u8);
                }
                data.extend_from_slice(tag.as_bytes());
            }
        }
        data.push(0x08);
        Self { data }
    }
    pub fn parse(&self) -> Vec<String> {
        let mut n = self.data[8] - 0xA0;
        let mut i = 9;
        if n == 0x0F {
            n = self.data[10];
            i += 2;
        }
        let n = n as usize;
        i += n;
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

                // TODO: handle color
                if is_utf16 {
                    let j = i + 2 * m;
                    let slc = &self.data[i..j];
                    let iter = (0..m)
                        .map(|i| u16::from_be_bytes([slc[2 * i], slc[2 * i + 1]]))
                        .take_while(|c| *c != 0x000A);
                    i = j;
                    String::from_utf16(&iter.collect::<Vec<u16>>()).unwrap()
                } else {
                    let slc = &self.data[i..i + m];
                    i += m;
                    String::from_utf8(slc.iter().cloned().take_while(|&c| c != 0x0A).collect())
                        .unwrap()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_chinese() {
        let tags = Tags::from_hex(
            &"62 70 6C 69 73 74 30 30 A1 01 6F 10 0F 4E 00 4E
        8C 4E 09 56 DB 4E 94 51 6D 4E 03 51 6B 4E 5D 53
        41 4E 00 4E 8C 4E 09 56 DB 4E 94 08 0A 00 00 00
        00 00 00 01 01 00 00 00 00 00 00 00 02 00 00 00
        00 00 00 00 00 00 00 00 00 00 00 00 2B",
        );
        assert_eq!(tags.parse(), vec!["一二三四五六七八九十一二三四五"]);
    }

    #[test]
    fn write_long_ascii() {
        let tags = vec!["123456789abcdef".to_string()];
        let tags = Tags::from_tags(tags);
        println!("{:x?}", tags.data);
    }
}
