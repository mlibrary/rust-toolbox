Feature: OCFL Object and Version Deletion
  As a repository manager
  I want to delete entire objects or specific versions
  So that I can manage storage and comply with retention policies

  Scenario: Delete an entire OCFL object
    Given an OCFL repository with object "obj-del" at version "v2"
    When I delete object "obj-del"
    Then the object "obj-del" should not exist in the repository
    And the inventory for object "obj-del" should not exist

  Scenario: Delete a specific version of an OCFL object
    Given an OCFL repository with object "obj-delver" at version "v2"
    When I delete version "v1" of object "obj-delver"
    Then the inventory for object "obj-delver" should indicate version "v2"
    And the inventory for object "obj-delver" should not list version "v1"
    And the content for version "v1" of object "obj-delver" should not exist

