#![allow(dead_code)]

// stored in column major form only, as not intended to be used for large memory spans
// where many cache misses are likely, just to avoid indirection from nesting Vec<Vec<T>>

use std::{ops::Index};

pub struct FlatMatrix<T>
{
    pub rows: usize,
    pub columns: usize,
    pub data: Vec<T>
}

impl<T> FlatMatrix<T>
where T: Default + Clone
{

    pub fn new(columns: usize, rows: usize) -> Self{
        let data = vec![T::default(); rows * columns];
        FlatMatrix{
            rows,
            columns,
            data
        }
    }

    pub fn get(&self, col: usize, row: usize) -> &T{
        &self.data[col * self.rows + row]
    }

    pub fn set(&mut self, col: usize, row: usize, val: T){
        self.data[(col * self.rows) + row] = val;
    }

}

impl<T> Index<usize> for FlatMatrix<T>
{

    type Output = [T];

    fn index(&self, col_idx: usize) -> &Self::Output{

        let data_start_ptr = &self.data[col_idx * self.rows] as *const T;
        let data_slice = unsafe {std::slice::from_raw_parts(data_start_ptr, self.rows)};

        data_slice
    }

}

impl<T> IndexMut<usize> for FlatMatrix<T>
{

fn index_mut(&mut self, col_idx: usize) -> &mut Self::Output{

        let data_start_ptr = &mut self.data[col_idx * self.rows] as *mut T;
        let data_slice = unsafe {std::slice::from_raw_parts_mut(data_start_ptr, self.rows)};

        data_slice
    }
    
}

#[test]
fn test_flatmat(){

    let mut m = FlatMatrix::new(4, 4);

    m.set(0, 0, 1);
    assert_eq!(*m.get(0,0), 1);

    m.set(2, 3, 5);
    assert_eq!(*m.get(2, 3), 5);
}