Feature: OCFL Object Retrieval

  Scenario: Retrieve an object from the repository
    Given an empty OCFL repository
    And object "obj-get" has been added with content "hello"
    When I retrieve object "obj-get" to path "/tmp/ocfl_local_bdd_retrieved.txt"
    Then the response body is "ok"
    And the file "/tmp/ocfl_local_bdd_retrieved.txt" exists and contains "hello"

  Scenario: Cannot retrieve a non-existent object
    Given an empty OCFL repository
    When I retrieve object "obj-missing" to path "/tmp/ocfl_local_bdd_missing.txt"
    Then the response body is not "ok"
