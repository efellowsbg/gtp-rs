use crate::gtpv2::{header::*, messages::{commons::*,ies::*}, errors::*, utils::*};

// According to 3GPP TS 29.274 V15.9.0 (2019-09)

pub const BEARER_RSRC_FAIL:u8 = 69;

// Definition of GTPv2-C Bearer Resource Failure Indication Message

#[derive(Debug, Clone, PartialEq)]
pub struct BearerResourceFailureInd {
    pub header:Gtpv2Header,
    pub cause: Cause,
    pub linked_ebi: Ebi,
    pub pti: Pti,  
    pub indication: Option<Indication>,
    pub overload_info: Vec<OverloadControlInfo>,
    pub recovery: Option<Recovery>,
    pub nbifom: Option<Fcontainer>,
    pub private_ext:Vec<PrivateExtension>,
}

impl Default for BearerResourceFailureInd {
    fn default() -> Self {
        let mut hdr = Gtpv2Header::default();
        hdr.msgtype = BEARER_RSRC_FAIL;
        hdr.teid = Some(0);
        BearerResourceFailureInd {
            header: hdr,
            cause: Cause::default(),
            linked_ebi: Ebi::default(),
            pti: Pti::default(),
            indication: None,
            overload_info: vec!(),
            recovery: None,
            nbifom: None,
            private_ext: vec!(),
        }
    }
}

impl Messages for BearerResourceFailureInd {

    fn marshal (&self, buffer: &mut Vec<u8>) {
        self.header.marshal(buffer);
        let elements = self.to_vec();
        elements.into_iter().for_each(|k| k.marshal(buffer));
        set_msg_length(buffer);
    }

    fn unmarshal (buffer: &[u8]) -> Result<Self, GTPV2Error> {
        let mut message = BearerResourceFailureInd::default();
        match Gtpv2Header::unmarshal(buffer) {
            Ok(i) => message.header=i,
            Err(j) => return Err(j),
        }

        if message.header.msgtype != BEARER_RSRC_FAIL {
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

        elements.push(self.linked_ebi.clone().into());

        elements.push(self.pti.clone().into());
                
        match self.indication.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.overload_info.iter().for_each(|x| elements.push(InformationElement::OverloadControlInfo(x.clone())));

        match self.recovery.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }
        match self.nbifom.clone() {
            Some(i) => elements.push(i.into()),
            None => (),
        }

        self.private_ext.iter().for_each(|x| elements.push(InformationElement::PrivateExtension(x.clone())));    
        elements
    }
    
