use netlink_sys::{
    nl_connect, nl_send_auto, nl_socket_alloc, nlmsg_data, nlmsg_hdr, rtnl_link_get_by_name,
    rtnl_link_ifinfomsg, rtnl_link_info, rtnl_link_info_data, rtnl_link_set_addr,
    rtnl_link_set_flags, rtnl_link_set_ifname, rtnl_link_set_ipv4_addr, rtnl_link_set_link,
    rtnl_link_set_mtu, IFF_UP, NLMSG_DONE, NLM_F_ACK, NLM_F_ACK_TLVS, NLM_F_ATOMIC, NLM_F_CREATE,
    NLM_F_DUMP, NLM_F_EXCL, NLM_F_REPLACE, NLM_F_REQUEST, NLM_F_ROOT,
};
use std::ffi::CString;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    let mut socket = nl_socket_alloc();
    if socket.is_null() {
        return Err(Error::new(
            ErorKind::Other,
            "Failed to allocate netlink socker",
        ));
    }

    if unsafe { nl_connect(socket, 0) } < 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to connect to netlink socket",
        ));
    }

    let mut link_info = rtnl_link_info {
        n: nlmsg_hdr {
            nlmsg_len: 0,
            nlmsg_type: 0,
            nlmsg_flags: 0,
            nlmsg_seq: 0,
            nlmsg_pid: 0,
        },
        ninfo: rtnl_link_info_data {
            nla_len: 0,
            nla_type: 0,
            nla_data: [0; 0],
        },
    };

    let mut ifindex = 0;
    let ifname = CString::new("eth0").unwrap();
    if unsafe { rtnl_link_get_by_name(socket, ifname.as_ptr(), &mut link_info) } == 0 {
        ifindex = unsafe {
            nlmsg_data(
                link_info.n.nh,
                &mut rtnl_link_ifinfomsg::new().header as *mut _ as *mut u8,
            )
        }
        .ifi_index;
    }
    if ifindex == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to get interface index",
        ));
    }

    let ip_addr = "192.168.1.10";
    let mask = "255.255.255.0";
    let gateway = "192.168.1.1";
    let ip_addr = ip_addr.parse().expect("Invalid IP address");
    let mask = mask.parse().expect("Invalid subnet mask");
    let gateway = gateway.parse().expect("Invalid gateway address");
    if unsafe { rtnl_link_set_ipv4_addr(socket, ifindex, ip_addr, mask, gateway) } < 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to set interface IP address",
        ));
    }
    let flags = IFF_UP;
    if unsafe { rtnl_link_set_flags(socket, ifindex, flags, flags) } < 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to set interface flags",
        ));
    }
    if unsafe { nl_send_auto(socket, NLMSG_DONE, NLM_F_ACK | NLM_F_REQUEST) } < 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Failed to send netlink message",
        ));
    }
    Ok(())
}
