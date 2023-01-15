## Purpose

organize media files in given `./folder/` into

- `<folder>/media/`
  - `image/` for all image formats
  - `gif/` for all gifs
  - `video/` for a all video files

If the media folder exists already, just move files there

## MVP

- splits everything into pre-defined paths `/img` `/vid` `/gif`
- no lookup that folder already has file with the same anme before move ops
```shell
# to preview whats gonna be moved
cargo run -- ~/Downloads --dry-run

# to execute move
cargo run -- ~/Downloads 
```

### Todos
- [] better messaging to distinguish between run and dry-run
- [] artifact build to be downloadable with `curl`