Who needs frameworks anyways?

This is a tinker project to see how much a can cram in to a statically hosted GitHub Pages site.

Current ideas that may make it in:
- [x] Generate pages from markdown
- [ ] a full fat site raw html static site generator
- [ ] a set of services for getting and processing data
- [ ] a content index used for full content site search

I'm just having fun learning to work with Rust and Async Rust outside of our very specific Zed ecosystem.

Misc TODO:
- look into using [lightningcss](https://crates.io/crates/lightningcss) for handling css
- use this as a chance to learn [palette](https://crates.io/crates/palette)

## Usage

- Add content in the `/content` directory
- Add the appropriate ENV values to your github project
- `cargo run` to build the project
- pushes to main will deploy the site to GitHub Pages
