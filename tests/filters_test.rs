mod helpers;

use helpers::in_directory as cwd;

#[test]
fn can_convert_table_to_csv_text_and_from_csv_text_back_into_table() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open caco3_plastics.csv | to-csv | from-csv | first 1 | get origin | echo $it"
    );

    assert_eq!(output, "SPAIN");
}

#[test]
fn can_convert_table_to_json_text_and_from_json_text_back_into_table() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open sgml_description.json | to-json | from-json | get glossary.GlossDiv.GlossList.GlossEntry.GlossSee | echo $it"
    );

    assert_eq!(output, "markup");
}

#[test]
fn can_convert_table_to_toml_text_and_from_toml_text_back_into_table() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open cargo_sample.toml | to-toml | from-toml | get package.name | echo $it"
    );

    assert_eq!(output, "nu");
}

#[test]
fn can_convert_table_to_yaml_text_and_from_yaml_text_back_into_table() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open appveyor.yml | to-yaml | from-yaml | get environment.global.PROJECT_NAME | echo $it"
    );

    assert_eq!(output, "nushell");
}

#[test]
fn can_sort_by_column() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open cargo_sample.toml --raw | lines | skip 1 | first 4 | split-column \"=\" | sort-by Column1 | skip 1 | first 1 | get Column1 | trim | echo $it"
    );

    assert_eq!(output, "description");
}

#[test]
fn can_sort_by_column_reverse() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open cargo_sample.toml --raw | lines | skip 1 | first 4 | split-column \"=\" | sort-by Column1 --reverse | skip 1 | first 1 | get Column1 | trim | echo $it"
    );

    assert_eq!(output, "name");
}

#[test]
fn can_split_by_column() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open cargo_sample.toml --raw | lines | skip 1 | first 1 | split-column \"=\" | get Column1 | trim | echo $it"
    );

    assert_eq!(output, "name");
}

#[test]
fn can_sum() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "open sgml_description.json | get glossary.GlossDiv.GlossList.GlossEntry.Sections | sum | echo $it"
    );

    assert_eq!(output, "203")
}

#[test]
fn can_filter_by_unit_size_comparison() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "ls | where size > 1kb | sort-by size | get name | skip 1 | trim | echo $it"
    );

    assert_eq!(output, "caco3_plastics.csv");
}

#[test]
fn can_get_last() {
    nu!(
        output,
        cwd("tests/fixtures/formats"),
        "ls | sort-by name | last 1 | get name | trim | echo $it"
    );

    assert_eq!(output, "utf16.ini");
}
