use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app::App;

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.size());
    let titles = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_first_tab(f, app, chunks[1]),
        1 => draw_second_tab(f, app, chunks[1]),
        2 => draw_third_tab(f, app, chunks[1]),
        _ => {}
    };
}

fn draw_summary(f: &mut Frame, _app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25), Constraint::Percentage(25)])
        //.margin(1)
        .split(area);
    let text1 = vec![
        text::Line::from("Name: eth0"),
        text::Line::from("Index: 1"),
    ];
    let block1 = Block::default().borders(Borders::ALL).title("Network Interface");
    let paragraph1 = Paragraph::new(text1).block(block1).wrap(Wrap { trim: true });
    f.render_widget(paragraph1, chunks[0]);

    let text2 = vec![
        text::Line::from("Count: 1000"),
        text::Line::from("Bytes: 4000"),
    ];
    let block2 = Block::default().borders(Borders::ALL).title("Total Packets");
    let paragraph2 = Paragraph::new(text2).block(block2).wrap(Wrap { trim: true });
    f.render_widget(paragraph2, chunks[1]);

    let text3 = vec![
        text::Line::from("Count: 1000"),
        text::Line::from("Bytes: 4000"),
    ];
    let block3 = Block::default().borders(Borders::ALL).title("Total Ingress Bytes");
    let paragraph3 = Paragraph::new(text3).block(block3).wrap(Wrap { trim: true });
    f.render_widget(paragraph3, chunks[2]);

    let text4 = vec![
        text::Line::from("Count: 1000"),
        text::Line::from("Bytes: 4000"),
    ];
    let block4 = Block::default().borders(Borders::ALL).title("Total Egress Bytes");
    let paragraph4 = Paragraph::new(text4).block(block4).wrap(Wrap { trim: true });
    f.render_widget(paragraph4, chunks[3]);

}

fn draw_top_data(f: &mut Frame, _app: &mut App, area: Rect) {
    let area_chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .direction(Direction::Horizontal)
        .split(area);
    {
        let inner_chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area_chunks[0]);

        // Draw top Remote Address Table
        let rows = [
            Row::new(vec!["Cell1-1", "Cell1-2", "Cell1-3", "Cell1-4", "Cell1-5"]),
            Row::new(vec!["Cell2-1", "Cell2-2", "Cell2-3", "Cell2-4", "Cell1-5"]),
            Row::new(vec!["Cell3-1", "Cell3-2", "Cell3-3", "Cell3-4", "Cell1-5"]),
            Row::new(vec!["Cell4-1", "Cell4-2", "Cell4-3", "Cell4-4", "Cell1-5"]),
            ];
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let mut table_state = TableState::default();
        let table = Table::new(rows, widths)
        .column_spacing(1)
        //.style(Style::new().blue())
        .header(
            Row::new(vec!["IP Address", "Host Name", "ASN", "AS Name", "Country"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Remote Address"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");
        
        f.render_stateful_widget(table, inner_chunks[0], &mut table_state);
        //f.render_stateful_widget(processes, chunks[0], &mut app.top_processes.state);

        let chunks = Layout::default()
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .direction(Direction::Horizontal)
            .split(inner_chunks[1]);

        // Draw top Network-Active Processes Table
        let mut table_state = TableState::default();
        let rows = [
            Row::new(vec!["Cell1-1", "Cell1-2", "Cell1-3", "Cell1-4"]),
            Row::new(vec!["Cell2-1", "Cell2-2", "Cell2-3", "Cell2-4"]),
            Row::new(vec!["Cell3-1", "Cell3-2", "Cell3-3", "Cell3-4"]),
            Row::new(vec!["Cell4-1", "Cell4-2", "Cell4-3", "Cell4-4"]),
            ];
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["Process ID", "Process Name", "↓ Bytes", "↑ Bytes"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Network-Active Processes"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>");
        
        //f.render_widget(table, chunks[0]);
        f.render_stateful_widget(table, chunks[0], &mut table_state);
        //f.render_stateful_widget(remote_hosts, chunks[0], &mut app.remote_hosts.state);

        // Draw top Protocols Table
        let rows = [
            Row::new(vec!["Cell1-1", "Cell1-2", "Cell1-3", "Cell1-4"]),
            Row::new(vec!["Cell2-1", "Cell2-2", "Cell2-3", "Cell2-4"]),
            Row::new(vec!["Cell3-1", "Cell3-2", "Cell3-3", "Cell3-4"]),
            Row::new(vec!["Cell4-1", "Cell4-2", "Cell4-3", "Cell4-4"]),
            ];
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Length(12),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
        ];

        let mut table_state = TableState::default();
        let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["Service Name", "Port", "↓ Bytes", "↑ Bytes"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Protocols"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");
        
        f.render_stateful_widget(table, chunks[1], &mut table_state);
        //f.render_stateful_widget(protocols, chunks[1], &mut app.top_protocols.state);
    }
}

fn draw_text(f: &mut Frame, area: Rect) {
    let text = vec![
        text::Line::from("This is a paragraph with several lines. You can change style your text the way you want"),
        text::Line::from(""),
        text::Line::from(vec![
            Span::from("For example: "),
            Span::styled("under", Style::default().fg(Color::Red)),
            Span::raw(" "),
            Span::styled("the", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("rainbow", Style::default().fg(Color::Blue)),
            Span::raw("."),
        ]),
        text::Line::from(vec![
            Span::raw("Oh and if you didn't "),
            Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw(" you can "),
            Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
            Span::raw(" your "),
            Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
            Span::raw(".")
        ]),
        text::Line::from(
            "One more thing is that it should display unicode characters: 10€"
        ),
    ];
    let block = Block::default().borders(Borders::ALL).title(Span::styled(
        "Footer",
        Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    ));
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

fn draw_color_table(f: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(100)])
        .split(area);
    let colors = [
        Color::Reset,
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::Gray,
        Color::DarkGray,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::White,
    ];
    let items: Vec<Row> = colors
        .iter()
        .map(|c| {
            let cells = vec![
                Cell::from(Span::raw(format!("{c:?}: "))),
                Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
                Cell::from(Span::styled("Background", Style::default().bg(*c))),
            ];
            Row::new(cells)
        })
        .collect();
    let table = Table::new(
        items,
        [
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ],
    )
    .block(Block::default().title("Colors").borders(Borders::ALL));
    f.render_widget(table, chunks[0]);
}

fn draw_first_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
        ])
        .split(area);
    draw_summary(f, app, chunks[0]);
    draw_top_data(f, app, chunks[1]);
}

fn draw_second_tab(f: &mut Frame, _app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![
            Constraint::Percentage(60), 
            Constraint::Percentage(40)])
        .split(area);
    draw_color_table(f, chunks[0]);
    draw_text(f, chunks[1]);
}

fn draw_third_tab(f: &mut Frame, _app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![
            Constraint::Percentage(60), 
            Constraint::Percentage(40)])
        .split(area);
    draw_color_table(f, chunks[0]);
    draw_text(f, chunks[1]);
}
