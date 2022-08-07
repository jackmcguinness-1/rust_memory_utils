mod util;

#[cfg(test)]
mod tests {
    use crate::util::byte_matrix::FlatByteMatrix32;

    #[test]
    fn it_works() {
        
        let mut mat = FlatByteMatrix32::new(4, 4);

        for i in 0..4{
            for j in 0..4{
                mat[i][j] = (i + 4*j) as u8;
            }
        }

        for i in 0..4{
            println!("{:?}", mat[i].iter());
        }

        println!("{:?}", mat.col_major_data);

    }
}
