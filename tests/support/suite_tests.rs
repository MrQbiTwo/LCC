// Protect selection and catalog contracts without depending on group totals.
use super::*;
use crate::Expectation;

#[path = "../group_template.rs"]
mod template;

fn options(args: &[&str]) -> Result<Options, String> {
    Options::parse(args.iter().map(|arg| (*arg).into()))
}

fn alpha() -> Vec<Case> {
    vec![
        Case::new("alpha_one", "first boundary", "one", Expectation::Exit(0)),
        Case::new(
            "alpha_two",
            "second boundary",
            "two",
            Expectation::Reject("E008".into()),
        ),
    ]
}

fn beta() -> Vec<Case> {
    vec![Case::new(
        "beta_one",
        "foreign boundary",
        "foreign",
        Expectation::CInterop("endpoint".into()),
    )]
}

const GROUPS: &[Group] = &[
    Group {
        name: "alpha",
        suite: Suite::Ordinary,
        cases: alpha,
    },
    Group {
        name: "beta",
        suite: Suite::Interop,
        cases: beta,
    },
];

#[test]
fn legacy_filter_matches_explicit_filter() {
    assert_eq!(
        options(&["lcc", "array_bounds"]).unwrap(),
        options(&["lcc", "--filter", "array_bounds"]).unwrap()
    );
}

#[test]
fn listing_does_not_require_a_compiler() {
    let parsed = options(&["--interop", "--list", "--group", "beta", "--jobs", "2"]).unwrap();
    assert!(parsed.compiler.is_none());
    assert_eq!(parsed.jobs, 2);
    assert_eq!(select(GROUPS, &parsed).unwrap()[0].id, "beta_one");
}

#[test]
fn invalid_options_fail_explicitly() {
    for args in [
        vec![],
        vec!["--unknown"],
        vec!["lcc", "--jobs", "0"],
        vec!["lcc", "--jobs", "9"],
        vec!["lcc", "--jobs", "two"],
        vec!["lcc", "--jobs"],
        vec!["--list", "--group"],
        vec!["--list", "--group", "--interop"],
        vec!["lcc", "one", "two"],
        vec!["lcc", "one", "--filter", "two"],
        vec!["--list", "--groups"],
    ] {
        assert!(options(&args).is_err(), "unexpectedly accepted {args:?}");
    }
}

#[test]
fn group_and_id_filters_are_intersected() {
    let parsed = options(&["--list", "--group", "alpha", "--filter", "two"]).unwrap();
    let cases = select(GROUPS, &parsed).unwrap();
    assert_eq!(cases.len(), 1);
    assert_eq!(cases[0].id, "alpha_two");
}

#[test]
fn missing_or_wrong_suite_group_is_an_error() {
    for args in [
        vec!["--list", "--group", "missing"],
        vec!["--list", "--group", "beta"],
        vec!["--list", "--filter", "missing"],
    ] {
        assert!(select(GROUPS, &options(&args).unwrap()).is_err());
    }
}

#[test]
fn duplicate_case_ids_are_checked_before_filtering() {
    let groups = [
        Group {
            name: "alpha",
            suite: Suite::Ordinary,
            cases: alpha,
        },
        Group {
            name: "duplicate",
            suite: Suite::Ordinary,
            cases: alpha,
        },
    ];
    let parsed = options(&["--list", "--group", "alpha"]).unwrap();
    assert!(select(&groups, &parsed)
        .unwrap_err()
        .contains("duplicate case ID"));
}

#[test]
fn unsafe_case_names_and_missing_purposes_are_rejected() {
    for id in ["", ".", "..", "../outside", "a/b", "a\\b", "a:b"] {
        assert!(validate(&Case::new(id, "purpose", "", Expectation::Exit(0))).is_err());
    }
    assert!(validate(&Case::new("valid", " ", "", Expectation::Exit(0))).is_err());
}

#[test]
fn auxiliary_names_cannot_collide_or_escape() {
    for files in [
        vec![("../outside", "")],
        vec![("input.c", "")],
        vec![("same.h", "one"), ("same.h", "two")],
    ] {
        let case = Case::new("valid", "purpose", "", Expectation::Exit(0)).with_files(files);
        assert!(validate(&case).is_err());
    }
    let case = Case::new("valid", "purpose", "", Expectation::Exit(0))
        .with_files(vec![("local.h", "#define VALUE 1\n")]);
    assert!(validate(&case).is_ok());
}

#[test]
fn complete_catalogs_obey_the_shared_contract() {
    for args in [vec!["--list"], vec!["--list", "--interop"]] {
        assert!(!select(catalog::GROUPS, &options(&args).unwrap())
            .unwrap()
            .is_empty());
    }
}

#[test]
fn template_uses_the_same_case_contract() {
    let cases = template::cases();
    assert!(!cases.is_empty());
    let mut ids = HashSet::new();
    for case in cases {
        validate(&case).unwrap();
        assert!(ids.insert(case.id));
    }
}
