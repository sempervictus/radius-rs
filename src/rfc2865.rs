// Code generated by machine generator; DO NOT EDIT.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::attribute::Attribute;
use crate::attributes::AVPType;
use crate::packet::Packet;

pub struct RFC2865 {}
impl RFC2865 {
    pub const USER_NAME_TYPE: AVPType = 1;
    pub fn delete_user_name(packet: &mut Packet) {
        packet.delete(Self::USER_NAME_TYPE);
    }
    pub fn lookup_user_name(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::USER_NAME_TYPE)
    }
    pub fn lookup_all_user_name(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::USER_NAME_TYPE)
    }
    pub fn add_user_name(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::USER_NAME_TYPE, &attr);
    }

    pub const USER_PASSWORD_TYPE: AVPType = 2;
    pub fn delete_user_password(packet: &mut Packet) {
        packet.delete(Self::USER_PASSWORD_TYPE);
    }
    pub fn lookup_user_password(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::USER_PASSWORD_TYPE)
    }
    pub fn lookup_all_user_password(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::USER_PASSWORD_TYPE)
    }
    pub fn add_user_password(packet: &mut Packet, value: &[u8]) -> Result<(), String> {
        let attr =
            Attribute::from_user_password(value, packet.get_secret(), packet.get_authenticator())?;
        packet.add(Self::USER_PASSWORD_TYPE, &attr);
        Ok(())
    }

    pub const CHAP_PASSWORD_TYPE: AVPType = 3;
    pub fn delete_chap_password(packet: &mut Packet) {
        packet.delete(Self::CHAP_PASSWORD_TYPE);
    }
    pub fn lookup_chap_password(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CHAP_PASSWORD_TYPE)
    }
    pub fn lookup_all_chap_password(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CHAP_PASSWORD_TYPE)
    }
    pub fn add_chap_password(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::CHAP_PASSWORD_TYPE, &attr);
    }

    pub const NAS_IP_ADDRESS_TYPE: AVPType = 4;
    pub fn delete_nas_ip_address(packet: &mut Packet) {
        packet.delete(Self::NAS_IP_ADDRESS_TYPE);
    }
    pub fn lookup_nas_ip_address(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::NAS_IP_ADDRESS_TYPE)
    }
    pub fn lookup_all_nas_ip_address(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::NAS_IP_ADDRESS_TYPE)
    }
    pub fn add_nas_ip_address(packet: &mut Packet, value: &Ipv4Addr) {
        let attr = Attribute::from_ipv4(value);
        packet.add(Self::NAS_IP_ADDRESS_TYPE, &attr);
    }

    pub const FRAMED_IP_ADDRESS_TYPE: AVPType = 8;
    pub fn delete_framed_ip_address(packet: &mut Packet) {
        packet.delete(Self::FRAMED_IP_ADDRESS_TYPE);
    }
    pub fn lookup_framed_ip_address(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FRAMED_IP_ADDRESS_TYPE)
    }
    pub fn lookup_all_framed_ip_address(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FRAMED_IP_ADDRESS_TYPE)
    }
    pub fn add_framed_ip_address(packet: &mut Packet, value: &Ipv4Addr) {
        let attr = Attribute::from_ipv4(value);
        packet.add(Self::FRAMED_IP_ADDRESS_TYPE, &attr);
    }

    pub const FRAMED_IP_NETMASK_TYPE: AVPType = 9;
    pub fn delete_framed_ip_netmask(packet: &mut Packet) {
        packet.delete(Self::FRAMED_IP_NETMASK_TYPE);
    }
    pub fn lookup_framed_ip_netmask(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FRAMED_IP_NETMASK_TYPE)
    }
    pub fn lookup_all_framed_ip_netmask(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FRAMED_IP_NETMASK_TYPE)
    }
    pub fn add_framed_ip_netmask(packet: &mut Packet, value: &Ipv4Addr) {
        let attr = Attribute::from_ipv4(value);
        packet.add(Self::FRAMED_IP_NETMASK_TYPE, &attr);
    }

    pub const FILTER_ID_TYPE: AVPType = 11;
    pub fn delete_filter_id(packet: &mut Packet) {
        packet.delete(Self::FILTER_ID_TYPE);
    }
    pub fn lookup_filter_id(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FILTER_ID_TYPE)
    }
    pub fn lookup_all_filter_id(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FILTER_ID_TYPE)
    }
    pub fn add_filter_id(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::FILTER_ID_TYPE, &attr);
    }

    pub const LOGIN_IP_HOST_TYPE: AVPType = 14;
    pub fn delete_login_ip_host(packet: &mut Packet) {
        packet.delete(Self::LOGIN_IP_HOST_TYPE);
    }
    pub fn lookup_login_ip_host(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::LOGIN_IP_HOST_TYPE)
    }
    pub fn lookup_all_login_ip_host(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::LOGIN_IP_HOST_TYPE)
    }
    pub fn add_login_ip_host(packet: &mut Packet, value: &Ipv4Addr) {
        let attr = Attribute::from_ipv4(value);
        packet.add(Self::LOGIN_IP_HOST_TYPE, &attr);
    }

    pub const REPLY_MESSAGE_TYPE: AVPType = 18;
    pub fn delete_reply_message(packet: &mut Packet) {
        packet.delete(Self::REPLY_MESSAGE_TYPE);
    }
    pub fn lookup_reply_message(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::REPLY_MESSAGE_TYPE)
    }
    pub fn lookup_all_reply_message(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::REPLY_MESSAGE_TYPE)
    }
    pub fn add_reply_message(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::REPLY_MESSAGE_TYPE, &attr);
    }

    pub const CALLBACK_NUMBER_TYPE: AVPType = 19;
    pub fn delete_callback_number(packet: &mut Packet) {
        packet.delete(Self::CALLBACK_NUMBER_TYPE);
    }
    pub fn lookup_callback_number(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CALLBACK_NUMBER_TYPE)
    }
    pub fn lookup_all_callback_number(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CALLBACK_NUMBER_TYPE)
    }
    pub fn add_callback_number(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::CALLBACK_NUMBER_TYPE, &attr);
    }

    pub const CALLBACK_ID_TYPE: AVPType = 20;
    pub fn delete_callback_id(packet: &mut Packet) {
        packet.delete(Self::CALLBACK_ID_TYPE);
    }
    pub fn lookup_callback_id(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CALLBACK_ID_TYPE)
    }
    pub fn lookup_all_callback_id(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CALLBACK_ID_TYPE)
    }
    pub fn add_callback_id(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::CALLBACK_ID_TYPE, &attr);
    }

    pub const FRAMED_ROUTE_TYPE: AVPType = 22;
    pub fn delete_framed_route(packet: &mut Packet) {
        packet.delete(Self::FRAMED_ROUTE_TYPE);
    }
    pub fn lookup_framed_route(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FRAMED_ROUTE_TYPE)
    }
    pub fn lookup_all_framed_route(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FRAMED_ROUTE_TYPE)
    }
    pub fn add_framed_route(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::FRAMED_ROUTE_TYPE, &attr);
    }

    pub const FRAMED_IPX_NETWORK_TYPE: AVPType = 23;
    pub fn delete_framed_ipx_network(packet: &mut Packet) {
        packet.delete(Self::FRAMED_IPX_NETWORK_TYPE);
    }
    pub fn lookup_framed_ipx_network(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FRAMED_IPX_NETWORK_TYPE)
    }
    pub fn lookup_all_framed_ipx_network(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FRAMED_IPX_NETWORK_TYPE)
    }
    pub fn add_framed_ipx_network(packet: &mut Packet, value: &Ipv4Addr) {
        let attr = Attribute::from_ipv4(value);
        packet.add(Self::FRAMED_IPX_NETWORK_TYPE, &attr);
    }

    pub const STATE_TYPE: AVPType = 24;
    pub fn delete_state(packet: &mut Packet) {
        packet.delete(Self::STATE_TYPE);
    }
    pub fn lookup_state(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::STATE_TYPE)
    }
    pub fn lookup_all_state(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::STATE_TYPE)
    }
    pub fn add_state(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::STATE_TYPE, &attr);
    }

    pub const CLASS_TYPE: AVPType = 25;
    pub fn delete_class(packet: &mut Packet) {
        packet.delete(Self::CLASS_TYPE);
    }
    pub fn lookup_class(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CLASS_TYPE)
    }
    pub fn lookup_all_class(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CLASS_TYPE)
    }
    pub fn add_class(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::CLASS_TYPE, &attr);
    }

    pub const CALLED_STATION_ID_TYPE: AVPType = 30;
    pub fn delete_called_station_id(packet: &mut Packet) {
        packet.delete(Self::CALLED_STATION_ID_TYPE);
    }
    pub fn lookup_called_station_id(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CALLED_STATION_ID_TYPE)
    }
    pub fn lookup_all_called_station_id(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CALLED_STATION_ID_TYPE)
    }
    pub fn add_called_station_id(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::CALLED_STATION_ID_TYPE, &attr);
    }

    pub const CALLING_STATION_ID_TYPE: AVPType = 31;
    pub fn delete_calling_station_id(packet: &mut Packet) {
        packet.delete(Self::CALLING_STATION_ID_TYPE);
    }
    pub fn lookup_calling_station_id(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CALLING_STATION_ID_TYPE)
    }
    pub fn lookup_all_calling_station_id(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CALLING_STATION_ID_TYPE)
    }
    pub fn add_calling_station_id(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::CALLING_STATION_ID_TYPE, &attr);
    }

    pub const NAS_IDENTIFIER_TYPE: AVPType = 32;
    pub fn delete_nas_identifier(packet: &mut Packet) {
        packet.delete(Self::NAS_IDENTIFIER_TYPE);
    }
    pub fn lookup_nas_identifier(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::NAS_IDENTIFIER_TYPE)
    }
    pub fn lookup_all_nas_identifier(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::NAS_IDENTIFIER_TYPE)
    }
    pub fn add_nas_identifier(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::NAS_IDENTIFIER_TYPE, &attr);
    }

    pub const PROXY_STATE_TYPE: AVPType = 33;
    pub fn delete_proxy_state(packet: &mut Packet) {
        packet.delete(Self::PROXY_STATE_TYPE);
    }
    pub fn lookup_proxy_state(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::PROXY_STATE_TYPE)
    }
    pub fn lookup_all_proxy_state(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::PROXY_STATE_TYPE)
    }
    pub fn add_proxy_state(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::PROXY_STATE_TYPE, &attr);
    }

    pub const LOGIN_LAT_SERVICE_TYPE: AVPType = 34;
    pub fn delete_login_lat_service(packet: &mut Packet) {
        packet.delete(Self::LOGIN_LAT_SERVICE_TYPE);
    }
    pub fn lookup_login_lat_service(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::LOGIN_LAT_SERVICE_TYPE)
    }
    pub fn lookup_all_login_lat_service(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::LOGIN_LAT_SERVICE_TYPE)
    }
    pub fn add_login_lat_service(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::LOGIN_LAT_SERVICE_TYPE, &attr);
    }

    pub const LOGIN_LAT_NODE_TYPE: AVPType = 35;
    pub fn delete_login_lat_node(packet: &mut Packet) {
        packet.delete(Self::LOGIN_LAT_NODE_TYPE);
    }
    pub fn lookup_login_lat_node(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::LOGIN_LAT_NODE_TYPE)
    }
    pub fn lookup_all_login_lat_node(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::LOGIN_LAT_NODE_TYPE)
    }
    pub fn add_login_lat_node(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::LOGIN_LAT_NODE_TYPE, &attr);
    }

    pub const LOGIN_LAT_GROUP_TYPE: AVPType = 36;
    pub fn delete_login_lat_group(packet: &mut Packet) {
        packet.delete(Self::LOGIN_LAT_GROUP_TYPE);
    }
    pub fn lookup_login_lat_group(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::LOGIN_LAT_GROUP_TYPE)
    }
    pub fn lookup_all_login_lat_group(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::LOGIN_LAT_GROUP_TYPE)
    }
    pub fn add_login_lat_group(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::LOGIN_LAT_GROUP_TYPE, &attr);
    }

    pub const FRAMED_APPLE_TALK_ZONE_TYPE: AVPType = 39;
    pub fn delete_framed_apple_talk_zone(packet: &mut Packet) {
        packet.delete(Self::FRAMED_APPLE_TALK_ZONE_TYPE);
    }
    pub fn lookup_framed_apple_talk_zone(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::FRAMED_APPLE_TALK_ZONE_TYPE)
    }
    pub fn lookup_all_framed_apple_talk_zone(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::FRAMED_APPLE_TALK_ZONE_TYPE)
    }
    pub fn add_framed_apple_talk_zone(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::FRAMED_APPLE_TALK_ZONE_TYPE, &attr);
    }

    pub const CHAP_CHALLENGE_TYPE: AVPType = 60;
    pub fn delete_chap_challenge(packet: &mut Packet) {
        packet.delete(Self::CHAP_CHALLENGE_TYPE);
    }
    pub fn lookup_chap_challenge(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::CHAP_CHALLENGE_TYPE)
    }
    pub fn lookup_all_chap_challenge(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::CHAP_CHALLENGE_TYPE)
    }
    pub fn add_chap_challenge(packet: &mut Packet, value: &[u8]) {
        let attr = Attribute::from_bytes(value);
        packet.add(Self::CHAP_CHALLENGE_TYPE, &attr);
    }

    pub const LOGIN_LAT_PORT_TYPE: AVPType = 63;
    pub fn delete_login_lat_port(packet: &mut Packet) {
        packet.delete(Self::LOGIN_LAT_PORT_TYPE);
    }
    pub fn lookup_login_lat_port(packet: &Packet) -> Option<&Attribute> {
        packet.lookup(Self::LOGIN_LAT_PORT_TYPE)
    }
    pub fn lookup_all_login_lat_port(packet: &Packet) -> Vec<&Attribute> {
        packet.lookup_all(Self::LOGIN_LAT_PORT_TYPE)
    }
    pub fn add_login_lat_port(packet: &mut Packet, value: &str) {
        let attr = Attribute::from_string(value);
        packet.add(Self::LOGIN_LAT_PORT_TYPE, &attr);
    }
}