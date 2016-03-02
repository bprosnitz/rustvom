use std::io;
use vdl;

struct encoder {
    writer: io::Write,
}

impl vdl::Target for encoder {
    fn from_bool(&self, src: bool, tt: *mut vdl::Type) -> Option<io::Error> {

    }
}
