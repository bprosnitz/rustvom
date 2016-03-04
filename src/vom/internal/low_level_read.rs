use std::io;

enum UintReadResult {
    Uint(u64),
    ControlCode(u8),
}

pub fn read_uint<R: io::Read>(reader: &mut R) -> Result<(UintReadResult, usize), io::Error> {
    let &mut firstByte [u8] = [0];
    match reader.read() {
        Ok(amt) => if amt != 1 {
            return Err(io)
        } else {
            ()
        },
        Err(err) => return Err(err)
    };
    if firstByte[0] <= 0x7f {
        return Ok(Uint(firstByte[0] as u64), 1)
    } else {
        ()
    };
    if firstByte[0] <= 0xef {
        return Ok(ControlCode(firstByte[0]), 1)
    } else {
        ()
    };
    let byteLen = -firstByte[0]
    /*if byteLen < 1 {
        // hate error
    }*/
}
