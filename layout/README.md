# Layout

Root content for each page, such as body color and font.

When creating a new page that uses this layout:

1. Copy this directory.
2. Update `Cargo.toml`
   - Set the project name
   - Add a path dependency on the `layout` crate
3. In `index.html`, set the `<title>` and `<h1>` contents
4. In the root `index.html`, add a link to the new page
   - The route should match the directory name
5. On the deployment branch (`render`), update `Trunk.toml`
   - Set the HTML output file name; the base name should match the directory
   - Enable minification

