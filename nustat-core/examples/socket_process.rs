use nustat_core::socket::SocketInfoOption;

fn main() {
    let socket_info = nustat_core::socket::get_sockets_info(SocketInfoOption::default());
    for info in socket_info {
        let process_info = info.process.unwrap();
        println!("[{}][{}][{:?}] {}:{} -> {:?}:{:?}", process_info.pid, process_info.name, info.protocol, info.local_ip_addr, info.local_port, info.remote_ip_addr, info.remote_port);
    }
}
