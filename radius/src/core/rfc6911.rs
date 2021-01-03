// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc6911 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #	Attributes and values defined in RFC 6911
//! #	http://www.ietf.org/rfc/rfc6911.txt
//! #
//!
//! ATTRIBUTE	Framed-IPV6-Address			168	ipv6addr
//! ATTRIBUTE	DNS-Server-IPV6-Address			169	ipv6addr
//! ATTRIBUTE	Route-IPV6-Information			170	ipv6prefix
//! ATTRIBUTE	Delegated-IPV6-Prefix-Pool		171	string
//! ATTRIBUTE	Stateful-IPV6-Address-Pool		172	string
//! ```

use std::net::Ipv6Addr;

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

pub const FRAMED_IPV6_ADDRESS_TYPE: AVPType = 168;
/// Delete all of `framed_ipv6_address` values from a packet.
pub fn delete_framed_ipv6_address(packet: &mut Packet) {
    packet.delete(FRAMED_IPV6_ADDRESS_TYPE);
}
/// Add `framed_ipv6_address` ipv6addr value to a packet.
pub fn add_framed_ipv6_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(FRAMED_IPV6_ADDRESS_TYPE, value));
}
/// Lookup a `framed_ipv6_address` ipv6addr value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_ipv6_address`, it returns `None`.
pub fn lookup_framed_ipv6_address(packet: &Packet) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(FRAMED_IPV6_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
/// Lookup all of the `framed_ipv6_address` ipv6addr value from a packet.
pub fn lookup_all_framed_ipv6_address(packet: &Packet) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_IPV6_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const DNS_SERVER_IPV6_ADDRESS_TYPE: AVPType = 169;
/// Delete all of `dns_server_ipv6_address` values from a packet.
pub fn delete_dns_server_ipv6_address(packet: &mut Packet) {
    packet.delete(DNS_SERVER_IPV6_ADDRESS_TYPE);
}
/// Add `dns_server_ipv6_address` ipv6addr value to a packet.
pub fn add_dns_server_ipv6_address(packet: &mut Packet, value: &Ipv6Addr) {
    packet.add(AVP::from_ipv6(DNS_SERVER_IPV6_ADDRESS_TYPE, value));
}
/// Lookup a `dns_server_ipv6_address` ipv6addr value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `dns_server_ipv6_address`, it returns `None`.
pub fn lookup_dns_server_ipv6_address(packet: &Packet) -> Option<Result<Ipv6Addr, AVPError>> {
    packet
        .lookup(DNS_SERVER_IPV6_ADDRESS_TYPE)
        .map(|v| v.encode_ipv6())
}
/// Lookup all of the `dns_server_ipv6_address` ipv6addr value from a packet.
pub fn lookup_all_dns_server_ipv6_address(packet: &Packet) -> Result<Vec<Ipv6Addr>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DNS_SERVER_IPV6_ADDRESS_TYPE) {
        vec.push(avp.encode_ipv6()?)
    }
    Ok(vec)
}

pub const ROUTE_IPV6_INFORMATION_TYPE: AVPType = 170;
/// Delete all of `route_ipv6_information` values from a packet.
pub fn delete_route_ipv6_information(packet: &mut Packet) {
    packet.delete(ROUTE_IPV6_INFORMATION_TYPE);
}
/// Add `route_ipv6_information` ipv6 prefix value to a packet.
pub fn add_route_ipv6_information(packet: &mut Packet, value: &[u8]) -> Result<(), AVPError> {
    packet.add(AVP::from_ipv6_prefix(ROUTE_IPV6_INFORMATION_TYPE, value)?);
    Ok(())
}
/// Lookup a `route_ipv6_information` ipv6 prefix value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `route_ipv6_information`, it returns `None`.
pub fn lookup_route_ipv6_information(packet: &Packet) -> Option<Result<Vec<u8>, AVPError>> {
    packet
        .lookup(ROUTE_IPV6_INFORMATION_TYPE)
        .map(|v| v.encode_ipv6_prefix())
}
/// Lookup all of the `route_ipv6_information` ipv6 prefix value from a packet.
pub fn lookup_all_route_ipv6_information(packet: &Packet) -> Result<Vec<Vec<u8>>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(ROUTE_IPV6_INFORMATION_TYPE) {
        vec.push(avp.encode_ipv6_prefix()?)
    }
    Ok(vec)
}

pub const DELEGATED_IPV6_PREFIX_POOL_TYPE: AVPType = 171;
/// Delete all of `delegated_ipv6_prefix_pool` values from a packet.
pub fn delete_delegated_ipv6_prefix_pool(packet: &mut Packet) {
    packet.delete(DELEGATED_IPV6_PREFIX_POOL_TYPE);
}
/// Add `delegated_ipv6_prefix_pool` string value to a packet.
pub fn add_delegated_ipv6_prefix_pool(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(DELEGATED_IPV6_PREFIX_POOL_TYPE, value));
}
/// Lookup a `delegated_ipv6_prefix_pool` string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `delegated_ipv6_prefix_pool`, it returns `None`.
pub fn lookup_delegated_ipv6_prefix_pool(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(DELEGATED_IPV6_PREFIX_POOL_TYPE)
        .map(|v| v.encode_string())
}
/// Lookup all of the `delegated_ipv6_prefix_pool` string value from a packet.
pub fn lookup_all_delegated_ipv6_prefix_pool(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(DELEGATED_IPV6_PREFIX_POOL_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const STATEFUL_IPV6_ADDRESS_POOL_TYPE: AVPType = 172;
/// Delete all of `stateful_ipv6_address_pool` values from a packet.
pub fn delete_stateful_ipv6_address_pool(packet: &mut Packet) {
    packet.delete(STATEFUL_IPV6_ADDRESS_POOL_TYPE);
}
/// Add `stateful_ipv6_address_pool` string value to a packet.
pub fn add_stateful_ipv6_address_pool(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(STATEFUL_IPV6_ADDRESS_POOL_TYPE, value));
}
/// Lookup a `stateful_ipv6_address_pool` string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `stateful_ipv6_address_pool`, it returns `None`.
pub fn lookup_stateful_ipv6_address_pool(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(STATEFUL_IPV6_ADDRESS_POOL_TYPE)
        .map(|v| v.encode_string())
}
/// Lookup all of the `stateful_ipv6_address_pool` string value from a packet.
pub fn lookup_all_stateful_ipv6_address_pool(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(STATEFUL_IPV6_ADDRESS_POOL_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}
