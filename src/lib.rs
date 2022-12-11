use std::fmt::Debug;
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
    }
}
