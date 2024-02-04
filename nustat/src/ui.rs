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

fn draw_summary(f: &mut Frame, app: &mut App, area: Rect) {
    // Draw network interface
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(25), Constraint::Percentage(25)])
        //.margin(1)
        .split(area);
    let text1 = vec![
        text::Line::from(format!("Name: {}", app.netstat_data.if_name)),
        text::Line::from(format!("Index: {}", app.netstat_data.if_index)),
    ];
    let block1 = Block::default().borders(Borders::ALL).title("Network Interface");
    let paragraph1 = Paragraph::new(text1).block(block1).wrap(Wrap { trim: true });
    f.render_widget(paragraph1, chunks[0]);

    // Draw total ingress
    let text2 = vec![
        text::Line::from(format!("Packets: {}", app.netstat_data.traffic.packet_received)),
        text::Line::from(format!("Bytes: {}", app.netstat_data.traffic.bytes_received)),
    ];
    let block2 = Block::default().borders(Borders::ALL).title("Total Ingress");
    let paragraph2 = Paragraph::new(text2).block(block2).wrap(Wrap { trim: true });
    f.render_widget(paragraph2, chunks[1]);

    // Draw total egress
    let text3 = vec![
        text::Line::from(format!("Packets: {}", app.netstat_data.traffic.packet_sent)),
        text::Line::from(format!("Bytes: {}", app.netstat_data.traffic.bytes_sent)),
    ];
    let block3 = Block::default().borders(Borders::ALL).title("Total Egress");
    let paragraph3 = Paragraph::new(text3).block(block3).wrap(Wrap { trim: true });
    f.render_widget(paragraph3, chunks[2]);

}

fn draw_top_data(f: &mut Frame, app: &mut App, area: Rect) {
    let area_chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .direction(Direction::Horizontal)
        .split(area);
    {
        let inner_chunks = Layout::default()
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area_chunks[0]);

        // Draw top Remote Address Table        
        let rows = app.top_remote_hosts.iter().map(|host| {
            Row::new(vec![
                host.ip_addr.to_string(),
                host.asn.to_string(),
                host.as_name.clone(),
                host.country_code.clone(),
                host.traffic.bytes_received.to_string(),
                host.traffic.bytes_sent.to_string(),
            ])
        }).collect::<Vec<Row>>();
        let widths = [
            Constraint::Length(40),
            Constraint::Length(8),
            Constraint::Length(24),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
        ];

        //let mut table_state = TableState::default();
        let table = Table::new(rows, widths)
        .column_spacing(1)
        //.style(Style::new().blue())
        .header(
            Row::new(vec!["IP Address", "ASN", "AS Name", "Country","↓ Bytes", "↑ Bytes"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Remote Address"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

        f.render_widget(table, inner_chunks[0]);
        //f.render_stateful_widget(table, inner_chunks[0], &mut table_state);
        //f.render_stateful_widget(processes, chunks[0], &mut app.top_processes.state);
        // Draw top Network-Active Processes Table
        /* let mut table_state = TableState::default();
        let rows = app.top_processes.iter().map(|process| {
            Row::new(vec![
                process.pid.to_string(),
                process.name.clone(),
                process.traffic.bytes_received.to_string(),
                process.traffic.bytes_sent.to_string(),
            ])
        }).collect::<Vec<Row>>();
        // Columns widths are constrained in the same way as Layout...
        let widths = [
            Constraint::Length(10),
            Constraint::Length(40),
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
        f.render_stateful_widget(table, inner_chunks[1], &mut table_state);
        //f.render_stateful_widget(remote_hosts, chunks[0], &mut app.remote_hosts.state); */
        // Draw top Connections Table
        //let mut table_state = TableState::default();
        let rows = app.top_connections.iter().map(|conn| {
            let remote_ip_string = if let Some(remote_ip_addr) = &conn.remote_ip_addr {
                remote_ip_addr.to_string()
            } else {"".to_string()};
            let remote_port_string = if let Some(remote_port) = &conn.remote_port {
                remote_port.to_string()
            } else {"".to_string()};
            let mut process_id_string = "".to_string();
            let mut process_name_string = "".to_string();
            if let Some(process) = &conn.process {
                process_id_string = process.pid.to_string();
                process_name_string = process.name.clone();
            }
            Row::new(vec![
                process_id_string,
                process_name_string,
                format!("{}:{}", conn.local_ip_addr.to_string(), conn.local_port.to_string()),
                format!("{}:{}", remote_ip_string, remote_port_string),
                conn.protocol.as_str().to_string(),
                conn.traffic.bytes_received.to_string(),
                conn.traffic.bytes_sent.to_string(),
            ])
        }).collect::<Vec<Row>>();
        let widths = [
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(20),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ];
        let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["Process ID", "Process Name", "Local Socket", "Remote Socket", "Protocol", "↓ Bytes", "↑ Bytes"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Network-Active Processes"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>");
        f.render_widget(table, inner_chunks[1]);
        //f.render_stateful_widget(table, inner_chunks[1], &mut table_state);
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
