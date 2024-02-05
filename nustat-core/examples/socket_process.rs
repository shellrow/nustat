use nustat_core::socket::SocketInfoOption;

fn main() {
    let local_ip_map = nustat_core::net::interface::get_local_ip_map();
    let socket_info = nustat_core::socket::get_sockets_info(SocketInfoOption::default());
    for info in socket_info {
        let process_info = info.process.unwrap();
        let mut interface_name = "unknown";
        if let Some(name) = local_ip_map.get(&info.local_ip_addr) {
            interface_name = name;
        }
        println!("[{}][{}][{}][{:?}] {}:{} -> {:?}:{:?}", process_info.pid, process_info.name, interface_name, info.protocol, info.local_ip_addr, info.local_port, info.remote_ip_addr, info.remote_port);
    }
}
