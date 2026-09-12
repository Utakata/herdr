use std::borrow::Cow;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Wrap},
    Frame,
};

use super::release_notes::release_notes_close_button_rect;
use super::scrollbar::{release_notes_scrollbar_rect, render_scrollbar};
use super::widgets::{
    modal_stack_areas, panel_contrast_fg, render_action_button, render_modal_header,
    render_modal_shell,
};
use crate::app::AppState;

pub(super) type HelpEntry = (String, Cow<'static, str>);
pub(super) type HelpGroup = (&'static str, Vec<HelpEntry>);

fn help_entry(key: impl Into<String>, label: &'static str) -> HelpEntry {
    (key.into(), Cow::Borrowed(label))
}

fn keybind_label(bindings: &crate::config::ActionKeybinds) -> String {
    bindings.label().unwrap_or_else(|| "未設定".to_string())
}

fn indexed_label(bindings: &[crate::config::IndexedKeybind]) -> String {
    if bindings.is_empty() {
        return "未設定".to_string();
    }

    let mut parts = Vec::new();
    let mut index = 0;
    while index < bindings.len() {
        if let Some(prefix) = indexed_range_prefix(&bindings[index..]) {
            parts.push(format!("{prefix}1..9"));
            index += 9;
        } else {
            parts.push(bindings[index].label.clone());
            index += 1;
        }
    }

    parts.join(" / ")
}

fn indexed_range_prefix(bindings: &[crate::config::IndexedKeybind]) -> Option<&str> {
    let run = bindings.get(..9)?;
    let prefix = run[0].label.strip_suffix('1')?;
    for (offset, binding) in run.iter().enumerate() {
        let digit = char::from(b'1' + offset as u8);
        if binding.label.strip_suffix(digit) != Some(prefix) {
            return None;
        }
    }
    Some(prefix)
}

pub(super) fn keybind_help_groups(app: &AppState) -> Vec<HelpGroup> {
    let kb = &app.keybinds;
    let mut groups = Vec::new();

    groups.push((
        "グローバル",
        vec![
            help_entry(
                crate::config::format_key_combo((app.prefix_code, app.prefix_mods)),
                "プレフィックスモード",
            ),
            help_entry(keybind_label(&kb.help), "キーバインド"),
            help_entry(keybind_label(&kb.settings), "設定"),
            help_entry(keybind_label(&kb.detach), "デタッチ"),
            help_entry(keybind_label(&kb.reload_config), "設定を再読み込み"),
            help_entry(
                keybind_label(&kb.open_notification_target),
                "通知ターゲットを開く",
            ),
        ],
    ));

    groups.push((
        "ナビゲーション",
        vec![
            help_entry("esc", "back"),
            help_entry(
                format!(
                    "{} / {}",
                    keybind_label(&kb.navigate.workspace_up),
                    keybind_label(&kb.navigate.workspace_down)
                ),
                "ワークスペースリスト",
            ),
            help_entry(
                format!(
                    "{} / {} / {} / {} / left / right",
                    keybind_label(&kb.navigate.pane_left),
                    keybind_label(&kb.navigate.pane_down),
                    keybind_label(&kb.navigate.pane_up),
                    keybind_label(&kb.navigate.pane_right)
                ),
                "フォーカスを移動",
            ),
            help_entry("tab / shift+tab", "ペインを循環"),
            help_entry("enter", "ワークスペースを開く"),
            help_entry("1..9", "ワークスペースを切り替え"),
        ],
    ));

    let workspace_tab = vec![
        help_entry(
            keybind_label(&kb.workspace_picker),
            "ワークスペースナビゲーション",
        ),
        help_entry(keybind_label(&kb.goto), "セッションナビゲーター"),
        help_entry(keybind_label(&kb.new_workspace), "新しいワークスペース"),
        help_entry(keybind_label(&kb.new_worktree), "新しいワークツリー"),
        help_entry(keybind_label(&kb.open_worktree), "ワークツリーを開く"),
        help_entry(
            keybind_label(&kb.remove_worktree),
            "ワークツリーのチェックアウトを削除",
        ),
        help_entry(
            keybind_label(&kb.rename_workspace),
            "ワークスペース名を変更",
        ),
        help_entry(keybind_label(&kb.close_workspace), "ワークスペースを閉じる"),
        help_entry(keybind_label(&kb.previous_workspace), "前のワークスペース"),
        help_entry(keybind_label(&kb.next_workspace), "次のワークスペース"),
        help_entry(indexed_label(&kb.switch_workspace), "switch workspace 1-9"),
        help_entry(keybind_label(&kb.previous_agent), "前のエージェント"),
        help_entry(keybind_label(&kb.next_agent), "次のエージェント"),
        help_entry(indexed_label(&kb.focus_agent), "focus agent 1-9"),
        help_entry(keybind_label(&kb.new_tab), "新しいタブ"),
        help_entry(keybind_label(&kb.rename_tab), "タブ名を変更"),
        help_entry(keybind_label(&kb.previous_tab), "前のタブ"),
        help_entry(keybind_label(&kb.next_tab), "次のタブ"),
        help_entry(keybind_label(&kb.move_tab_previous), "タブを左に移動"),
        help_entry(keybind_label(&kb.move_tab_next), "タブを右に移動"),
        help_entry(indexed_label(&kb.switch_tab), "switch tab 1-9"),
        help_entry(keybind_label(&kb.close_tab), "タブを閉じる"),
    ];
    groups.push(("workspaces / tabs", workspace_tab));

    let panes = vec![
        help_entry(keybind_label(&kb.split_vertical), "垂直分割"),
        help_entry(keybind_label(&kb.split_horizontal), "水平分割"),
        help_entry(keybind_label(&kb.close_pane), "ペインを閉じる"),
        help_entry(keybind_label(&kb.rename_pane), "ペイン名を変更"),
        help_entry(keybind_label(&kb.edit_scrollback), "スクロールバックを編集"),
        help_entry(keybind_label(&kb.copy_mode), "コピーモード"),
        help_entry(keybind_label(&kb.zoom), "ペインをズーム"),
        help_entry(keybind_label(&kb.resize_mode), "サイズ変更モード"),
        help_entry(
            keybind_label(&kb.resize_pane_left),
            "ペインを左にサイズ変更",
        ),
        help_entry(
            keybind_label(&kb.resize_pane_down),
            "ペインを下にサイズ変更",
        ),
        help_entry(keybind_label(&kb.resize_pane_up), "ペインを上にサイズ変更"),
        help_entry(
            keybind_label(&kb.resize_pane_right),
            "ペインを右にサイズ変更",
        ),
        help_entry(keybind_label(&kb.toggle_sidebar), "サイドバーの切り替え"),
        help_entry(keybind_label(&kb.focus_pane_left), "左のペインにフォーカス"),
        help_entry(keybind_label(&kb.focus_pane_down), "下のペインにフォーカス"),
        help_entry(keybind_label(&kb.focus_pane_up), "上のペインにフォーカス"),
        help_entry(
            keybind_label(&kb.focus_pane_right),
            "右のペインにフォーカス",
        ),
        help_entry(keybind_label(&kb.cycle_pane_next), "次のペインへ循環"),
        help_entry(keybind_label(&kb.cycle_pane_previous), "前のペインへ循環"),
        help_entry(keybind_label(&kb.last_pane), "最後のペイン"),
    ];
    groups.push(("ペイン", panes));

    if !kb.custom_commands.is_empty() {
        groups.push((
            "カスタム",
            kb.custom_commands
                .iter()
                .map(|binding| {
                    (
                        binding.label.clone(),
                        binding
                            .description
                            .clone()
                            .map(Cow::Owned)
                            .unwrap_or(Cow::Borrowed("カスタムコマンド")),
                    )
                })
                .collect(),
        ));
    }

    groups
}

fn filter_keybind_help_groups(groups: Vec<HelpGroup>, query: &str) -> Vec<HelpGroup> {
    if query.is_empty() {
        return groups;
    }

    let query = query.to_lowercase();
    groups
        .into_iter()
        .filter_map(|(group, entries)| {
            let entries = entries
                .into_iter()
                .filter(|(key, label)| {
                    key.to_lowercase().contains(&query) || label.to_lowercase().contains(&query)
                })
                .collect::<Vec<_>>();
            (!entries.is_empty()).then_some((group, entries))
        })
        .collect()
}

pub(crate) fn keybind_help_lines(app: &AppState) -> Vec<(usize, Line<'static>)> {
    let heading_style = Style::default()
        .fg(app.palette.accent)
        .add_modifier(Modifier::BOLD);
    let key_style = Style::default()
        .fg(app.palette.mauve)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(app.palette.text);

    let groups = filter_keybind_help_groups(keybind_help_groups(app), &app.keybind_help.query);
    let key_width = groups
        .iter()
        .flat_map(|(_, entries)| entries.iter().map(|(key, _)| key.chars().count()))
        .max()
        .unwrap_or(8);

    let mut lines = Vec::new();

    if groups.is_empty() {
        let message = " 一致するキーバインドがありません";
        return vec![(
            message.chars().count(),
            Line::from(Span::styled(
                message,
                Style::default().fg(app.palette.overlay1),
            )),
        )];
    }

    for (group, entries) in groups {
        lines.push((
            group.len() + 1,
            Line::from(vec![Span::styled(format!(" {group}"), heading_style)]),
        ));
        for (key, label) in entries {
            let padded_key = format!(" {:<width$} ", key, width = key_width);
            let width = padded_key.chars().count() + label.chars().count();
            lines.push((
                width,
                Line::from(vec![
                    Span::styled(padded_key, key_style),
                    Span::styled(label.into_owned(), label_style),
                ]),
            ));
        }
        lines.push((0, Line::raw("")));
    }

    lines
}

pub(super) fn render_keybind_help_overlay(app: &AppState, frame: &mut Frame) {
    super::dim_background(frame, frame.area());

    let Some(inner) = render_modal_shell(frame, frame.area(), 76, 22, &app.palette) else {
        return;
    };
    if inner.height < 6 || inner.width < 20 {
        return;
    }

    let stack = modal_stack_areas(inner, 2, 1, 0, 1);
    let header_rows =
        Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas::<2>(stack.header);

    render_modal_header(frame, header_rows[0], "キーバインド", &app.palette);
    render_action_button(
        frame,
        release_notes_close_button_rect(header_rows[0]),
        Some("esc"),
        if app.keybind_help.search_focused {
            "back"
        } else {
            "close"
        },
        Style::default()
            .fg(panel_contrast_fg(&app.palette))
            .bg(app.palette.accent)
            .add_modifier(Modifier::BOLD),
    );
    let search_line = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(
                " / ",
                Style::default()
                    .fg(app.palette.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                app.keybind_help.query.as_str(),
                Style::default()
                    .fg(app.palette.text)
                    .add_modifier(Modifier::BOLD),
            ),
        ])
    } else {
        Line::from(Span::styled(
            " press / to filter by command or shortcut",
            Style::default().fg(app.palette.overlay0),
        ))
    };
    frame.render_widget(Paragraph::new(search_line), header_rows[1]);

    let body_area = stack.content;
    let metrics = crate::pane::ScrollMetrics {
        offset_from_bottom: app
            .keybind_help_max_scroll()
            .saturating_sub(app.keybind_help.scroll) as usize,
        max_offset_from_bottom: app.keybind_help_max_scroll() as usize,
        viewport_rows: body_area.height.max(1) as usize,
    };
    let track = release_notes_scrollbar_rect(body_area, metrics);
    let text_area = track
        .map(|_| {
            Rect::new(
                body_area.x,
                body_area.y,
                body_area.width.saturating_sub(1),
                body_area.height,
            )
        })
        .unwrap_or(body_area);

    let body = Paragraph::new(
        keybind_help_lines(app)
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>(),
    )
    .wrap(Wrap { trim: false })
    .scroll((app.keybind_help.scroll, 0));
    frame.render_widget(body, text_area);
    if let Some(track) = track {
        render_scrollbar(
            frame,
            metrics,
            track,
            app.palette.overlay0,
            app.palette.overlay1,
            "▐",
        );
    }

    let footer = if app.keybind_help.search_focused {
        Line::from(vec![
            Span::styled(" フィルター ", Style::default().fg(app.palette.overlay0)),
            Span::styled("type/backspace", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("クリア ", Style::default().fg(app.palette.overlay0)),
            Span::styled("ctrl+u", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("スクロール ", Style::default().fg(app.palette.overlay0)),
            Span::styled("↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("back ", Style::default().fg(app.palette.overlay0)),
            Span::styled("esc", Style::default().fg(app.palette.text)),
        ])
    } else {
        Line::from(vec![
            Span::styled(" 検索 ", Style::default().fg(app.palette.overlay0)),
            Span::styled("/", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("スクロール ", Style::default().fg(app.palette.overlay0)),
            Span::styled("j/k/↑↓/pgup/pgdn", Style::default().fg(app.palette.text)),
            Span::styled(" · ", Style::default().fg(app.palette.overlay0)),
            Span::styled("close ", Style::default().fg(app.palette.overlay0)),
            Span::styled("esc/enter", Style::default().fg(app.palette.text)),
        ])
    };
    frame.render_widget(Paragraph::new(footer), stack.footer.unwrap_or_default());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn groups() -> Vec<HelpGroup> {
        vec![
            (
                "workspaces / tabs",
                vec![
                    help_entry("w", "ワークスペースナビゲーション"),
                    help_entry("c", "新しいタブ"),
                ],
            ),
            (
                "panes",
                vec![
                    help_entry("v", "垂直分割"),
                    help_entry("x", "ペインを閉じる"),
                ],
            ),
        ]
    }

    #[test]
    fn keybind_help_filter_matches_labels_case_insensitively() {
        let filtered = filter_keybind_help_groups(groups(), "WoRk");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "workspaces / tabs");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "ワークスペースナビゲーション");
    }

    #[test]
    fn keybind_help_filter_matches_shortcuts_without_matching_group_headings() {
        let filtered = filter_keybind_help_groups(groups(), "x");

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].0, "panes");
        assert_eq!(filtered[0].1.len(), 1);
        assert_eq!(filtered[0].1[0].1, "ペインを閉じる");

        assert!(filter_keybind_help_groups(groups(), "panes").is_empty());
    }
}
