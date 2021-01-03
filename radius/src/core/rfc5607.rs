// Code generated by machine generator; DO NOT EDIT.

//! Utility for rfc5607 packet.
//!
//! This module handles the packet according to the following definition:
//! ```text
//! //! # -*- text -*-
//! # Copyright (C) 2020 The FreeRADIUS Server project and contributors
//! # This work is licensed under CC-BY version 4.0 https://creativecommons.org/licenses/by/4.0
//! # Version $Id$
//! #
//! #	Attributes and values defined in RFC 5607.
//! #	http://www.ietf.org/rfc/rfc5607.txt
//! #
//! #	$Id$
//! #
//!
//! VALUE	Service-Type			Framed-Management	18
//!
//! ATTRIBUTE	Framed-Management			133	integer
//!
//! VALUE	Framed-Management		SNMP			1
//! VALUE	Framed-Management		Web-Based		2
//! VALUE	Framed-Management		Netconf			3
//! VALUE	Framed-Management		FTP			4
//! VALUE	Framed-Management		TFTP			5
//! VALUE	Framed-Management		SFTP			6
//! VALUE	Framed-Management		RCP			7
//! VALUE	Framed-Management		SCP			8
//!
//! ATTRIBUTE	Management-Transport-Protection		134	integer
//!
//! VALUE	Management-Transport-Protection	No-Protection		1
//! VALUE	Management-Transport-Protection	Integrity-Protection	2
//! VALUE	Management-Transport-Protection	Integrity-Confidentiality-Protection	3
//!
//! ATTRIBUTE	Management-Policy-Id			135	string
//!
//! ATTRIBUTE	Management-Privilege-Level		136	integer
//! ```

use crate::core::avp::{AVPError, AVPType, AVP};
use crate::core::packet::Packet;

use crate::core::rfc2865;

pub const FRAMED_MANAGEMENT_TYPE: AVPType = 133;
/// Delete all of `framed_management` values from a packet.
pub fn delete_framed_management(packet: &mut Packet) {
    packet.delete(FRAMED_MANAGEMENT_TYPE);
}
/// Add `framed_management` value-defined integer value to a packet.
pub fn add_framed_management(packet: &mut Packet, value: FramedManagement) {
    packet.add(AVP::from_u32(FRAMED_MANAGEMENT_TYPE, value as u32));
}
/// Lookup a `framed_management` value-defined integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `framed_management`, it returns `None`.
pub fn lookup_framed_management(packet: &Packet) -> Option<Result<FramedManagement, AVPError>> {
    packet
        .lookup(FRAMED_MANAGEMENT_TYPE)
        .map(|v| Ok(v.encode_u32()? as FramedManagement))
}
/// Lookup all of the `framed_management` value-defined integer value from a packet.
pub fn lookup_all_framed_management(packet: &Packet) -> Result<Vec<FramedManagement>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(FRAMED_MANAGEMENT_TYPE) {
        vec.push(avp.encode_u32()? as FramedManagement)
    }
    Ok(vec)
}

pub const MANAGEMENT_TRANSPORT_PROTECTION_TYPE: AVPType = 134;
/// Delete all of `management_transport_protection` values from a packet.
pub fn delete_management_transport_protection(packet: &mut Packet) {
    packet.delete(MANAGEMENT_TRANSPORT_PROTECTION_TYPE);
}
/// Add `management_transport_protection` value-defined integer value to a packet.
pub fn add_management_transport_protection(
    packet: &mut Packet,
    value: ManagementTransportProtection,
) {
    packet.add(AVP::from_u32(
        MANAGEMENT_TRANSPORT_PROTECTION_TYPE,
        value as u32,
    ));
}
/// Lookup a `management_transport_protection` value-defined integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `management_transport_protection`, it returns `None`.
pub fn lookup_management_transport_protection(
    packet: &Packet,
) -> Option<Result<ManagementTransportProtection, AVPError>> {
    packet
        .lookup(MANAGEMENT_TRANSPORT_PROTECTION_TYPE)
        .map(|v| Ok(v.encode_u32()? as ManagementTransportProtection))
}
/// Lookup all of the `management_transport_protection` value-defined integer value from a packet.
pub fn lookup_all_management_transport_protection(
    packet: &Packet,
) -> Result<Vec<ManagementTransportProtection>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_TRANSPORT_PROTECTION_TYPE) {
        vec.push(avp.encode_u32()? as ManagementTransportProtection)
    }
    Ok(vec)
}

