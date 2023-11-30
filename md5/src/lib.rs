// "derived from the RSA Data Security, Inc. MD5 Message-Digest Algorithm"
// https://datatracker.ietf.org/doc/html/rfc1321
const SHIFTS: [i32;16] = [7,12,17,22,5,9,14,20,4,11,16,23,6,10,15,21];
const SINES: [u32;64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,

    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,

    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,

    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391];

#[allow(arithmetic_overflow)]
pub fn md5hash(buffer: &[u8]) -> [u8;16] {
    let blocks = (buffer.len() + 8) / 64 + 1;

    let mut aa: u32 = 0x67452301;
    let mut bb: u32 = 0xefcdab89;
    let mut cc: u32 = 0x98badcfe;
    let mut dd: u32 = 0x10325476;

    for i in 0..blocks {
        let mut block = buffer.to_owned();
        let mut offset: usize = i * 64;

        if offset + 64 > buffer.len() {
            block = [0;64].to_vec();
            for j in offset..buffer.len() {
                block[j - offset] = buffer[j];
            }
            if offset <= buffer.len() {
                block[buffer.len() - offset] = 0x80;
            }
            if i == blocks - 1 {
                block[56] = (buffer.len() << 3) as u8;
                block[57] = (buffer.len() >> 5) as u8;
                block[58] = (buffer.len() << 13) as u8;
                block[59] = (buffer.len() >> 21) as u8;
            }

            offset = 0;
        }

        let mut a = aa;
        let mut b = bb;
        let mut c = cc;
        let mut d = dd;

        #[allow(unused_assignments)]
        let mut f: u32 = 0;
        #[allow(unused_assignments)]
        let mut g: u32 = 0;

        for j in 0..64 {
            if j < 16 {
                f = b & c | !b & d;
                g = j;
            } else if j < 32 {
                f = b & d | c & !d;
                g = 5 * j + 1;
            } else if j < 48 {
                f = b ^ c ^ d;
                g = 3 * j + 5;
            }else {
                f = c ^ (b | !d);
                g = 7 * j;
            }

            g = (g & 0x0f) * 4 + offset as u32;

            let hold: u32 = d;
            d = c;
            c = b;
            b = a.wrapping_add(f)
                .wrapping_add(SINES[j as usize])
                .wrapping_add(block[g as usize] as u32 + 
                    ((block[g as usize+1] as u32).rotate_left(8)) + 
                    ((block[g as usize+2] as u32).rotate_left(16)) + 
                    ((block[g as usize+3] as u32).rotate_left(24)) 
                as u32);
            b = b << SHIFTS[j as usize & 3| j as usize >> 2 & !3] | b >> 32 - SHIFTS[j as usize & 3 | j as usize >> 2 & !3];
            b = b.wrapping_add(c);

            a = hold;
        }

        aa = aa.wrapping_add(a);
        bb = bb.wrapping_add(b);
        cc = cc.wrapping_add(c);
        dd = dd.wrapping_add(d);
    }

    return [
        aa as u8, (aa >> 8) as u8, (aa >> 16) as u8, (aa >> 24) as u8,
        bb as u8, (bb >> 8) as u8, (bb >> 16) as u8, (bb >> 24) as u8,
        cc as u8, (cc >> 8) as u8, (cc >> 16) as u8, (cc >> 24) as u8,
        dd as u8, (dd >> 8) as u8, (dd >> 16) as u8, (dd >> 24) as u8,
    ];
}

pub fn to_hex(hash: &[u8;16]) -> String {
    let mut s = String::new();
    for i in hash {   
        let q = format!("{:02x}",i);
        s.push_str(q.as_str());
    }
    return s;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = to_hex(&md5hash("".as_bytes()));
        assert_eq!(result, "d41d8cd98f00b204e9800998ecf8427e");

        let result = to_hex(&md5hash("a".as_bytes()));
        assert_eq!(result, "0cc175b9c0f1b6a831c399e269772661");

        let result = to_hex(&md5hash("abc".as_bytes()));
        assert_eq!(result, "900150983cd24fb0d6963f7d28e17f72");

        let result = to_hex(&md5hash("message digest".as_bytes()));
        assert_eq!(result, "f96b697d7cb7938d525a2f31aaf161d0");
    }
}
