use promkit::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    listbox::Listbox,
    text_editor, PromptSignal,
};

pub type Keymap = fn(&Event, &mut crate::jnv::Jnv) -> anyhow::Result<PromptSignal>;

pub fn default(event: &Event, jnv: &mut crate::jnv::Jnv) -> anyhow::Result<PromptSignal> {
    let query_editor_after_mut = jnv.query_editor_snapshot.after_mut();

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            let query = query_editor_after_mut
                .texteditor
                .text_without_cursor()
                .to_string();
            if let Some(mut candidates) = jnv.suggest.prefix_search(query) {
                candidates.sort_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));

                jnv.suggest_state.listbox = Listbox::from_iter(candidates);
                query_editor_after_mut
                    .texteditor
                    .replace(&jnv.suggest_state.listbox.get());

                jnv.keymap.borrow_mut().switch("on_suggest");
            }
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Ok(PromptSignal::Quit),

        // Move cursor.
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            query_editor_after_mut.texteditor.backward();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            query_editor_after_mut.texteditor.forward();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut.texteditor.move_to_head(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('e'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut.texteditor.move_to_tail(),

        Event::Key(KeyEvent {
            code: KeyCode::Char('b'),
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut
            .texteditor
            .move_to_previous_nearest(&query_editor_after_mut.word_break_chars),

        Event::Key(KeyEvent {
            code: KeyCode::Char('f'),
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut
            .texteditor
            .move_to_next_nearest(&query_editor_after_mut.word_break_chars),

        // Erase char(s).
        Event::Key(KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut.texteditor.erase(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('u'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut.texteditor.erase_all(),

        // Erase to the nearest character.
        Event::Key(KeyEvent {
            code: KeyCode::Char('w'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut
            .texteditor
            .erase_to_previous_nearest(&query_editor_after_mut.word_break_chars),

        Event::Key(KeyEvent {
            code: KeyCode::Char('d'),
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => query_editor_after_mut
            .texteditor
            .erase_to_next_nearest(&query_editor_after_mut.word_break_chars),

        // Move up.
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('k'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.backward();
        }

        // Move down.
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char('j'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.forward();
        }

        // Move to tail
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.move_to_tail();
        }

        // Move to head
        Event::Key(KeyEvent {
            code: KeyCode::Char('l'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.move_to_head();
        }

        // Toggle collapse/expand
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.toggle();
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('p'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.expand_all();
        }

        Event::Key(KeyEvent {
            code: KeyCode::Char('n'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.json_state.stream.collapse_all();
        }

        // Input char.
        Event::Key(KeyEvent {
            code: KeyCode::Char(ch),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Char(ch),
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => match query_editor_after_mut.edit_mode {
            text_editor::Mode::Insert => query_editor_after_mut.texteditor.insert(*ch),
            text_editor::Mode::Overwrite => query_editor_after_mut.texteditor.overwrite(*ch),
        },

        _ => (),
    }
    Ok(PromptSignal::Continue)
}

pub fn on_suggest(event: &Event, jnv: &mut crate::jnv::Jnv) -> anyhow::Result<PromptSignal> {
    let query_editor_after_mut = jnv.query_editor_snapshot.after_mut();

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => return Ok(PromptSignal::Quit),

        Event::Key(KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        })
        | Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.suggest_state.listbox.forward();
            query_editor_after_mut
                .texteditor
                .replace(&jnv.suggest_state.listbox.get());
        }

        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }) => {
            jnv.suggest_state.listbox.backward();
            query_editor_after_mut
                .texteditor
                .replace(&jnv.suggest_state.listbox.get());
        }

        _ => {
            jnv.suggest_state.listbox = Listbox::from_iter(Vec::<String>::new());
            jnv.keymap.borrow_mut().switch("default");

            // This block is specifically designed to prevent the default action of toggling collapse/expand
            // from being executed when the Enter key is pressed. This is done from the perspective of user
            // experimentation, ensuring that pressing Enter while in the suggest mode does not trigger
            // the default behavior associated with the Enter key in the default mode.
            if let Event::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) = event
            {
            } else {
                return default(event, jnv);
            }
        }
    }
    Ok(PromptSignal::Continue)
}
