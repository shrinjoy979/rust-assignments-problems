/*
  Problem 34: Matrix Transpose

  Write a function that takes a Vec<Vec<i32>> representing a matrix and returns its transpose.
  The transpose of a matrix swaps rows and columns. Assume the input is a valid rectangular
  matrix (all rows have the same length). Return an empty vec for empty input.

  Run the tests for this problem with:
    cargo test --test matrix_transpose_test
*/

pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  if matrix.is_empty() {
    return matrix;
  }

  let mut results = vec![];

  for j in 0..matrix[0].len() {
    let mut row = vec![];

    for i in 0..matrix.len() {
      row.push(matrix[i][j]); // 0,0 | 0,1 | 0,2 | 1,0 | 1,1 | 1,2
    }

    results.push(row);
  }

  return results;
}
