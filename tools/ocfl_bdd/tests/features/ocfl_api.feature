Feature: OCFL HTTP API

  Background:
    Given the OCFL endpoint is running

  Scenario: Initialize a repository
    When I POST to "/init"
    Then the response body is "ok"

  Scenario: Add an object to the repository
    Given the repository is initialized
    And a source file exists at "/tmp/bdd_test_obj.txt"
    When I add object "obj-001" from src_path "/tmp/bdd_test_obj.txt"
    Then the response body is "ok"

  Scenario: List objects after adding one
    Given the repository is initialized
    And object "obj-001" has been added from "/tmp/bdd_test_obj.txt"
    When I GET "/list"
    Then the object list contains "obj-001"

  Scenario: List an empty repository
    Given the repository is initialized
    When I GET "/list"
    Then the object list is empty

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

  Scenario: Initialize repository twice
    Given the repository is initialized
    When I POST to "/init"
    Then the response body is not "ok"
