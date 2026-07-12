"""Spawn a target executable and capture how it behaves.

Probes never call ``subprocess`` directly; they drive a :class:`Target`, which
runs the executable with a closed stdin (so a tool that reads stdin gets EOF
rather than hanging the linter), captures stdout/stderr/exit code/timing, and
memoizes identical invocations so repeated probes are cheap.
"""

from __future__ import annotations

import os
import re
import subprocess
import time
from dataclasses import dataclass, field
from typing import Mapping, Sequence

# Matches ANSI SGR (color / style) escape sequences.
ANSI_RE = re.compile(r"\x1b\[[0-9;]*m")


def has_ansi(text: str) -> bool:
    """True if ``text`` contains ANSI color/style escape sequences."""
    return bool(ANSI_RE.search(text or ""))


@dataclass
class Invocation:
    """The captured result of running the target once."""

    args: list
    exit_code: int | None  # None when the process timed out
    stdout: str
    stderr: str
    duration_ms: float
    timed_out: bool

    def as_evidence(self, max_len: int = 400) -> dict:
        """A compact, JSON-friendly summary suitable for a finding's evidence."""
        return {
            "args": self.args,
            "exit_code": self.exit_code,
            "timed_out": self.timed_out,
            "duration_ms": round(self.duration_ms, 1),
            "stdout_len": len(self.stdout),
            "stderr_len": len(self.stderr),
            "stdout_head": self.stdout[:max_len],
            "stderr_head": self.stderr[:max_len],
        }


class Target:
    """An executable under test, invoked with a closed stdin and a timeout."""

    def __init__(self, argv: Sequence[str], timeout: float = 10.0, cwd: str | None = None):
        # argv is the base command, e.g. ["/path/to/tool"] or ["python", "tool.py"].
        self.argv = list(argv)
        self.timeout = timeout
        self.cwd = cwd
        self._cache: dict = {}

    def run(
        self,
        args: Sequence[str] = (),
        env: Mapping[str, str] | None = None,
        timeout: float | None = None,
    ) -> Invocation:
        args = list(args)
        env_over = dict(env or {})
        key = (tuple(args), tuple(sorted(env_over.items())), timeout)
        if key in self._cache:
            return self._cache[key]

        full_env = os.environ.copy()
        full_env.update({k: str(v) for k, v in env_over.items()})
        limit = self.timeout if timeout is None else timeout

        start = time.monotonic()
        timed_out = False
        try:
            proc = subprocess.run(
                self.argv + args,
                input="",  # closed stdin: never inherit the terminal, never hang
                capture_output=True,
                text=True,
                env=full_env,
                timeout=limit,
                cwd=self.cwd,
            )
            code, out, err = proc.returncode, proc.stdout, proc.stderr
        except subprocess.TimeoutExpired as exc:
            timed_out = True
            code = None
            out = _to_text(exc.stdout)
            err = _to_text(exc.stderr)
        duration_ms = (time.monotonic() - start) * 1000.0

        inv = Invocation(args, code, out, err, duration_ms, timed_out)
        self._cache[key] = inv
        return inv


def _to_text(value) -> str:
    if value is None:
        return ""
    if isinstance(value, bytes):
        return value.decode(errors="replace")
    return value
