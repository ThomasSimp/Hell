/// Represents a two-dimensional matrix with `f64` elements.
///
/// # Fields
/// - `rows`: The number of rows in the matrix.
/// - `cols`: The number of columns in the matrix.
/// - `data`: A `Vec<Vec<f64>>` holding the matrix elements. The outer `Vec` represents rows, while the inner `Vec` represents columns within each row.
///
/// # Examples
///
/// Creating a new matrix:
/// ```
/// use hell::Matrix;
///
/// let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let matrix = Matrix::new(2, 2, data);
/// ```
///
/// Creating an identity matrix:
/// ```
/// use hell::Matrix;
///
/// let identity_matrix = Matrix::identity(3);
/// ```
///
/// Transposing a matrix:
/// ```
/// use hell::Matrix;
///
/// let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let matrix = Matrix::new(2, 2, data);
/// let transposed_matrix = matrix.transpose();
/// ```
///
/// Adding two matrices:
/// ```
/// use hell::Matrix;
///
/// let data1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let data2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
/// let matrix1 = Matrix::new(2, 2, data1);
/// let matrix2 = Matrix::new(2, 2, data2);
/// let result = matrix1.add(&matrix2).unwrap();
/// ```
///
/// Multiplying two matrices:
/// ```
/// use hell::Matrix;
///
/// let data1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
/// let data2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
/// let matrix1 = Matrix::new(2, 2, data1);
/// let matrix2 = Matrix::new(2, 2, data2);
/// let result = matrix1.multiply(&matrix2).unwrap();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    /// Creates a new `Matrix` instance with the specified number of rows and columns, and initializes it with the given data.
    ///
    /// # Arguments
    /// - `rows`: The number of rows in the matrix.
    /// - `cols`: The number of columns in the matrix.
    /// - `data`: A `Vec<Vec<f64>>` where each inner `Vec` represents a row of the matrix.
    ///
    /// # Panics
    /// Panics if `rows` does not match the number of rows in `data`, or if any row in `data` does not have exactly `cols` elements.
    ///
    /// # Examples
    /// ```
    /// use hell::Matrix;
    ///
    /// let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix = Matrix::new(2, 2, data);
    /// ```
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        assert_eq!(rows, data.len(), "Number of rows does not match data length.");
        assert!(data.iter().all(|row| row.len() == cols), "Not all rows have the same number of columns.");
        Matrix { rows, cols, data }
    }

    /// Creates an identity matrix of the given size.
    ///
    /// An identity matrix is a square matrix with ones on the diagonal and zeros elsewhere.
    ///
    /// # Arguments
    /// - `size`: The size of the identity matrix (i.e., the number of rows and columns).
    ///
    /// # Examples
    /// ```
    /// use hell::Matrix;
    ///
    /// let identity_matrix = Matrix::identity(3);
    /// ```
    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix { rows: size, cols: size, data }
    }

    /// Transposes the matrix.
    ///
    /// The transpose of a matrix is obtained by swapping rows and columns.
    ///
    /// # Examples
    /// ```
    /// use hell::Matrix;
    ///
    /// let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let matrix = Matrix::new(2, 2, data);
    /// let transposed_matrix = matrix.transpose();
    /// ```
    pub fn transpose(&self) -> Self {
        let mut transposed = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed[j][i] = self.data[i][j];
            }
        }
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: transposed,
        }
    }

    /// Adds two matrices.
    ///
    /// The matrices must have the same dimensions for addition.
    ///
    /// # Arguments
    /// - `other`: The matrix to add to the current matrix.
    ///
    /// # Returns
    /// - `Ok(Matrix)`: The result of the addition if dimensions match.
    /// - `Err(&'static str)`: An error message if the dimensions do not match.
    ///
    /// # Examples
    /// ```
    /// use hell::Matrix;
    ///
    /// let data1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let data2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    /// let matrix1 = Matrix::new(2, 2, data1);
    /// let matrix2 = Matrix::new(2, 2, data2);
    /// let result = matrix1.add(&matrix2).unwrap();
    /// ```
    pub fn add(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrices dimensions do not match for addition.");
        }
        let mut result = self.data.clone();
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] += other.data[i][j];
            }
        }
        Ok(Matrix::new(self.rows, self.cols, result))
    }

    /// Multiplies two matrices.
    ///
    /// The number of columns in the first matrix must equal the number of rows in the second matrix.
    ///
    /// # Arguments
    /// - `other`: The matrix to multiply with the current matrix.
    ///
    /// # Returns
    /// - `Ok(Matrix)`: The result of the multiplication if dimensions are compatible.
    /// - `Err(&'static str)`: An error message if the dimensions are not compatible.
    ///
    /// # Examples
    /// ```
    /// use hell::Matrix;
    ///
    /// let data1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    /// let data2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
    /// let matrix1 = Matrix::new(2, 2, data1);
    /// let matrix2 = Matrix::new(2, 2, data2);
    /// let result = matrix1.multiply(&matrix2).unwrap();
    /// ```
    pub fn multiply(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("Matrices dimensions do not match for multiplication.");
        }
        let mut result = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Ok(Matrix::new(self.rows, other.cols, result))
    }
}
