# ratatui-auto-grid

[![acctions status][actions-badge]][actions-url]
[![crate version][crate-version-badge]][crate-url]
[![documentation][docs-badge]][docs-url]
[![dependencies status][deps-badge]][deps-url]
![licenses][licenses-badge]

[actions-badge]: https://github.com/yozhgoor/ratatui-auto-grid/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/yozhgoor/ratatui-auto-grid/actions
[crate-version-badge]: https://img.shields.io/crates/v/ratatui-auto-grid
[crate-url]: https://crates.io/crates/ratatui-auto-grid
[docs-badge]: https://deps.rs/repo/github/yozhgoor/ratatui-auto-grid/status.svg
[docs-url]: https://deps.rs/repo/github/yozhgoor/ratatui-auto-grid
[licenses-badge]: https://img.shields.io/crates/l/ratatui-auto-grid

A simple auto-grid layout utility for [ratatui][ratatui] TUI applications.

This crate provides a single function, `auto_grid()`, that automatically arranges N items in a grid
layout using a square-root approach.

## Usage

```rs
use ratatui::layout::Rect;
use ratatui_auto_grid::auto_grid;

let area = Rect::new(0, 0, 100, 100);
// Returns 9 Rects arranged in a 3x3 grid with 1 cell spacing.
let cells = auto_grid(area, 9, 1);
```
