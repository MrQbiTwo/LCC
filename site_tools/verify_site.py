"""Validate the generated project-path site and catalog-backed demonstration reports."""
from html.parser import HTMLParser
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / "build/site"


class Page(HTMLParser):
    def __init__(self, path):
        super().__init__()
        self.path, self.links, self.tests, self.ids = path, [], [], []
        self.feed(path.read_text(encoding="utf-8"))

    def handle_starttag(self, tag, attributes):
        attrs = dict(attributes)
        self.links += [attrs[key] for key in ("href", "src") if key in attrs]
        if "id" in attrs:
            self.ids.append(attrs["id"])
        if "data-test" in attrs:
            self.tests.append(attrs["data-result"])


def data(name):
    return json.loads((ROOT / "_data" / f"{name}.json").read_text(encoding="utf-8"))


pages = [Page(path) for path in SITE.rglob("*.html")]
assert len(pages) == 13, f"Expected three core pages, nine reports, one 404; got {len(pages)}"
links = 0
for page in pages:
    assert len(page.ids) == len(set(page.ids)), f"Duplicate HTML IDs: {page.path}"
    content = page.path.read_text(encoding="utf-8")
    assert "Results are simulated" in content and "Sergey Sherbakov and Others 2026." in content
    assert not re.search(r"{[{%].*?[}%]}", content), f"Unrendered Liquid: {page.path}"
    for link in page.links:
        parsed = urlsplit(link)
        if parsed.scheme or parsed.netloc or not parsed.path:
            continue
        assert parsed.path.startswith("/LCC/"), f"Incorrect project base path: {link}"
        target = SITE / unquote(parsed.path.removeprefix("/LCC/"))
        if target.is_dir():
            target /= "index.html"
        assert target.is_file(), f"Broken link in {page.path}: {link}"
        links += 1
    if page.tests:
        release = next(r for r in data("releases") if r["slug"] in page.path.parts)
        expected_total = sum(len(group["tests"]) for group in data("test_catalog"))
        assert len(page.tests) == expected_total
        assert page.tests.count("Failed") == len(release["failed_ids"])
        assert set(page.tests) <= {"Passed", "Failed"}

assert not any((SITE / directory).exists() for directory in ("src", "tests", "runtime", "tools", "site_tools", "build"))
assert not any((ROOT / path).exists() for path in ("src", "runtime", "include", "examples", "tests", "tools", "version", "makefile", "site_tools/export_catalog.rs")), 'Compiler source must not be present in the website checkout'
assert all(release['commit'] == data('catalog_metadata')['source_commit'] for release in data('releases')), 'Catalog provenance must not use the website commit'
assert set(data("standards")) == {"C89", "C90", "C99", "C11", "C17", "C23"}
assert {f["status"] for f in data("feature_examples")} == {"Supported", "Partially Supported", "Not Supported", "Not Tested"}
assert len(data("platforms")) == 7
assert any("RV64" in p["arch"] for p in data("platforms"))
assert "mailto:sergey@littleccompiler.org" in (SITE / "developers/index.html").read_text(encoding="utf-8")
css_root = SITE / "assets"
for css in css_root.rglob("*.css"):
    for dependency in re.findall(r"url\(['\"]?([^)'\"]+)", css.read_text(encoding="utf-8")):
        if dependency.startswith("data:"):
            continue
        assert (css.parent / dependency).is_file(), f"Missing CSS asset: {dependency}"
print(f"PASS: {len(pages)} pages, {links} local link/asset references, three complete catalog reports, demo boundaries and support inventories.")
