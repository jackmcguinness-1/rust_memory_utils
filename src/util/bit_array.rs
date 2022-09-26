
#![allow(dead_code)]
#![allow(unused_parens)]

// I don't use any clamping here so use carefully

#[derive(Debug, Copy, Clone)]
pub struct BitArray8{
    data: u8,
}

const TOP_SET_8: u8 = 0b1 << 7;
const TOP_UNSET_8: u8 = (0b1 << 7) - 1;

impl BitArray8{

    pub fn new() -> Self{
        BitArray8{
            data: 0
        }
    }

    pub fn get(&self, idx: usize) -> u8 {
        ((self.data << idx) & TOP_SET_8) >> 7
    }

    pub fn set_one(&mut self, idx: usize){
        self.data |= (TOP_SET_8 >> idx);
    }

    pub fn set_zero(&mut self, idx: usize){
        self.data &= (TOP_UNSET_8 >> idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        self.data ^= (TOP_SET_8 >> idx);
    }

    pub fn popcnt(&self) -> usize {
        u8::count_ones(self.data) as usize
    }

    pub fn lzcnt(&self) -> usize {
        u8::leading_zeros(self.data) as usize
    }

    pub fn lscnt(&self) -> usize {
        u8::leading_ones(self.data) as usize
    }

}

//------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct BitArray16{
    data: u16,
}

const TOP_SET_16: u16 = 0b1 << 15;
const TOP_UNSET_16: u16 = (0b1 << 15) - 1;

impl BitArray16{

    pub fn new() -> Self{
        BitArray16{
            data: 0
        }
    }

    pub fn get(&self, idx: usize) -> u16 {
        ((self.data << idx) & TOP_SET_16) >> 15
    }

    pub fn set_one(&mut self, idx: usize){
        self.data |= (TOP_SET_16 >> idx);
    }

    pub fn set_zero(&mut self, idx: usize){
        self.data &= (TOP_UNSET_16 >> idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        self.data ^= (TOP_SET_16 >> idx);
    }

    pub fn popcnt(&self) -> usize {
        u16::count_ones(self.data) as usize
    }

    pub fn lzcnt(&self) -> usize {
        u16::leading_zeros(self.data) as usize
    }

    pub fn lscnt(&self) -> usize {
        u16::leading_ones(self.data) as usize
    }

}

//------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct BitArray32{
    data: u32,
}

const TOP_SET_32: u32 = 0b1 << 31;
const TOP_UNSET_32: u32 = (0b1 << 31) - 1;

impl BitArray32{

    pub fn new() -> Self{
        BitArray32{
            data: 0
        }
    }

    pub fn get(&self, idx: usize) -> u32 {
        ((self.data << idx) & TOP_SET_32) >> 31
    }

    pub fn set_one(&mut self, idx: usize){
        self.data |= (TOP_SET_32 >> idx);
    }

    pub fn set_zero(&mut self, idx: usize){
        self.data &= (TOP_UNSET_32 >> idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        self.data ^= (TOP_SET_32 >> idx);
    }

    pub fn popcnt(&self) -> usize {
        u32::count_ones(self.data) as usize
    }

    pub fn lzcnt(&self) -> usize {
        u32::leading_zeros(self.data) as usize
    }

    pub fn lscnt(&self) -> usize {
        u32::leading_ones(self.data) as usize
    }

}

//------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct BitArray64{
    data: u64,
}

const TOP_SET_64: u64 = 0b1 << 63;
const TOP_UNSET_64: u64 = (0b1 << 63) - 1;

impl BitArray64{

    pub fn new() -> Self{
        BitArray64{
            data: 0
        }
    }

    pub fn get(&self, idx: usize) -> u64 {
        ((self.data << idx) & TOP_SET_64) >> 63
    }

    pub fn set_one(&mut self, idx: usize){
        self.data |= (TOP_SET_64 >> idx);
    }

    pub fn set_zero(&mut self, idx: usize){
        self.data &= (TOP_UNSET_64 >> idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        self.data ^= (TOP_SET_64 >> idx);
    }

    pub fn popcnt(&self) -> usize {
        u64::count_ones(self.data) as usize
    }

    pub fn lzcnt(&self) -> usize {
        u64::leading_zeros(self.data) as usize
    }

    pub fn lscnt(&self) -> usize {
        u64::leading_ones(self.data) as usize
    }

}

//------------------------------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct BitArray128{
    data: u128,
}

const TOP_SET_128: u128 = 0b1 << 127;
const TOP_UNSET_128: u128 = (0b1 << 127) - 1;

impl BitArray128{

    pub fn new() -> Self{
        BitArray128{
            data: 0
        }
    }

    pub fn get(&self, idx: usize) -> u128 {
        ((self.data << idx) & TOP_SET_128) >> 127
    }

    pub fn set_one(&mut self, idx: usize){
        self.data |= (TOP_SET_128 >> idx);
    }

    pub fn set_zero(&mut self, idx: usize){
        self.data &= (TOP_UNSET_128 >> idx);
    }

    pub fn toggle(&mut self, idx: usize) {
        self.data ^= (TOP_SET_128 >> idx);
    }

    pub fn popcnt(&self) -> usize {
        u128::count_ones(self.data) as usize
    }

    pub fn lzcnt(&self) -> usize {
        u128::leading_zeros(self.data) as usize
    }

    pub fn lscnt(&self) -> usize {
        u128::leading_ones(self.data) as usize
    }

}