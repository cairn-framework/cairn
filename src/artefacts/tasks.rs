//! Task-checkbox accounting for a change directory's `tasks.md`.
//!
//! One parser serves both readers: the scan check that emits
//! `CAIRN_CHANGE_TASKS_COMPLETE` and the `cairn change show` progress surface.

/// Completion counts parsed from a change's `tasks.md` checkbox list.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TaskProgress {
    /// Boxes marked `[x]` or `[X]`.
    pub completed: usize,
    /// All recognised checkboxes, completed and outstanding.
    pub total: usize,
}

impl TaskProgress {
    /// Outstanding boxes: `total - completed`.
    #[must_use]
    pub const fn remaining(self) -> usize {
        self.total - self.completed
    }

    /// True when the change tracks at least one task and none remain.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        self.total > 0 && self.completed == self.total
    }
}

/// Counts markdown task checkboxes in `source`, ignoring fenced code blocks.
///
/// Recognises `- [ ]`, `- [x]`, `- [X]` and their `*` bullet spellings, with or
/// without trailing text.
#[must_use]
pub fn count_tasks(source: &str) -> TaskProgress {
    let mut progress = TaskProgress::default();
    let mut in_fence = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .map(str::trim_start)
        else {
            continue;
        };
        if rest == "[x]" || rest == "[X]" || rest.starts_with("[x] ") || rest.starts_with("[X] ") {
            progress.completed += 1;
            progress.total += 1;
        } else if rest == "[ ]" || rest.starts_with("[ ] ") {
            progress.total += 1;
        }
    }
    progress
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tasks_counts_both_states_and_skips_fences() {
        let source = "\
# Tasks

- [x] done one
* [X] done two
- [ ] open one
- [ ]
- not a task

```
- [ ] fenced, ignored
```

~~~
- [x] fenced, ignored
~~~
";
        let progress = count_tasks(source);
        assert_eq!(progress.completed, 2, "two checked boxes outside fences");
        assert_eq!(progress.total, 4, "four checkboxes outside fences");
        assert_eq!(progress.remaining(), 2);
        assert!(!progress.is_complete());
    }

    #[test]
    fn test_count_tasks_without_checkboxes_is_empty_and_incomplete() {
        let progress = count_tasks("# Tasks\n\nProse only.\n");
        assert_eq!(progress, TaskProgress::default());
        assert!(
            !progress.is_complete(),
            "a change with no checkboxes is not complete"
        );
    }

    #[test]
    fn test_count_tasks_all_checked_is_complete() {
        let progress = count_tasks("- [x] one\n- [x] two\n");
        assert!(progress.is_complete());
        assert_eq!(progress.remaining(), 0);
    }
}
