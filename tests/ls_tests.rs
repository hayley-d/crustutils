use crustutils::ls;

#[test]
fn test_ls_integration() {
    crate::ls::list_directory(".");
}
