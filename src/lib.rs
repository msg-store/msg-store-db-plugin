pub use bytes::Bytes;
use msg_store::Uuid;

pub trait Db: Send + Sync {
    fn get(&mut self, uuid: Uuid) -> Result<Bytes, String>;
    fn add(&mut self, uuid: Uuid, msg: Bytes, msg_byte_size: u32) -> Result<(), String>;
    fn del(&mut self, uuid: Uuid) -> Result<(), String>;
    fn fetch(&mut self) -> Result<Vec<(Uuid, u32)>, String>;
}

#[cfg(test)]
mod tests {


    use msg_store::Uuid;
    use std::collections::BTreeMap;

    use crate::{Db, Bytes};

    struct MemDb {
        msgs: BTreeMap<Uuid, Bytes>,
        msg_data: BTreeMap<Uuid, u32>
    }

    impl MemDb {
        pub fn new() -> MemDb {
            MemDb {
                msgs: BTreeMap::new(),
                msg_data: BTreeMap::new()
            }
        }
    }

    impl Db for MemDb {
        fn get<'a>(&mut self, uuid: Uuid) -> Result<Bytes, String> {
            let msg = match self.msgs.get(&uuid){
                Some(msg) => Ok(msg),
                None => Err("Hello".to_string())
            }?;
            Ok(msg.clone())
        }
        fn add(&mut self, uuid: Uuid, msg: Bytes, msg_byte_size: u32) -> Result<(), String> {
            self.msgs.insert(uuid, msg);
            self.msg_data.insert(uuid, msg_byte_size);
            Ok(())
        }
        fn del(&mut self, uuid: Uuid) -> Result<(), String> {
            self.msgs.remove(&uuid);
            self.msg_data.remove(&uuid);
            Ok(())
        }
        fn fetch(&mut self) -> Result<Vec<(Uuid, u32)>, String> {
            Ok(vec![])
        }
    }

    #[test]
    fn it_works() {

        let mut db = MemDb::new();

        let uuid = Uuid::from_string("0-0-0").unwrap();
        let inserted_msg = "hello, world".as_bytes();

        db.add(uuid, Bytes::copy_from_slice(inserted_msg), 4).unwrap();
        let received_msg = db.get(uuid).unwrap();
        assert_eq!(inserted_msg, received_msg);
    }
}
