use ratatui::layout::{Constraint, Layout, Rect};

/// Arranges `n` items in an automatic grid layout within the given area.
///
/// Uses a square root approach to determine grid dimensions:
/// - Calculates columns as √n (rounded up)
/// - Calculates rows as n/cols (rounded up)
///
/// # Arguments
///
/// * `area` - The rectangular area to split into a grid
/// * `n` - Number of cells needed in the grid
/// * `spacing` - Space between cells
///
/// # Returns
///
/// A vector of `n` Rects, arranged in row-major order (left-to-right, top-to-bottom)
///
/// # Example
///
/// ```
/// use ratatui::layout::Rect;
/// use ratatui_auto_grid::auto_grid;
///
/// let area = Rect::new(0, 0, 100, 100);
/// let cells = auto_grid(area, 9, 1);
/// assert_eq!(cells.len(), 9);
/// ```
pub fn auto_grid(area: Rect, n: usize, spacing: u16) -> Vec<Rect> {
    if n == 0 {
        return Vec::new();
    }

    let cols = (n as f64).sqrt().ceil() as u16;
    let rows = ((n as f64) / f64::from(cols)).ceil() as u16;

    let row_constraints: Vec<Constraint> = std::iter::repeat(Constraint::Ratio(1, rows.into()))
        .take(rows as usize)
        .collect();

    let col_constraints: Vec<Constraint> = std::iter::repeat(Constraint::Ratio(1, cols.into()))
        .take(cols as usize)
        .collect();

    let row_areas = Layout::vertical(row_constraints)
        .spacing(spacing)
        .split(area);

    let mut out = Vec::with_capacity(n);
    'outer: for r in 0..rows as usize {
        let col_areas = Layout::horizontal(col_constraints.clone())
            .spacing(spacing)
            .split(row_areas[r]);
        for &rect in col_areas.iter() {
            if out.len() == n {
                break 'outer;
            }
            out.push(rect);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_grid() {
        let area = Rect::new(0, 0, 100, 100);
        let result = auto_grid(area, 0, 0);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn single_cell() {
        let area = Rect::new(0, 0, 100, 100);
        let result = auto_grid(area, 1, 0);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], area, "Single cell should fill entire area");
    }

    #[test]
    fn four_cells_perfect_square() {
        let area = Rect::new(0, 0, 100, 100);
        let result = auto_grid(area, 4, 0);

        assert_eq!(result.len(), 4);

        // Top left
        assert_eq!(result[0].x, 0);
        assert_eq!(result[0].y, 0);
        assert_eq!(result[0].width, 50);
        assert_eq!(result[0].height, 50);

        // Top right
        assert_eq!(result[1].x, 50);
        assert_eq!(result[1].y, 0);

        // Bottom left
        assert_eq!(result[2].x, 0);
        assert_eq!(result[2].y, 50);

        // Bottom right
        assert_eq!(result[3].x, 50);
        assert_eq!(result[3].y, 50);
    }

    #[test]
    fn nine_cells() {
        let area = Rect::new(0, 0, 99, 99);
        let result = auto_grid(area, 9, 0);

        assert_eq!(result.len(), 9);

        // first cell
        assert_eq!(result[0].x, 0);
        assert_eq!(result[0].y, 0);
        assert_eq!(result[0].width, 33);
        assert_eq!(result[0].height, 33);

        // last cell
        assert_eq!(result[8].x, 66);
        assert_eq!(result[8].y, 66);
    }

    #[test]
    fn non_square_grid() {
        let area = Rect::new(0, 0, 100, 100);
        let result = auto_grid(area, 6, 0);

        assert_eq!(result.len(), 6);

        // First row
        assert_eq!(result[0].y, result[1].y);
        assert_eq!(result[1].y, result[2].y);

        // Second row
        assert_eq!(result[3].y, result[4].y);
        assert_eq!(result[4].y, result[5].y);

        assert_ne!(result[0].y, result[3].y);
    }

    #[test]
    fn with_spacing() {
        let area = Rect::new(0, 0, 100, 100);
        let result_no_spacing = auto_grid(area, 4, 0);
        let result_with_spacing = auto_grid(area, 4, 2);

        assert_eq!(result_no_spacing.len(), 4);
        assert_eq!(result_with_spacing.len(), 4);

        assert!(result_with_spacing[0].width <= result_no_spacing[0].width);
        assert!(result_with_spacing[0].height <= result_no_spacing[0].height);

        let gap_no_spacing =
            result_no_spacing[1].x - (result_no_spacing[0].x + result_no_spacing[0].width);
        let gap_with_spacing =
            result_with_spacing[1].x - (result_with_spacing[0].x + result_with_spacing[0].width);
        assert!(gap_with_spacing >= gap_no_spacing);
    }

    #[test]
    fn all_cells_within_bounds() {
        let area = Rect::new(10, 10, 200, 150);
        let result = auto_grid(area, 7, 1);

        for (i, rect) in result.iter().enumerate() {
            assert!(
                rect.x >= area.x,
                "Cell {} x position {} should be >= area.x {}",
                i,
                rect.x,
                area.x
            );
            assert!(
                rect.y >= area.y,
                "Cell {} y position {} should be >= area.y {}",
                i,
                rect.y,
                area.y
            );
            assert!(
                rect.x + rect.width <= area.x + area.width,
                "Cell {} right edge should be within area bounds",
                i
            );
            assert!(
                rect.y + rect.height <= area.y + area.height,
                "Cell {} bottom edge should be withing area bounds",
                i
            );
        }
    }

    #[test]
    fn row_major_order() {
        let area = Rect::new(0, 0, 100, 100);
        let result = auto_grid(area, 6, 0);

        assert_eq!(result[0].y, result[1].y);
        assert_eq!(result[1].y, result[2].y);

        assert_eq!(result[3].y, result[4].y);
        assert_eq!(result[4].y, result[5].y);

        assert!(result[0].y < result[3].y);
    }

    #[test]
    fn exact_count_returned() {
        for n in 1..=20 {
            let area = Rect::new(0, 0, 100, 100);
            let result = auto_grid(area, n, 0);
            assert_eq!(
                result.len(),
                n,
                "should return exactly {} cells, got {}",
                n,
                result.len()
            );
        }
    }
}
