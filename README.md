# ratatui-auto-grid

A simple auto-grid layout utility for [ratatui][ratatui] TUI applications.

This crate provides a single function, `auto_grid()` that automatically arranges N items in a grid
layout using a square-root approach.

## Usage

```rs
use ratatui::layout::Rect;
use ratatui_auto_grid::auto_grid;

let area = Rect::new(0, 0, 100, 100);
// Returns 9 Rects arranged in a 3x3 grid with 1 cell spacing.
let cells = auto_grid(area, 9, 1);
```
