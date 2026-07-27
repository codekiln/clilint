# Checker CLI contract comparison

This experiment tests whether every bundle-owned Checker can be a CLI without
making Clilint manage the Checker's internal tools or resources.

It asks:

1. Can a Checker CLI run from the project directory and still find resources
   installed with the Checker?
2. Can mechanistic and judgment-based Checks use the same Check Request and
   Check Outcome boundary?
3. Can a judgment-based Checker keep an Agent Skill behind its CLI interface
   instead of requiring Clilint to support an Agent Skill checker type?
4. Does the CLI boundary preserve separate standard output and standard error?

## Prototypes

`mechanistic-check/check.py` invokes the fixture CLI and checks whether its
default output tells the caller to use `--help`. It locates its expectation
text beside the Checker rather than through the process working directory.

`judgment-check/check.py` reads the fixture project's README and locates its
Skill and rubric beside the Checker. With no Assessment, it returns an
Awaiting Assessment outcome. When given the recorded Assessment, the same CLI
validates the request binding and returns a Check Result.

Both Checkers:

- run with `fixture-project/` as their current directory;
- read one versioned Check Request from standard input;
- write one versioned Check Outcome to standard output;
- write operational logs to standard error; and
- provide `--help` and `--version`.

The recorded Assessment stands in for an external agent. This experiment tests
the handoff shape, not the quality of a model's judgment.

## Run

From this directory:

```sh
python3 compare.py
```

The prototypes use only the Python standard library.

## Result

See [findings.md](findings.md).
