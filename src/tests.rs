
#[cfg(test)]
mod tests {
    use crate::matrix::IntoMatrixArrayData;


    #[test]
    fn a() {
        let m = [[0]].into_matrix();

        assert!(m[(0,0)]==0);
    }
}
