use std::{
    fmt::Debug,
    ops::{
        Index,
        IndexMut,
        Deref,
        DerefMut
    }
};


#[derive(Debug)]
pub struct Matrix<T, const ROWS: usize, const COLS: usize> {
    content: [[T; COLS]; ROWS],
}

#[macro_export]
macro_rules! mtrx {
    ($([$($elem:expr),* $(,)?]),* $(,)?) => {{
        Matrix::from([
            $([$($elem),*]),*
        ])
    }};
    
    ($($elem:expr),* $(,)?) => {{
        Matrix::from([$($elem),*])
    }};
}


impl<T, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    pub fn new() -> Self
    where T: Default + Copy {
        Self { content: [[T::default(); COLS]; ROWS] }
    }

    pub fn into_inner(self) -> [[T; COLS]; ROWS] {
        self.content
    }


    pub fn t(&self) -> Matrix<T, COLS, ROWS>
    where T: Default + Copy {
        let mut content: [[T; ROWS]; COLS] = [[T::default(); ROWS]; COLS];

        for x in 0..ROWS {
            for y in 0..COLS {
                content[y][x] = self.content[x][y];
            }
        }

        Matrix { content }
    }


    pub fn prnt(mtrx: &Matrix<T, ROWS, COLS>)
    where T: Debug {
        println!("[");

        for y in 0..(ROWS - 1) {
            println!("  {:?},", mtrx.content[y]);
        }

        println!("  {:?}", mtrx.content[ROWS - 1]);

        println!("]");
    }
}


impl<T, const ROWS: usize, const COLS: usize> From<[T; COLS]>
for Matrix<T, ROWS, COLS> where T: Copy {
    fn from(array: [T; COLS]) -> Self {
        Matrix { content: [array; ROWS] }
    }
}

impl<T, const ROWS: usize, const COLS: usize> From<[[T; COLS]; ROWS]>
for Matrix<T, ROWS, COLS> {
    fn from(array: [[T; COLS]; ROWS]) -> Self {
        Matrix { content: array }
    }
}


impl<T, const ROWS: usize, const COLS: usize> Deref
for Matrix<T, ROWS, COLS> {
    type Target = [[T; COLS]; ROWS];

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T, const ROWS: usize, const COLS: usize> DerefMut
for Matrix<T, ROWS, COLS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}


impl<T, const ROWS: usize, const COLS: usize> Index<usize>
for Matrix<T, ROWS, COLS> {
    type Output = [T; COLS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.content[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> Index<(usize, usize)>
for Matrix<T, ROWS, COLS> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.content[index.0][index.1]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<usize>
for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.content[index]
    }
}

impl<T, const ROWS: usize, const COLS: usize> IndexMut<(usize, usize)>
for Matrix<T, ROWS, COLS> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.content[index.0][index.1]
    }
}


impl<T, const ROWS: usize, const COLS: usize> Into<[[T; COLS]; ROWS]>
for Matrix<T, ROWS, COLS> where T: Copy {
    fn into(self) -> [[T; COLS]; ROWS] {
        self.content
    }
}

