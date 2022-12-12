use std::fmt::Debug;
use std::ops::{Index, IndexMut};
use array_linked_list::ArrayLinkedList;

#[derive(Debug)]
pub struct LinkedSpacedList<T: Debug> {
    list: ArrayLinkedList<Entry<T>>,
    length: usize,
}

#[derive(Debug)]
pub(crate) struct Entry<T: Debug> {
    pub spacing: usize,
    pub value: T,
}

impl<T: Debug> Entry<T> {
    pub fn new(spacing: usize, value: T) -> Self {
        Self { spacing, value }
    }
}

impl<T: Debug> LinkedSpacedList<T> {
    pub fn new() -> Self {
        Self { list: ArrayLinkedList::new(), length: 0 }
    }

    pub fn push(&mut self, spacing: usize, value: T) -> usize {
        self.length += spacing;
        self.list.push_back(Entry::new(spacing, value))
    }

    pub fn insert_after(&mut self, position: usize, value: T) -> usize {
        if position >= self.length {
            return self.push(position - self.length, value);
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            index = index_;
            if sum + spacing > position {
                break;
            }
            sum += spacing;
        }
        self.list[index].as_mut().unwrap().spacing -= position - sum;
        self.list.insert_before(index, Entry::new(position - sum, value)).unwrap()
    }

    pub fn insert_before(&mut self, position: usize, value: T) -> usize {
        if position > self.length {
            return self.push(position - self.length, value);
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            index = index_;
            if sum + spacing >= position {
                break;
            }
            sum += spacing;
        }
        self.list[index].as_mut().unwrap().spacing -= position - sum;
        self.list.insert_before(index, Entry::new(position - sum, value)).unwrap()
    }

    pub fn remove(&mut self, index: usize) -> T {
        let next_index = self.list.indices_after(index).next();
        if let Some(next_index) = next_index {
            self.list[next_index].as_mut().unwrap().spacing += self.list[index].as_ref().unwrap().spacing;
        }
        self.list.remove(index).unwrap().value
    }

    pub fn inflate_after(&mut self, position: usize, spacing: usize) {
        if self.list.is_empty() {
            panic!("cannot inflate empty list");
        }
        if position >= self.length {
            return;
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            sum += spacing;
            if sum > position {
                index = index_;
                break;
            }
        }
        self.list[index].as_mut().unwrap().spacing += spacing;
        self.length += spacing;
    }

    pub fn deflate_after(&mut self, position: usize, spacing: usize) {
        if self.list.is_empty() {
            panic!("cannot deflate empty list");
        }
        if position >= self.length {
            return;
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            sum += spacing;
            if sum > position {
                index = index_;
                break;
            }
        }
        let spacing_ = &mut self.list[index].as_mut().unwrap().spacing;
        *spacing_ = spacing_.checked_sub(spacing).expect("cannot deflate below zero");
        self.length -= spacing;
    }

    pub fn inflate_before(&mut self, position: usize, spacing: usize) {
        if self.list.is_empty() {
            panic!("cannot inflate empty list");
        }
        if position == 0 || position > self.length {
            return;
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            sum += spacing;
            if sum >= position {
                index = index_;
                break;
            }
        }
        self.list[index].as_mut().unwrap().spacing += spacing;
        self.length += spacing;
    }

    pub fn deflate_before(&mut self, position: usize, spacing: usize) {
        if self.list.is_empty() {
            panic!("cannot deflate empty list");
        }
        if position == 0 || position > self.length {
            return;
        }
        let mut index = 0;
        let mut sum = 0;
        for (index_, &Entry { spacing, .. }) in self.list.indexed() {
            sum += spacing;
            if sum >= position {
                index = index_;
                break;
            }
        }
        let spacing_ = &mut self.list[index].as_mut().unwrap().spacing;
        *spacing_ = spacing_.checked_sub(spacing).expect("cannot deflate below zero");
        self.length -= spacing;
    }
}

impl<T: Debug> Index<usize> for LinkedSpacedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.list[index].as_ref().unwrap().value
    }
}

