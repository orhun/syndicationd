use crate::{
    application::{Direction, IndexOutOfRange, ListAction},
    client::payload,
    types::{self, TimeExt},
    ui::{self, Context},
};
use ratatui::{
    prelude::{Alignment, Buffer, Constraint, Layout, Margin, Rect},
    text::{Span, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Table, TableState, Widget, Wrap,
    },
};

pub struct Entries {
    selected_entry_index: usize,
    entries: Vec<types::Entry>,
}

impl Entries {
    pub fn new() -> Self {
        Self {
            selected_entry_index: 0,
            entries: Vec::new(),
        }
    }

    pub fn update_entries(&mut self, action: ListAction, payload: payload::FetchEntriesPayload) {
        match action {
            ListAction::Append => self.entries.extend(payload.entries),
            ListAction::Replace => self.entries = payload.entries,
        }
    }

    pub fn remove_unsubscribed_entries(&mut self, url: &str) {
        self.entries.retain(|entry| entry.feed_url != url);
    }

    pub fn move_selection(&mut self, direction: &Direction) {
        self.selected_entry_index = direction.apply(
            self.selected_entry_index,
            self.entries.len(),
            IndexOutOfRange::Wrapping,
        );
    }

    pub fn move_first(&mut self) {
        self.selected_entry_index = 0;
    }

    pub fn move_last(&mut self) {
        if !self.entries.is_empty() {
            self.selected_entry_index = self.entries.len() - 1;
        }
    }

    pub fn selected_entry_website_url(&self) -> Option<&str> {
        self.selected_entry()
            .and_then(|entry| entry.website_url.as_deref())
    }

    fn selected_entry(&self) -> Option<&types::Entry> {
        self.entries.get(self.selected_entry_index)
    }
}

impl Entries {
    pub fn render(&self, area: Rect, buf: &mut Buffer, cx: &Context<'_>) {
        let vertical = Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]);
        let [entries_area, summary_area] = vertical.areas(area);

        self.render_entries(entries_area, buf, cx);
        self.render_summary(summary_area, buf, cx);
    }

    fn render_entries(&self, area: Rect, buf: &mut Buffer, cx: &Context<'_>) {
        // padding
        let entries_area = area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        let mut entries_state = TableState::new()
            .with_offset(0)
            .with_selected(self.selected_entry_index);

        let (header, widths, rows) = self.entry_rows(cx);

        let entries = Table::new(rows, widths)
            .header(header.style(cx.theme.entries.header))
            .column_spacing(2)
            .style(cx.theme.entries.background)
            .highlight_symbol(ui::TABLE_HIGHLIGHT_SYMBOL)
            .highlight_style(cx.theme.entries.selected_entry)
            .highlight_spacing(ratatui::widgets::HighlightSpacing::WhenSelected);

        StatefulWidget::render(entries, entries_area, buf, &mut entries_state);

        let scrollbar_area = Rect {
            y: area.y + 2, // table header
            height: area.height.saturating_sub(3),
            ..area
        };

        // https://github.com/ratatui-org/ratatui/pull/911
        // passing None to track_symbol cause incorrect rendering
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(self.entries.len())
            .position(self.selected_entry_index);
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(Some(" "))
            .thumb_symbol("▐")
            .render(scrollbar_area, buf, &mut scrollbar_state);
    }

    fn entry_rows<'a>(
        &'a self,
        _cx: &'a Context<'_>,
    ) -> (
        Row<'a>,
        impl IntoIterator<Item = Constraint>,
        impl IntoIterator<Item = Row<'a>>,
    ) {
        let header = Row::new([
            Cell::from(" Published"),
            Cell::from("󰯂 Entry"),
            Cell::from("󰑫 Feed"),
        ]);

        let constraints = [
            Constraint::Length(11),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ];

        let row = |entry: &'a types::Entry| {
            let title = entry.title.as_deref().unwrap_or(ui::UNKNOWN_SYMBOL);
            let published = entry
                .published
                .as_ref()
                .or(entry.updated.as_ref())
                .map_or_else(|| ui::UNKNOWN_SYMBOL.to_string(), TimeExt::local_ymd);

            let feed_title = entry.feed_title.as_deref().unwrap_or(ui::UNKNOWN_SYMBOL);

            Row::new([
                Cell::from(Span::from(published)),
                Cell::from(Span::from(title)),
                Cell::from(Span::from(feed_title)),
            ])
        };

        (header, constraints, self.entries.iter().map(row))
    }

    fn render_summary(&self, area: Rect, buf: &mut Buffer, _cx: &Context<'_>) {
        let block = Block::new()
            .padding(Padding {
                left: 3,
                right: 3,
                top: 1,
                bottom: 1,
            })
            .title(
                Title::from("Summary")
                    .position(Position::Top)
                    .alignment(Alignment::Center),
            )
            .borders(Borders::TOP)
            .border_type(BorderType::Plain);

        let inner = block.inner(area);
        Widget::render(block, area, buf);

        let Some(entry) = self.selected_entry() else {
            return;
        };
        let Some(summary) = entry.summary_text(inner.width.into()) else {
            return;
        };
        // should to Lines?
        let paragraph = Paragraph::new(Text::from(summary))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);

        Widget::render(paragraph, inner, buf);
    }
}
