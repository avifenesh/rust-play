const SPACE_CHAR: u8 = b' '; // ASCII 32: Empty cell representation
const MINE_CHAR: u8 = b'*'; // ASCII 42: Mine marker
const DIGIT_ONE: u8 = b'1'; // ASCII 49: Starting digit for mine count

/// Annotates a minefield by replacing empty spaces with adjacent mine counts.
///
/// This function implements an optimized minesweeper annotation algorithm that:
/// 1. Creates a mutable copy of the input grid
/// 2. Finds all mines and increments their neighbors
/// 3. Converts the result back to strings
///
/// Time Complexity: O(M × 8 + R × C) where M = mines, R = rows, C = columns
/// Space Complexity: O(R × C) for the intermediate grid
///
/// # Arguments
/// * `minefield` - A slice of string slices representing the minefield
///
/// # Returns
/// * `Vec<String>` - The annotated minefield with mine counts
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // Handle edge case: empty minefield
    if minefield.is_empty() {
        return vec![];
    }

    let total_rows = minefield.len();
    let total_cols = minefield[0].len();

    // Pre-allocate result vector with exact capacity to avoid reallocations
    let mut result_grid = Vec::with_capacity(total_rows);

    // Phase 1: Initialize working grid by copying input data
    // Using extend_from_slice for efficient bulk copying of byte data
    for row_str in minefield {
        let mut row_bytes = Vec::with_capacity(total_cols);
        row_bytes.extend_from_slice(row_str.as_bytes());
        result_grid.push(row_bytes);
    }

    // Phase 2: Process mines and update adjacent cells
    // Use iterator with enumerate for idiomatic Rust (Clippy suggestion)
    for (row_index, current_row_str) in minefield.iter().enumerate() {
        let current_row_bytes = current_row_str.as_bytes();

        // Process each column using iterator with enumerate (Clippy suggestion)
        for (col_index, &current_cell) in current_row_bytes.iter().enumerate() {
            // Use branchless comparison to avoid CPU branch prediction penalties
            let is_mine_present = (current_cell == MINE_CHAR) as u8;

            // Only process neighbor updates if a mine is found
            // This is much more efficient than checking every cell's neighbors
            if is_mine_present != 0 {
                update_adjacent_cells(
                    &mut result_grid,
                    row_index,
                    col_index,
                    total_rows,
                    total_cols,
                );
            }
        }
    }

    // Phase 3: Convert byte vectors back to UTF-8 strings
    // Using unwrap() is safe here as we only work with valid ASCII characters
    result_grid
        .into_iter()
        .map(|row_bytes| String::from_utf8(row_bytes).unwrap())
        .collect()
}

/// Updates all cells adjacent to a mine by incrementing their count or converting spaces to '1'.
///
/// This function uses a macro for compile-time optimization and manual loop unrolling.
/// Each of the 8 directions is processed as a separate, optimized code path.
///
/// # Arguments
/// * `grid` - Mutable reference to the 2D byte grid being processed (using slice for flexibility)
/// * `mine_row` - Row index of the mine (0-based)
/// * `mine_col` - Column index of the mine (0-based)  
/// * `max_rows` - Total number of rows (for bounds checking)
/// * `max_cols` - Total number of columns (for bounds checking)
#[inline(always)]
fn update_adjacent_cells(
    grid: &mut [Vec<u8>], // Changed from &mut Vec<Vec<u8>> to &mut [Vec<u8>] (Clippy suggestion)
    mine_row: usize,
    mine_col: usize,
    max_rows: usize,
    max_cols: usize,
) {
    /// Macro to generate optimized code for each direction.
    /// Using macros instead of loops eliminates iterator overhead and enables
    /// compile-time constant folding for maximum performance.
    ///
    /// # Parameters
    /// * `$row_offset` - Relative row offset (-1, 0, or 1)
    /// * `$col_offset` - Relative column offset (-1, 0, or 1)
    macro_rules! process_neighbor_cell {
        ($row_offset:expr, $col_offset:expr) => {
            // Calculate target cell coordinates using wrapping arithmetic
            // wrapping_add_signed handles potential underflow when subtracting from 0
            let target_row = mine_row.wrapping_add_signed($row_offset as isize);
            let target_col = mine_col.wrapping_add_signed($col_offset as isize);

            // Bounds check using bitwise AND for branchless operation
            // This is faster than logical AND (&&) as it avoids short-circuit evaluation
            if (target_row < max_rows) & (target_col < max_cols) {
                increment_cell_value(&mut grid[target_row][target_col]);
            }
        };
    }

    // Manual loop unrolling: Process all 8 adjacent cells
    // Direction layout:
    //   (-1,-1) (-1, 0) (-1, 1)
    //   ( 0,-1)  MINE   ( 0, 1)
    //   ( 1,-1) ( 1, 0) ( 1, 1)

    process_neighbor_cell!(-1, -1); // Top-left diagonal
    process_neighbor_cell!(-1, 0); // Top center
    process_neighbor_cell!(-1, 1); // Top-right diagonal
    process_neighbor_cell!(0, -1); // Left center
    process_neighbor_cell!(0, 1); // Right center
    process_neighbor_cell!(1, -1); // Bottom-left diagonal
    process_neighbor_cell!(1, 0); // Bottom center
    process_neighbor_cell!(1, 1); // Bottom-right diagonal
}

/// Increments a cell's mine count using branchless bit manipulation.
///
/// This function uses advanced bit manipulation techniques to avoid conditional branches,
/// which can cause CPU pipeline stalls due to branch misprediction.
///
/// Logic:
/// - If cell is a mine ('*'): no change
/// - If cell is empty (' '): convert to '1'
/// - If cell is a digit ('1'-'8'): increment by 1
///
/// # Arguments
/// * `cell` - Mutable reference to the cell byte to be updated
#[inline(always)]
fn increment_cell_value(cell: &mut u8) {
    let current_value = *cell;

    // Generate boolean flags as u8 values (0 or 1) for branchless arithmetic
    let is_not_mine = (current_value != MINE_CHAR) as u8; // 1 if not mine, 0 if mine
    let is_empty_space = (current_value == SPACE_CHAR) as u8; // 1 if space, 0 otherwise

    // Check if current value is a digit (ASCII '0'-'8')
    // Bit manipulation explanation:
    // - ASCII digits have pattern 0011xxxx (0x30-0x38)
    // - Upper 4 bits (0xF0 mask) should equal 0x30
    // - Lower 4 bits (0x0F mask) should be <= 8
    let is_digit = (((current_value & 0xF0) == 0x30) as u8) & (((current_value & 0x0F) <= 8) as u8);

    // Branchless update using arithmetic instead of conditionals
    // Breakdown:
    // - is_not_mine * (...): Only apply changes if not a mine
    // - (DIGIT_ONE - SPACE_CHAR) * is_empty_space: Converts space (32) to '1' (49) by adding 17
    // - is_digit: Increments digits by 1
    *cell += is_not_mine
        * (
            (DIGIT_ONE - SPACE_CHAR) * is_empty_space +  // Space → '1' (adds 17)
            is_digit
            // Digit → digit+1 (adds 1)
        );
}
