use std::slice::IterMut;

use crate::common::num::{
    irect::IRect,
    ivec2::{i2, IVec2},
};

pub struct Arr2D<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Arr2D<T> {
    pub fn empty() -> Self {
        Self { data: vec![], width: 0 }
    }

    pub fn new(data: Vec<T>, width: usize) -> Self {
        assert!(width > 0);
        Self { data, width }
    }

    pub fn get(&self, pos: IVec2) -> Option<&T> {
        if !self.bounds().contains(pos) {
            None
        } else {
            let i = self.to_idx(pos);
            self.get_i(i)
        }
    }

    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut T> {
        if !self.bounds().contains(pos) {
            None
        } else {
            let i = self.to_idx(pos);
            self.get_i_mut(i)
        }
    }

    pub fn get_i(&self, idx: usize) -> Option<&T> {
        if idx < self.data.len() {
            Some(&self.data[idx])
        } else {
            None
        }
    }

    pub fn get_i_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx < self.data.len() {
            Some(&mut self.data[idx])
        } else {
            None
        }
    }

    pub fn set(&mut self, pos: IVec2, value: T) -> bool {
        if self.bounds().contains(pos) {
            let idx = self.to_idx(pos);
            self.data[idx] = value;
            true
        } else {
            false
        }
    }

    pub fn set_i(&mut self, idx: usize, value: T) -> bool {
        if idx < self.data.len() {
            self.data[idx] = value;
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> IVec2 {
        let last_idx = self.count().max(1) - 1;
        let height = self.to_pos(last_idx).y + 1;

        i2(self.width as i32, height as i32)
    }

    pub fn count(&self) -> usize {
        self.data.len()
    }

    pub fn bounds(&self) -> IRect {
        IRect::of_size(self.size())
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn swap(&mut self, pos_1: IVec2, pos_2: IVec2) {
        let idx_1 = self.to_idx(pos_1);
        let idx_2 = self.to_idx(pos_2);

        self.data.swap(idx_1, idx_2);
    }

    pub fn to_pos(&self, idx: usize) -> IVec2 {
        if self.width == 0 {
            return IVec2::ZERO;
        }
        let x = (idx % self.width) as i32;
        let y = (idx / self.width) as i32;
        i2(x, y)
    }

    pub fn to_idx(&self, pos: IVec2) -> usize {
        if self.width == 0 {
            return 0;
        }
        let x = pos.x as usize;
        let y = pos.y as usize;
        y * self.width + x
    }

    pub fn iter(&self) -> Arr2DIter<'_, T> {
        Arr2DIter {
            arr2d: self,
            curr_pos: i2(0, 0),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T: Clone> Arr2D<T> {
    pub fn default(default_value: T, size: IVec2) -> Self {
        let data = vec![default_value; size.product() as usize];
        Self::new(data, size.x as usize)
    }
}

impl<T> Arr2D<Option<T>> {
    pub fn default_optional(size: IVec2) -> Self {
        let mut data = vec![];
        for _ in 0..size.product() {
            data.push(None);
        }
        Self::new(data, size.x as usize)
    }
}

impl<T: Copy> Arr2D<T> {
    pub fn copy_area(&self, area: IRect) -> Arr2D<T> {
        if let Some(area) = self.bounds().intersection(area) {
            let default = *self.get(area.pos).unwrap();
            let mut copy = Arr2D::default(default, area.size);
            for dst_pos in copy.bounds().iter() {
                let src_pos = area.pos + dst_pos;
                let value = *self.get(src_pos).unwrap();
                copy.set(dst_pos, value);
            }

            copy
        } else {
            Self::new(vec![], 0)
        }
    }

    pub fn copy_from(&mut self, other: &Arr2D<T>, src: IRect, dst_pos: IVec2) {
        for src_pos in src.iter() {
            let offset = src_pos - src.pos;
            let dst_pos = dst_pos + offset;

            if let Some(value) = other.get(src_pos) {
                self.set(dst_pos, *value);
            }
        }
    }

    pub fn set_area(&mut self, origin: IVec2, other: Arr2D<T>) {
        for src_pos in other.bounds().iter() {
            let value = other.get(src_pos).unwrap();
            let dst_pos = src_pos + origin;
            self.set(dst_pos, *value);
        }
    }
}

pub struct Arr2DIter<'a, T> {
    arr2d: &'a Arr2D<T>,
    curr_pos: IVec2,
}

impl<'a, T> Iterator for Arr2DIter<'a, T> {
    type Item = (IVec2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let curr_pos = self.curr_pos;
        let idx = self.arr2d.to_idx(curr_pos);

        self.curr_pos = self.arr2d.to_pos(idx + 1);

        if let Some(value) = self.arr2d.get_i(idx) {
            Some((curr_pos, value))
        } else {
            None
        }
    }
}

impl<T> Clone for Arr2D<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            width: self.width,
        }
    }
}
