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
        writer.write(&[0xff as u8, (uval >> 48) as u8, (uval >> 40) as u8, (uval >> 32) as u8, (uval >> 24) as u8, (uval >> 16) as u8, (uval >> 8) as u8, uval as u8])
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

    #[test]
    fn test_write_uint() {
        let buf = Vec::new();
        let mut writer = io::BufWriter::with_capacity(0, buf);
        write_uint(&mut writer, 128).unwrap();
        assert_eq!(*writer.get_ref(), [255, 128]);

        //write_uint(writer, 128).expect(2);
        /*let mut writer = io::BufWriter::with_capacity(2, inner);

        writer.write(&[0, 1]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.write(&[2]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.write(&[3]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1]);

        writer.flush().unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

        writer.write(&[4]).unwrap();
        writer.write(&[5]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3]);

        writer.write(&[6]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5]);

        writer.write(&[7, 8]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8]);

        writer.write(&[9, 10, 11]).unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);

        writer.flush().unwrap();
        assert_eq!(*writer.get_ref(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);*/
    }
}
