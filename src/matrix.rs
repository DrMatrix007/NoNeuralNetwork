use std::{marker::PhantomData, ops::{Index, IndexMut}, usize};

pub trait MatrixData<T,const M:usize,const N: usize>: Index<(usize,usize),Output = T>+IndexMut<(usize,usize)> {
}

pub struct Matrix<T,const M:usize,const N:usize,Data:MatrixData<T,M,N>>{
    data:Data,
    marker: PhantomData<T>
}

impl<T, const M:usize, const N:usize, Data: MatrixData<T,M,N>> From<Data> for Matrix<T, M, N, Data> {
    fn from(value: Data) -> Self {
        Self {
            data:value,
            marker: PhantomData
        }
    }
}

impl<T, const M:usize, const N:usize, Data: MatrixData<T,M,N>> Index<(usize,usize)> for Matrix<T, M, N, Data> {
    type Output = T;

    fn index(&self, index: (usize,usize)) -> &Self::Output {
        &self.data[index]
    }
}
impl<T, const M:usize, const N:usize, Data: MatrixData<T,M,N>> IndexMut<(usize,usize)> for Matrix<T, M, N, Data> {

    fn index_mut(&mut self, index: (usize,usize)) -> &mut Self::Output {
        &mut self.data[index]
    }
}
pub struct MatrixArrayData<T,const M:usize,const N:usize>(pub [[T;N];M]);

impl<T,const M:usize,const N:usize> MatrixData<T,M,N> for MatrixArrayData<T,M,N>
{
}

impl<T,const M:usize,const N:usize> Index<(usize,usize)> for MatrixArrayData<T,M,N>
{
    type Output = T;

    fn index(&self, (a,b): (usize,usize)) -> &Self::Output {
        &self.0[a][b]
    }
}

impl<T,const M:usize,const N:usize> IndexMut<(usize,usize)> for MatrixArrayData<T,M,N>
{

    fn index_mut(&mut self, (a,b): (usize,usize)) -> &mut Self::Output {
        &mut self.0[a][b]
    }
}

pub trait IntoMatrix<T,const M:usize,const N:usize>{
    type Data:MatrixData<T,M,N>; 
    fn into_matrix(self) -> Matrix<T,M,N,Self::Data> ;
}

impl<T,const M:usize,const N:usize,Data:MatrixData<T,M,N>> IntoMatrix<T,M,N> for Data {
    type Data = Self;
    fn into_matrix(self) -> Matrix<T,M,N,Self> where Self:Sized {
        self.into()
    }
}

pub trait IntoMatrixArrayData<T,const M:usize,const N:usize> {
    fn into_matrix(self) -> Matrix<T,M,N,MatrixArrayData<T,M,N>>;
}

impl<T,const M:usize,const N:usize> IntoMatrixArrayData<T,M,N> for [[T;N];M]{
    fn into_matrix(self) -> Matrix<T,M,N,MatrixArrayData<T,M,N>> {
        MatrixArrayData(self).into()
    }
}
