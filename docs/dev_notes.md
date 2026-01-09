# Developer Notes

## Entrypoint, Main Loop, State Model

- `src/main.rs:main` calls `termal_msa::run()` in `src/runner.rs`.
- `src/runner.rs:run` parses CLI (`Cli`), loads sequences (`read_fasta_file`/`read_stockholm_file`),
  builds `Alignment::from_file`, constructs `App::new`, then builds `UI::new`.
- Main event loop is in `src/runner.rs:run`:
  `event::poll`/`event::read`, `handle_key_press` in `src/ui/key_handling.rs`, and redraws via
  `render::render_ui` on key presses or resize; exit when `handle_key_press` returns true.
- State model:
  - `src/app.rs:App` owns data (`Alignment`), ordering (`ordering`, `reverse_ordering`),
    metric/search (`metric`, `search_state`), and modeline messaging (`current_msg`).
  - `src/ui.rs:UI` owns view state (zoom, panes, scrolling, color scheme, input mode) and borrows
    `App`.
  - `src/alignment.rs:Alignment` stores headers/sequences plus precomputed per-column and
    per-sequence metrics (consensus, entropies, densities, id_wrt_consensus, relative_seq_len).

## Rendering Pipeline (Widgets/Layout)

- `src/ui/render.rs:render_ui` is the entry point; it calls `make_layout`, updates `ui.aln_pane_size`
  and `ui.frame_size`, then calls `render_*` pane functions.
- Layout is built with `ratatui::Layout` in `src/ui/render.rs:make_layout`, producing `Panes`
  (labels/metrics, alignment, corner, bottom, dialog).
- Alignment pane:
  - `src/ui/render.rs:render_alignment_pane` selects `aln_widget::SeqPane` (zoomed in) or
    `aln_widget::SeqPaneZoomedOut` (zoomed out), and uses `style::build_style_lut`.
  - `src/ui/aln_widget.rs` renders cell-by-cell, indexing into `sequences` by row and column and
    applying per-byte `Style`.
- Labels/metrics panes render `Paragraph`s from text computed in `compute_*` helpers in
  `src/ui/render.rs`.

## Alignment Data Storage (Gaps, Indexing)

- `src/alignment.rs:Alignment` stores:
  - `headers: Vec<String>` and `sequences: Vec<String>` (all padded to `max_len` in
    `Alignment::from_file`).
  - Gap characters are handled in `col_density` as `-`, `.`, or `' '` (space) and are excluded
    from entropy calculations in `to_freq_distrib`.
- Ordering and indexing:
  - `src/app.rs:App::ordering` is a list of indices into `Alignment.headers`/`sequences`; it is
    recomputed by `App::recompute_ordering`.
  - `reverse_ordering` maps alignment rank to screen line; see `App::rank_to_screenline`.
  - `src/ui/render.rs:retained_col_ndx` and `retained_seq_ndx` use `every_nth` to build sampled
    indices for zoomed-out views.

## Coloring

- Color sources:
  - `src/ui/color_scheme.rs:ColorScheme` holds theme colors and residue colormaps.
  - `UI` maintains the active `ColorScheme` and selected residue colormap.
- Application:
  - `src/ui/style.rs:build_style_lut` builds a 256-entry `Style` LUT using
    `get_residue_style` and the current colormap.
  - `src/ui/aln_widget.rs:SeqPane`/`SeqPaneZoomedOut` look up `style_lut[byte]` per character.
  - `src/ui/render.rs:render_bottom_pane` colors the consensus line with
    `get_residue_style` and the current colormap; label/metric colors are derived from
    `ColorScheme`.

## Conservation / Sparkline

- Computation:
  - `Alignment::from_file` computes `entropies` and `densities` in `src/alignment.rs`.
  - `src/ui/render.rs:render_bottom_pane` computes conservation via
    `product(densities, ones_complement(normalize(entropies)))`.
- Rendering:
  - `src/ui/barchart.rs:values_barchart` converts the conservation vector into a sparkline of
    block characters.
  - The sparkline is drawn as the fourth line of the bottom pane in
    `src/ui/render.rs:render_bottom_pane`, styled with `ColorScheme::conservation_color`.
