# Concise default help rubric

This rubric applies when a CLI program requires arguments and is not
interactive by default.

The evidence contains the observations produced by running the target without
arguments and with `--help`.

## Mechanistic findings

Issue an error finding when the default output:

- is empty;
- contains no example invocation; or
- does not tell the caller how to request full help.

Each finding cites the observation and the relevant output.

## Agent judgment

Judge whether:

- the description says what the program helps the caller do;
- the examples teach one or two likely tasks; and
- the amount of default output is concise enough to scan before taking the
  next action.

Issue a warning or error finding for each meaningful gap. State what the output
currently says, what the rubric calls for, and which evidence supports the
judgment.

## Overall rating

- `poor`: The default output gives little useful direction.
- `minimal`: The caller can discover the syntax but lacks important guidance.
- `acceptable`: The output describes the program, shows a likely task, and
  points to full help.
- `good`: The output makes the likely next action clear with little irrelevant
  text.
- `excellent`: The output is unusually effective without becoming verbose.

The rating vocabulary is experimental. Question 16 has not settled whether the
shared assessment uses categories or numbers.
