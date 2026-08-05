use std::ops::Range;

use mir_ast::surface_v0::{FixtureSource, ParseErrorKind, parse_surface_v0};

const SCN01_VISIBILITY_SOURCE: &str = r#"
module M10.Visibility.Scn01

locus World
locus BrowserClient
principal self
type Participant

state player[p: Participant] at World {
  position: Int
  hp: Int
  visible observer_safe fields (position)
}
"#;

fn byte_range(source: &str, needle: &str) -> Range<usize> {
    let start = source
        .find(needle)
        .unwrap_or_else(|| panic!("source contains {needle:?}"));
    start..start + needle.len()
}

fn byte_range_after(source: &str, anchor: &str, needle: &str) -> Range<usize> {
    let anchor_start = source
        .find(anchor)
        .unwrap_or_else(|| panic!("source contains anchor {anchor:?}"));
    let relative_start = source[anchor_start..]
        .find(needle)
        .unwrap_or_else(|| panic!("source contains {needle:?} after {anchor:?}"));
    let start = anchor_start + relative_start;
    start..start + needle.len()
}

#[test]
fn m6_retains_state_block_observer_safe_visible_fields_and_private_fields_stay_unspecified() {
    let source_path = "tmp/m10/scn01_visibility.mir";
    let ast = parse_surface_v0(FixtureSource::new(source_path, SCN01_VISIBILITY_SOURCE))
        .expect("M10 state visibility extension parses through ordinary M6");

    let state = ast.state("player").expect("indexed player state");
    assert_eq!(state.owner_locus(), "World");
    assert_eq!(
        state.span().byte_range(),
        byte_range(
            SCN01_VISIBILITY_SOURCE,
            "state player[p: Participant] at World {\n  position: Int\n  hp: Int\n  visible observer_safe fields (position)\n}"
        )
    );

    let visibility = state
        .visibility()
        .expect("M6 state block retains explicit observer-safe visibility");
    assert_eq!(visibility.channel(), "observer_safe");
    assert_eq!(
        visibility.span().lexeme(SCN01_VISIBILITY_SOURCE),
        "visible observer_safe fields (position)"
    );
    assert_eq!(visibility.span().file(), source_path);
    assert_eq!(
        visibility
            .fields()
            .iter()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        vec!["position"]
    );

    let visible_position = visibility
        .field("position")
        .expect("position is the only observer-safe field");
    assert_eq!(
        visible_position.span().byte_range(),
        byte_range_after(
            SCN01_VISIBILITY_SOURCE,
            "visible observer_safe fields",
            "position"
        )
    );
    assert!(visibility.field("hp").is_none());

    let hp = state
        .field("hp")
        .expect("private/unlisted field remains present in the state schema");
    assert!(
        hp.visibility().is_none(),
        "M6 must not invent a private visibility marker for unlisted fields"
    );
}

#[test]
fn m6_visibility_requires_nonempty_terminal_field_list() {
    let source = r#"
module M10.Visibility.Empty

locus World
type Player

state player[p: Player] at World {
  hp: Int
  visible observer_safe fields ()
}
"#;

    let diagnostics = parse_surface_v0(FixtureSource::new("tmp/m10/empty_visibility.mir", source))
        .expect_err("Canon M6 visibility grammar requires at least one FieldName");
    let primary = diagnostics.primary();
    assert_eq!(primary.kind(), ParseErrorKind::UnexpectedSyntax);
    assert_eq!(primary.span().lexeme(source), ")");
}

#[test]
fn m6_visibility_is_terminal_and_single_within_state_body() {
    for (path, source, rejected) in [
        (
            "tmp/m10/late_field_after_visibility.mir",
            r#"
module M10.Visibility.LateField

locus World
type Player

state player[p: Player] at World {
  hp: Int
  visible observer_safe fields (hp)
  secret: Int
}
"#,
            "secret",
        ),
        (
            "tmp/m10/second_visibility.mir",
            r#"
module M10.Visibility.Second

locus World
type Player

state player[p: Player] at World {
  hp: Int
  secret: Int
  visible observer_safe fields (hp)
  visible observer_safe fields (secret)
}
"#,
            "visible",
        ),
    ] {
        let diagnostics = parse_surface_v0(FixtureSource::new(path, source))
            .expect_err("visibility declaration is terminal and may appear once");
        let primary = diagnostics.primary();
        assert_eq!(primary.kind(), ParseErrorKind::UnexpectedSyntax);
        assert_eq!(primary.span().lexeme(source), rejected);
    }
}
