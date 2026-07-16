#[test]
fn intentional_release_failure_blocks_auto_merge() {
    panic!("intentional failure: verifies required checks block a release pull request");
}
