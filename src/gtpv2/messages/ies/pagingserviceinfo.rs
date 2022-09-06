// Paging and Service Information IE - according to 3GPP TS 29.274 V15.9.0 (2019-09)

use crate::gtpv2::{utils::*, errors::GTPV2Error, messages::ies::{commons::*,ie::*}};

// Paging and Service Information IE TL

pub const PAGING_SRVC_INFO:u8 = 174;
pub const PAGING_SRVC_INFO_LENGTH:usize = 2;

// Paging and Service Information IE implementation 

#[derive(Debug, Clone, PartialEq)]
pub struct PagingServiceInfo {
    pub t:u8,
    pub length:u16,
    pub ins:u8,
    pub ebi:u8,
    pub paging_policy:Option<u8>,        
}

impl Default for PagingServiceInfo {
    fn default() -> Self {
        PagingServiceInfo { t: PAGING_SRVC_INFO, length:PAGING_SRVC_INFO_LENGTH as u16, ins:0, ebi:0, paging_policy:None }        
    }
}

impl From<PagingServiceInfo> for InformationElement {
    fn from(i: PagingServiceInfo) -> Self {
        InformationElement::PagingServiceInfo(i)
    }
}

impl IEs for PagingServiceInfo {
    fn marshal (&self, buffer: &mut Vec<u8>) {
        let mut buffer_ie:Vec<u8> = vec!();  
        buffer_ie.push(self.t);
        buffer_ie.extend_from_slice(&self.length.to_be_bytes());
        buffer_ie.push(self.ins);
        buffer_ie.push(self.ebi);
        match self.paging_policy {
            Some(i) => {
                buffer_ie.push(0x01);
                buffer_ie.push(i);
            },
            None => buffer_ie.push(0x00),
        }
        set_tliv_ie_length(&mut buffer_ie);
        buffer.append(&mut buffer_ie);
    }

    fn unmarshal (buffer:&[u8]) -> Result<Self, GTPV2Error> {
        if buffer.len()>= PAGING_SRVC_INFO_LENGTH + MIN_IE_SIZE {
            let mut data = PagingServiceInfo::default();
            data.length = u16::from_be_bytes([buffer[1], buffer[2]]);
            data.ins = buffer[3];
            data.ebi = buffer[4] & 0x0f;
            match buffer[5] {
                0 => (),
                1 => {
                    match buffer[6].try_into() {
                        Ok(i) => data.paging_policy = Some(i),
                        Err(_) => return Err(GTPV2Error::IEInvalidLength(PAGING_SRVC_INFO)),
                    }
                },
                _ => return Err(GTPV2Error::IEIncorrect(PAGING_SRVC_INFO)),
            }
            Ok(data)
        } else {
            Err(GTPV2Error::IEInvalidLength(PAGING_SRVC_INFO))
        }
    }
    
    fn len (&self) -> usize {
       (self.length as usize) + MIN_IE_SIZE 
    }
}

#[test]
fn paging_service_info_ie_unmarshal_test () {
    let encoded:[u8;7]=[0xba, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03];
    let decoded = PagingServiceInfo { t:PAGING_SRVC_INFO, length: 3, ins:0, ebi:2, paging_policy:Some(0x03) };
    let i = PagingServiceInfo::unmarshal(&encoded);
    assert_eq!(i.unwrap(), decoded);
}

#[test]
fn paging_service_info_ie_marshal_test () {
    let encoded:[u8;7]=[0xba, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03];
    let decoded = PagingServiceInfo { t:PAGING_SRVC_INFO, length: 3, ins:0, ebi:2, paging_policy:Some(0x03) };
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer, encoded);
}