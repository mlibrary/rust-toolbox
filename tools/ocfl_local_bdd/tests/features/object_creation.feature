Feature: OCFL Object Creation

  Scenario: Add an object to the repository
    Given an empty OCFL repository
    When I add an object with id "obj-001" and content file "v1.txt"
    Then the response body is "ok"
    And the object "obj-001" should exist in the repository

  Scenario: Cannot add the same object twice
    Given an empty OCFL repository
    When I add an object with id "obj-dup" and content file "v1.txt"
    Then the response body is "ok"
    When I add an object with id "obj-dup" and content file "v2.txt"
    Then the response body is not "ok"

  Scenario: Cannot add object to uninitialised repository
    When I add an object with id "obj-norepo" and content file "v1.txt"
    Then the response body is not "ok"
