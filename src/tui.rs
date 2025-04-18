use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    prelude::{Alignment, Constraint::Max, Position, Stylize},
    style::{palette::tailwind, Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};

use crate::messages::Content;
use crate::messages::Message;
use crate::models::Model;
use crate::stream::StreamBuilder;
use crate::traits::CTrait;
use crate::utils::extract_text_from_partial_json;
use crate::Client;

use futures_util::StreamExt;
use std::io;
use std::thread;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::{Display as DeriveDisplay, EnumIter as DeriveEnumIter};
use tui_input::{backend::crossterm::EventHandler as InputHandler, Input};

#[derive(Debug, Clone, DeriveEnumIter, DeriveDisplay, PartialEq)]
enum CurrentInput {
    ApiKey,
    SelectedModel,
    ChatInput,
}

#[derive(Debug, Clone, DeriveEnumIter, DeriveDisplay, PartialEq)]
enum Tab {
    #[strum(to_string = "🔐 Settings")]
    Settings,
    #[strum(to_string = "💬 Chat")]
    Chat,
    #[strum(to_string = "📜 History")]
    History,
    #[strum(to_string = "⚙️ System")]
    System,
}

impl Tab {
    fn next(self) -> Self {
        let mut iter = Tab::iter().cycle();
        while let Some(tab) = iter.next() {
            if tab == self {
                return iter.next().unwrap_or(self);
            }
        }
        self
    }

    fn previous(self) -> Self {
        let tabs: Vec<_> = Tab::iter().collect();
        let idx = tabs.iter().position(|t| *t == self).unwrap_or(0);
        if idx == 0 {
            tabs.last().cloned().unwrap_or(self)
        } else {
            tabs[idx - 1].clone()
        }
    }
}

#[derive(Debug, Clone, DeriveEnumIter, DeriveDisplay, PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

struct App {
    input_mode: InputMode,
    selected_tab: Tab,
    api_key: Input,
    selected_model: Input,
    user_input: Input,
    client: Option<Client>,
    chat_history: Vec<Line<'static>>,
    current_input: Option<CurrentInput>,
    scroll_chat: u16,
}

impl Default for App {
    fn default() -> Self {
        Self {
            input_mode: InputMode::Normal,
            selected_tab: Tab::Settings,
            api_key: Input::default(),
            selected_model: Input::new("gemini-2.0-flash".to_string()),
            user_input: Input::default(),
            chat_history: vec![],
            current_input: None,
            client: None,
            scroll_chat: 0,
        }
    }
}

