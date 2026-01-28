use std::net::{Ipv4Addr, Ipv6Addr};
use bevy::prelude::*;
use bevy_quinnet::server::*;
use bevy_quinnet::server::certificate::CertificateRetrievalMode;

pub fn start_server(mut server: ResMut<QuinnetServer>) {

    server.start_endpoint(
        ServerEndpointConfiguration {
            addr_config: EndpointAddrConfiguration::from_ip(Ipv4Addr::UNSPECIFIED, 6000),
            cert_mode: CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: Ipv6Addr::LOCALHOST.to_string(),
            },
            defaultables: default(),
        }).expect("Could not open Server");




}