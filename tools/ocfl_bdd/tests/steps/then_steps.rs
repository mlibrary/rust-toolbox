use cucumber::then;

use super::OcflWorld;

#[then(expr = "the response body is {string}")]
async fn response_body_is(world: &mut OcflWorld, expected: String) {
    let actual = world
        .last_response_text
        .as_deref()
        .expect("no response recorded");
    // The endpoint returns a JSON string, e.g. `"ok"` (with quotes).
    assert_eq!(actual, format!("\"{expected}\""));
}

#[then(expr = "the response body is not {string}")]
async fn response_body_is_not(world: &mut OcflWorld, not_expected: String) {
    let actual = world
        .last_response_text
        .as_deref()
        .expect("no response recorded");
    // The endpoint returns a JSON string, e.g. `"ok"` (with quotes).
    assert_ne!(actual, format!("\"{not_expected}\""));
}

#[then(expr = "the object list contains {string}")]
async fn object_list_contains(world: &mut OcflWorld, object_id: String) {
    assert!(
        world.last_object_list.contains(&object_id),
        "expected '{object_id}' in list, got {:?}",
        world.last_object_list
    );
}

#[then("the object list is empty")]
async fn object_list_is_empty(world: &mut OcflWorld) {
    assert!(
        world.last_object_list.is_empty(),
        "expected empty list, got {:?}",
        world.last_object_list
    );
}
