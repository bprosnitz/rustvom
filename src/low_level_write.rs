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

fn write_float<W: io::Write>(writer: &mut W, fval: f64) -> Result<usize, io::Error> {
    let f64ptr: *const f64 = &fval;
    let u64ptr: *const u64 = f64ptr as *const _;
    let uval = unsafe { *u64ptr };
    write_uint(writer, reverse_byte_order(uval))
}

fn write_bool<W: io::Write>(writer: &mut W, bval: bool) -> Result<usize, io::Error> {
    if bval {
        write_uint(writer, 1)
    } else {
        write_uint(writer, 0)
    }
}

fn write_string<W: io::Write>(writer: &mut W, sval: &str) -> Option<io::Error> {
    match write_uint(writer, sval.len() as u64) {
        Ok(n) => {},
        Err(err) => return Some(err)
    };
    match writer.write(sval.as_bytes()) {
        Ok(n) => None,
        Err(err) => Some(err)
    }
}

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
    use super::{write_uint,write_int,write_float,write_bool,write_string};

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

    fn write_float_test_helper(input: f64, output: &[u8])    {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);

        write_float(&mut writer, input).unwrap();
        assert_eq!(*writer.get_ref(), output);
    }

    #[test]
    fn test_write_float() {
        write_float_test_helper(0.0, &[0x00]);
        write_float_test_helper(1.0, &[0xfe, 0xf0, 0x3f]);
        write_float_test_helper(17.0, &[0xfe, 0x31, 0x40]);
        write_float_test_helper(18.0, &[0xfe, 0x32, 0x40]);
    }

    fn write_bool_test_helper(input: bool, output: &[u8])    {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);

        write_bool(&mut writer, input).unwrap();
        assert_eq!(*writer.get_ref(), output);
    }

    #[test]
    fn test_write_bool() {
        write_bool_test_helper(false, &[0x00]);
        write_bool_test_helper(true, &[0x01]);
    }

    fn write_string_test_helper(input: &str, output: &[u8])    {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);

        assert_eq!(write_string(&mut writer, input).is_none(), true);
        assert_eq!(*writer.get_ref(), output);
    }

    #[test]
    fn test_write_string() {
        write_string_test_helper("", &[0x00]);
        write_string_test_helper("abc", &[3, 97, 98, 99]);
    }
}
