use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const DELETE_BEARER_RESP:u8 = 100;

// Definition of GTPv2-C Delete Bearer Response Message

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteBearerResponse {
    pub header:Gtpv2Header,
    pub cause:Cause,
    pub linked_ebi:Option<Ebi>,
    pub bearer_ctxs:Vec<BearerContext>,
    pub recovery:Option<Recovery>,
    pub mme_fqcsid: Option<Fqcsid>,
    pub sgw_fqcsid: Option<Fqcsid>,
    pub epdg_fqcsid: Option<Fqcsid>,
    pub twan_fqcsid: Option<Fqcsid>,
    pub pco:Option<Pco>,
    pub uetimezone: Option<UeTimeZone>,
    pub uli: Option<Uli>,
    pub uli_timestamp:Option<UliTimestamp>,
    pub twan_id: Option<TwanId>,
    pub twan_id_timestamp:Option<TwanIdTimeStamp>,
    pub overload_info:Vec<OverloadControlInfo>,
    pub mme_id: Option<IpAddress>,
    pub wlan_loc: Option<TwanId>,
    pub wlan_loc_timestamp: Option<TwanIdTimeStamp>,
    pub ue_localip: Option<IpAddress>,
    pub ue_udpport: Option<PortNumber>,
    pub nbifom:Option<Fcontainer>,
    pub ue_tcpport: Option<PortNumber>,
    pub secondary_rat_usage_report:Vec<SecondaryRatUsageDataReport>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for DeleteBearerResponse {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = DELETE_BEARER_RESP;
        hdr.teid = Some(0);
        DeleteBearerResponse {
            header:hdr,
            cause:Cause::default(),
            linked_ebi:None,
            bearer_ctxs:vec!(),
            recovery:None,
            mme_fqcsid:None,
            sgw_fqcsid:None,
            epdg_fqcsid:None,
            twan_fqcsid:None,
            pco:None,
            uetimezone:None,
            uli:None,
            uli_timestamp:None,
            twan_id:None,
            twan_id_timestamp:None,
            overload_info:vec!(),
            mme_id:None,
            wlan_loc:None,
            wlan_loc_timestamp:None,
            ue_localip:None,
            ue_udpport:None,
            nbifom:None,
            ue_tcpport:None,
            secondary_rat_usage_report:vec!(),
            private_ext:vec!(),
        }
    }
}

impl Messages for DeleteBearerResponse {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = DeleteBearerResponse::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != DELETE_BEARER_RESP {
            return Err(GTPV2Error::MessageIncorrectMessageType);
        }

