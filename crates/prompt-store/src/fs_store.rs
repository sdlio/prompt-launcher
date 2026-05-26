use crate::parser::parse_file;
use anyhow::{anyhow, Context, Result};
use app_core::{Frontmatter, Prompt};
use chrono::Utc;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

/// Events emitted by an `FsPromptStore` watcher.
#[derive(Debug, Clone)]
pub enum Event {
    /// One or more files under the prompts root changed; consumer should re-`list()`.
    Reloaded,
}

/// Guard returned by [`FsPromptStore::watch`]. Dropping it stops the watcher thread
/// (by dropping the underlying `notify` watcher, which closes the internal channel).
pub struct WatchHandle {
    _watcher: RecommendedWatcher,
    _thread: thread::JoinHandle<()>,
}

/// Filesystem-backed prompt store rooted at a directory containing `*.md` files.
#[derive(Debug, Clone)]
pub struct FsPromptStore {
    root: PathBuf,
}

impl FsPromptStore {
    /// Open a store at `root`. Creates `root` if it doesn't exist.
    pub fn new(root: PathBuf) -> Result<Self> {
        if !root.exists() {
            std::fs::create_dir_all(&root).with_context(|| format!("mkdir {}", root.display()))?;
        }
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Snapshot every parseable prompt under `root`. Best-effort: files that fail to parse
    /// are logged and skipped (we don't want one malformed file to nuke the launcher).
    pub fn list(&self) -> Vec<Prompt> {
        let mut out = Vec::new();
        for entry in WalkDir::new(&self.root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        {
            match parse_file(&self.root, entry.path()) {
                Ok(Some(p)) => out.push(p),
                Ok(None) => {}
                Err(e) => tracing::warn!("skip {}: {e:#}", entry.path().display()),
            }
        }
        out
    }

    /// Find the prompt with `id` and atomically rewrite its file with `last_used = now`.
    pub fn touch(&self, id: &str) -> Result<()> {
        let prompt = self
            .list()
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| anyhow!("no prompt with id {id}"))?;
        let mut fm = prompt.frontmatter.clone();
        fm.last_used = Some(Utc::now());
        write_atomic(&prompt.path, &fm, &prompt.body)?;
        Ok(())
    }

    /// Spawn a background thread that re-emits a single [`Event::Reloaded`] to `sink`
    /// after 200ms of filesystem-event quiet. Returns a [`WatchHandle`]; dropping the
    /// handle stops the watcher.
    pub fn watch(&self, sink: Sender<Event>) -> Result<WatchHandle> {
        let (raw_tx, raw_rx) = channel::<notify::Result<notify::Event>>();
        let mut watcher: RecommendedWatcher =
            notify::recommended_watcher(raw_tx).context("create watcher")?;
        watcher
            .watch(self.root.as_path(), RecursiveMode::Recursive)
            .context("watch root")?;

        let handle = thread::Builder::new()
            .name("prompt-store-watch".to_string())
            .spawn(move || {
                const DEBOUNCE: Duration = Duration::from_millis(200);
                loop {
                    // Block for the first event.
                    if raw_rx.recv().is_err() {
                        // Channel closed -> watcher dropped -> exit.
                        return;
                    }
                    // Drain extras within the debounce window.
                    let deadline = Instant::now() + DEBOUNCE;
                    loop {
                        let remaining = deadline.saturating_duration_since(Instant::now());
                        if remaining.is_zero() {
                            break;
                        }
                        match raw_rx.recv_timeout(remaining) {
                            Ok(_) => continue, // got another event, keep draining
                            Err(_) => break,   // timeout -> quiet -> emit
                        }
                    }
                    if sink.send(Event::Reloaded).is_err() {
                        return; // consumer dropped
                    }
                }
            })
            .context("spawn watch thread")?;

        Ok(WatchHandle {
            _watcher: watcher,
            _thread: handle,
        })
    }
}

/// Serialize a YAML frontmatter block + body and atomically rewrite the file
/// (write to `<path>.tmp`, then `rename` — survives crashes mid-write).
fn write_atomic(path: &Path, frontmatter: &Frontmatter, body: &str) -> Result<()> {
    let yaml = serde_yaml::to_string(frontmatter).context("serialize frontmatter")?;
    let mut out = String::new();
    out.push_str("---\n");
    out.push_str(&yaml);
    if !yaml.ends_with('\n') {
        out.push('\n');
    }
    out.push_str("---\n");
    out.push_str(body);

    let tmp = path.with_extension("md.tmp");
    std::fs::write(&tmp, out.as_bytes()).with_context(|| format!("write tmp {}", tmp.display()))?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}
