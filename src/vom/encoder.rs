use std::io;
use vdl;
use super::low_level_write;

struct encoder {
    writer: io::Write,
}

impl vdl::Target for encoder {
    fn start(&self, tt: u64) -> Option<vdl::TargetError> {}
    fn finish(&self) -> Option<vdl::TargetError> {}
    fn start_any(&self, tt: u64) -> Option<vdl::TargetError> {}
    fn finish_any(&self) -> Option<vdl::TargetError> {}

    fn from_bool(&self, src: bool) -> Option<vdl::TargetError> {
        match low_level_write::write_bool(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_uint(&self, src: u64) -> Option<vdl::TargetError> {
        match low_level_write::write_uint(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_int(&self, src: i64) -> Option<vdl::TargetError> {
        match low_level_write::write_int(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_float(&self, src: f64) -> Option<vdl::TargetError> {
        match low_level_write::write_float(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_bytes(&self, src: &[u8]) -> Option<vdl::TargetError> {
        match low_level_write::write_byte_slice(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_string(&self, src: &str) -> Option<vdl::TargetError> {
        match low_level_write::write_string(self.writer, src) {
            Ok(n) => None,
            Err(err) => Some(err),
        }
    }

    fn from_enum_label(&self, src: &str) -> Option<vdl::TargetError> {
        let labels = unsafe{ (*tt).labels };
        for (no, label) in &labels.enumerate() {
            if label == src {
                return low_level_write::write_uint(self.writer, no)
            }
        }
        vdl::TargetError::UnknownEnumLabelError(src)
    }

    fn start_list(&self, len: usize) -> Result<vdl::ListTarget, vdl::TargetError> {
        match low_level_write::write_uint(self.writer, len) {
            Ok(n) => Ok(&self),
            Err(err) => return Err(err),
        }
    }

    fn finish_list(&self, x: vdl::ListTarget) -> Option<vdl::TargetError> {
        None
    }
}

impl vdl::ListTarget for encoder {
    fn start_elem(&self, index: usize) -> Result<vdl::Target, vdl::TargetError> {
        Ok(&self)

    }

    fn finish_elem(&self, elem: vdl::Target) -> Option<vdl::TargetError> {
        None
    }
}

impl vdl::SetTarget for encoder {
    fn start_key(&self) -> Result<vdl::Target, vdl::TargetError> {
        Ok(&self)
    }

    fn finish_key(&self, key: vdl::Target) ->Option<vdl::TargetError> {
        None
    }
}

impl vdl::MapTarget for encoder {
    fn start_key(&self) -> Result<vdl::Target, vdl::TargetError> {
        Ok(&self)
    }

    fn finish_key_start_field(&self, key: vdl::Target) -> Result<vdl::Target, vdl::TargetError> {
        Ok(&self)
    }

    fn finish_field(&self, key: vdl::Target, field: vdl::Target) -> Option<vdl::TargetError> {
        None
    }
}

impl vdl::FieldsTarget for encoder {
    fn start_field(&self, name: &str) -> Result<(vdl::Target, vdl::Target), vdl::TargetError> {

    }

    fn finish_field(&self, key: vdl::Target, field: vdl::Target) -> Option<vdl::TargetError> {

    }
}
