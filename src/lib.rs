use std::process::Command;
use std::time::Duration;
use std::time::Instant;

#[macro_export]
macro_rules! status {
    [ $([$prefix:expr, $suffix:expr, $interval:expr, $path:expr]),* $(,)? ] => {
        use $crate::*;
        StatusHandler::new()
            $(.add_block($prefix, $suffix, $interval, $path))*
            .start()
    };

    [ gap: $gap:expr, $([$prefix:expr, $suffix:expr, $interval:expr, $path:expr]),* $(,)? ] => {
        use $crate::*;
        StatusHandler::new($gap)
            $(.add_block($prefix, $suffix, $interval, $path))*
            .start()
    };

    [ gap: $gap:expr, base_path: $base_path:expr, $([$prefix:expr, $suffix:expr, $interval:expr, $path:expr]),* $(,)? ] => {
        use $crate::*;
        StatusHandler::new($gap)
            $(.add_block($prefix, $suffix, $interval, concat!($base_path, $path)))*
            .start()
    };


    [ base_path: $base_path:expr, gap: $gap:expr,  $([$prefix:expr, $suffix:expr, $interval:expr, $path:expr]),* $(,)? ] => {
        use $crate::*;
        StatusHandler::new($gap)
            $(.add_block($prefix, $suffix, $interval, concat!($base_path, $path)))*
            .start()
    };
}

pub struct StatusHandler {
    gap: String,
    blocks: Vec<(u64, StatusBlock)>,
    text_placeholder: Vec<String>,
}

#[derive(Clone)]
pub struct StatusBlock {
    /// <prefix><output><suffix>
    prefix: String,

    /// <prefix><output><suffix>
    suffix: String,

    script_path: String,
}

impl StatusBlock {
    pub fn new(prefix: String, suffix: String, script_path: String) -> Self {
        Self {
            prefix,
            suffix,
            script_path,
        }
    }
}

impl StatusHandler {
    pub fn new(gap: &str) -> Self {
        Self {
            gap: gap.to_owned(),
            blocks: vec![],
            text_placeholder: vec![],
        }
    }

    pub fn add_block(
        &mut self,
        prefix: &str,
        suffix: &str,
        interval_secs: u64,
        script_path: &str,
    ) -> &mut Self {
        self.text_placeholder.push(String::new());

        self.blocks.push((
            interval_secs,
            StatusBlock::new(prefix.to_owned(), suffix.to_owned(), script_path.to_owned()),
        ));

        self
    }

    fn execute(&self, script_path: &str) -> (String, Duration) {
        let clock = Instant::now();
        let command = Command::new("bash").arg(script_path).output().unwrap();

        if !command.stderr.is_empty() {
            println!(
                "Error while running '{script_path}': {}",
                String::from_utf8(command.stderr).unwrap()
            );
        }

        (
            String::from_utf8(command.stdout)
                .unwrap()
                .trim()
                .to_string(),
            clock.elapsed(),
        )
    }

    pub fn start(&mut self) -> ! {
        let mut elapsed: u64 = 0;

        loop {
            for idx in self.get_blocks(elapsed) {
                let StatusBlock {
                    prefix,
                    suffix,
                    script_path,
                } = &self.blocks[idx].1;

                let (output, time_taken) = self.execute(script_path);

                self.text_placeholder[idx] = format!("{prefix}{output}{suffix}");
                elapsed += time_taken.as_secs();

                self.update();
            }

            std::thread::sleep(Duration::from_secs(1));
            elapsed += 1;
        }
    }

    fn get_blocks(&self, elapsed: u64) -> Vec<usize> {
        self.blocks
            .iter()
            .enumerate()
            .filter(|(_, (interval, _))| elapsed % *interval == 0)
            .map(|(idx, _)| idx)
            .collect()
    }

    fn update(&self) {
        let text = self.text_placeholder.join(&self.gap);
        self.set_status(&text);
    }

    fn set_status(&self, text: &str) {
        Command::new("xsetroot")
            .arg("-name")
            .arg(text)
            .spawn()
            .unwrap();
    }
}
