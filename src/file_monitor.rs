use notify::{RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::path::Path;

pub fn watch_for_zip(watch_dir: &std::path::Path) -> Result<()> {
  let (tx, rx) = std::sync::mpsc::channel();

  // Automatically select the best implementation for your platform.
  // You can also access each implementation directly e.g. INotifyWatcher.
  let mut watcher: RecommendedWatcher = Watcher::new_immediate(move |res| tx.send(res).unwrap())?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(watch_dir, RecursiveMode::Recursive)?;

  for res in rx {
      match res {
          Ok(event) => println!("changed: {:?}", event),
          Err(e) => println!("watch error: {:?}", e),
      }
  }

  Ok(())
}
