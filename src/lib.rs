use std::cmp;

fn get_bytes_u16(x: u16) -> [u8; 2] {
    let mut buf = [0 as u8; 2];
    buf[0] = (x >> 8) as u8;
    buf[1] = (x & 0xFF) as u8;
    buf
}

fn get_bytes_u32(x: u32) -> [u8; 4] {
    let mut buf = [0 as u8; 4];
    buf[0] = ((x >> 24) & 0xFF) as u8;
    buf[1] = ((x >> 16) & 0xFF) as u8;
    buf[2] = ((x >> 8) & 0xFF) as u8;
    buf[3] = (x & 0xFF) as u8;
    buf
}

fn get_bytes_u64(x: u64) -> [u8; 8] {
    let mut buf = [0 as u8; 8];
    buf[0] = ((x >> 56) & 0xFF) as u8;
    buf[1] = ((x >> 48) & 0xFF) as u8;
    buf[2] = ((x >> 40) & 0xFF) as u8;
    buf[3] = ((x >> 32) & 0xFF) as u8;
    buf[4] = ((x >> 24) & 0xFF) as u8;
    buf[5] = ((x >> 16) & 0xFF) as u8;
    buf[6] = ((x >> 8) & 0xFF) as u8;
    buf[7] = (x & 0xFF) as u8;
    buf
}

pub struct ByteBufMut<'a> {
    pos: usize,
    buf: &'a mut [u8],
}

impl<'a> ByteBufMut<'a> {

    pub fn wrap(buf: &'a mut [u8]) -> ByteBufMut<'a> {
        ByteBufMut { pos: 0, buf }
    }

    pub fn put_u8(&mut self, x: u8) -> usize {
        self.put_bytes(&[x])
    }

    pub fn put_u16(&mut self, x: u16) -> usize {
        self.put_bytes(&get_bytes_u16(x))
    }

    pub fn put_u32(&mut self, x: u32) -> usize {
        self.put_bytes(&get_bytes_u32(x))
    }

    pub fn put_u64(&mut self, x: u64) -> usize {
        self.put_bytes(&get_bytes_u64(x))
    }

    fn put_bytes(&mut self, bytes: &[u8]) -> usize {
        let pos = self.pos;
        for (i, &x) in bytes.iter().enumerate() {
            self.pos = pos + i;
            if self.pos >= self.buf.len() {
                return i;
            }
            self.buf[self.pos] = x;
        }
        self.pos += 1;
        bytes.len()
    }
}

pub struct ByteBuf<'a> {
    pos: usize,
    buf: &'a [u8],
}

impl<'a> ByteBuf<'a> {

