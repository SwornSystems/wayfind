use alloc::vec;
use alloc::vec::Vec;

/// A growable array with inline storage.
#[derive(Debug)]
pub(crate) enum Storage<T, const N: usize> {
    Empty,
    Inline([T; N], usize),
    Heap(Vec<T>),
}

impl<T: Copy, const N: usize> Storage<T, N> {
    pub(crate) const fn new() -> Self {
        Self::Empty
    }

    pub(crate) fn get(&self, index: usize) -> Option<&T> {
        match self {
            Self::Empty => None,
            Self::Inline(items, length) => {
                if index < *length {
                    items.get(index)
                } else {
                    None
                }
            },
            Self::Heap(items) => items.get(index),
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self {
            Self::Empty => None,
            Self::Inline(items, _) => items.get_mut(index),
            Self::Heap(items) => items.get_mut(index),
        }
    }

    pub(crate) fn slot(&mut self, index: usize, fill: T) -> Option<&mut T> {
        let ready = match self {
            Self::Empty => false,
            Self::Inline(..) => index < N,
            Self::Heap(items) => index < items.len(),
        };

        if ready {
            self.get_mut(index)
        } else {
            self.grow(index, fill)
        }
    }

    #[cold]
    fn grow(&mut self, index: usize, fill: T) -> Option<&mut T> {
        match self {
            Self::Empty => {
                if index < N {
                    *self = Self::Inline([fill; N], N);
                } else {
                    *self = Self::Heap(vec![fill; index + 1]);
                }
            },
            Self::Inline(items, _) => {
                let mut spilled = Vec::with_capacity(index + 1);
                spilled.extend_from_slice(items);
                spilled.resize(index + 1, fill);
                *self = Self::Heap(spilled);
            },
            Self::Heap(items) => {
                items.resize(index + 1, fill);
            },
        }

        self.get_mut(index)
    }

    pub(crate) fn push(&mut self, item: T) {
        if N > 0 && matches!(self, Self::Empty) {
            *self = Self::Inline([item; N], 1);
            return;
        }

        if let Self::Inline(items, length) = self {
            if let Some(slot) = items.get_mut(*length) {
                *slot = item;
                *length += 1;
                return;
            }
        }

        self.push_grow(item);
    }

    #[cold]
    fn push_grow(&mut self, item: T) {
        match self {
            Self::Empty => *self = Self::Heap(vec![item]),
            Self::Inline(items, _) => {
                let mut spilled = Vec::with_capacity(N * 2);
                spilled.extend_from_slice(items);
                spilled.push(item);
                *self = Self::Heap(spilled);
            },
            Self::Heap(items) => items.push(item),
        }
    }

    pub(crate) fn pop(&mut self) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::Inline(items, length) => {
                *length = length.checked_sub(1)?;
                items.get(*length).copied()
            },
            Self::Heap(items) => items.pop(),
        }
    }

    pub(crate) fn as_slice(&self) -> &[T] {
        match self {
            Self::Empty => &[],
            Self::Inline(items, length) => items.get(..*length).unwrap_or(&[]),
            Self::Heap(items) => items,
        }
    }
}

#[cfg(test)]
mod tests {
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn empty() {
        let mut storage: Storage<u32, 2> = Storage::new();

        assert_eq!(storage.get(0), None);
        assert_eq!(storage.get_mut(0), None);
        assert_eq!(storage.pop(), None);
        assert!(storage.as_slice().is_empty());
    }

    #[test]
    fn inline() {
        let mut storage: Storage<u32, 2> = Storage::new();
        storage.push(1);
        storage.push(2);

        assert_eq!(storage.as_slice(), &[1, 2]);
        assert_eq!(storage.get(1), Some(&2));
        assert_eq!(storage.get(2), None);

        assert_eq!(storage.pop(), Some(2));
        assert_eq!(storage.get(1), None);
        assert_eq!(storage.pop(), Some(1));
        assert_eq!(storage.pop(), None);
        assert!(storage.as_slice().is_empty());
    }

    #[test]
    fn heap() {
        let mut storage: Storage<u32, 2> = Storage::new();
        storage.push(1);
        storage.push(2);
        storage.push(3);

        assert_eq!(storage.as_slice(), &[1, 2, 3]);
        assert_eq!(storage.get(2), Some(&3));

        storage.push(4);
        assert_eq!(storage.pop(), Some(4));
    }

    #[test]
    fn zero() {
        let mut storage: Storage<u32, 0> = Storage::new();
        storage.push(1);

        assert_eq!(storage.as_slice(), &[1]);
    }

    #[test]
    fn slots() {
        let mut storage: Storage<u32, 2> = Storage::new();
        *storage.slot(1, 9).unwrap() = 5;
        assert_eq!(storage.get(0), Some(&9));
        assert_eq!(storage.get(1), Some(&5));

        *storage.slot(0, 9).unwrap() = 4;
        assert_eq!(storage.get(0), Some(&4));

        *storage.slot(4, 9).unwrap() = 6;
        assert_eq!(storage.get(3), Some(&9));
        assert_eq!(storage.get(4), Some(&6));

        *storage.slot(7, 9).unwrap() = 7;
        *storage.slot(5, 9).unwrap() = 1;
        assert_eq!(storage.get(7), Some(&7));

        let mut storage: Storage<u32, 2> = Storage::new();
        *storage.slot(5, 9).unwrap() = 5;
        assert_eq!(storage.get(5), Some(&5));
        assert_eq!(storage.get(0), Some(&9));
    }
}
