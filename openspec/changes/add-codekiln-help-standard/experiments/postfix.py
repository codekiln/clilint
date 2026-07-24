#!/usr/bin/env python3
"""Prototype A: a help subcommand beneath every command path."""

from __future__ import annotations

import argparse
import json
import sys

from corpus import (
    AGENT_PAGES,
    PAGES,
    WEB_URLS,
    command_address,
    command_label,
    digest,
    iter_documents,
    section_text,
    sections,
)


OPERATIONS = {"outline", "section", "tree", "search", "view"}


def parse_query(arguments: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("operation", nargs="?", choices=sorted(OPERATIONS))
    parser.add_argument("value", nargs="?")
    parser.add_argument("--agent", action="store_true")
    parser.add_argument("--format", choices=("markdown", "json"), default="markdown")
    levels = parser.add_mutually_exclusive_group()
    levels.add_argument("--level", type=int)
    levels.add_argument("--max-level", type=int)
    parser.add_argument("--recursive", action="store_true")
    parser.add_argument("--depth", type=int)
    parser.add_argument("--web", action="store_true")
    return parser.parse_args(arguments)


def choose_document(path: tuple[str, ...], agent: bool) -> str:
    source = AGENT_PAGES if agent else PAGES
    try:
        return source[path]
    except KeyError as error:
        raise SystemExit(f"unknown command path: {command_label(path)}") from error


def emit_outline(path: tuple[str, ...], markdown: str, query: argparse.Namespace) -> None:
    selected = []
    for item in sections(markdown):
        if query.level is not None and item.level != query.level:
            continue
        if query.max_level is not None and item.level > query.max_level:
            continue
        selected.append(item)

    if query.format == "json":
        print(
            json.dumps(
                {
                    "schema_version": 1,
                    "command": list(path),
                    "audience": "agent" if query.agent else "human",
                    "document_digest": digest(markdown),
                    "sections": [
                        {
                            "id": item.id,
                            "title": item.title,
                            "level": item.level,
                            "parent": item.parent,
                        }
                        for item in selected
                    ],
                },
                indent=2,
            )
        )
    else:
        for item in selected:
            print(f"H{item.level}\t{item.id}\t{item.title}")


def emit_tree(query: argparse.Namespace) -> None:
    paths = [
        path
        for path in sorted(PAGES)
        if query.depth is None or len(path) <= query.depth
    ]
    if query.format == "json":
        print(
            json.dumps(
                {
                    "schema_version": 1,
                    "commands": [
                        {"path": list(path), "address": command_address(path)}
                        for path in paths
                    ],
                },
                indent=2,
            )
        )
    else:
        for path in paths:
            print(command_label(path))


def emit_search(query: argparse.Namespace) -> None:
    if not query.value:
        raise SystemExit("search requires a query")
    needle = query.value.casefold()
    matches = []
    for path, markdown in iter_documents(query.agent):
        for item in sections(markdown):
            text = section_text(markdown, item.id, recursive=False)
            if needle in text.casefold():
                matches.append(
                    {
                        "command": list(path),
                        "section": item.id,
                        "title": item.title,
                    }
                )
    if query.format == "json":
        print(json.dumps({"schema_version": 1, "matches": matches}, indent=2))
    else:
        for match in matches:
            path = tuple(match["command"])
            print(f"{command_address(path)}#{match['section']}\t{match['title']}")


def run(arguments: list[str]) -> int:
    if arguments and arguments[-1] == "--help":
        path = tuple(arguments[:-1])
        print(choose_document(path, agent=False), end="")
        return 0

    try:
        help_index = arguments.index("help")
    except ValueError as error:
        raise SystemExit("expected '<command path> help'") from error

    path = tuple(arguments[:help_index])
    query = parse_query(arguments[help_index + 1 :])
    markdown = choose_document(path, query.agent)

    if query.operation is None:
        print(markdown, end="")
    elif query.operation == "outline":
        emit_outline(path, markdown, query)
    elif query.operation == "section":
        if not query.value:
            raise SystemExit("section requires a section identifier")
        try:
            print(section_text(markdown, query.value, query.recursive), end="")
        except KeyError as error:
            raise SystemExit(f"unknown section identifier: {query.value}") from error
    elif query.operation == "tree":
        emit_tree(query)
    elif query.operation == "search":
        emit_search(query)
    elif query.operation == "view":
        if query.web:
            print(WEB_URLS[path])
        else:
            print(markdown, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(run(sys.argv[1:]))
