use std::path::Path;

const GH_STUB: &str = r#"#!/usr/bin/env bash
# Stub gh: serve canned lists, record every mutating call, and expose body payloads.
root="{root}"
args="$*"
if [[ "$1 $2" == "issue list" ]]; then
    [[ -e "$root/list-fails" ]] && exit 1
    if [[ "$args" == *"--label"* ]]; then
        while IFS=$'\t' read -r number state title slug status node_field; do
            [ -n "$number" ] || continue
            body_b64=""
            if [[ -e "$root/body.$slug" ]]; then
                body_b64="$(base64 < "$root/body.$slug" | tr -d '\n')"
            fi
            printf '%s\t%s\t%s\t%s\t%s\n' \
                "$number" "$state" "$title" "$slug" "$body_b64"
        done < "$root/projection.tsv"
    else
        cat "$root/unmapped.txt"
    fi
    exit 0
fi
if [[ "$1 $2" == "issue create" ]]; then
    title_value=""
    body_value=""
    i=1
    while (( i <= $# )); do
        if [[ "${!i}" == "--title" ]]; then
            title_arg=$((i + 1))
            title_value="${!title_arg}"
        elif [[ "${!i}" == "--body" ]]; then
            body_arg=$((i + 1))
            body_value="${!body_arg}"
        fi
        i=$((i + 1))
    done
    number="$(cat "$root/next-number" 2>/dev/null || printf '100')"
    printf '%s' "$((number + 1))" > "$root/next-number"
    slug="$(printf '%s' "$body_value" | sed -n 's/^cairn-todo: todo\.\(.*\)$/\1/p')"
    printf '%s\tOPEN\t%s\t%s\topen\tcairn.root\n' \
        "$number" "$title_value" "$slug" >> "$root/projection.tsv"
    printf '%s' "$body_value" > "$root/body.$slug"
fi
if [[ "$1 $2" == "issue edit" ]]; then
    number="$3"
    title_value=""
    body_value=""
    has_title=0
    has_body=0
    i=1
    while (( i <= $# )); do
        if [[ "${!i}" == "--title" ]]; then
            title_arg=$((i + 1))
            title_value="${!title_arg}"
            has_title=1
        elif [[ "${!i}" == "--body" ]]; then
            body_arg=$((i + 1))
            body_value="${!body_arg}"
            has_body=1
        fi
        i=$((i + 1))
    done
    issue_slug=""
    while IFS=$'\t' read -r existing_number state title slug status node_field; do
        if [[ "$existing_number" == "$number" ]]; then
            issue_slug="$slug"
            break
        fi
    done < "$root/projection.tsv"
    if (( has_body == 1 )) && [[ -n "$issue_slug" ]]; then
        printf '%s' "$body_value" > "$root/body.$issue_slug"
    fi
fi
if [[ "$1 $2" == "issue close" || "$1 $2" == "issue reopen" ]]; then
    number="$3"
    next_state="CLOSED"
    [[ "$2" == "reopen" ]] && next_state="OPEN"
    tmp="$root/projection.next"
    : > "$tmp"
    while IFS=$'\t' read -r existing_number state title slug status node_field; do
        [ -n "$existing_number" ] || continue
        [[ "$existing_number" == "$number" ]] && state="$next_state"
        printf '%s\t%s\t%s\t%s\t%s\t%s\n' \
            "$existing_number" "$state" "$title" "$slug" "$status" "$node_field" >> "$tmp"
    done < "$root/projection.tsv"
    mv "$tmp" "$root/projection.tsv"
fi
if [[ "$1 $2" == "issue create" || "$1 $2" == "issue edit" ]]; then
    i=1
    while (( i <= $# )); do
        if [[ "${!i}" == "--body" ]]; then
            body_arg=$((i + 1))
            printf '%s' "${!body_arg}" > "$root/last-body"
            break
        fi
        i=$((i + 1))
    done
fi
echo "$args" >> "$root/mutations.log"
"#;

pub fn script(root: &Path) -> String {
    GH_STUB.replace("{root}", &root.display().to_string())
}
