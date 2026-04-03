Feature: OCFL Object Deletion
  # Requires ocfl_lib deletion support (delete_object, delete_object_version)

  Scenario: Delete an object from the repository
    Given an empty OCFL repository
    And object "obj-del" has been added with content "to delete"
    When I delete object "obj-del"
    Then the response body is "ok"
    And the object "obj-del" should not exist in the repository
