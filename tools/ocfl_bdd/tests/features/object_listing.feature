Feature: OCFL Object Listing
  As a user
  I want to list objects in the repository
  So that I can see what is stored

  Background:
    Given the OCFL endpoint is running

  Scenario: List objects after adding one
    Given the repository is initialized
    And object "obj-001" has been added from "/tmp/bdd_test_obj.txt"
    When I GET "/list"
    Then the object list contains "obj-001"

  Scenario: List an empty repository
    Given the repository is initialized
    When I GET "/list"
    Then the object list is empty