        if (message.header.length as usize)+4<=buffer.len() {
            let ies:Vec<InformationElement>;
            match InformationElement::decoder(&buffer[12..]) {
                Ok(i) => ies = i,
                Err(j) => return Err(j),
            }
            match message.from_vec(ies) {
                Ok(_) => Ok(message),
                Err(j) => Err(j),
            }
        } else {
            Err(GTPV2Error::MessageInvalidMessageFormat)
        }
    }

    fn to_vec(&self) -> Vec<InformationElement> {
        let mut elements:Vec<InformationElement> = vec!();
        
        elements.push(self.cause.clone().into());

        match self.linked_ebi.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.bearer_ctxs.iter().for_each(|x| elements.push(InformationElement::BearerContext(x.clone())));

        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        match self.mme_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.sgw_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.epdg_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.twan_fqcsid.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        }
        match self.pco.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }    
        match self.uetimezone.clone() {
            Some(i) => elements.push(InformationElement::UeTimeZone(i)),
            None => (),
        }
        match self.uli.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.uli_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.twan_id.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.twan_id_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.mme_id.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }        
        match self.wlan_loc.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.wlan_loc_timestamp.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ue_localip.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ue_udpport.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.nbifom.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.ue_tcpport.clone() {
            Some(i) => elements.push(i.into()),
            None => ()
        } 

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::SecondaryRatUsageDataReport(x.clone())));  

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));  

        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory=false;
        for e in elements.into_iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory) {
                        (0, false) => (self.cause, mandatory[0]) = (j, true),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, self.linked_ebi.is_none()) {
                        (0, true) => self.linked_ebi = Some(j),
                        _ => (),
                    }
                },
                InformationElement::BearerContext(j) => {
                    match j.ins {
                        0 => {
                            self.bearer_ctxs.push(j);
                        },
                        _ => (),
                    }
                }
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Fqcsid(j) => {  // 4 instances
                    match (j.ins, self.mme_fqcsid.is_none(), self.sgw_fqcsid.is_none(), self.epdg_fqcsid.is_none(), self.twan_fqcsid.is_none()) {
                        (0, true, _, _, _) => self.mme_fqcsid = Some(j),
                        (1, _, true, _, _) => self.sgw_fqcsid = Some(j),
                        (2, _, _, true,_) => self.epdg_fqcsid = Some(j),
                        (3, _, _, _, true) => self.twan_fqcsid = Some(j),
                        _ => (),
                    }
                }, 
                InformationElement::Pco(j) => {
                    match (j.ins, self.pco.is_none()) {
                        (0, true) => self.pco = Some(j),
                        _ => (),
                    }
                },
                InformationElement::UeTimeZone(j) => {
                    match (j.ins, self.uetimezone.is_none()) {
                        (0, true) => self.uetimezone = Some(j),
                        _ => (),
                    }
                },
                InformationElement::Uli(j) => {
                    match (j.ins, self.uli.is_none()) {
                        (0, true) => self.uli = Some(j),
                        _ => (),
                    }
                },
                InformationElement::UliTimestamp(j) => {
                    match (j.ins, self.uli_timestamp.is_none()) {
                        (0, true) => self.uli_timestamp = Some(j),
                        _ => (),
                    }
                },
                InformationElement::TwanId(j) => { // 2 instances
                    match (j.ins, self.twan_id.is_none(), self.wlan_loc.is_none()) {
                        (0, true, _) => self.twan_id = Some(j),
                        (1, _, true) => self.wlan_loc = Some(j),
                        _ => (),
                    }
                },
                InformationElement::TwanIdTimeStamp(j) => { // 2 instances
                    match (j.ins, self.twan_id_timestamp.is_none(), self.wlan_loc_timestamp.is_none()) {
                        (0, true, _) => self.twan_id_timestamp = Some(j),
                        (1, _, true) => self.wlan_loc_timestamp = Some(j),
                        _ => (),
                    }
                },
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<3 => self.overload_info.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::PresenceReportingAreaInformation(j) => {
                    match (j.ins, self.prai.is_none()) {
                        (0, true) => self.prai = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::IpAddress(j) => {
                    match (j.ins, self.mme_id.is_none()) {
                        (0, true) => self.mme_id = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::TwanIdTimeStamp(j) => {
                    match (j.ins, self.wlan_loc_timestamp.is_none()) {
                        (1, true) => self.wlan_loc_timestamp = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PortNumber(j) => {
                    match (j.ins, self.ue_udpport.is_none(), self.ue_tcpport.is_none()) {
                        (0, true, _) => self.ue_udpport = Some(j.clone()),
                        (1, _, true) => self.ue_tcpport = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::Fcontainer(j) => {  
                    match (j.ins, self.nbifom.is_none()) {
                        (0, true) => self.nbifom = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::PrivateExtension(j) => self.private_ext.push(j.clone()),
                _ => (),
            }
        }
        match (mandatory[0], mandatory[1]) {
            (false,false) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (false,true) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true,false) => Err(GTPV2Error::MessageMandatoryIEMissing(BEARER_CTX)), 
            (true,true) => Ok(true),
        }
    }
}

#[test]
fn test_create_bearer_resp_unmarshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;109] = [
        0x48,0x60,0x00,0x69,0x09,0x09,0xa4,0x56,
        0x00,0x00,0x2f,0x00,0x02,0x00,0x02,0x00,
        0x10,0x00,0x5d,0x00,0x3a,0x00,0x02,0x00,
        0x02,0x00,0x10,0x00,0x49,0x00,0x01,0x00,
        0x05,0x57,0x00,0x09,0x02,0x85,0x3b,0x95,
        0x98,0x5a,0x3e,0x99,0x89,0x55,0x50,0x00,
        0x16,0x00,0x2c,0x09,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x5e,0x00,0x04,0x00,0x01,0x62,0x9c,0xc4,
        0x03,0x00,0x01,0x00,0x11,0x4e,0x00,0x14,
        0x00,0x80,0x80,0x21,0x10,0x02,0x00,0x00,
        0x10,0x81,0x06,0x08,0x08,0x08,0x08,0x83,
        0x06,0x0a,0x40,0xd0,0x61,
    ];
    let mut decoded = CreateBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:105, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:17,
        }    
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: 93, 
            length: 58, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:16,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:Some(
                ChargingId {
                    t: CHARGINGID,
                    length:4,
                    ins: 0,
                    charging_id: 23239876,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    
    let message = CreateBearerResponse::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_create_bearer_resp_marshal () {
    use std::net::Ipv4Addr;
    let encoded:[u8;109] = [
        0x48,0x60,0x00,0x69,0x09,0x09,0xa4,0x56,
        0x00,0x00,0x2f,0x00,0x02,0x00,0x02,0x00,
        0x10,0x00,0x5d,0x00,0x3a,0x00,0x02,0x00,
        0x02,0x00,0x10,0x00,0x49,0x00,0x01,0x00,
        0x05,0x57,0x00,0x09,0x02,0x85,0x3b,0x95,
        0x98,0x5a,0x3e,0x99,0x89,0x55,0x50,0x00,
        0x16,0x00,0x2c,0x09,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
        0x5e,0x00,0x04,0x00,0x01,0x62,0x9c,0xc4,
        0x03,0x00,0x01,0x00,0x11,0x4e,0x00,0x14,
        0x00,0x80,0x80,0x21,0x10,0x02,0x00,0x00,
        0x10,0x81,0x06,0x08,0x08,0x08,0x08,0x83,
        0x06,0x0a,0x40,0xd0,0x61,
    ];
    let mut decoded = CreateBearerResponse::default();
    decoded.header = Gtpv2Header {
            msgtype:CREATE_BEARER_RESP,
            piggyback:false,
            message_prio:None, 
            length:105, 
            teid:Some(0x0909a456), 
            sqn:0x2f };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:16,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.recovery = Some (
        Recovery {
            t:RECOVERY,
            length:1,
            ins:0,
            recovery:17,
        }    
    );
    decoded.pco = Some (
        Pco {
            t:PCO,
            length:20,
            ins:0,
            pco: vec!(0x80, 0x80, 0x21, 0x10, 0x02, 0x00, 0x00, 0x10, 0x81, 0x06, 0x08, 0x08, 0x08, 0x08, 0x83, 0x06, 
                    0x0a, 0x40, 0xd0, 0x61),
        });
    
    decoded.bearer_ctxs = vec!(
        BearerContext { 
            t: 93, 
            length: 58, 
            ins: 0,
            cause: Some(
                Cause {
                    t:CAUSE,
                    length:2,
                    ins:0,
                    value:16,
                    pce:false,
                    bce:false,
                    cs:false,
                    offend_ie_type:None,
                }
            ),
            tft:None,
            charging_id:Some(
                ChargingId {
                    t: CHARGINGID,
                    length:4,
                    ins: 0,
                    charging_id: 23239876,
                }
            ),
            bearer_flags:None,
            pco:None,
            apco:None,
            epco:None,
            max_packet_loss:None, 
            ebi: Ebi { t: EBI, length: 1, ins: 0, value: 5 },
            fteids: Some(vec!( Fteid { t: 87, length: 9, ins: 2, interface: 5, teid: 0x3b95985a, ipv4: Some(Ipv4Addr::new(62,153,137,85)), ipv6: None })),
            bearer_qos:Some(BearerQos { t: 80, length: 22, ins: 0, pre_emption_vulnerability: 0, priority_level: 11, pre_emption_capability: 0, qci: 9, maxbr_ul: 0, maxbr_dl: 0, gbr_ul: 0, gbr_dl: 0 }),
            });
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}
