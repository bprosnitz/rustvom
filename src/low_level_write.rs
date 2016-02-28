use std::io;



/*
#[test]
fn test_write_uint() {
    let inner = Vec::new();
    let mut writer = io::BufWriter::with_capacity(2, inner);
    writer.write(&[34,4])
    //write_uint(&buf, 128).expect(2)
}
*/
pub fn write_uint<W: io::Write>(writer: &mut W, uval: u64) -> Result<usize, io::Error> {
    if uval <= 0x7f {
         writer.write(&[uval as u8])
    } else if uval <= 0xff {
        writer.write(&[0xff as u8, uval as u8])
    } else if uval <= 0xffff {
        writer.write(&[0xff as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffff {
        writer.write(&[0xff as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffff {
        writer.write(&[0xff as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffff {
        writer.write(&[0xff as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffffff {
        writer.write(&[0xff as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffffffff {
        writer.write(&[0xff as u8, (uval >> 48) as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else {
        writer.write(&[0xff as u8, (uval >> 56) as u8, (uval >> 48) as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    }
}
/*
fn write_int<W: io::Write>(&mut writer: &mut W, ival: i64) -> Result<usize, io::Error> {
    write_uint(writer, int_to_uint(ival))
}*/
/*
fn write_float<W: io::Write>(mut writer: W, fval: f64) -> Result<usize, io::Error> {
    let uvalptr = (&fval as *const u64);
    let uval = unsafe{*uvalptr};
}
*/
fn reverse_byte_order(v: u64) -> u64 {
    (v&0xff)<<56 |
	(v&0xff00)<<40 |
	(v&0xff0000)<<24 |
    (v&0xff000000)<<8 |
	(v&0xff00000000)>>8 |
	(v&0xff0000000000)>>24 |
	(v&0xff000000000000)>>40 |
	(v&0xff00000000000000)>>56
}

fn int_to_uint(ival: i64) -> u64 {
	if ival < 0 {
	    (((!ival)<<1) | 1) as u64
	} else {
		(ival << 1) as u64
	}
}


#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::io;
    use std::vec::Vec;
    use super::{write_uint};

    fn write_uint_test_helper(input: u64, output: &[u8])    {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);

        write_uint(&mut writer, input).unwrap();
        assert_eq!(*writer.get_ref(), output);
    }

    #[test]
    fn test_write_uint() {
        write_uint_test_helper(0x00, &[0x00]);
        write_uint_test_helper(0x42, &[0x42]);
        write_uint_test_helper(0x7f, &[0x7f]);
        write_uint_test_helper(0x80, &[0xff, 0x80]);
        write_uint_test_helper(0xff, &[0xff, 0xff]);
        write_uint_test_helper(0x0100, &[0xff, 0x01, 0x00]);
        write_uint_test_helper(0xffff, &[0xff, 0xff, 0xff]);
        write_uint_test_helper(0x010000, &[0xff, 0x01, 0x00, 0x00]);
        write_uint_test_helper(0xffffff, &[0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x01000000, &[0xff, 0x01, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xdeadbeef, &[0xff, 0xde, 0xad, 0xbe, 0xef]);
        write_uint_test_helper(0xffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x0100000000, &[0xff, 0x01, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x010000000000, &[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x01000000000000, &[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x0100000000000000, &[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffffffff, &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }
}
