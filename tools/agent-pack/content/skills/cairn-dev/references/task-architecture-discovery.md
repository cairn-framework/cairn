# Architecture discovery

Goal: explain how a system fits together, and why it is shaped that way.

## Query sequence

```bash
cairn context                            # every node and edge, plus findings
cairn context --scope <node> --depth 1   # a readable slice once you know the area
cairn islands                            # components with no edges to the rest
cairn order                              # a valid dependency order over all nodes
cairn get <node>                         # one node: ownership, paths, state
cairn deps <node> --transitive           # what it rests on
cairn deps <node> --direction in --transitive   # what rests on it
cairn rationale <node>                   # the decisions that produced this shape
cairn sources <node>                     # external material the node cites
```

Work outside in: whole graph, then the area, then the node. Answer "what owns
this" before "how does this work".

Two questions the graph answers that reading code does not:

- **Why is it like this.** `cairn rationale <node>` returns the accepted decisions
  covering the node and its one-hop neighbours, with the research and sources
  behind them. That is the intent, and it is not recoverable from source.
- **What would this change break.** The inbound transitive dependency set is the
  blast radius, computed from declared edges rather than guessed from imports.

## When the graph stops helping

The graph is structural. It carries no control flow, no data flow, and no runtime
behaviour. Once you know which nodes are involved:

- Read the entry points and the contract, not every file.
  `cairn contract <node>` gives the declared interface; `cairn get <node>
  --symbols --json` gives each symbol with `file`, `line`, and `end_line` so you
  can read exact spans.
- Use the language server to follow a call chain across nodes; edges tell you a
  dependency exists, not where it is exercised.

## Reporting

Name the owning nodes and the handoffs between them, and cite file and symbol for
each claim. An architecture answer without evidence is a guess with structure.

State the decisions that constrain the area, and say plainly when something is
undocumented rather than inferring intent from the code.

If discovery reveals that the blueprint is wrong (a missing edge, an island that
should be connected, a node owning files it does not), that is a finding worth
recording: either fix it in this change or write a todo with
`cairn todo new <slug> --node <id>`.
