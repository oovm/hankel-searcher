use super::*;

/// A list of groups, each group has a unique id
#[derive(Clone, Debug)]
pub struct MergeList<T> {
    next_id: u32,
    groups: BTreeMap<u32, Vec<T>>,
}

impl<'i, T> IntoIterator for &'i MergeList<T> {
    type Item = (usize, &'i [T]);
    type IntoIter = MergeListView<'i, T>;

    fn into_iter(self) -> Self::IntoIter {
        MergeListView { index: 0, view: self }
    }
}

/// A view of the merge list
pub struct MergeListView<'i, T> {
    index: usize,
    view: &'i MergeList<T>,
}

impl<'i, T> Debug for MergeListView<'i, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'i, T> Iterator for MergeListView<'i, T> {
    type Item = (usize, &'i [T]);

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.index;
        self.index += 1;
        let (_, value) = self.view.groups.range(..).nth(id)?;
        Some((id, value))
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