pub async fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let res = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        println!("{:?}", e);
    }

    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut app = App::default();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;

                        match app.selected_tab {
                            Tab::Settings => app.current_input = Some(CurrentInput::ApiKey),
                            Tab::Chat => app.current_input = Some(CurrentInput::ChatInput),
                            _ => app.current_input = None,
                        }
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        app.selected_tab = app.selected_tab.next()
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        app.selected_tab = app.selected_tab.previous()
                    }
                    KeyCode::Up => {
                        if app.scroll_chat > 0 {
                            app.scroll_chat -= 1;
                        }
                    }
                    KeyCode::Down => {
                        app.scroll_chat += 1;
                    }
                    KeyCode::PageUp => {
                        app.scroll_chat = app.scroll_chat.saturating_sub(5);
                    }
                    KeyCode::PageDown => {
                        app.scroll_chat += 5;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Enter => match app.selected_tab {
                        Tab::Chat => {
                            app.scroll_chat = 0;
                            let msg = app.user_input.value().to_string();

                            let user_msg =
                                Line::from(vec![Span::raw::<String>(format!("{} :You 🧑", msg))])
                                    .alignment(Alignment::Right);
                            app.chat_history.push(user_msg);

                            let parameters = StreamBuilder::default()
                                .model(Model::Flash20)
                                .input(Message::User {
                                    content: Content::Text(msg),
                                    name: None,
                                })
                                .build()?;

                            let response = app
                                .client
                                .clone()
                                .unwrap_or_default()
                                .stream()
                                .generate(parameters)
                                .await?;
                            let mut stream = response.bytes_stream();
                            while let Some(mut chunk) = stream.next().await {
                                if let Ok(parsed_json) =
                                    std::str::from_utf8(chunk.as_mut().unwrap())
                                {
                                    if let Some(text_value) =
                                        extract_text_from_partial_json(parsed_json)
                                    {
                                        let lines: Vec<&str> = text_value
                                            .split("\\n")
                                            .flat_map(|s| s.split('\n'))
                                            .collect();

                                        for line in lines {
                                            let cleaned_line = line.replace('\\', "");

                                            match app.chat_history.last_mut() {
                                                Some(last_msg)
                                                    if last_msg
                                                        .spans
                                                        .first()
                                                        .map(|s| {
                                                            s.content.starts_with("🤖 Gemini:")
                                                        })
                                                        .unwrap_or(false) =>
                                                {
                                                    last_msg.spans.push(Span::raw(format!(
                                                        " {}",
                                                        cleaned_line
                                                    )));
                                                }
                                                _ => {
                                                    app.chat_history.push(
                                                        Line::from(vec![
                                                            Span::styled(
                                                                "🤖 Gemini: ",
                                                                Style::default()
                                                                    .fg(Color::LightBlue),
                                                            ),
                                                            Span::raw(cleaned_line),
                                                        ])
                                                        .alignment(Alignment::Left),
                                                    );
                                                }
                                            }

                                            thread::sleep(Duration::from_millis(50));
                                        }
                                    }
                                } else {
                                    app.chat_history.push(
                                        format!(
                                            "Failed to parse chunk: {:?}",
                                            chunk.as_ref().unwrap()
                                        )
                                        .into(),
                                    );
                                }
                            }

                            app.user_input.reset();
                        }
                        Tab::Settings => {
                            let api_key = app.api_key.value().to_string();
                            let model = app.selected_model.value().to_string();
                            let gemini_client = Client::builder().model(&model).build()?;
                            gemini_client.set_api_key(api_key);
                            app.client = Some(gemini_client);
                        }
                        _ => {}
                    },
                    KeyCode::Tab => match app.current_input {
                        Some(CurrentInput::ApiKey) => {
                            app.current_input = Some(CurrentInput::SelectedModel);
                        }
                        Some(CurrentInput::SelectedModel) => {
                            app.current_input = Some(CurrentInput::ApiKey);
                        }
                        _ => {}
                    },
                    _ => match app.selected_tab {
                        Tab::Settings => match app.current_input {
                            Some(CurrentInput::ApiKey) => {
                                app.api_key.handle_event(&Event::Key(key));
                            }
                            Some(CurrentInput::SelectedModel) => {
                                app.selected_model.handle_event(&Event::Key(key));
                            }
                            _ => {}
                        },
                        Tab::Chat => {
                            let _ = app.user_input.handle_event(&Event::Key(key));
                        }
                        _ => {}
                    },
                },
            }
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    let titles = Tab::iter().map(|t| {
        Line::from(vec![Span::styled(
            t.to_string(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )])
    });

    let tabs = Tabs::new(titles.collect::<Vec<_>>())
        .select(app.selected_tab.clone() as usize)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .highlight_style(Style::default().fg(Color::Yellow));

    f.render_widget(tabs, chunks[0]);

    match app.selected_tab {
        Tab::Settings => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
                .split(chunks[1]);

            let scroll_api = app.api_key.visual_scroll(layout[0].width as usize);
            let scroll_model = app.selected_model.visual_scroll(layout[1].width as usize);

            let api = Paragraph::new(app.api_key.value())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => {
                        if let Some(CurrentInput::ApiKey) = app.current_input {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default()
                        }
                    }
                })
                .scroll((0, scroll_api as u16))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("🔑 Gemini API Key"),
                );

            let model = Paragraph::new(app.selected_model.value())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => {
                        if let Some(CurrentInput::SelectedModel) = app.current_input {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default()
                        }
                    }
                })
                .scroll((0, scroll_model as u16))
                .block(Block::default().borders(Borders::ALL).title("🤖 LLM Model"));

            f.render_widget(api, layout[0]);
            f.render_widget(model, layout[1]);

            if app.input_mode == InputMode::Editing {
                match app.current_input {
                    Some(CurrentInput::ApiKey) => {
                        let x = layout[0].x
                            + ((app.api_key.visual_cursor()).max(scroll_api) - scroll_api) as u16
                            + 1;
                        let y = layout[0].y + 1;
                        f.set_cursor_position(Position::new(x, y));
                    }
                    Some(CurrentInput::SelectedModel) => {
                        let x = layout[1].x
                            + ((app.selected_model.visual_cursor()).max(scroll_model)
                                - scroll_model) as u16
                            + 1;
                        let y = layout[1].y + 1;
                        f.set_cursor_position(Position::new(x, y));
                    }
                    _ => {}
                }
            }
        }

        Tab::Chat => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
                .split(chunks[1]);

            let chat = Paragraph::new(app.chat_history.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("💬 Conversation"),
                )
                .wrap(Wrap { trim: true })
                .scroll((app.scroll_chat, 0));

            let scroll_input = app.user_input.visual_scroll(layout[1].width as usize);

            let input = Paragraph::new(app.user_input.value())
                .style(match app.input_mode {
                    InputMode::Normal => Style::default(),
                    InputMode::Editing => {
                        if let Some(CurrentInput::ChatInput) = app.current_input {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default()
                        }
                    }
                })
                .scroll((0, scroll_input as u16))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Type your message"),
                );

            f.render_widget(chat, layout[0]);
            f.render_widget(input, layout[1]);

            if app.input_mode == InputMode::Editing {
                if let Some(CurrentInput::ChatInput) = app.current_input {
                    let x = layout[1].x
                        + ((app.user_input.visual_cursor()).max(scroll_input) - scroll_input)
                            as u16
                        + 1;
                    let y = layout[1].y + 1;
                    f.set_cursor_position(Position::new(x, y));
                }
            }
        }

        Tab::History => {
            let para = Paragraph::new("History feature coming soon...")
                .block(Block::default().borders(Borders::ALL).title("📜 History"));
            f.render_widget(para, chunks[1]);
        }
        Tab::System => {
            let para = Paragraph::new("System info and logs coming soon...")
                .block(Block::default().borders(Borders::ALL).title("⚙️ System"));
            f.render_widget(para, chunks[1]);
        }
    }

    let footer_layout = Layout::new(Direction::Vertical, [Max(1), Max(1), Max(1)]).split(chunks[2]);

    let top_footer = Line::raw(
        "◄ ► or a/d: switch tabs | ↑ ↓: scroll | e: edit | Tab: next input | Esc: cancel | Enter: save | q: quit",
    )
    .centered();

    let bottom_footer = Line::raw("© Kevin RS Foundation")
        .fg(Color::LightGreen)
        .bg(tailwind::SLATE.c700)
        .bold()
        .centered();

    f.render_widget(top_footer, footer_layout[0]);
    f.render_widget(bottom_footer, footer_layout[2]);
}
