/// 1-D Buffer Signal
///

use std::vec::IntoIter;
use super::*;

#[repr(transparent)]
#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct Buffer<T: Unit> {
    data: Vec<T>
}

// std::iter implementations ==========================================

impl<T: Unit> IntoIterator for Buffer<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

// std::ops Implementations ===========================================

impl<T: Unit> Add for Buffer<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

// std::io Implementations ============================================

// Signal Trait Implementations =======================================

impl<T: Unit> Signal for Buffer<T> {
    type Sample = T;
    fn view(&self) -> &[Self::Sample] {
        self.data.as_slice()
    }
}

impl<T: Unit> SignalMut for Buffer<T> {
    fn view_mut(&mut self) -> &mut [Self::Sample] {
        self.data.as_mut_slice()
    }
}

impl<T: Unit> SignalOwned for Buffer<T> {
    type Container = Vec<T>;
    fn as_container(&self) -> &Self::Container {
        &self.data
    }
    fn as_container_mut(&mut self) -> &mut Self::Container {
        &mut self.data
    }
    fn into_container(self) -> Self::Container {
        self.data
    }
}

impl<T: Unit> SignalResizable for Buffer<T> {
    fn resize(&mut self, new_len: usize, fill_value: Self::Sample) {
        todo!()
    }
    fn clear(&mut self) {
        todo!()
    }
    fn append(&mut self, value: Self::Sample) {
        todo!()
    }
}

// impl<T: Unit> SignalStream for Buffer<T> {}
// impl<T: Unit> SignalOps for Buffer<T> {}