impl<T: Debug> IndexMut<usize> for LinkedSpacedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.list[index].as_mut().unwrap().value
    }
}

#[derive(Debug)]
pub struct LinkedRangeSpacedList<T: Debug> {
    list: LinkedSpacedList<Bound<T>>
}

#[derive(Debug)]
enum Bound<T: Debug> {
    Start { end: usize, value: T },
    End { start: usize }
}

impl<T: Debug> LinkedRangeSpacedList<T> {
    pub fn new() -> Self {
        Self { list: LinkedSpacedList::new() }
    }

    pub fn push(&mut self, spacing: usize, length: usize, value: T) -> (usize, usize) {
        let start = self.list.push(spacing, Bound::Start { end: 0, value });
        let end = self.list.push(length, Bound::End { start });
        if let Bound::Start { end: end_, .. } = &mut self.list[start] {
            *end_ = end;
        } else {
            unreachable!();
        }
        (start, end)
    }

    pub fn insert_after(&mut self, start: usize, end: usize, value: T) -> (usize, usize) {
        assert!(start <= end, "start position must be before or at end position");
        if start >= self.list.length {
            return self.push(start - self.list.length, end - start, value);
        }
        let start = self.list.insert_after(start, Bound::Start { end: 0, value });
        let end = self.list.insert_after(end, Bound::End { start });
        if let Bound::Start { end: end_, .. } = &mut self.list[start] {
            *end_ = end;
        } else {
            unreachable!();
        }
        (start, end)
    }

    pub fn insert_before(&mut self, start: usize, end: usize, value: T) -> (usize, usize) {
        assert!(start <= end, "start position must be before or at end position");
        if start > self.list.length {
            return self.push(start - self.list.length, end - start, value);
        }
        let start = self.list.insert_before(start, Bound::Start { end: 0, value });
        let end = self.list.insert_before(end, Bound::End { start });
        if let Bound::Start { end: end_, .. } = &mut self.list[start] {
            *end_ = end;
        } else {
            unreachable!();
        }
        (start, end)
    }

    pub fn remove(&mut self, index: usize) -> T {
        match self.list.remove(index) {
            Bound::Start { end, value } => {
                self.list.remove(end);
                value
            }
            Bound::End { start } =>
                if let Bound::Start { value, .. } = self.list.remove(start) {
                    value
                } else {
                    unreachable!()
                }
        }
    }

    pub fn inflate_after(&mut self, position: usize, spacing: usize) {
        self.list.inflate_after(position, spacing)
    }

    pub fn deflate_after(&mut self, position: usize, spacing: usize) {
        self.list.deflate_after(position, spacing)
    }

    pub fn inflate_before(&mut self, position: usize, spacing: usize) {
        self.list.inflate_before(position, spacing)
    }

    pub fn deflate_before(&mut self, position: usize, spacing: usize) {
        self.list.deflate_before(position, spacing)
    }
}

#[cfg(test)]
#[allow(unused)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedSpacedList::new();
        let index_a = list.push(20, 'a');
        let index_b = list.push(5, 'b');
        let index_c = list.insert_after(12, 'c');

        for entry in list.list.iter() {
            println!("{:?}", entry);
        }
        println!();

        list.inflate_after(16, 6);

        for entry in list.list.iter() {
            println!("{:?}", entry);
        }
        println!();

        list.remove(index_a);

        for entry in list.list.iter() {
            println!("{:?}", entry);
        }
        println!();
    }

    #[test]
    fn it_works_with_ranges() {
        let mut list = LinkedRangeSpacedList::new();
        let index_a = list.push(10, 4, 'a');
        let index_b = list.push(2, 5, 'b');
        let index_c = list.insert_after(9, 12, 'c');

        for (index, entry) in list.list.list.indexed() {
            println!("{} {:?}", index, entry);
        }
        println!();

        list.inflate_after(16, 6);

        for (index, entry) in list.list.list.indexed() {
            println!("{} {:?}", index, entry);
        }
        println!();

        list.remove(index_a.0);

        for (index, entry) in list.list.list.indexed() {
            println!("{} {:?}", index, entry);
        }
    }
}
