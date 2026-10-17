use std::{fs, path::PathBuf};

use image::RgbImage;
use jiff::{Timestamp, tz::TimeZone};

#[derive(Clone)]
pub struct Queue {
    queue_dir: PathBuf,
    archive_dir: Option<PathBuf>,
}

impl Queue {
    pub fn new(queue_dir: PathBuf, archive_dir: Option<PathBuf>) -> anyhow::Result<Self> {
        fs::create_dir_all(&queue_dir)?;

        if let Some(ref dir) = archive_dir {
            fs::create_dir_all(dir)?;
        }

        Ok(Self {
            queue_dir,
            archive_dir,
        })
    }

    fn queue_entry_paths(&self) -> (PathBuf, PathBuf) {
        let now = Timestamp::now();
        let path = self.queue_dir.join(format!("{now}.png"));
        let hidden = self.queue_dir.join(format!(".{now}.png"));
        (path, hidden)
    }

    fn archive_entry_path(&self) -> Option<PathBuf> {
        let Some(archive_dir) = &self.archive_dir else {
            return None;
        };

        let now = Timestamp::now();
        let date = now.to_zoned(TimeZone::UTC).date();
        let path = archive_dir
            .join(format!("{date}"))
            .join(format!("{now}.png"));

        Some(path)
    }

    pub fn enqueue(&self, image: &RgbImage) -> anyhow::Result<()> {
        // Atomic save to avoid race conditions between enqueue and dequeue
        let (path, hidden) = self.queue_entry_paths();
        image.save(&hidden)?;
        fs::rename(&hidden, &path)?;
        println!("Enqueued {path:?}");
        Ok(())
    }

    pub fn next_entry(&self) -> anyhow::Result<Option<QueueEntry>> {
        let mut entries: Vec<_> = fs::read_dir(&self.queue_dir)?.collect::<Result<Vec<_>, _>>()?;
        entries.retain(|e| !e.file_name().to_string_lossy().starts_with("."));
        entries.sort_unstable_by_key(|e| e.file_name());
        let Some(entry) = entries.first() else {
            return Ok(None);
        };

        let queue_path = entry.path();
        let archive_path = self.archive_entry_path();
        let image = image::open(&queue_path)?.to_rgb8();
        Ok(Some(QueueEntry {
            queue_path,
            archive_path,
            image,
        }))
    }
}

pub struct QueueEntry {
    queue_path: PathBuf,
    archive_path: Option<PathBuf>,
    image: RgbImage,
}

impl QueueEntry {
    pub fn image(&self) -> &RgbImage {
        &self.image
    }

    pub fn dequeue(self) -> anyhow::Result<()> {
        let queue_path = self.queue_path;
        let Some(archive_path) = self.archive_path else {
            fs::remove_file(&queue_path)?;
            println!("Dequeued {queue_path:?}");
            return Ok(());
        };

        if let Some(parent) = archive_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&queue_path, &archive_path)?;
        println!("Archived {queue_path:?} to {archive_path:?}");

        Ok(())
    }
}
