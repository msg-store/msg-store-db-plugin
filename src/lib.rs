pub use bytes::Bytes;
use msg_store::Uuid;
use std::sync::Arc;

pub trait Db: Send + Sync {
    fn get(&mut self, uuid: Arc<Uuid>) -> Result<Bytes, String>;
    fn add(&mut self, uuid: Arc<Uuid>, msg: Bytes, msg_byte_size: u32) -> Result<(), String>;
    fn del(&mut self, uuid: Arc<Uuid>) -> Result<(), String>;
    fn fetch(&mut self) -> Result<Vec<(Arc<Uuid>, u32)>, String>;
}

#[cfg(test)]
mod tests {


    use msg_store::Uuid;
    use std::{
        collections::BTreeMap,
        sync::Arc
    };

    use crate::{Db, Bytes};

    struct MemDb {
        msgs: BTreeMap<Arc<Uuid>, Bytes>,
        msg_data: BTreeMap<Arc<Uuid>, u32>
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
        fn get(&mut self, uuid: Arc<Uuid>) -> Result<Bytes, String> {
            let msg = match self.msgs.get(&uuid){
                Some(msg) => Ok(msg),
                None => Err("Hello".to_string())
            }?;
            Ok(msg.clone())
        }
        fn add(&mut self, uuid: Arc<Uuid>, msg: Bytes, msg_byte_size: u32) -> Result<(), String> {
            self.msgs.insert(uuid.clone(), msg);
            self.msg_data.insert(uuid, msg_byte_size);
            Ok(())
        }
        fn del(&mut self, uuid: Arc<Uuid>) -> Result<(), String> {
            self.msgs.remove(&uuid);
            self.msg_data.remove(&uuid);
            Ok(())
        }
        fn fetch(&mut self) -> Result<Vec<(Arc<Uuid>, u32)>, String> {
            Ok(vec![])
        }
    }

    #[test]
    fn it_works() {

        let mut db = MemDb::new();

        let uuid = Uuid::from_string("0-0-0").unwrap();
        let inserted_msg = "hello, world".as_bytes();

        db.add(uuid.clone(), Bytes::copy_from_slice(inserted_msg), 4).unwrap();
        let received_msg = db.get(uuid).unwrap();
        assert_eq!(inserted_msg, received_msg);
    }
}
