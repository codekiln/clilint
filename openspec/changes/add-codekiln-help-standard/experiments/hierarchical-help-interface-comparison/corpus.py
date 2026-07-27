"""Shared Markdown corpus and structural helpers for the help experiments."""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
import re
from typing import Iterable


CommandPath = tuple[str, ...]


PAGES: dict[CommandPath, str] = {
    (): """# kiln-demo

Work with hosted repositories.

## Commands

Use `repo` to work with repositories and `view` to inspect an item.

## Examples

### List repositories

Run `kiln-demo repo list`.

### Open an item

Run `kiln-demo view 42`.
""",
    ("repo",): """# kiln-demo repo

Work with repositories.

## Commands

Use `clone` to make a local copy or `list` to find repositories.

## Examples

### Clone a repository

Run `kiln-demo repo clone OWNER/NAME`.
""",
    ("repo", "clone"): """# kiln-demo repo clone

Clone a repository into a new directory.

## Synopsis

`kiln-demo repo clone OWNER/NAME [DIRECTORY]`

## Options

`--depth N` limits downloaded history.

## Examples

### Public repository

`kiln-demo repo clone codekiln/example`

### Private repository

`kiln-demo repo clone codekiln/private`

#### Required permission

The current credentials need read access to the repository.

## Exit codes

The command exits with status zero after a successful clone.
""",
    ("repo", "list"): """# kiln-demo repo list

List repositories visible to the current credentials.

## Output

The default output is a table. Use `--json` for structured output.
""",
    ("view",): """# kiln-demo view

View one item by number.

## Example

`kiln-demo view 42`
""",
}


AGENT_PAGES: dict[CommandPath, str] = {
    (): """# kiln-demo agent help

Use structured output for automation.

## Non-interactive operation

Commands close standard input and do not start a pager when stdout is not a
terminal.

## Permissions

Commands can use locally configured credentials.
""",
    ("repo",): """# kiln-demo repo agent help

Repository commands may contact the configured hosting service.

## Discovery

Use `kiln-demo repo list --json` to discover repository identifiers.
""",
    ("repo", "clone"): """# kiln-demo repo clone agent help

Clone writes a new directory and contacts the repository host.

## Side effects

The destination directory is created when cloning begins.

## Recovery

Remove an incomplete destination before retrying.
""",
    ("repo", "list"): """# kiln-demo repo list agent help

Use `--json` and select explicit fields for stable automation.
""",
    ("view",): """# kiln-demo view agent help

Use `--json` to avoid terminal-oriented rendering.
""",
}


AGENT_FRAGMENTS: dict[CommandPath, str] = {
    (): """### General automation

Structured output is preferred. Standard input is closed when no input is
declared.
""",
    ("repo",): """### Repository access

Repository commands may use locally configured credentials and contact the
configured host.
""",
    ("repo", "clone"): """### Clone side effects

Clone creates a destination directory. Remove an incomplete destination before
retrying.
""",
    ("repo", "list"): """### List output

Use `--json` and select explicit fields.
""",
    ("view",): """### View output

Use `--json` to avoid terminal-oriented rendering.
""",
}


WEB_URLS: dict[CommandPath, str] = {
    path: "https://docs.example.test/kiln-demo/1.0/"
    + ("/".join(path) if path else "index")
    for path in PAGES
}


@dataclass(frozen=True)
class Section:
    id: str
    title: str
    level: int
    parent: str | None
    heading_start: int
    body_start: int
    subtree_end: int
    direct_end: int


HEADING = re.compile(r"^(#{1,6})[ \t]+(.+?)[ \t]*#*[ \t]*$", re.MULTILINE)
NON_SLUG = re.compile(r"[^a-z0-9]+")


def slug(text: str) -> str:
    value = NON_SLUG.sub("-", text.lower()).strip("-")
    return value or "section"


def sections(markdown: str) -> list[Section]:
    """Parse ATX headings and give each one a unique hierarchical identifier."""
    matches = list(HEADING.finditer(markdown))
    partial: list[dict[str, object]] = []
    stack: list[tuple[int, str]] = []
    used: dict[str, int] = {}

    for index, match in enumerate(matches):
        level = len(match.group(1))
        title = match.group(2).strip()
        while stack and stack[-1][0] >= level:
            stack.pop()
        parent = stack[-1][1] if stack else None
        base = f"{parent}.{slug(title)}" if parent else slug(title)
        used[base] = used.get(base, 0) + 1
        section_id = base if used[base] == 1 else f"{base}-{used[base]}"
        stack.append((level, section_id))

        subtree_end = len(markdown)
        direct_end = len(markdown)
        for later in matches[index + 1 :]:
            later_level = len(later.group(1))
            if direct_end == len(markdown) and later_level > level:
                direct_end = later.start()
            if later_level <= level:
                subtree_end = later.start()
                if direct_end == len(markdown):
                    direct_end = later.start()
                break

        partial.append(
            {
                "id": section_id,
                "title": title,
                "level": level,
                "parent": parent,
                "heading_start": match.start(),
                "body_start": match.end(),
                "subtree_end": subtree_end,
                "direct_end": direct_end,
            }
        )

    return [Section(**entry) for entry in partial]  # type: ignore[arg-type]


def section_text(markdown: str, section_id: str, recursive: bool) -> str:
    for item in sections(markdown):
        if item.id == section_id:
            end = item.subtree_end if recursive else item.direct_end
            return markdown[item.heading_start : end].strip() + "\n"
    raise KeyError(section_id)


def command_label(path: CommandPath) -> str:
    return "kiln-demo" + (" " + " ".join(path) if path else "")


def command_address(path: CommandPath) -> str:
    return "/".join(path) if path else "."


def parse_address(value: str) -> tuple[CommandPath, str | None]:
    document, separator, section_id = value.partition("#")
    path = () if document in ("", ".") else tuple(part for part in document.split("/") if part)
    return path, section_id if separator else None


def inherited_agent_document(path: CommandPath) -> str:
    """Compose the human page with inherited agent guidance."""
    fragments: list[str] = []
    for length in range(len(path) + 1):
        fragment = AGENT_FRAGMENTS.get(path[:length])
        if fragment:
            fragments.append(fragment.strip())
    return (
        PAGES[path].rstrip()
        + "\n\n## Agent guidance\n\n"
        + "\n\n".join(fragments)
        + "\n"
    )


def digest(markdown: str) -> str:
    return "sha256:" + hashlib.sha256(markdown.encode("utf-8")).hexdigest()


def iter_documents(agent: bool, inherited: bool = False) -> Iterable[tuple[CommandPath, str]]:
    for path in sorted(PAGES):
        if agent and inherited:
            yield path, inherited_agent_document(path)
        elif agent:
            yield path, AGENT_PAGES[path]
        else:
            yield path, PAGES[path]
