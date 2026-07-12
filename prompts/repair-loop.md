# The clilint repair loop

A procedure for driving a CLI to conformance with `clilint`. Follow it verbatim.

1. **Run the check as JSON.**
   `./clilint check <target> --format json --output clilint-report.json`
2. **Parse the findings.** Collect every finding with `status == "fail"`, then
   every one with `status == "warn"`. Ignore `pass` and `skip`.
3. **Apply fixes.** For each collected finding, apply its `remediation` field;
   read `evidence` to understand why it tripped. Group related fixes — handle all
   `help` findings together, all `error` findings together, and so on. Fix
   `fail` (error-severity first) before `warn`.
4. **Re-run and repeat.** Run `clilint check` again. Keep looping until there are
   no `fail` findings and the score is at or above your gate. Use
   `--min-score 90` as a reasonable target.
5. **Stop** when `./clilint check <target> --min-score 90` exits `0`.

## Compact loop

```bash
target="$1"
max_iter=5           # the loop MUST be bounded
for i in $(seq 1 "$max_iter"); do
  if ./clilint check "$target" --min-score 90 >/dev/null 2>&1; then
    echo "clilint passed on iteration $i"
    exit 0
  fi
  ./clilint check "$target" --format json --output clilint-report.json
  # Read clilint-report.json: for each finding with status "fail" then "warn",
  # apply its `remediation` and edit the CLI source. Then loop to re-check.
done
echo "clilint did not converge after $max_iter iterations" >&2
exit 1
```

## Terminate the loop

The loop must terminate. Cap iterations (e.g. 5). If the score stops improving
between rounds, or the same finding keeps recurring after you applied its
remediation, stop and report the remaining findings instead of churning — some
rules may need a human decision or a design change the loop cannot make.
