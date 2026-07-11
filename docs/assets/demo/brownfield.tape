# Re-render: vhs docs/assets/demo/brownfield.tape (run from the repo root).
# Records the existing-project first-map flow against a scratch project in
# /tmp so nothing dirties the real repo. See brownfield-setup.sh for env
# var overrides.

Output docs/assets/demo/brownfield.gif
Output docs/assets/demo/brownfield.mp4

Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 650
Set TypingSpeed 40ms
Set Padding 20

Hide
Type "source docs/assets/demo/brownfield-setup.sh"
Enter
Sleep 500ms
Show

Type "ls src/"
Enter
Sleep 1.5s

Type "cairn init --from-code"
Enter
Sleep 2s

Type "cat meta/changes/brownfield-init/proposal.md"
Enter
Sleep 2.5s

Type "cairn change apply brownfield-init"
Enter
Sleep 2s

Type "cairn scan"
Enter
Sleep 2.5s

Type "head -14 map.md"
Enter
Sleep 3s
