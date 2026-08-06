# The guided console: creation-journey prototype

The vehicle `todo.guided-console-prototype` ordered: a console the maintainer
runs as a user against a demo project, so the next design round argues with
real feedback instead of more hypothetical mockups (round 2 ruling of
`todo.console-orchestration-ux-design`, 2026-08-06).

Register reference: `studio/mocks/orchestration-guided-journey.html`, the one
screen called aligned. Plain language primary, the technical working behind a
drawer, one run action.

## The four moments, and nothing else

1. **Describe.** You write what you want built, in your own words.
2. **The map forms.** Your words become a real `cairn.blueprint`: layers,
   parts, and the dependencies between them. Every part is a ghost, because
   nothing is built yet.
3. **The grill drains the doubts.** Two to four plain questions with
   selectable answers, one of them marked as the call most worth arguing with.
   Answering the last one writes them all in as recorded decisions.
4. **Run.** Not wired. See "What is not wired" below.

Everything else is out of scope by the same ruling: the return-and-orient
layer, the queue-drain surfaces, the narrow layout, the driver-states four-up,
decision-to-consequence as its own screen, workflow tuning, and portability
across harnesses.

## Nothing on screen is invented

| On screen | Where it comes from |
|---|---|
| Layers and parts | `cairn context --json`, `cairn get <id> --json` |
| Band glosses | the container's declared description, cut to its leading clause, and dropped when even that is too long to read as a label |
| Dashed chips (not built yet) | node `state: ghost` from `cairn context` |
| Solid chips, and "3 of 7 built" | every other node state from `cairn context` |
| Wave numbers, wave order | `cairn frontier --json` tier, plus one |
| What a wave waits for | `frontier` `blocking`, mapped to plain names |
| Part and edge counts | `cairn context` |
| Rulings, from your answers | decision artefacts written into the target project |

The layer order is derived, not declared: the layer whose parts are built last
sits at the top. The wave notes are generated from the graph, so they cannot
drift from it. When the first wave is not 1, the panel says why: no unbuilt
part sits at that depth, which is only true when a built one does.

## Running it locally

Needs `cairn` and a harness on `PATH`.

```sh
node studio/prototype/server.mjs
# then open http://localhost:4400
```

| Variable | Default | What it does |
|---|---|---|
| `PORT` | `4400` | Listen port. Under compose this is pinned to 4400 and `CONSOLE_PORT` picks the host port instead. |
| `CAIRN_PROTOTYPE_DATA` | `studio/prototype/.data` | Target project and journey state |
| `CAIRN_PROTOTYPE_HARNESS` | `omp` | The harness binary. A path, not a portability seam: the event reader expects this harness's JSONL. |
| `CAIRN_PROTOTYPE_HARNESS_ARGS` | `-p --mode json --no-session --no-skills` | Non-interactive JSONL mode |
| `CAIRN_PROTOTYPE_CAIRN_BIN` | `cairn` | The cairn binary |
| `CAIRN_PROTOTYPE_DESIGN_SYSTEM` | `docs/design-system` | Served at `/design-system/` |

`GET /api/state` returns the same payload the console renders, which is the
quickest way to see what it believes without reading the screen.

## Checking copy against a part-built map

A map where everything is a ghost hides a whole class of wrong sentence: a
plan can open at wave 2, built parts carry no wave at all, and a legend that
only explains dashed chips leaves the solid ones unexplained. So any change to
what the console says is checked twice, once against a map the harness has
just written and once against `fixtures/part-built`, where three of seven
parts exist on disk. It needs no harness and no model spend.

```sh
# copy it out first: "Start again" clears the target project it is pointed at
cp -R studio/prototype/fixtures/part-built /tmp/part-built
PORT=4401 CAIRN_PROTOTYPE_DATA=/tmp/part-built node studio/prototype/server.mjs
```

## Running it on the server

```sh
docker compose -f studio/prototype/compose.yaml up -d --build
docker compose -f studio/prototype/compose.yaml logs -f
```

The image builds `cairn` from this checkout, so the prototype always tracks
local cairn rather than a published release. The first build is slow for that
reason. `docs/design-system/` is served from the image rather than copied into
a stylesheet, so a token change reaches the console without an edit here.

The harness needs credentials. Either put them in `studio/prototype/.env`, or
sign in once inside the container and let the `harness-home` volume keep the
session:

```sh
docker compose -f studio/prototype/compose.yaml exec console omp
```

## What is not wired, and why

The run plate refuses, and says so on its face. Run would record a ruling
(`cairn ruling run <plan>`) that the driver then observes and obeys. That verb
is proposed in `dec.webui-write-authority` clause 4 and is not signed yet, so
this console records no ruling and dispatches nothing. The refusal is stated in
one place, `RUN_NOT_WIRED` in `lib/journey.mjs`, but wiring run is not one edit:
the route, the click handler, and a dispatch path that does not exist yet all
have to move together, after the verb ships.

The console does write, and that is not a contradiction. It writes the target
project's own blueprint and decision artefacts, because turning your words into
a map is the journey being prototyped. What it does not do is dispatch agents
or write orchestration facts. That boundary is exactly the one
`dec.orchestration-placement` draws, and it is what the run plate is holding.

## Design authority

`docs/design-system/` tokens only, product lane per
`dec.marketing-visual-world`. `src/ui_assets/` is untouched: the production
console belongs to `todo.console-signed-widening`, and this prototype feeds
that unit rather than replacing it.

The class vocabulary in `ui/console.css` is the aligned specimen's, kept local
to the prototype instead of forking `docs/design-system/components.css`. When
the production console is built, these graduate into the component library.

## What the maintainer is asked to do

Acceptance for `todo.guided-console-prototype`:

1. Open the console, describe a calculator app in your own words. The
   specimen's wording, if you want the same scene:
   "I want a calculator app. Typing a sum shows the answer as I type, and I can
   see my past sums."
2. Watch the map form. Read the waves as sentences.
3. Answer the questions. Watch them get written in.
4. Press run and read the refusal.
5. Record what was wrong with it, as a dated amendment to
   `studio/orchestration-console-brief.md` or through `cairn feedback`. That
   becomes the input to the next design round.

Each harness step takes minutes, not seconds, and the activity panel shows what
is happening. Closing the tab does not stop it: the service and the harness keep
running, and reconnecting catches up. Restarting the service is different. A
finished journey is read back from `journey.json`, but a step that was in flight
dies with the process and has to be asked for again.
