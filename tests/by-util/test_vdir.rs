// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.
use regex::Regex;
use uutests::new_ucmd;
use uutests::util::TestScenario;
use uutests::util_name;

/*
 * As vdir use the same functions than ls, we don't have to retest them here.
 * We just test the default and the column output
*/

#[test]
fn test_vdir() {
    new_ucmd!().succeeds();
}

#[test]
fn test_default_output() {
    let scene = TestScenario::new(util_name!());
    let at = &scene.fixtures;
    at.mkdir("some-dir1");
    at.touch("some-file1");

    scene.ucmd().succeeds().stdout_contains("some-file1");

    scene
        .ucmd()
        .succeeds()
        .stdout_matches(&Regex::new("[rwx-]{10}.*some-file1\n$").unwrap());
}

#[test]
fn test_column_output() {
    let scene = TestScenario::new(util_name!());
    let at = &scene.fixtures;
    at.mkdir("some-dir1");
    at.touch("some-file1");

    scene
        .ucmd()
        .arg("-C")
        .succeeds()
        .stdout_contains("some-file1");

    scene
        .ucmd()
        .arg("-C")
        .succeeds()
        .stdout_does_not_match(&Regex::new("[rwx-]{10}.*some-file1$").unwrap());
}
