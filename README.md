<p align="center">
  <img src=".github/assets/logo.png" alt="TreeIndex logo" width="220">
</p>

# TreeIndex

A safe, cross-platform successor to `dir2html`. It recursively scans regular
files, skips symbolic links, sorts deterministically, emits relative links, and
HTML-escapes all names. It has no browser automation, fixed Windows paths,
temporary DBFs, or external JPEG executables.

```powershell
cargo test
cargo run -- E:\Pictures E:\Pictures\index.html
```

Next slices: thumbnail generation via a maintained image library, pagination,
incremental rebuilds, ignore rules, JSON output, and optional static gallery
themes.
