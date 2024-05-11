use crate::collections::vec2d::Vec2D;

pub fn bin_square(dim: usize, strength: usize) -> usize {
    // We are going to use a helper function to do the recursion as Rust does
    // not support overloading functions.

    let mut partial = Vec2D::new(dim, dim);

    let agg = extend_partial(dim, strength, &mut partial, (0, 0));
    return agg;
}

fn extend_partial(
    dim: usize,
    strength: usize,
    partial: &mut Vec2D<bool>,
    current_loc: (usize, usize),
) -> usize {
    // Determine if we just finished a row. This is required so that we know
    // if the previous bit in the row was set, in the case that the row had
    // strength
    if current_loc.0 > 0 && current_loc.1 == 0 {
        if row_strength(partial, (current_loc.0 - 1, dim)) < strength {
            return 0;
        }
    // The only way for our current row to be the dimension of the square is
    // if we have completed a square.
    } else if current_loc.0 == dim {
        // If we were interested in the output of this program we would print
        // here.
        
        // println!("{}", partial);

        return 1;
    // Last base case is an early rejection condition. It checks to make sure
    // that there is room left in the square for the current row and column to
    // have the desired strength.
    } else if strength - row_strength(partial, current_loc) > dim - (current_loc.1) &&
              strength - col_strength(partial, current_loc) > dim - (current_loc.0) {
        return 0;
    }

    let mut agg = 0;
    let next_loc = if current_loc.1 == dim - 1 {
        (current_loc.0 + 1, 0)
    } else {
        (current_loc.0, current_loc.1 + 1)
    };

    // Compute the row strength and the column strength and decide if there is
    // room remaining in the square for the 1
    if row_strength(partial, current_loc) < strength
        && col_strength(partial, current_loc) < strength
    {
        partial[current_loc] = true;
        agg += extend_partial(dim, strength, partial, next_loc);
        partial[current_loc] = false;
    }

    // We might always be able to extend with a 0. So we do this here.
    agg += extend_partial(dim, strength, partial, next_loc);

    return agg;
}

// We compute the row strength by finding the number of 1's contained in the
// current row.
fn row_strength(partial: &Vec2D<bool>, current_loc: (usize, usize)) -> usize {
    let mut strength = 0;

    for i in 0..current_loc.1 {
        if partial[(current_loc.0, i)] {
            strength += 1;
        }
    }

    return strength;
}

// We compute the column strength by finding the number of 1's contained in
// the current column.
fn col_strength(partial: &Vec2D<bool>, current_loc: (usize, usize)) -> usize {
    let mut strength = 0;

    for i in 0..current_loc.0 {
        if partial[(i, current_loc.1)] {
            strength += 1;
        }
    }

    return strength;
}