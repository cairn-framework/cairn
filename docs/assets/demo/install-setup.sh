# Sourced by install.tape to prepare a scratch project and a scratch
# CARGO_HOME before recording the real installer against the live GitHub
# release. Not a standalone script (no shebang): it must be `source`d so
# PATH/cwd/env changes apply to the recording shell itself.
#
# Override INSTALL_DEMO_DIR if you want the scratch install to live
# somewhere else.
INSTALL_DEMO_DIR="${INSTALL_DEMO_DIR:-/tmp/cairn-install-demo}"
rm -rf "$INSTALL_DEMO_DIR"
mkdir -p "$INSTALL_DEMO_DIR/proj/src"
echo 'fn main() {}' > "$INSTALL_DEMO_DIR/proj/src/main.rs"
export CARGO_HOME="$INSTALL_DEMO_DIR/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
export NO_MODIFY_PATH=1
cd "$INSTALL_DEMO_DIR" || exit 1
clear
