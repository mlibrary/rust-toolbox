Feature: OCFL Object Listing

  Scenario: List objects in an empty repository
    Given an empty OCFL repository
    When I list all objects
    Then the object list is empty

  Scenario: List objects after adding one
    Given an empty OCFL repository
    When I add an object with id "obj-001" and content file "v1.txt"
    And I list all objects
    Then the object list contains "obj-001"

  Scenario: List objects after adding multiple
    Given an empty OCFL repository
    When I add an object with id "obj-001" and content file "v1.txt"
    And I add an object with id "obj-002" and content file "v2.txt"
    And I list all objects
    Then the object list contains "obj-001"
    And the object list contains "obj-002"
