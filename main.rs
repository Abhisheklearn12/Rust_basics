fn main() {
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let matrix_b = vec![
        vec![9, 8, 7],
        vec![6, 5, 4],
        vec![3, 2, 1],
    ];

    match matrix_multiply(&matrix_a, &matrix_b) {
        Some(result) => {
            println!("Matrix A:");
            print_matrix(&matrix_a);
            println!("Matrix B:");
            print_matrix(&matrix_b);
            println!("Result of Matrix A * Matrix B:");
            print_matrix(&result);
        }
        None => println!("Matrix multiplication failed: incompatible dimensions."),
    }
}

fn matrix_multiply(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let rows_b = b.len();
    let cols_b = b[0].len();

    if cols_a != rows_b {
        return None;
    }

    let mut result = vec![vec![0; cols_b]; rows_a];

    for i in 0..rows_a {
        for j in 0..cols_b {
            for k in 0..cols_a {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    Some(result)
}

fn print_matrix(matrix: &Vec<Vec<i32>>) {
    for row in matrix {
        for &val in row {
            print!("{}\t", val);
        }
        println!();
    }
}

