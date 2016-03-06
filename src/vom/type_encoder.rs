use vdl;
use std::collections;
use std::sync;
use super::encoder::encoder;

enum TypeEncoderError {
    TypeNotFound(*mut vdl::Type)
}

struct type_encoder_locked_by_type_mu {
    typeToId: collections::HashMap<*mut vdl::Type, u64>,

}

struct type_encoder_locked_by_enc_mu {
    enc: *mut encoder,
    sentVersionByte: bool,
    nextId: u64,
}

impl type_encoder {
    fn lookup_type_id(&self, tt: *mut vdl::Type) -> Result<u64, TypeEncoderError> {

    }
}

struct type_encoder<'a> {
    typeMu: sync::RwLock<&'a type_encoder_locked_by_type_mu>,
    encMu: sync::Mutex<&'a type_encoder_locked_by_enc_mu>,
}

impl<'a> type_encoder<'a> {


    fn encode(&self, tt: *mut vdl::Type) -> Result<u64, TypeEncoderError> {
        {
            let typeMuBody = self.typeMu.read().unwrap();
            match typeMuBody.lookup_type_id(tt) {
                Ok(tid) => return Ok(tid),
                Err(err) => ()
            };
        }

        let encMuBody = self.encMu.lock().unwrap();
        if !encMuBody.sentVersionByte {
            encMuBody.enc.
        }
    }
}
