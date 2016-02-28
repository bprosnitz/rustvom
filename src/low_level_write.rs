use std::io;

pub fn write_uint<W: io::Write>(writer: &mut W, uval: u64) -> Result<usize, io::Error> {
    if uval <= 0x7f {
         writer.write(&[uval as u8])
    } else if uval <= 0xff {
        writer.write(&[0xff as u8, uval as u8])
    } else if uval <= 0xffff {
        writer.write(&[0xfe as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffff {
        writer.write(&[0xfd as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffff {
        writer.write(&[0xfc as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffff {
        writer.write(&[0xfb as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffffff {
        writer.write(&[0xfa as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else if uval <= 0xffffffffffffff {
        writer.write(&[0xf9 as u8, (uval >> 48) as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    } else {
        writer.write(&[0xf8 as u8, (uval >> 56) as u8, (uval >> 48) as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
    }
}

fn write_int<W: io::Write>(writer: &mut W, ival: i64) -> Result<usize, io::Error> {
    write_uint(writer, int_to_uint(ival))
}
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
	    (!ival<<1 | 1) as u64
	} else {
		(ival << 1) as u64
	}
}


#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::io;
    use std::vec::Vec;
    use super::{write_uint,write_int};

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
        write_uint_test_helper(0x0100, &[0xfe, 0x01, 0x00]);
        write_uint_test_helper(0xffff, &[0xfe, 0xff, 0xff]);
        write_uint_test_helper(0x010000, &[0xfd, 0x01, 0x00, 0x00]);
        write_uint_test_helper(0xffffff, &[0xfd, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x01000000, &[0xfc, 0x01, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xdeadbeef, &[0xfc, 0xde, 0xad, 0xbe, 0xef]);
        write_uint_test_helper(0xffffffff, &[0xfc, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x0100000000, &[0xfb, 0x01, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffff, &[0xfb, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x010000000000, &[0xfa, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffff, &[0xfa, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x01000000000000, &[0xf9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffffff, &[0xf9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
        write_uint_test_helper(0x0100000000000000, &[0xf8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
        write_uint_test_helper(0xffffffffffffffff, &[0xf8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }

    fn write_int_test_helper(input: i64, output: &[u8])    {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);

        write_int(&mut writer, input).unwrap();
        assert_eq!(*writer.get_ref(), output);
    }

    #[test]
    fn test_write_int() {
        write_int_test_helper(0, &[0x00]);
        write_int_test_helper(1, &[0x02]);
        write_int_test_helper(2, &[0x04]);

        write_int_test_helper(63, &[0x7e]);
        write_int_test_helper(64, &[0xff, 0x80]);
        write_int_test_helper(65, &[0xff, 0x82]);
        write_int_test_helper(127, &[0xff, 0xfe]);
        write_int_test_helper(128, &[0xfe, 0x01, 0x00]);
        write_int_test_helper(129, &[0xfe, 0x01, 0x02]);
        write_int_test_helper((1<<15)-1, &[0xfe, 0xff, 0xfe]);
        write_int_test_helper(0x7fffffff, &[0xfc, 0xff, 0xff, 0xff, 0xfe]);
        write_int_test_helper(0x7fffffffffffffff, &[0xf8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe]);

        write_int_test_helper(-1, &[0x01]);
        write_int_test_helper(-2, &[0x03]);
        write_int_test_helper(-64, &[0x7f]);
        write_int_test_helper(-65, &[0xff, 0x81]);
        write_int_test_helper(-66, &[0xff, 0x83]);
        write_int_test_helper(-128, &[0xff, 0xff]);
        write_int_test_helper(-129, &[0xfe, 0x01, 0x01]);
        write_int_test_helper(-130, &[0xfe, 0x01, 0x03]);
        write_int_test_helper(-(1<<15), &[0xfe, 0xff, 0xff]);
        write_int_test_helper(-0x80000000, &[0xfc, 0xff, 0xff, 0xff, 0xff]);
        write_int_test_helper(-0x8000000000000000, &[0xf8, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    }
}
