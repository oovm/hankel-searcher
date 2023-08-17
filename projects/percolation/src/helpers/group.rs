use super::*;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Debug)]
pub struct MergeList<T> {
    next_id: u32,
    groups: BTreeMap<u32, Vec<T>>,
}

impl<T> Default for MergeList<T> {
    fn default() -> Self {
        Self { next_id: 1, groups: BTreeMap::default() }
    }
}

impl<T> MergeList<T> {
    pub fn merge_group(&mut self, keep: u32, remove: u32) -> Option<&mut Vec<T>> {
        // update group
        let rm = self.groups.remove(&remove)?;
        let up = self.groups.get_mut(&keep)?;
        up.extend(rm);
        Some(up)
    }
    pub fn insert(&mut self, key: u32, value: T) -> Option<&mut Vec<T>> {
        match self.groups.get_mut(&key) {
            Some(list) => {
                list.push(value);
                Some(list)
            }
            None => panic!("Failed to get left group {:?}", key),
        }
    }
    pub fn insert_or_create(&mut self, key: u32, value: T) -> &mut Vec<T> {
        self.groups.entry(key).or_insert_with(|| vec![value])
    }
}
