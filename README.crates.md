# msafara

`msafara` is a terminal-based viewer for multiple sequence alignments (MSAs).
It provides a smooth interface to explore alignments directly from the
command line — especially useful when working over SSH or in headless
environments.

---

##  Installation

```bash
cargo install msafara
```

---

##  Quick Usage

Once installed, run:

```bash
msafara <my-alignment>
```
where `my-alignment` is a multiple alignment in FASTA, Clustal, or Stockholm
format. If an unaligned FASTA is provided and `mafft` is configured, `msafara`
will align it on load.

For help, run

```bash
msafara -h
```

Or press `?` while running to see key bindings.


---

## Features

- Zoomed-in and zoomed-out views of the alignment
- Consensus sequence display and conservation indicators
- Sequence metrics such as ungapped length and similarity to consensus
- Regex and EMBOSS motif search with live highlighting
- Views for filtered/rejected subsets and selection-based exports
- Tree-aware ordering and tree navigation (when available)
- SVG export of the current view
- Color maps for nucleotides and amino acids
- Full keyboard control, no mouse required

Best results in a dark-themed terminal.

Configuration is read from `.msafara.config` (first in `$HOME`, then the current
directory) for tool paths (EMBOSS/mafft) and color settings. Sessions can be
saved as `.msfr` files and re-opened later.

---

## More Info

- Source, releases, and test data:  
  [https://github.com/pmcarlton/msafara](https://github.com/pmcarlton/msafara)

- Platform-specific binaries (Linux, macOS) available on the [Releases](https://github.com/pmcarlton/msafara/releases) page.

---

## 📝 License

MIT
