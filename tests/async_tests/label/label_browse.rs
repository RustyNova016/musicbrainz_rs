use musicbrainz_rs::entity::label::*;
use musicbrainz_rs::prelude::*;

#[tokio::test]
#[serial_test::serial]
async fn should_browse_label_by_area() {
    let labels_in_paris = Label::browse()
        .by_area("dc10c22b-e510-4006-8b7f-fecb4f36436e")
        .execute_async()
        .await;

    assert!(labels_in_paris.is_ok());

    let labels_in_paris = labels_in_paris.unwrap();

    assert!(labels_in_paris.count > 1);
    assert_eq!(labels_in_paris.offset, 0);
    assert!(!labels_in_paris.entities.is_empty());
}

#[tokio::test]
#[serial_test::serial]
async fn should_browse_label_by_release() {
    let label_of_justice_cross_release = Label::browse()
        .by_release("4642ee19-7790-3c8d-ab5e-d133de942db6")
        .execute_async()
        .await;

    assert!(label_of_justice_cross_release.is_ok());

    let label_of_justice_cross_release = label_of_justice_cross_release.unwrap();

    assert!(label_of_justice_cross_release.count > 1);
    assert_eq!(label_of_justice_cross_release.offset, 0);
    assert!(!label_of_justice_cross_release.entities.is_empty());
}

// TODO: find non empty result
#[tokio::test]
#[serial_test::serial]
async fn should_browse_label_by_collection() {
    let label_on_collection = Label::browse()
        .by_collection("91565a03-bce8-47e9-ab70-e4d4e1684d7f")
        .execute_async()
        .await;

    assert!(label_on_collection.is_ok());

    let _label_on_collection = label_on_collection.unwrap();
}
