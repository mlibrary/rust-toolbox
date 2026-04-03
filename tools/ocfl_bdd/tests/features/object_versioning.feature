Feature: OCFL Object Versioning and Inventory
  As a user of the OCFL API
  I want to create, update, and retrieve object versions with inventory.json
  So that I can manage and audit object version history per the OCFL 1.1 specification

  Scenario: Create a new OCFL object with version 1 and inventory
    Given an empty OCFL repository
    When I add an object with id "obj-100" and content file "file-v1.txt"
    Then the object "obj-100" should exist in the repository
    And the inventory for object "obj-100" should exist
    And the inventory for object "obj-100" should indicate version "v1"

  Scenario: Add a new version to an existing object
    Given an OCFL repository with object "obj-100" at version "v1"
    When I add a new version to object "obj-100" with content file "file-v2.txt"
    Then the inventory for object "obj-100" should indicate version "v2"
    And the inventory for object "obj-100" should list both versions "v1" and "v2"

  Scenario: Retrieve inventory.json for an object
    Given an OCFL repository with object "obj-100" at version "v2"
    When I retrieve the inventory for object "obj-100"
    Then the inventory should be valid per OCFL 1.1 spec
    And the inventory should list all versions and content digests

  Scenario: List all versions for an object
    Given an OCFL repository with object "obj-100" at version "v2"
    When I list versions for object "obj-100"
    Then the result should include "v1" and "v2"

