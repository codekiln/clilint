# Findings from the Checker CLI contract comparison

## Result

Both prototypes ran from the fixture project directory and found their own
installed resources without using the bundle directory as their current
directory.

The mechanistic and judgment-based prototypes used the same outer exchange:

```text
Check Request on standard input
             |
             v
        Checker CLI
             |
             v
Check Outcome on standard output
```

The judgment-based CLI also supported a two-pass exchange:

```text
Checker CLI returns Awaiting Assessment
             |
             v
external agent follows the bundled Skill and rubric
             |
             v
Checker CLI validates the Assessment and returns a Check Result
```

This prototype did not require Clilint to treat an Agent Skill as a separate
checker implementation. The Checker CLI owned evidence gathering, Skill and
rubric discovery, Assessment validation, and production of the Check Outcome.

## What the experiment supports

- A bundle-owned Checker can be a CLI.
- Clilint can run the Checker from the directory in which the user invoked
  Clilint, normally the tested project directory.
- Clilint needs to resolve and invoke the Checker CLI, but it does not need to
  use the bundle directory as the Checker's current directory.
- A Checker CLI can locate its own installed resources relative to its
  executable or language package.
- Mechanistic and judgment-based Checks can share the same Check Request and
  Check Outcome boundary.
- An Agent Skill can be a resource used behind a judgment-based Checker CLI.
- Standard output can remain protocol output while standard error carries
  Checker logs.

## What remains open

The experiment does not settle:

- how a check bundle installs or names a Checker CLI on every operating system;
- whether an Assessment handoff uses a file, a command, or another agent
  harness integration;
- which environment variables the Checker inherits;
- timeout, output, and log-retention defaults; or
- whether one Checker CLI may expose several Checks in a later protocol.

## Project-owner decision tested

`codekiln` chose a CLI as the first required interface for every bundle-owned
Checker. The experiment supports that decision. It does not change the logical
scope of a Check: one Check may still gather evidence once and verify several
related matters.
