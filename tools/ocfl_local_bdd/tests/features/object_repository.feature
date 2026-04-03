Feature: OCFL Repository Initialization

  Scenario: Initialize an OCFL repository
    Given an empty OCFL repository
    Then the response body is "ok"

  Scenario: Cannot initialize the same repository twice
    Given an empty OCFL repository
    When I initialize the repository
    Then the response body is not "ok"
