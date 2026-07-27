#!/usr/bin/env python3
"""Prototype B: help documents addressed as resources."""

from __future__ import annotations

import argparse
import json
import sys

from corpus import (
    PAGES,
    WEB_URLS,
    command_address,
    digest,
    inherited_agent_document,
    iter_documents,
    parse_address,
    section_text,
    sections,
)


def document(path: tuple[str, ...], audience: str) -> str:
    try:
        return (
            inherited_agent_document(path)
            if audience == "agent"
            else PAGES[path]
        )
    except KeyError as error:
        raise SystemExit(f"unknown document: {command_address(path)}") from error


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser()
    operations = root.add_subparsers(dest="operation", required=True)

    list_parser = operations.add_parser("list")
    list_parser.add_argument("address", nargs="?", default=".")
    list_parser.add_argument("--audience", choices=("human", "agent"), default="human")
    list_parser.add_argument("--level", type=int)
    list_parser.add_argument("--max-level", type=int)
    list_parser.add_argument("--format", choices=("text", "json"), default="text")

    get_parser = operations.add_parser("get")
    get_parser.add_argument("address")
    get_parser.add_argument("--audience", choices=("human", "agent"), default="human")
    get_parser.add_argument("--recursive", action="store_true")
    get_parser.add_argument("--format", choices=("markdown", "json"), default="markdown")

    search_parser = operations.add_parser("search")
    search_parser.add_argument("query")
    search_parser.add_argument("--audience", choices=("human", "agent"), default="human")
    search_parser.add_argument("--format", choices=("text", "json"), default="text")

    view_parser = operations.add_parser("view")
    view_parser.add_argument("address", nargs="?", default=".")
    view_parser.add_argument("--audience", choices=("human", "agent"), default="human")
    view_parser.add_argument("--web", action="store_true")
    return root


def emit_list(args: argparse.Namespace) -> None:
    path, address_section = parse_address(args.address)
    if address_section:
        raise SystemExit("list accepts a document address without a section")
    markdown = document(path, args.audience)
    selected = [
        item
        for item in sections(markdown)
        if (args.level is None or item.level == args.level)
        and (args.max_level is None or item.level <= args.max_level)
    ]
    records = [
        {
            "address": f"{command_address(path)}#{item.id}",
            "title": item.title,
            "level": item.level,
            "parent": item.parent,
        }
        for item in selected
    ]
    if args.format == "json":
        print(
            json.dumps(
                {
                    "schema_version": 1,
                    "document": command_address(path),
                    "audience": args.audience,
                    "document_digest": digest(markdown),
                    "sections": records,
                },
                indent=2,
            )
        )
    else:
        for record in records:
            print(f"H{record['level']}\t{record['address']}\t{record['title']}")


def emit_get(args: argparse.Namespace) -> None:
    path, section_id = parse_address(args.address)
    markdown = document(path, args.audience)
    if section_id:
        try:
            content = section_text(markdown, section_id, args.recursive)
        except KeyError as error:
            raise SystemExit(f"unknown section: {section_id}") from error
    else:
        content = markdown
    if args.format == "json":
        print(
            json.dumps(
                {
                    "schema_version": 1,
                    "address": args.address,
                    "audience": args.audience,
                    "document_digest": digest(markdown),
                    "content_format": "markdown",
                    "content": content,
                },
                indent=2,
            )
        )
    else:
        print(content, end="")


def emit_search(args: argparse.Namespace) -> None:
    needle = args.query.casefold()
    matches = []
    for path, markdown in iter_documents(
        agent=args.audience == "agent",
        inherited=args.audience == "agent",
    ):
        for item in sections(markdown):
            direct = section_text(markdown, item.id, recursive=False)
            if needle in direct.casefold():
                matches.append(
                    {
                        "address": f"{command_address(path)}#{item.id}",
                        "title": item.title,
                    }
                )
    if args.format == "json":
        print(json.dumps({"schema_version": 1, "matches": matches}, indent=2))
    else:
        for match in matches:
            print(f"{match['address']}\t{match['title']}")


def emit_view(args: argparse.Namespace) -> None:
    path, section_id = parse_address(args.address)
    if section_id:
        raise SystemExit("view accepts a document address without a section")
    if args.web:
        print(WEB_URLS[path])
    else:
        print(document(path, args.audience), end="")


def run(arguments: list[str]) -> int:
    args = parser().parse_args(arguments)
    {
        "list": emit_list,
        "get": emit_get,
        "search": emit_search,
        "view": emit_view,
    }[args.operation](args)
    return 0


if __name__ == "__main__":
    raise SystemExit(run(sys.argv[1:]))
