use std::{ops::{Index, IndexMut}};


///<summary>
///Interface to access as a 2D array but treated in memory as a contiguous allocation
///Contains a maximum of 16 bytes : 128 bits
///</summary>
pub struct FlatByteMatrix16{
    pub row_count: usize,
    pub column_count: usize,

    pub col_major_data: [u8; 16],
}
impl FlatByteMatrix16
{
    
    pub fn new(row_count: usize, column_count: usize) -> Self{
        // this implementation dodges having to use copy
        FlatByteMatrix16{
            row_count,
            column_count,
            col_major_data: [0; 16]
        }
    }
}

impl Index<usize> for FlatByteMatrix16{
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &self.col_major_data[start_idx..end_idx]
    }

}

impl<'a> IndexMut<usize> for FlatByteMatrix16{
    
    fn index_mut(&mut self, idx: usize) -> &mut [u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &mut self.col_major_data[start_idx..end_idx]
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

///<summary>
///Interface to access as a 2D array but treated in memory as a contiguous allocation
///Contains a maximum of 32 bytes : 256 bits
///</summary>
pub struct FlatByteMatrix32{
    pub row_count: usize,
    pub column_count: usize,

    pub col_major_data: [u8; 32],
}
impl FlatByteMatrix32
{
    
    pub fn new(row_count: usize, column_count: usize) -> Self{
        // this implementation dodges having to use copy
        FlatByteMatrix32{
            row_count,
            column_count,
            col_major_data: [0; 32]
        }
    }
}

impl Index<usize> for FlatByteMatrix32{
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &self.col_major_data[start_idx..end_idx]
    }

}

impl<'a> IndexMut<usize> for FlatByteMatrix32{
    
    fn index_mut(&mut self, idx: usize) -> &mut [u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &mut self.col_major_data[start_idx..end_idx]
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

///<summary>
///Interface to access as a 2D array but treated in memory as a contiguous allocation
///Contains a maximum of 64 bytes : 512 bits
///</summary>
pub struct FlatByteMatrix64{
    pub row_count: usize,
    pub column_count: usize,

    pub col_major_data: [u8; 64],
}
impl FlatByteMatrix64
{
    
    pub fn new(row_count: usize, column_count: usize) -> Self{
        // this implementation dodges having to use copy
        FlatByteMatrix64{
            row_count,
            column_count,
            col_major_data: [0; 64]
        }
    }
}

impl Index<usize> for FlatByteMatrix64{
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &self.col_major_data[start_idx..end_idx]
    }

}

impl<'a> IndexMut<usize> for FlatByteMatrix64{
    
    fn index_mut(&mut self, idx: usize) -> &mut [u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &mut self.col_major_data[start_idx..end_idx]
    }
}

//-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

///<summary>
///Interface to access as a 2D array but treated in memory as a contiguous allocation
///Contains a maximum of 128 bytes : 1024 bits
///</summary>
pub struct FlatByteMatrix128{
    pub row_count: usize,
    pub column_count: usize,

    pub col_major_data: [u8; 128],
}
impl FlatByteMatrix128
{
    
    pub fn new(row_count: usize, column_count: usize) -> Self{
        // this implementation dodges having to use copy
        FlatByteMatrix128{
            row_count,
            column_count,
            col_major_data: [0; 128]
        }
    }
}

impl Index<usize> for FlatByteMatrix128{
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &self.col_major_data[start_idx..end_idx]
    }

}

impl<'a> IndexMut<usize> for FlatByteMatrix128{
    
    fn index_mut(&mut self, idx: usize) -> &mut [u8]{
        let start_idx = self.row_count * idx;
        let end_idx = start_idx + self.row_count;
        &mut self.col_major_data[start_idx..end_idx]
    }
}