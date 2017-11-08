/*#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[0, 0, 0, 0, 0, 0]);
    assert_eq!(x + y, x);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
    }
}*/
use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}
impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        Matrix{
            data:values.to_vec(),
            row:row,
            col:col,
        }
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let mut v: Vec<T> = Vec::new();
        Matrix{
            data:v,
            row:row,
            col:col,
        }
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row,self.col)
    }
}

impl<T: ops::Add<Output = T> + Copy + Default> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!();
        }
        else{
                let mut v: Vec<T> = Vec::new();
                for i in 0..self.data.len(){
                    let mut temp:T=Default::default();
                    temp=self.data[i]+rhs.data[i];
                    v.push(temp);
                }
            Matrix {
                data:v,
                row:self.row,
                col:self.col,
            }
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy + Default> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!();
        }
        else{
            let mut v: Vec<T> = Vec::new();
            for i in 0..self.data.len(){
                let mut temp:T=Default::default();
                temp=self.data[i]-rhs.data[i];
                v.push(temp);
            }
            Matrix {data:v,
            row:self.row,
            col:self.col,
            }
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row {
            panic!();
        }
        else{
            let mut v: Vec<T> = Vec::new();
            let x=self.row;
            let y=self.col;
            for i in 0..x{
                for j in 0..y{
                    let mut temp:T=Default::default();
                    for k in 0..y{
                        temp=temp+self.data[i*y+k]*rhs.data[k*y+j];
                    }
                        v.push(temp);
                    }
            }
            Matrix {data:v,
            row:self.row,
            col:self.col,
            }
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x=self.row;
        let y=self.col;
        let mut s = String::new();
        for i in 0..x{
            let mut v = String::new();
            for j in 0..y{
                let element = &self.data[i*y+j];
                if j!=y-1{
                v.push_str(&element.to_string());
                v.push_str(" ");
            }
            else {
                v.push_str(&element.to_string());
            }
            }
            if i != x-1{
            s=format!("{}\n",v);
        }else {
            s=format!("{}",v);
        }
        }
        write!(f,"{}",s)
    }
}
