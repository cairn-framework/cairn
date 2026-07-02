# Re-render: vhs docs/assets/demo/install.tape (run from the repo root).
# Records the real curl installer against the live GitHub release into a
# scratch CARGO_HOME (see install-setup.sh), then a first-run walkthrough
# (cairn init + cairn scan) against an empty scratch project. Nothing here
# touches the real ~/.cargo or the repo itself. Requires a published
# release (cairn-installer.sh + tarballs) reachable at .../releases/latest.

Output docs/assets/demo/install.gif
Output docs/assets/demo/install.mp4

Set Shell "bash"
Set FontSize 18
Set Width 1200
Set Height 650
Set TypingSpeed 40ms
Set Padding 20

Hide
Type "source docs/assets/demo/install-setup.sh"
Enter
Sleep 500ms
Show

Type "curl --proto '=https' --tlsv1.2 -LsSf https://github.com/cairn-framework/cairn/releases/latest/download/cairn-installer.sh | sh"
Enter
Sleep 5s

Type "cairn --version"
Enter
Sleep 1.5s

Hide
Type "cd proj"
Enter
Sleep 500ms
Show

Type "cairn init"
Enter
Sleep 2.5s

Type "cairn scan"
Enter
Sleep 2s
