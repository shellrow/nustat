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
        .highlight_style(Style::default().fg(Color::LightBlue))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_overview_tab(f, app, chunks[1]),
        1 => draw_remotehosts_tab(f, app, chunks[1]),
        2 => draw_connections_tab(f, app, chunks[1]),
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
        text::Line::from(format!("Index: {}", app.netstat_data.if_index)),
        text::Line::from(format!("Name: {}", app.netstat_data.if_name)),
    ];
    let block1 = Block::default().borders(Borders::ALL).title("Default Network Interface");
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
        let rows = app.remote_hosts.iter().take(10).map(|host| {
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
        .header(
            Row::new(vec!["IP Address", "ASN", "AS Name", "Country","↓ Bytes", "↑ Bytes"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Remote Addresses"))
        .highlight_style(Style::new().reversed())
        .highlight_symbol(">>");

        f.render_widget(table, inner_chunks[0]);
        
        let rows = app.connections.iter().take(10).map(|conn| {
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
                format!("{}:{}", conn.interface_name, conn.local_port.to_string()),
                format!("{}:{}", remote_ip_string, remote_port_string),
                conn.protocol.as_str().to_string(),
                conn.traffic.bytes_received.to_string(),
                conn.traffic.bytes_sent.to_string(),
                process_id_string,
                process_name_string,
            ])
        }).collect::<Vec<Row>>();
        let widths = [
            Constraint::Length(20),
            Constraint::Length(45),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(8),
            Constraint::Length(5),
            Constraint::Length(20),
        ];
        let table = Table::new(rows, widths)
        .column_spacing(1)
        .header(
            Row::new(vec!["Local Socket", "Remote Socket", "Protocol", "↓ Bytes", "↑ Bytes", "PID", "Process Name"])
                .style(Style::new().bold())
                //.bottom_margin(1),
        )
        .block(Block::default().borders(Borders::ALL).title("Top Connections"))
        .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">>");
        f.render_widget(table, inner_chunks[1]);
        //f.render_stateful_widget(table, inner_chunks[1], &mut app.talbe_state);
    }
}

fn draw_remotehosts_table(f: &mut Frame, app: &mut App, area: Rect) {
    // Draw top Remote Address Table        
    let rows = app.remote_hosts.iter().map(|host| {
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
    .block(Block::default().borders(Borders::ALL).title("Remote Addresses"))
    .highlight_style(Style::new().reversed())
    .highlight_symbol(">>");

    //f.render_widget(table, area);
    f.render_stateful_widget(table, area, &mut app.talbe_state);
}

fn draw_connection_table(f: &mut Frame, app: &mut App, area: Rect) {
    let rows = app.connections.iter().map(|conn| {
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
            format!("{}:{}", conn.interface_name, conn.local_port.to_string()),
            format!("{}:{}", remote_ip_string, remote_port_string),
            conn.protocol.as_str().to_string(),
            conn.traffic.bytes_received.to_string(),
            conn.traffic.bytes_sent.to_string(),
            process_id_string,
            process_name_string,
        ])
    }).collect::<Vec<Row>>();
    let widths = [
        Constraint::Length(20),
        Constraint::Length(45),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(5),
        Constraint::Length(20),
    ];
    let table = Table::new(rows, widths)
    .column_spacing(1)
    .header(
        Row::new(vec!["Local Socket", "Remote Socket", "Protocol", "↓ Bytes", "↑ Bytes", "PID", "Process Name"])
            .style(Style::new().bold())
            //.bottom_margin(1),
    )
    .block(Block::default().borders(Borders::ALL).title("Connections"))
    .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
    .highlight_symbol(">>");
    //f.render_widget(table, area);
    f.render_stateful_widget(table, area, &mut app.talbe_state);
}

fn draw_overview_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(4),
            Constraint::Min(8),
        ])
        .split(area);
    draw_summary(f, app, chunks[0]);
    draw_top_data(f, app, chunks[1]);
}

fn draw_remotehosts_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .split(area);
    draw_remotehosts_table(f, app, chunks[0]);
}

fn draw_connections_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .constraints(vec![Constraint::Percentage(100)])
        .split(area);
    draw_connection_table(f, app, chunks[0]);
}
