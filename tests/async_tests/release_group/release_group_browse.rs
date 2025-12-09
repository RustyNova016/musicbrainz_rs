use musicbrainz_rs::entity::release_group::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_group_by_artist() {
    let release_groups_by_svinkels = ReleaseGroup::browse()
        .by_artist("770d490e-c89b-4775-8508-aca7c75142cd")
        .execute_async()
        .await;

    assert!(release_groups_by_svinkels.is_ok());

    let release_groups_by_svinkels = release_groups_by_svinkels.unwrap();

    assert!(release_groups_by_svinkels.count > 1);
    assert_eq!(release_groups_by_svinkels.offset, 0);
    assert!(!release_groups_by_svinkels.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_group_by_release() {
    let release_groups_of_we_want_miles = ReleaseGroup::browse()
        .by_release("84928705-cb99-4c49-a820-73cc460c0bd2")
        .execute_async()
        .await;

    assert!(release_groups_of_we_want_miles.is_ok());

    let release_groups_of_we_want_miles = release_groups_of_we_want_miles.unwrap();

    assert!(release_groups_of_we_want_miles.count > 0);
    assert_eq!(release_groups_of_we_want_miles.offset, 0);
    assert!(!release_groups_of_we_want_miles.entities.is_empty());
}

// TODO: find non empty result
#[tokio::test]
#[serial_test::serial]
async fn should_browse_release_group_by_collection() {
    let releases_group_in_collection = ReleaseGroup::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute_async()
        .await;

    assert!(releases_group_in_collection.is_ok());
}
