"""End-to-end tests for clilint, run against the bundled fixtures.

These invoke the ``clilint`` launcher as a subprocess (true end-to-end) and assert
the score, exit code, and report shape. Run with: ``python -m unittest discover tests``.
"""

import json
import subprocess
import sys
import unittest
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
CLILINT = REPO / "clilint"
GOOD = REPO / "fixtures" / "good-cli" / "greet"
BAD = REPO / "fixtures" / "bad-cli" / "badtool"


def run_clilint(*args):
    proc = subprocess.run(
        [sys.executable, str(CLILINT), *args],
        capture_output=True,
        text=True,
    )
    return proc


def check_json(target, *extra):
    proc = run_clilint("check", str(target), "--format", "json", *extra)
    return proc, json.loads(proc.stdout)


class TestConformance(unittest.TestCase):
    def test_good_fixture_scores_100_and_exits_0(self):
        proc, report = check_json(GOOD)
        self.assertEqual(report["score"], 100)
        self.assertEqual(report["summary"]["fail"], 0)
        self.assertEqual(proc.returncode, 0)

    def test_bad_fixture_has_failures_and_exits_nonzero(self):
        proc, report = check_json(BAD)
        self.assertGreater(report["summary"]["fail"], 0)
        self.assertLess(report["score"], 50)
        self.assertNotEqual(proc.returncode, 0)

    def test_clilint_dogfoods_itself(self):
        _, report = check_json(CLILINT)
        self.assertGreaterEqual(report["score"], 90)

    def test_report_has_required_keys(self):
        _, report = check_json(GOOD)
        for key in ("clilint_version", "standard", "profile", "target", "score", "summary", "findings"):
            self.assertIn(key, report)
        self.assertEqual(len(report["findings"]), report["summary"]["total"])

    def test_modern_profile_excludes_agent_rules(self):
        _, report = check_json(BAD, "--profile", "modern-cli")
        rules = {f["rule"] for f in report["findings"]}
        self.assertNotIn("CLI-AGENT-001", rules)
        self.assertNotIn("CLI-AGENT-002", rules)

    def test_min_score_gate(self):
        proc = run_clilint("score", str(GOOD), "--min-score", "101")
        self.assertNotEqual(proc.returncode, 0)  # 100 < 101 → gate fails

    def test_explain_known_and_unknown(self):
        ok = run_clilint("explain", "CLI-OUTPUT-001")
        self.assertEqual(ok.returncode, 0)
        self.assertIn("CLI-OUTPUT-001", ok.stdout)
        missing = run_clilint("explain", "CLI-NOPE-999")
        self.assertNotEqual(missing.returncode, 0)


if __name__ == "__main__":
    unittest.main()
