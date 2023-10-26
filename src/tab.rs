// SPDX-License-Identifier: GPL-3.0-only

use cosmic_text::{Attrs, Buffer, Edit, Metrics, SyntaxEditor, ViEditor};
use std::{fs, path::PathBuf, sync::Mutex};

use crate::{FONT_SYSTEM, SYNTAX_SYSTEM};

static FONT_SIZES: &'static [Metrics] = &[
    Metrics::new(10.0, 14.0), // Caption
    Metrics::new(14.0, 20.0), // Body
    Metrics::new(20.0, 28.0), // Title 4
    Metrics::new(24.0, 32.0), // Title 3
    Metrics::new(28.0, 36.0), // Title 2
    Metrics::new(32.0, 44.0), // Title 1
];

pub struct Tab {
    pub path_opt: Option<PathBuf>,
    attrs: Attrs<'static>,
    pub editor: Mutex<ViEditor<'static>>,
}

impl Tab {
    pub fn new() -> Self {
        let attrs = cosmic_text::Attrs::new().family(cosmic_text::Family::Monospace);

        let editor = SyntaxEditor::new(
            Buffer::new(&mut FONT_SYSTEM.lock().unwrap(), FONT_SIZES[1 /* Body */]),
            &SYNTAX_SYSTEM,
            "base16-eighties.dark",
        )
        .unwrap();

        let mut editor = ViEditor::new(editor);
        editor.set_passthrough(false);

        Self {
            path_opt: None,
            attrs,
            editor: Mutex::new(editor),
        }
    }

    pub fn open(&mut self, path: PathBuf) {
        let mut editor = self.editor.lock().unwrap();
        let mut font_system = FONT_SYSTEM.lock().unwrap();
        let mut editor = editor.borrow_with(&mut font_system);
        match editor.load_text(&path, self.attrs) {
            Ok(()) => {
                log::info!("opened '{}'", path.display());
                self.path_opt = Some(path);
            }
            Err(err) => {
                log::error!("failed to open '{}': {}", path.display(), err);
                self.path_opt = None;
            }
        }
    }

    pub fn save(&mut self) {
        if let Some(path) = &self.path_opt {
            let editor = self.editor.lock().unwrap();
            let mut text = String::new();
            for line in editor.buffer().lines.iter() {
                text.push_str(line.text());
                text.push('\n');
            }
            match fs::write(path, text) {
                Ok(()) => {
                    log::info!("saved '{}'", path.display());
                }
                Err(err) => {
                    log::error!("failed to save '{}': {}", path.display(), err);
                }
            }
        } else {
            log::warn!("tab has no path yet");
        }
    }

    pub fn title(&self) -> String {
        //TODO: show full title when there is a conflict
        if let Some(path) = &self.path_opt {
            match path.file_name() {
                Some(file_name_os) => match file_name_os.to_str() {
                    Some(file_name) => file_name.to_string(),
                    None => format!("{}", path.display()),
                },
                None => format!("{}", path.display()),
            }
        } else {
            "New document".to_string()
        }
    }
}
