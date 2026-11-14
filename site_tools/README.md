# Release-site development

This `gh-pages` worktree contains only Jekyll website sources, imported report
metadata and website tooling. It does not need a compiler source checkout or Rust
to build. The compiler is maintained independently on the repository's `main`
branch. No deployment workflow is enabled by this prototype.

Install Ruby and Bundler, then run `bundle install` from the worktree. On Windows,
`site_tools/preview.ps1` starts the site at <http://127.0.0.1:4173/LCC/>; pass
`-Mode build` to build without serving. The helper also recognizes the existing
isolated gem installation under `build/gems`. Ruby can be selected with `-Ruby`.
Alternatively use `bundle exec jekyll serve --host 127.0.0.1 --port 4173 --no-watch`.
Stop the server with Ctrl+C. Rebuild/restart after editing source.

Run `python site_tools/verify_site.py` after building to check generated links,
catalog coverage, demonstration labels, support inventories, asset dependencies,
and the absence of compiler sources from this checkout.

## Data and templates

- `_layouts/default.html`: shared shell, transparent logo, theme control and footer.
- `_layouts/report.html`: version context and report navigation.
- `_includes`: focused release rows, test, feature and platform presentations.
- `_plugins/release_pages.rb`: three report pages per demonstration release.
- `_data/test_catalog.json`: imported group names, case IDs and descriptions only.
- `_data/catalog_metadata.json`: immutable provenance of that imported snapshot.
- `_data/releases.json`: examples with simulated outcomes.
- `_data/feature_examples.json`, `platforms.json`: illustrative support data.
- `assets/css`: focused shell, index, report, author and responsive styles.
- `assets/vendor/fontawesome`: self-hosted Font Awesome Free 7.3.1 with license.

The three examples are not published compiler versions. The source SHA identifies
the catalog snapshot, not the website commit. Dates and Passed/Failed outcomes are
simulated. Feature/platform examples are shared across demo versions; production
integration must store independent, verified support records for each release.
No downloads are offered for synthetic versions.

## Refreshing sample records

Run `python site_tools/prepare_demo.py` to regenerate samples from the imported
catalog. This only reads JSON; it neither imports compiler code nor executes tests.
Failure samples are selected relative to catalog size rather than fixed indices.

The imported catalog retains `Case.id` and `Case.purpose` from its recorded source.
The UI counts logical cases once across ordinary and interop suites. Tooling
tests and benchmark fixtures are outside this catalog. Production reporting will
export metadata and actual outcomes from the compiler pipeline into the site.
Do not reintroduce the compiler tree into this branch or use simulated results as
verification evidence.
