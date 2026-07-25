use crate::id::Id;
use serbytes::prelude::{BBReadResult, ReadByteBufferRefMut, SerBytes, WriteByteBufferOwned};

impl SerBytes for Id {
    fn from_buf(buf: &mut ReadByteBufferRefMut) -> BBReadResult<Self>
    where
        Self: Sized,
    {
        let maybe_header = u8::from_buf(buf)?;

        let header;
        let increment;

        if Id::is_header(maybe_header) {
            header = maybe_header;
            increment = u8::from_buf(buf)?;
        } else {
            header = 0b00000000;
            increment = maybe_header;
        }

        let time = u16::from_buf(buf)?;

        let data = buf
            .read_bytes(12)?
            .try_into()
            .expect("Just read 12 bytes from the buffer, can't fail");

        Ok(Self {
            header,
            increment,
            time,
            data,
        })
    }

    fn to_buf(&self, buf: &mut WriteByteBufferOwned) {
        if self.has_header() {
            self.header.to_buf(buf);
        }

        self.increment.to_buf(buf);
        self.time.to_buf(buf);
        buf.write_bytes(&self.data);
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serbytes::prelude::*;

    #[test]
    fn test_serbytes() {
        let mut buf = WriteByteBufferOwned::new();

        let id = SmallRngIdGenerator::default().generate_new_id();

        id.to_buf(&mut buf);

        let mut rbb = ReadByteBufferOwned::from_vec(buf.into_vec());
        let deser_id = Id::from_buf(&mut rbb.rbb_ref_mut()).expect("Deserialize id");

        assert_eq!(id, deser_id);
    }
}
// not static sized
