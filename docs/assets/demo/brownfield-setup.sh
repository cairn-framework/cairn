# Sourced by brownfield.tape to prepare a scratch "existing project" before
# recording the from-code first-map flow. Not a standalone script (no
# shebang): it must be `source`d so PATH/cwd changes apply to the recording
# shell itself.
#
# Override BROWNFIELD_DEMO_DIR / CAIRN_BIN_DIR if your checkout or built
# binaries live somewhere else.
REPO_ROOT="$PWD"
BROWNFIELD_DEMO_DIR="${BROWNFIELD_DEMO_DIR:-/tmp/cairn-brownfield-tape}"
case "$BROWNFIELD_DEMO_DIR" in
  "" | "/" | "$HOME" | "$REPO_ROOT")
    echo "refusing to delete BROWNFIELD_DEMO_DIR='$BROWNFIELD_DEMO_DIR'" >&2
    return 1 2>/dev/null || exit 1
    ;;
esac
rm -rf -- "$BROWNFIELD_DEMO_DIR"
mkdir -p "$BROWNFIELD_DEMO_DIR/src/api" "$BROWNFIELD_DEMO_DIR/src/auth" "$BROWNFIELD_DEMO_DIR/src/db"
cd "$BROWNFIELD_DEMO_DIR" || exit 1

printf '[package]\nname = "shipd"\nversion = "0.4.2"\nedition = "2021"\n' > Cargo.toml
printf 'mod api;\nmod auth;\nmod db;\n\nfn main() { api::serve(); }\n' > src/main.rs

cat > src/db/mod.rs <<'EOF'
pub mod pool;
pub mod schema;

pub fn connect() {}
EOF
printf 'pub fn get_pool() {}\n' > src/db/pool.rs
printf 'pub fn migrate() {}\n' > src/db/schema.rs

cat > src/auth/mod.rs <<'EOF'
pub mod tokens;
pub mod session;
use crate::db;

pub fn login() { db::connect(); }
EOF
printf 'pub fn issue() {}\n' > src/auth/tokens.rs
printf 'pub fn refresh() {}\n' > src/auth/session.rs

cat > src/api/mod.rs <<'EOF'
pub mod routes;
pub mod handlers;
use crate::auth;

pub fn serve() { auth::login(); }
EOF
printf 'pub fn register() {}\n' > src/api/routes.rs
printf 'pub fn users() {}\n' > src/api/handlers.rs

export PATH="${CAIRN_BIN_DIR:-$REPO_ROOT/target/release}:$PATH"
clear
