use std::{ops::Range, rc::Rc, slice::SliceIndex};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct RcSlice<T> {
    rc: Rc<Box<[T]>>,
    range: Range<usize>
}

impl<T: Sized> RcSlice<T> {
    pub fn new(value: [T]) -> Self where [T]: Sized {
        let rc = Rc::new(Box::new(value));
        Self {
            range: 0..rc.len(),
            rc
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        Self {
            range: 0..v.len(),
            rc: Rc::new(v.into_boxed_slice()),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.rc[self.range.clone()]
    }

    pub fn ref_slice(&self, range: Range<usize>) -> Self {
        Self {
            rc: Rc::clone(&self.rc),
            range
        }
    }

} 