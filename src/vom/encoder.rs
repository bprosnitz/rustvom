use std::io;
use vdl;
use super::low_level_write;

struct encoder {
    writer: io::Write,
}

impl vdl::Target for encoder {
    fn from_bool(&self, src: bool, tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_bool(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_uint(&self, src: u64, tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_uint(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_int(&self, src: i64, tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_int(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_float(&self, src: f64, tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_float(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_bytes(&self, src: &[u8], tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_byte_slice(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_string(&self, src: &str, tt: *mut vdl::Type) -> Option<io::Error> {
        match low_level_write::write_string(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn start_list(&self, tt: *mut vdl::Type, len: usize) -> Result<vdl::ListTarget, io::Error> {
        match low_level_write::write_uint(self.writer, len) {
            Ok(n) => Ok(&self),
            Err(err) => return Err(err),
        }
    }

    fn finish_list(&self, x: vdl::ListTarget) -> Option<io::Error> {
        None
    }
}

impl vdl::ListTarget for encoder {
    fn start_elem(&self, index: usize) -> Result<vdl::Target, io::Error> {

    }

    fn finish_elem(&self, elem: vdl::Target) -> Option<io::Error> {

    }
}

impl vdl::SetTarget for encoder {
}

impl vdl::MapTarget for encoder {
}

impl vdl::FieldsTarget for encoder {
}
