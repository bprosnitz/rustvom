use std::io;
use vdl;
use low_level_write;

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
}