    fn from_vec(&mut self, elements:Vec<InformationElement>) -> Result<bool, GTPV2Error> {
        let mut mandatory:[bool;3]=[false;3];
        for e in elements.iter() {
            match e {
                InformationElement::Cause(j) => {
                    match (j.ins, mandatory[0]) {
                        (0, false) => (self.cause, mandatory[0]) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Ebi(j) => {
                    match (j.ins, mandatory[1]) {
                        (0, false) => (self.linked_ebi, mandatory[1]) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Pti(j) => {
                    match (j.ins, mandatory[2]) {
                        (0, false) => (self.pti, mandatory[2]) = (j.clone(), true),
                        _ => (),
                    }
                },
                InformationElement::Indication(j) => {
                    match (j.ins, self.indication.is_none()) {
                        (0, true) => self.indication = Some(j.clone()),
                        _ => (),
                    }
                },
                InformationElement::OverloadControlInfo(j) => {  
                    match j.ins {
                        k if k<2 => self.overload_info.push(j.clone()),
                        _ => (),
                    }
                }, 
                InformationElement::Recovery(j) => {
                    match (j.ins, self.recovery.is_none()) {
                        (0, true) => self.recovery = Some(j.clone()),
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
        match (mandatory[0], mandatory[1], mandatory[2]) {
            (false, _, _) => Err(GTPV2Error::MessageMandatoryIEMissing(CAUSE)),
            (true, false, _) => Err(GTPV2Error::MessageMandatoryIEMissing(EBI)),
            (true, true, false) => Err(GTPV2Error::MessageMandatoryIEMissing(PTI)), 
            (true,true,true) => Ok(true),
        }
    }   
}

#[test]
fn test_bearer_failure_ind_unmarshal () {
    let encoded:[u8;72] = [
        0x48,0x45,0x00,0x44,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x02,0x00,0x02,0x00,
        0x4d,0x00,0x49,0x00,0x01,0x00,0x05,0x64,
        0x00,0x01,0x00,0xff,0xb4,0x00,0x12,0x00,
        0xb7,0x00,0x04,0x00,0xff,0xaa,0xee,0x11,
        0xb6,0x00,0x01,0x00,0x60,0x9c,0x00,0x01,
        0x00,0x7f,0xb4,0x00,0x12,0x01,0xb7,0x00,
        0x04,0x00,0xff,0xaa,0xee,0x22,0xb6,0x00,
        0x01,0x00,0x60,0x9c,0x00,0x01,0x00,0x7e,
    ];
    let mut decoded = BearerResourceFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:BEARER_RSRC_FAIL,
            piggyback:false,
            message_prio:None, 
            length:68, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:77,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.linked_ebi = Ebi { t: 73, length: 1, ins: 0, value: 5 };
    decoded.pti = Pti { t: PTI, length:1, ins:0, pti: 0xff};
    decoded.overload_info = vec!(
        OverloadControlInfo {
                t: OVERLOAD_CNTRL, 
                length: 18, 
                ins: 0, 
                sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
                metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
                validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
                list: None,
        },
        OverloadControlInfo {
            t: OVERLOAD_CNTRL, 
            length: 18, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee22 },
            metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:30 },
            list: None,
    },   
    );
    let message = BearerResourceFailureInd::unmarshal(&encoded).unwrap();
    assert_eq!(message,decoded);
}

#[test]
fn test_bearer_failure_ind_marshal () {
    let encoded:[u8;72] = [
        0x48,0x45,0x00,0x44,0x00,0x00,0x00,0x00,
        0x00,0x00,0x68,0x00,0x02,0x00,0x02,0x00,
        0x4d,0x00,0x49,0x00,0x01,0x00,0x05,0x64,
        0x00,0x01,0x00,0xff,0xb4,0x00,0x12,0x00,
        0xb7,0x00,0x04,0x00,0xff,0xaa,0xee,0x11,
        0xb6,0x00,0x01,0x00,0x60,0x9c,0x00,0x01,
        0x00,0x7f,0xb4,0x00,0x12,0x01,0xb7,0x00,
        0x04,0x00,0xff,0xaa,0xee,0x22,0xb6,0x00,
        0x01,0x00,0x60,0x9c,0x00,0x01,0x00,0x7e,
    ];
    let mut decoded = BearerResourceFailureInd::default();
    decoded.header = Gtpv2Header {
            msgtype:BEARER_RSRC_FAIL,
            piggyback:false,
            message_prio:None, 
            length:68, 
            teid:Some(0), 
            sqn:0x68 };
    decoded.cause = Cause{
        t:CAUSE,
        length:2,
        ins:0,
        value:77,
        pce:false,
        bce:false,
        cs:false,
        offend_ie_type:None,
    };
    decoded.linked_ebi = Ebi { t: 73, length: 1, ins: 0, value: 5 };
    decoded.pti = Pti { t: PTI, length:1, ins:0, pti: 0xff};
    decoded.overload_info = vec!(
        OverloadControlInfo {
                t: OVERLOAD_CNTRL, 
                length: 18, 
                ins: 0, 
                sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee11 },
                metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
                validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:31 },
                list: None,
        },
        OverloadControlInfo {
            t: OVERLOAD_CNTRL, 
            length: 18, 
            ins: 1, 
            sqn: Sqn { t:SQN, length: SQN_LENGTH as u16, ins:0, sqn: 0xffaaee22 },
            metric: Metric { t:METRIC, length: METRIC_LENGTH as u16, ins:0, metric: 0x60 },
            validity: EpcTimer { t:EPC_TIMER, length: EPC_TIMER_LENGTH as u16, ins:0, timer_unit:3, timer_value:30 },
            list: None,
    },   
    );
    let mut buffer:Vec<u8>=vec!();
    decoded.marshal(&mut buffer);
    assert_eq!(buffer,encoded);
}