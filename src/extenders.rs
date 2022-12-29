use crate::rustypickle::Pickle;
use serde::Serialize;

pub struct PickleListExtender<'a> {
    pub(crate) db: &'a mut Pickle,
    pub(crate) list_name: String,
}
impl<'a> PickleListExtender<'a> {
    pub fn ladd<V>(&mut self, value: &V) -> PickleListExtender
    where
        V: Serialize,
    {
        self.db.ladd(&self.list_name, value).unwrap()
    }

    pub fn lextend<'i, V, I>(&mut self, seq: I) -> PickleListExtender
    where
        V: 'i + Serialize,
        I: IntoIterator<Item = &'i V>,
    {
        self.db.lextend(&self.list_name, seq).unwrap()
    }
}
