Feature: OCFL Object Creation
  As a user
  I want to add objects to the repository and handle errors
  So that I can store content and avoid duplicates or missing files

  Background:
    Given the OCFL endpoint is running

  Scenario: Add an object to the repository
    Given the repository is initialized
    And a source file exists at "/tmp/bdd_test_obj.txt"
    When I add object "obj-001" from src_path "/tmp/bdd_test_obj.txt"
    Then the response body is "ok"

  Scenario: Add the same object twice
    Given the repository is initialized
    And a source file exists at "/tmp/bdd_test_obj.txt"
    When I add object "obj-dup" from src_path "/tmp/bdd_test_obj.txt"
    Then the response body is "ok"
    When I add object "obj-dup" from src_path "/tmp/bdd_test_obj.txt"
    Then the response body is not "ok"

  Scenario: Add object with missing source file
    Given the repository is initialized
    When I add object "obj-missing" from src_path "/tmp/does_not_exist.txt"
    Then the response body is not "ok"

