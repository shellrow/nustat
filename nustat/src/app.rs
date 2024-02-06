use nustat_core::{net::{host::HostDisplayInfo, service::ServiceDisplayInfo, stat::NetStatData}, process::ProcessDisplayInfo, socket::SocketTrafficInfo};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub netstat_data: NetStatData,
    pub top_remote_hosts: Vec<HostDisplayInfo>,
    pub top_processes: Vec<ProcessDisplayInfo>,
    pub top_connections: Vec<SocketTrafficInfo>,
    pub top_app_protocols: Vec<ServiceDisplayInfo>,
    pub enhanced_graphics: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Dashboard", "RemoteAddresses", "Connections"]),
            netstat_data: NetStatData::new(),
            top_remote_hosts: vec![],
            top_processes: vec![],
            top_connections: vec![],
            top_app_protocols: vec![],
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        
    }

    pub fn on_down(&mut self) {
        
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {
                // TODO!
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update the state of the application
        // TODO!
    }
}