    pub fn wrap(buf: &'a [u8]) -> ByteBuf<'a> {
        ByteBuf { pos: 0, buf }
    }

    fn get_bytes(&mut self, n: usize) -> &[u8] {
        let at = self.pos;
        let to = cmp::min(self.buf.len(), self.pos + n);
        &self.buf[at..to]
    }

    pub fn get_u8(&mut self) -> Option<u8> {
        let n = 1;
        let bytes = self.get_bytes(n);
        if bytes.len() == n {
            let x = bytes[0];
            self.pos += n;
            Some(x)
        } else {
            None
        }
    }

    pub fn get_u16(&mut self) -> Option<u16> {
        let n = 2;
        let bytes = self.get_bytes(n);
        if bytes.len() == n {
            let x = ((bytes[0] as u16) << 8) + bytes[1] as u16;
            self.pos += n;
            Some(x)
        } else {
            None
        }
    }

    pub fn get_u32(&mut self) -> Option<u32> {
        let n = 4;
        let bytes = self.get_bytes(n);
        if bytes.len() == n {
            let x = ((bytes[0] as u32) << 24) +
                ((bytes[1] as u32) << 16) +
                ((bytes[2] as u32) << 8) +
                bytes[3] as u32;
            self.pos += n;
            Some(x)
        } else {
            None
        }
    }

    pub fn get_u64(&mut self) -> Option<u64> {
        let n = 8;
        let bytes = self.get_bytes(n);
        if bytes.len() == n {
            let x = ((bytes[0] as u64) << 56) +
                ((bytes[1] as u64) << 48) +
                ((bytes[2] as u64) << 40) +
                ((bytes[3] as u64) << 32) +
                ((bytes[4] as u64) << 24) +
                ((bytes[5] as u64) << 16) +
                ((bytes[6] as u64) << 8) +
                bytes[7] as u64;
            self.pos += n;
            Some(x)
        } else {
            None
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_u8() {
        let x: u8 = 0xAB;
        let mut buf = [0 as u8; 1];
        let mut bb = ByteBufMut::wrap(&mut buf);
        assert_eq!(bb.put_u8(x), 1);
        assert_eq!(bb.pos, 1);
        assert_eq!([x], buf);
    }

    #[test]
    fn test_write_u16() {
        let x: u16 = 0xABCD;
        let mut buf = [0 as u8; 2];
        let mut bb = ByteBufMut::wrap(&mut buf);
        assert_eq!(bb.put_u16(x), 2);
        assert_eq!(bb.pos, 2);
        assert_eq!(buf, [0xAB, 0xCD]);
    }

    #[test]
    fn test_write_u32() {
        let x: u32 = 0xAABBCCDD;
        let mut buf = [0 as u8; 4];
        let mut bb = ByteBufMut::wrap(&mut buf);
        assert_eq!(bb.put_u32(x), 4);
        assert_eq!(bb.pos, 4);
        assert_eq!(buf, [0xAA, 0xBB, 0xCC, 0xDD]);
    }

    #[test]
    fn test_write_u64() {
        let x: u64 = 0xAABBCCDDEEFFABCD;
        let mut buf = [0 as u8; 8];
        let mut bb = ByteBufMut::wrap(&mut buf);
        assert_eq!(bb.put_u64(x), 8);
        assert_eq!(bb.pos, 8);
        assert_eq!(buf, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xAB, 0xCD]);
    }

    #[test]
    fn test_write_all() {
        let a: u8 = 0x01;
        let b: u16 = 0x0002;
        let c: u32 = 0x00000003;
        let d: u64 = 0x0000000000000004;
        let mut buf = [0 as u8; 15];
        {
            let mut bb = ByteBufMut::wrap(&mut buf);
            assert_eq!(bb.put_u8(a), 1);
            assert_eq!(bb.pos, 1);
            assert_eq!(bb.put_u16(b), 2);
            assert_eq!(bb.pos, 3);
            assert_eq!(bb.put_u32(c), 4);
            assert_eq!(bb.pos, 7);
            assert_eq!(bb.put_u64(d), 8);
            assert_eq!(bb.pos, 15);
        }
        assert_eq!(buf, [0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04]);
    }

    #[test]
    fn test_read_u8() {
        let x = 0xAA as u8;
        let buf = [x];
        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(bb.get_u8(), Some(x));
        assert_eq!(bb.pos, 1);
    }

    #[test]
    fn test_read_u16() {
        let x = 0xAABB as u16;
        let buf = [0xAA, 0xBB];
        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(bb.get_u16(), Some(x));
        assert_eq!(bb.pos, 2);
    }

    #[test]
    fn test_read_u32() {
        let x = 0xAABBCCDD as u32;
        let buf = [0xAA, 0xBB, 0xCC, 0xDD];
        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(bb.get_u32(), Some(x));
        assert_eq!(bb.pos, 4);
    }

    #[test]
    fn test_read_u64() {
        let x = 0xAABBCCDDEEFFABCD as u64;
        let buf = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0xAB, 0xCD];
        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(bb.get_u64(), Some(x));
        assert_eq!(bb.pos, 8);
    }

    #[test]
    fn test_read_all() {
        let buf = [0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04];
        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(bb.get_u8(), Some(0x01));
        assert_eq!(bb.pos, 1);
        assert_eq!(bb.get_u16(), Some(0x0002));
        assert_eq!(bb.pos, 3);
        assert_eq!(bb.get_u32(), Some(0x00000003));
        assert_eq!(bb.pos, 7);
        assert_eq!(bb.get_u64(), Some(0x0000000000000004));
        assert_eq!(bb.pos, 15);
    }

    #[test]
    fn test_write_all_read_all() {
        let mut buf = [0 as u8; 1024];

        {
            let mut bb = ByteBufMut::wrap(&mut buf);
            bb.put_u8(1 as u8);
            bb.put_u16(2 as u16);
            bb.put_u32(3 as u32);
            bb.put_u64(4 as u64);
        }

        let exp: [u8; 15] = [1, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4];
        assert_eq!(exp, buf[0..15]);

        let mut bb = ByteBuf::wrap(&buf);
        assert_eq!(1 as u8, bb.get_u8().unwrap());
        assert_eq!(bb.pos, 1);
        assert_eq!(2 as u16, bb.get_u16().unwrap());
        assert_eq!(bb.pos, 3);
        assert_eq!(3 as u32, bb.get_u32().unwrap());
        assert_eq!(bb.pos, 7);
        assert_eq!(4 as u64, bb.get_u64().unwrap());
        assert_eq!(bb.pos, 15);
    }
}
