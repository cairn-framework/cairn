# Re-render: vhs docs/assets/demo/tour.tape (run from the repo root).
# Records against a throwaway copy of examples/demo so `cairn scan` output
# never dirties the real repo. See record-setup.sh for env var overrides.

Output docs/assets/demo/tour.gif
Output docs/assets/demo/tour.mp4

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

Type "cairn status"
Enter
Sleep 2s

Type "cairn scan"
Enter
Sleep 2s

Type "cairn get tasks.api"
Enter
Sleep 2.5s

Type "cairn lint"
Enter
Sleep 2s
