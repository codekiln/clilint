# Hierarchical help experiments

These small prototypes compare two help interfaces using the same command hierarchy
and Markdown help documents.

## Prototype A: postfix command help

`postfix.py` makes `help` a child of every command path:

```sh
python3 postfix.py repo clone help
python3 postfix.py repo clone help outline --max-level 3 --agent
python3 postfix.py repo clone help section examples.private-repository
python3 postfix.py view help
```

This interface follows the command path that a person or agent is already
using.

## Prototype B: addressable documentation

`resource.py` gives each command document an address and uses one query
interface:

```sh
python3 resource.py list repo/clone --audience agent
python3 resource.py get repo/clone#examples.private-repository
python3 resource.py search permissions --audience agent
```

This interface makes command documents easy to address from JSON output.
It adds inherited programmatic guidance to the shared help document.

## Running the checks

```sh
python3 -m unittest -v test_experiments.py
```

The prototypes use only the Python standard library. They inform the adjacent
OpenSpec artifacts.

The postfix prototype retains the earlier `--agent` flag so the experiment
remains reproducible. The current design uses `--programmatic`.

`codekiln-help` will turn the chosen behavior into a complete Clilint check
bundle. Its bundle file, checks, fixtures, and documentation will also show
bundle authors how to build a custom check bundle.
