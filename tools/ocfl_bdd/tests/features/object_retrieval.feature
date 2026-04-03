Feature: OCFL Object Retrieval
  As a user
  I want to retrieve objects from the repository
  So that I can access stored content

  Background:
    Given the OCFL endpoint is running

  Scenario: Retrieve an object from the repository
    Given the repository is initialized
    And a source file exists at "/tmp/bdd_test_obj.txt"
    And object "obj-get" has been added from "/tmp/bdd_test_obj.txt"
    And the file "/tmp/bdd_test_obj_out.txt" does not exist
    When I GET "/get?object_id=obj-get&dest_path=/tmp/bdd_test_obj_out.txt"
    Then the response body is "ok"
    And the file "/tmp/bdd_test_obj_out.txt" exists and contains "hello"

