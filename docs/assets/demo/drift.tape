# Re-render: vhs docs/assets/demo/drift.tape (run from the repo root).
# Records against a throwaway copy of examples/demo so `cairn scan` output
# never dirties the real repo. See record-setup.sh for env var overrides.
#
# The money shot: cairn catching a plan-breaking change. A copy-paste
# mistake claims an id another module already owns; `cairn scan` surfaces
# it as an Error and `cairn hook structural` blocks. Restoring the good
# blueprint clears the block.

Output docs/assets/demo/drift.gif
Output docs/assets/demo/drift.mp4

Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 650
Set TypingSpeed 40ms
Set Padding 20

Hide
Type "source docs/assets/demo/record-setup.sh"
Enter
Sleep 500ms
Show

# NOTE: line 15 of examples/demo/cairn.blueprint is `Module DB ... id
# "tasks.db" {`. If that fixture ever changes shape, update this line
# number (a stale anchor makes the hook pass instead of block, which is
# obviously wrong on playback).
Type "sed -i.bak '15s/tasks.db/tasks.api/' cairn.blueprint"
Enter
Sleep 1s

Type "cairn scan"
Enter
Sleep 2s

Type "cairn hook structural"
Enter
Sleep 2.5s

Type "mv cairn.blueprint.bak cairn.blueprint"
Enter
Sleep 1s

Type "cairn hook structural"
Enter
Sleep 2s
