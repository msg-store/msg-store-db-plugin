use msg_store::Uuid;
use std::{
    collections::{
        BTreeMap,
        VecDeque
    }
};

pub enum DbAction {
    Writing,
    Reading,
    Deleting
}

pub enum DbCommand {
    Write,
    Read,
    Delete
}

pub struct Cache<T> {
    pub queue: VecDeque<(Uuid, DbCommand)>,
    pub cache: BTreeMap<Uuid, T>,
    pub actions: BTreeMap<Uuid, DbAction>
}

pub trait Db<T, E> {
    fn get(&mut self, uuid: Uuid) -> Result<T, E>;
    fn add(&mut self, uuid: Uuid, msg: T) -> Result<(), E>;
    fn del(&mut self, uuid: Uuid) -> Result<(), E>;
    fn fetch(&mut self) -> Result<Vec<(Uuid, u32)>, E>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