pub const MANAGEMENT_POLICY_ID_TYPE: AVPType = 135;
/// Delete all of `management_policy_id` values from a packet.
pub fn delete_management_policy_id(packet: &mut Packet) {
    packet.delete(MANAGEMENT_POLICY_ID_TYPE);
}
/// Add `management_policy_id` string value to a packet.
pub fn add_management_policy_id(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(MANAGEMENT_POLICY_ID_TYPE, value));
}
/// Lookup a `management_policy_id` string value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `management_policy_id`, it returns `None`.
pub fn lookup_management_policy_id(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(MANAGEMENT_POLICY_ID_TYPE)
        .map(|v| v.encode_string())
}
/// Lookup all of the `management_policy_id` string value from a packet.
pub fn lookup_all_management_policy_id(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_POLICY_ID_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const MANAGEMENT_PRIVILEGE_LEVEL_TYPE: AVPType = 136;
/// Delete all of `management_privilege_level` values from a packet.
pub fn delete_management_privilege_level(packet: &mut Packet) {
    packet.delete(MANAGEMENT_PRIVILEGE_LEVEL_TYPE);
}
/// Add `management_privilege_level` integer value to a packet.
pub fn add_management_privilege_level(packet: &mut Packet, value: u32) {
    packet.add(AVP::from_u32(MANAGEMENT_PRIVILEGE_LEVEL_TYPE, value));
}
/// Lookup a `management_privilege_level` integer value from a packet.
///
/// It returns the first looked up value. If there is no associated value with `management_privilege_level`, it returns `None`.
pub fn lookup_management_privilege_level(packet: &Packet) -> Option<Result<u32, AVPError>> {
    packet
        .lookup(MANAGEMENT_PRIVILEGE_LEVEL_TYPE)
        .map(|v| v.encode_u32())
}
/// Lookup all of the `management_privilege_level` integer value from a packet.
pub fn lookup_all_management_privilege_level(packet: &Packet) -> Result<Vec<u32>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(MANAGEMENT_PRIVILEGE_LEVEL_TYPE) {
        vec.push(avp.encode_u32()?)
    }
    Ok(vec)
}

pub type FramedManagement = u32;
pub const FRAMED_MANAGEMENT_SNMP: FramedManagement = 1;
pub const FRAMED_MANAGEMENT_WEB_BASED: FramedManagement = 2;
pub const FRAMED_MANAGEMENT_NETCONF: FramedManagement = 3;
pub const FRAMED_MANAGEMENT_FTP: FramedManagement = 4;
pub const FRAMED_MANAGEMENT_TFTP: FramedManagement = 5;
pub const FRAMED_MANAGEMENT_SFTP: FramedManagement = 6;
pub const FRAMED_MANAGEMENT_RCP: FramedManagement = 7;
pub const FRAMED_MANAGEMENT_SCP: FramedManagement = 8;

pub type ManagementTransportProtection = u32;
pub const MANAGEMENT_TRANSPORT_PROTECTION_NO_PROTECTION: ManagementTransportProtection = 1;
pub const MANAGEMENT_TRANSPORT_PROTECTION_INTEGRITY_PROTECTION: ManagementTransportProtection = 2;
pub const MANAGEMENT_TRANSPORT_PROTECTION_INTEGRITY_CONFIDENTIALITY_PROTECTION:
    ManagementTransportProtection = 3;

pub const SERVICE_TYPE_FRAMED_MANAGEMENT: rfc2865::ServiceType = 18;
