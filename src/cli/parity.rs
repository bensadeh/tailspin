//! Parity tests anchored on `Base` and `Extra`: the exhaustive matches below
//! are the registry — a new variant fails to compile here until it declares
//! an exemplar — and the tests then prove the variant is wired end to end:
//! `--enable`/`--extras` must visibly highlight it, the e2e fixture must
//! exercise it, the library default must include it, and the man page must
//! list it.

use crate::cli::builtins::get_builtin_keywords;
use crate::cli::highlighter::build_highlighter;
use crate::cli::resolution::{BaseSet, resolve_extras};
use crate::cli::{Base, Extra};
use crate::theme::Theme;
use clap::ValueEnum;
use tailspin::Highlighter;

/// A line each base group must visibly highlight when enabled alone.
fn exemplar(base: Base) -> &'static str {
    match base {
        Base::Numbers => "count 42",
        Base::Urls => "https://example.com/path",
        Base::Emails => "user@example.com",
        Base::Pointers => "0xd7b3b2f446e2c21b",
        Base::Dates => "2024-09-14T07:57:30.659Z",
        Base::Durations => "took 150ms",
        Base::Paths => "/var/log/nginx/error.log",
        Base::Quotes => "\"quoted text\"",
        Base::KeyValuePairs => "key=value",
        Base::Uuids => "550e8400-e29b-41d4-a716-446655440000",
        Base::Ipv4 => "192.168.0.1",
        Base::Processes => "sshd[4242]",
        Base::Json => r#"{"level": "info"}"#,
    }
}

fn extra_exemplar(extra: Extra) -> &'static str {
    match extra {
        Extra::Ipv6 => "peer 2001:db8::ff00:42:8329",
        Extra::JvmStackTrace => "        at com.example.EmailService.send(EmailService.kt:171)",
    }
}

/// Builtin keywords are disabled and the theme is empty, so only the groups
/// under test can produce highlights.
fn build(base: &BaseSet, extras: &[Extra]) -> Highlighter {
    build_highlighter(base, &resolve_extras(extras), Theme::default(), &[], true).unwrap()
}

fn only(base: Base) -> BaseSet {
    BaseSet::resolve(&[base], &[]).unwrap()
}

fn fixture() -> String {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/files/e2e.log");
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn every_base_group_highlights_its_exemplar_when_enabled_alone() {
    for &base in Base::value_variants() {
        let input = exemplar(base);

        let untouched = build(&BaseSet::none(), &[]).apply(input).into_owned();
        assert_eq!(untouched, input, "exemplar for {base:?} is lit by an always-on finder");

        let highlighted = build(&only(base), &[]).apply(input).into_owned();
        assert_ne!(highlighted, input, "--enable {base:?} changes nothing on `{input}`");
    }
}

#[test]
fn every_extra_highlights_its_exemplar_when_enabled_alone() {
    for &extra in Extra::value_variants() {
        let input = extra_exemplar(extra);

        let highlighted = build(&BaseSet::none(), &[extra]).apply(input).into_owned();
        assert_ne!(highlighted, input, "--extras {extra:?} changes nothing on `{input}`");
    }
}

#[test]
fn the_e2e_fixture_exercises_every_group() {
    let fixture = fixture();

    for &base in Base::value_variants() {
        let h = build(&only(base), &[]);
        assert!(
            fixture.lines().any(|line| h.apply(line) != line),
            "tests/files/e2e.log has no line for {base:?}"
        );
    }
    for &extra in Extra::value_variants() {
        let h = build(&BaseSet::none(), &[extra]);
        assert!(
            fixture.lines().any(|line| h.apply(line) != line),
            "tests/files/e2e.log has no line for {extra:?}"
        );
    }
}

#[test]
fn library_default_covers_every_base_group() {
    let default = Highlighter::default();

    for &base in Base::value_variants() {
        let input = exemplar(base);
        assert_ne!(
            default.apply(input).into_owned(),
            input,
            "Highlighter::default() misses {base:?}"
        );
    }
}

/// `Highlighter::default()` must apply the same groups with the same
/// precedence as the CLI's default configuration (minus builtin keywords).
/// The seeds exercise overlaps where precedence order shows.
#[test]
fn library_default_matches_the_cli_default_configuration() {
    let cli = build(&BaseSet::resolve(&[], &[]).unwrap(), &[]);
    let default = Highlighter::default();

    let fixture = fixture();
    let overlap_seeds = [
        "http://192.168.0.1/health",
        "/var/lib/550e8400-e29b-41d4-a716-446655440000/data",
    ];
    let exemplars = Base::value_variants().iter().map(|&base| exemplar(base));

    for line in fixture.lines().chain(overlap_seeds).chain(exemplars) {
        assert_eq!(
            cli.apply(line),
            default.apply(line),
            "CLI defaults and Highlighter::default() diverge on `{line}`"
        );
    }
}

/// Anchored on the builtin data itself rather than a hand-written list, so
/// new builtin groups are covered automatically.
#[test]
fn every_builtin_keyword_highlights_by_default() {
    let h = build_highlighter(&BaseSet::none(), &resolve_extras(&[]), Theme::default(), &[], false).unwrap();

    for group in get_builtin_keywords(false) {
        for word in &group.words {
            assert_ne!(
                h.apply(word).as_ref(),
                word.as_str(),
                "builtin keyword `{word}` does not highlight"
            );
        }
    }
}

#[test]
fn man_page_lists_every_group_and_extra() {
    let adoc = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/util/tspin.adoc")).unwrap();

    let name = |value: clap::builder::PossibleValue| value.get_name().to_string();

    let groups: Vec<String> = Base::value_variants()
        .iter()
        .map(|base| name(base.to_possible_value().unwrap()))
        .collect();
    let expected = format!("Possible groups: {}.", groups.join(", "));
    assert_eq!(
        adoc.matches(&expected).count(),
        2,
        "the --enable/--disable group lists in util/tspin.adoc are stale"
    );

    let extras: Vec<String> = Extra::value_variants()
        .iter()
        .map(|extra| name(extra.to_possible_value().unwrap()))
        .collect();
    let expected = format!("Possible values: {}.", extras.join(", "));
    assert!(
        adoc.contains(&expected),
        "the --extras list in util/tspin.adoc is stale"
    );
}
