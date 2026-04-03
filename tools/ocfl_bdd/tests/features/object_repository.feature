Feature: OCFL Repository Initialization
  As a user
  I want to initialize a repository and handle re-initialization
  So that I can start with a valid OCFL repo and handle errors

  Background:
    Given the OCFL endpoint is running

  Scenario: Initialize a repository
    When I POST to "/init"
    Then the response body is "ok"

  Scenario: Initialize repository twice
    Given the repository is initialized
    When I POST to "/init"
    Then the response body is not "ok"

