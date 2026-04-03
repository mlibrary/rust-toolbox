Refactoring Plan: The 99 Bottles of OOP Methodology

As architects, we must recognize that code is not an end in itself but an economic asset that must be managed for cost-effectiveness and maintainability. This methodology provides a rigorous, test-driven roadmap for transforming rigid procedures into fluid, object-oriented designs.

1. The Dual-Loop Refactoring Framework

Systematic refactoring operates within two distinct loops that balance local correctness with architectural openness.

* The Inner Loop (The Horizontal Path): This is the foundational Red/Green/Refactor cycle. On this path, we scramble horizontally toward functionality, completing a set of concrete examples. The goal is to reach "Shameless Green"—a state where the code works and is easy to understand, even if it is not yet abstract or changeable.
* The Outer Loop (The Vertical Path): Triggered by new requirements (e.g., the "Six-Pack" requirement), this loop focuses on the Open/Closed Principle (OCP). We move vertically into abstractions only when the existing code is not "open" to change. We refactor the structure until the new feature can be added by merely writing new code, rather than modifying the old.

Core Goals of the Framework:

* Understandability: Code must prioritize the reader's comprehension over the writer's cleverness.
* Changeability: The structure should minimize the cost of future modifications.
* Cost-Effectiveness: We resist speculative abstractions, waiting for the code to "insist" upon change to ensure we don't over-invest in the wrong architecture.


--------------------------------------------------------------------------------


2. Phase I: Reaching "Shameless Green" (The Inner Loop)

"Shameless Green" is the most economical solution to a problem. It prioritizes clarity and speed to safety over architectural purity. It is the act of resisting abstractions until you have maximal information.

Shameless Green Criteria Checklist

* [ ] Prioritize understandability over changeability: Do not guess at future needs; focus on making current logic transparent.
* [ ] Use tests to drive comprehension: Use the test suite to document domain behavior.
* [ ] Tolerate temporary duplication: It is cheaper to manage duplication than to recover from a wrong abstraction.

"Quick green excuses all sins." — Kent Beck

TDD Patterns for the Horizontal Path

To reach green bars efficiently, utilize these three patterns:

1. Fake It ('Til You Make It): Return a hard-coded constant to pass the test, then gradually replace it with variables.
2. Obvious Implementation: If the solution is trivial, simply write it. However, if you encounter an error, you must retreat to smaller steps.
3. Triangulate: Use multiple tests to conservatively drive abstraction. By providing several concrete examples, you unearth the common rule.

The execution remains a strict three-part cycle: Setup (define the expected string), Do (invoke verse(n)), and Verify (assert equality).


--------------------------------------------------------------------------------


3. Phase II: Identifying the Need for Change (The Outer Loop)

The transition to the vertical path is a reaction to the "Six-Pack" requirement—a specific catalyst requiring the code to output "1 six-pack" instead of "6 bottles."

The Open/Closed Flowchart

Before modifying logic, evaluate the architecture:

1. Is the code open to the new requirement? If yes, add the feature.
2. If not, do you know how to make it open? If so, rearrange the code first.
3. If not, identify and remove code smells. Systematic removal of flaws will reveal the necessary abstraction.

Primary Code Smells

We target two specific flaws to force openness:

* Duplicated Code: Similar logic in multiple branches.
* Switch Statements/Conditionals: Large case structures that signal a violation of OCP.


--------------------------------------------------------------------------------


4. Phase III: Systematic Refactoring via Flocking Rules

The "Flocking Rules" are the non-negotiable methodology for turning difference into sameness, allowing complex abstractions to emerge from iterative changes.

The Three Flocking Rules

1. Select the things that are most alike.
2. Find the smallest difference between them.
3. Make the simplest change that will remove that difference.

The Four Sub-Steps for Changing Code

To maintain a constant green state, apply changes in this precise order:

1. Parse: Add the new code (e.g., a method definition) but do not use it.
2. Execute: Call the code but ignore the result.
3. Use Result: Replace the old logic with the new code's output.
4. Delete Unused: Remove the deprecated paths.

Gradual Cutover and the Temporary Shim

When changing method signatures, use a temporary shim (e.g., number = :FIXME). This allows the sender and receiver to be updated in separate, safe steps. Once all senders pass the required argument, the default is removed.

The Spreadsheet Technique for Naming

Identify the "Goldilocks" name—not too specific, not too abstract, but a domain-relevant category.

Number	Concrete Example	Domain Abstraction (The Name)
1	"bottle"	container (Right: Domain Category)
6	"six-pack"	container (Specific: "bottle" is wrong)
99	"bottles"	container (Abstract: "unit" is too broad)


--------------------------------------------------------------------------------


5. Phase IV: Converging on Domain Abstractions

As we refactor, we utilize the "Squint Test" to identify "same-shaped" methods. Methods with identical structures (consistent conditionals and arguments) signal that they belong together.

The "Flocked Five" Methods

These methods represent the extracted domain responsibilities:

Method	Domain Responsibility
quantity	Translates a number to its string description (e.g., 0 becomes "no more").
container	Identifies the vessel used (e.g., "bottle" vs "six-pack").
action	The next step in the song (e.g., "Take it down" vs "Go to the store").
pronoun	The unit being consumed ("one" vs "it").
successor	The "next verse" logic, including the 0 to 99 restart.

The Liskov Substitution Principle (LSP)

Abstractions must be trustworthy. If the quantity method returns a String for 0 ("no more") but an Integer for 99 (99), it violates LSP. This forces the caller to check the object's type before calling a method like .capitalize. We must ensure consistent return types (e.g., calling .to_s on integers) so the caller can treat all results as "capitalizable" strings without inspection.


--------------------------------------------------------------------------------


6. Phase V: Evaluation and Metrics

Code quality assessment must move from subjective opinions to measurable facts.

Flog Score Comparison

Solution	SLOC	Total Flog	Worst Bit Flog
Incomprehensibly Concise	19	42.5	36.2
Speculatively General	63	50.6	26.5
Concretely Abstract	92	61.9	14.4
Shameless Green	34	25.6	19.3

Analyzing the Data

Note that the Concretely Abstract solution (Listing 1.3) boasts the lowest "Worst Bit" score, which usually suggests high quality. However, this is deceptive. The metrics approve of its structure, but they cannot detect that its names are at the wrong level of abstraction (implementation-centered rather than domain-centered). Shameless Green remains superior because it is simpler and more honest about its current requirements.


--------------------------------------------------------------------------------


7. Refactoring Best Practices Summary

* Standardize Naming: Name methods after what they represent in the domain (the "what") rather than what they do (the "how").
* Incrementalism: Change one line at a time and run tests after every single change to maintain a constant "Green" state.
* Stability: Seek "stable landing points" by ensuring methods have consistent shapes and return types, making them predictable for the rest of the system.
* Intent over Implementation: Differentiate between the sender's intent and the receiver's implementation. A song method should express the intent to retrieve the entire work, even if it simply delegates to verses(99, 0). This protects senders from changes in how the song is constructed.
