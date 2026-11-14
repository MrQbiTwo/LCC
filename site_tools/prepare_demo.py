"""Generate demonstration records from imported metadata; no compiler checkout needed."""
import base64
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DATA = ROOT / "_data"


def save(name, value):
    (DATA / f"{name}.json").write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


catalog = json.loads((DATA / 'test_catalog.json').read_text(encoding='utf-8'))
ids = [test["id"] for group in catalog for test in group["tests"]]
commit = json.loads((DATA / 'catalog_metadata.json').read_text(encoding='utf-8'))['source_commit']
assert ids, 'The imported catalog must contain cases'
number = "LCC20261114DEMO1"
assert len(number) == 16
save("releases", [
    dict(slug="demo-nightly", title="Nightly · November 14", channel="Nightly", date="2026-11-14", demo=True,
         description="A development build with a few simulated failures to explore.", commit=commit,
         nightly_number=number, nightly_encoded=base64.b64encode(number.encode()).decode(),
         failed_ids=ids[::max(1, len(ids) // 4)][:4]),
    dict(slug="demo-snapshot", title="1.0-sp1", channel="Snapshot", date="2026-11-12", demo=True,
         description="A preview milestone with illustrative test results.", commit=commit,
         failed_ids=ids[::max(1, len(ids) // 18)][:18]),
    dict(slug="demo-stable", title="1.0", channel="Stable", date="2026-11-10", demo=True,
         description="An example stable release with an all-passing demonstration report.", commit=commit, failed_ids=[]),
])
save("report_types", [dict(key="tests", title="Tests Passed"), dict(key="features", title="Supported Features"), dict(key="platforms", title="Supported Platforms")])
standards = ["C89", "C90", "C99", "C11", "C17", "C23"]
save("standards", standards)
features = [
    ("Integer arithmetic", "Language · expressions", standards, "Supported", "Demonstrates a supported language entry; actual verification will be attached per release."),
    ("Structures and unions", "Language · types", standards, "Supported", "Example coverage of aggregate types."),
    ("Function pointers", "Language · declarators", standards, "Partially Supported", "Illustrates a feature with documented limitations."),
    ("printf", "<stdio.h> · function", standards, "Partially Supported", "Example: basic integer and string formatting; complete formatting coverage is not asserted."),
    ("fopen", "<stdio.h> · function", standards, "Not Tested", "Example with no completed verification evidence."),
    ("malloc", "<stdlib.h> · function", standards, "Supported", "Illustrative allocation support; this is not a release guarantee."),
    ("qsort", "<stdlib.h> · function", standards, "Not Supported", "Demonstrates an unavailable library function."),
    ("memcpy", "<string.h> · function", standards, "Supported", "Example byte-copying function entry."),
    ("size_t", "<stddef.h> · type", standards, "Supported", "Header coverage includes types as well as functions."),
    ("va_start / va_arg / va_end", "<stdarg.h> · macros", standards, "Partially Supported", "Example macro family with a documented support boundary."),
    ("Designated initializers", "Language · initialization", standards[2:], "Supported", "Example of a feature introduced in C99."),
    ("restrict", "Language · qualifiers", standards[2:], "Not Supported", "Demonstration status; semantic and optimization support must be verified separately."),
    ("snprintf", "<stdio.h> · function", standards[2:], "Not Tested", "Example C99 library function awaiting verification."),
    ("_Generic", "Language · generic selection", standards[3:], "Not Tested", "Example C11 feature awaiting a completed support report."),
    ("thrd_create", "<threads.h> · function", standards[3:], "Not Supported", "Example of an unsupported standard library facility."),
    ("atomic_load", "<stdatomic.h> · macro", standards[3:], "Not Tested", "Example atomic facility with no verified result."),
    ("nullptr", "Language · constants", ["C23"], "Not Tested", "Example C23 feature; inclusion is not a conformance claim."),
    ("ckd_add", "<stdckdint.h> · macro", ["C23"], "Not Supported", "Example checked-integer facility introduced in C23."),
]
save("feature_examples", [dict(name=n, category=c, standards=s, status=t, notes=d) for n, c, s, t, d in features])
save("platforms", [dict(os=os, arch=arch, host=status, target=status) for os, arch, status in [
    ("Windows", "x86-64", "Not Supported"), ("Windows", "arm64", "Not Supported"),
    ("Linux", "x86-64", "Supported"), ("Linux", "arm64", "Not Supported"), ("Linux", "RISC-V · RV64", "Not Supported"),
    ("macOS", "arm64 (Apple Silicon)", "Not Supported"), ("macOS", "x86-64 (Intel)", "Not Supported"),
]])
print(f"Loaded {len(ids)} cases in {len(catalog)} groups; generated three labeled demonstration records.")
