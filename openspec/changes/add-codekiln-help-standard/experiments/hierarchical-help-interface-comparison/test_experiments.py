from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import unittest


ROOT = Path(__file__).parent


def run(script: str, *arguments: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(ROOT / script), *arguments],
        check=False,
        capture_output=True,
        text=True,
    )


class PostfixPrototypeTests(unittest.TestCase):
    def test_help_subcommand_and_help_flag_share_a_document(self) -> None:
        subcommand = run("postfix.py", "repo", "clone", "help")
        flag = run("postfix.py", "repo", "clone", "--help")
        self.assertEqual(subcommand.returncode, 0)
        self.assertEqual(subcommand.stdout, flag.stdout)

    def test_exact_heading_level_is_distinct_from_maximum_level(self) -> None:
        exact = run("postfix.py", "repo", "clone", "help", "outline", "--level", "3")
        maximum = run(
            "postfix.py",
            "repo",
            "clone",
            "help",
            "outline",
            "--max-level",
            "3",
        )
        self.assertTrue(all(line.startswith("H3") for line in exact.stdout.splitlines()))
        self.assertIn("H1", maximum.stdout)
        self.assertIn("H2", maximum.stdout)
        self.assertIn("H3", maximum.stdout)

    def test_direct_section_excludes_child_section(self) -> None:
        direct = run(
            "postfix.py",
            "repo",
            "clone",
            "help",
            "section",
            "kiln-demo-repo-clone.examples.private-repository",
        )
        recursive = run(
            "postfix.py",
            "repo",
            "clone",
            "help",
            "section",
            "kiln-demo-repo-clone.examples.private-repository",
            "--recursive",
        )
        self.assertNotIn("Required permission", direct.stdout)
        self.assertIn("Required permission", recursive.stdout)

    def test_command_named_view_is_unambiguous(self) -> None:
        result = run("postfix.py", "view", "help")
        self.assertIn("# kiln-demo view", result.stdout)

    def test_json_outline_returns_copyable_identifiers(self) -> None:
        result = run(
            "postfix.py",
            "repo",
            "clone",
            "help",
            "outline",
            "--format",
            "json",
        )
        body = json.loads(result.stdout)
        self.assertEqual(body["command"], ["repo", "clone"])
        self.assertTrue(all(item["id"] for item in body["sections"]))


class ResourcePrototypeTests(unittest.TestCase):
    def test_address_selects_one_section(self) -> None:
        result = run(
            "resource.py",
            "get",
            "repo/clone#kiln-demo-repo-clone.examples.public-repository",
        )
        self.assertIn("Public repository", result.stdout)
        self.assertNotIn("Private repository", result.stdout)

    def test_agent_document_composes_inherited_guidance(self) -> None:
        result = run(
            "resource.py",
            "get",
            "repo/clone",
            "--audience",
            "agent",
        )
        self.assertIn("General automation", result.stdout)
        self.assertIn("Repository access", result.stdout)
        self.assertIn("Clone side effects", result.stdout)

    def test_structured_get_carries_markdown_and_document_digest(self) -> None:
        result = run(
            "resource.py",
            "get",
            "repo/clone#kiln-demo-repo-clone.options",
            "--format",
            "json",
        )
        body = json.loads(result.stdout)
        self.assertEqual(body["content_format"], "markdown")
        self.assertTrue(body["document_digest"].startswith("sha256:"))


if __name__ == "__main__":
    unittest.main()
