# Sourced by the .tape files in this directory to prepare a throwaway demo
# copy before recording. Not a standalone script (no shebang): it must be
# `source`d so PATH/cwd changes apply to the recording shell itself.
#
# Override DEMO_DIR / CAIRN_BIN_DIR if your checkout or built binaries live
# somewhere else.
REPO_ROOT="$PWD"
DEMO_DIR="${DEMO_DIR:-/tmp/cairn-demo-tape}"
rm -rf "$DEMO_DIR"
cp -r examples/demo "$DEMO_DIR"
export PATH="${CAIRN_BIN_DIR:-$REPO_ROOT/target/release}:$PATH"
cd "$DEMO_DIR" || exit 1
clear
