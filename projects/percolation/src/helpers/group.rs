use std::collections::btree_map::Values;
use std::iter::Enumerate;
use super::*;

/// A list of groups, each group has a unique id
#[derive(Clone, Debug)]
pub struct MergeList<T> {
    next_id: u32,
    groups: BTreeMap<u32, Vec<T>>,
}

impl<T> IntoIterator for MergeList<T> {
    type Item = ();
    type IntoIter = ();

    fn into_iter(self) -> Self::IntoIter {
        let i: Enumerate<Values<u32, Vec<T>>> = self.groups.values().enumerate()
    }
}

pub struct MergeListIter<'i>{
    index: usize,
    iter: &'i MergeList<T>,
}

impl Iterator for MergeListIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


impl<T> Default for MergeList<T> {
    fn default() -> Self {
        Self { next_id: 1, groups: BTreeMap::default() }
    }
}

impl<T> MergeList<T> {
    /// Merge two groups, and return the new group id
    pub fn merge_group(&mut self, keep: u32, remove: u32) -> &mut Vec<T> {
        let rm = match self.groups.remove(&remove) {
            Some(s) => s,
            None => panic!("Failed to remove group {}", remove),
        };
        let up = match self.groups.get_mut(&keep) {
            Some(s) => s,
            None => panic!("Failed to change group {}", keep),
        };
        up.extend(rm);
        up
    }
    /// Replace all the id in the slice with new id
    pub fn insert(&mut self, key: u32, value: T) -> &mut Vec<T> {
        match self.groups.get_mut(&key) {
            Some(list) => {
                list.push(value);
                list
            }
            None => panic!("Group {} does not exists.", key),
        }
    }
    /// Replace all the id in the slice with new id
    pub fn create(&mut self, value: T) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.groups.insert(id, vec![value]);
        id
    }
    // pub fn insert_or_create(&mut self, key: u32, value: T) -> &mut Vec<T> {
    //     self.groups.entry(key).or_insert_with(|| vec![value])
    // }
}
