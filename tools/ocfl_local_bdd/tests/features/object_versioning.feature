Feature: OCFL Object Versioning
  # Requires ocfl_lib versioning support (add_object_version, list_versions, get_inventory)

  Scenario: Add a new version to an existing object
    Given an empty OCFL repository
    When I add an object with id "obj-v" and content file "file-v1.txt"
    Then the response body is "ok"
    When I add a new version to object "obj-v" with content file "file-v2.txt"
    Then the response body is "ok"
