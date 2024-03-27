use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};
use serde::{Deserialize, Serialize};
use crate::utils::serialize;
/**
 *@brief HSMSMessage
 *MessageLength
 *HSMSHeader
 *MessageText
 */

/**
 * @brief MessageLength
 * 消息长度
 * 占4bytes
 * 长度为消息头长度加上消息文本，最小为10，仅有消息头
 */

/**
* @brief HSMSHeader
 * 共占10bytes
 * SessionID     0-1
 * HeaderByte2   2
 * HeaderByte3   3
 * PType         4
 * SType         5
 * System Bytes  6-9
 * 详细说明见下面定义
 */

/**
 * @brief SessionID
 * 占10bytes
 * 0000 0000 0000 0000
 * ^
 * 最左边第16位最高位代表消息发送方向，1为To host，0为To equip
 * 剩余15位唯一标识一台设备 0-32767 000 0000 0000 0000 - 111 1111 1111 1111
 */

/**
 * @brief HeaderByte2
 * 如果SType为0，此时为SECSⅡ消息，HeaderByte2代表W-Bit和Stream
 * 0000 0000
 * ^
 * W-Bit为最高位，指明发送SnFn消息时是否需要回复，0为不需要，1为需要。
 * Stream指明消息所在大类，参考SnFn
 */

/**
 * @brief HeaderByte3
 * 指明Function号，参考SnFn
 */

/**
 * @brief PType
 * 表示类型，0为HSMS消息
 * 其余为子标准定义或预留
 */

/**
 * @brief SType
 * Session Type
 * 0       DataMessage    指发送SnFn命令, SECSⅡ Encode
 * 1       Select.req     选择请求
 * 2       Select.rsp     选择回复
 * 3       Deselect.req   取消选择请求
 * 4       Deselect.rsp   取消选择回复
 * 5       Linktest.req   连接测试请求，类似心跳帧
 * 6       Linktest.rsp   连接测试回复
 * 7       Reject.req     拒绝请求
 * 8       未使用
 * 9       Separate.req   断开连接请求
 * 10      未使用
 * 11-127  子标准定义
 * 128-255 预留
 */

/**
 * @brief SystemBytes
 * 句柄
 */

/**
 * @brief MessageText
 * 消息文本 0-n bytes
 */
#[derive(Debug,Eq, PartialEq,IntoPrimitive,TryFromPrimitive)]
#[repr(u8)]
enum SessionType{
    SECS2 = 0,
    SelectReq =1,
    SelectRsp =2,
    DeselectReq = 3,
    DeselectRsp = 4,
    LinktestReq = 5,
    LinktestRsp = 6,
    RejectReq = 7,
    SeparateReq = 9
}
#[derive(Debug,Clone,Eq, PartialEq,Serialize,Deserialize)]
struct SessionID{
    session_id:u16
}

impl SessionID {
    fn from_direction_equip_id(direction:u16,equip_id:u16)->SessionID {
        SessionID {
            session_id:(direction&0x8000) |(equip_id&0x7FFF)
        }
    }
}
#[derive(Debug,Clone,Eq, PartialEq,Serialize,Deserialize)]
struct HeaderByte2{
    header_byte2:u8,
}

impl HeaderByte2 {
    fn from_w_bit_stream(w_bit:u8,stream:u8)->HeaderByte2{
        HeaderByte2{
            header_byte2: (w_bit&0x80)|(stream&0x7F)
        }
    }
}
#[derive(Debug,Clone,Eq, PartialEq,Serialize,Deserialize)]
struct HSMSHeader {
    session_id:SessionID,
    header_byte2:HeaderByte2,
    header_byte3:u8,
    p_type:u8,
    s_type:u8,
    system_bytes:u32,
}

impl HSMSHeader {
    fn new(session_type:SessionType,
           session_id:u16,
           direction:u16,
           equip_id:u16,
           header_byte2: u8,
           w_bit:u8,
           stream:u8,
           header_byte3: u8,
           system_bytes:u32
    )->HSMSHeader{
        match session_type {
            SessionType::SECS2=>{
                HSMSHeader{
                    session_id: SessionID::from_direction_equip_id(direction,equip_id),
                    header_byte2: HeaderByte2::from_w_bit_stream(w_bit,stream),
                    header_byte3: header_byte3,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::SelectReq=>{
                HSMSHeader{
                    session_id: SessionID {session_id:0xFFFF},
                    header_byte2: HeaderByte2 {header_byte2:0},
                    header_byte3: 0,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::SelectRsp => {
                HSMSHeader{
                    session_id: SessionID {session_id:session_id},
                    header_byte2: HeaderByte2{header_byte2:0},
                    header_byte3: header_byte3,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::DeselectReq => {
                HSMSHeader{
                    session_id: SessionID {session_id:0xFFFF},
                    header_byte2: HeaderByte2 {header_byte2:0},
                    header_byte3: 0,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::DeselectRsp => {
                HSMSHeader{
                    session_id: SessionID {session_id:session_id},
                    header_byte2: HeaderByte2{header_byte2:0},
                    header_byte3: header_byte3,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::LinktestReq => {
                HSMSHeader{
                    session_id: SessionID {session_id:0xFFFF},
                    header_byte2: HeaderByte2 {header_byte2:0},
                    header_byte3: 0,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::LinktestRsp => {
                HSMSHeader{
                    session_id: SessionID {session_id:0xFFFF},
                    header_byte2: HeaderByte2{header_byte2:0},
                    header_byte3: 0,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::RejectReq => {
                HSMSHeader{
                    session_id: SessionID {session_id:session_id},
                    header_byte2: HeaderByte2 {header_byte2:header_byte2},
                    header_byte3: header_byte3,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }
            SessionType::SeparateReq => {
                HSMSHeader{
                    session_id: SessionID {session_id:0xFFFF},
                    header_byte2: HeaderByte2{header_byte2:0},
                    header_byte3: 0,
                    p_type: 0,
                    s_type: session_type.into(),
                    system_bytes: system_bytes,
                }
            }

        }
    }
    fn get_session_type(&self) -> Result<SessionType, TryFromPrimitiveError<SessionType>> {
        Ok(SessionType::try_from(self.s_type)?)
    }
    fn len(&self)->u32{
        10
    }
}

#[derive(Debug,Clone,Eq, PartialEq)]
struct HSMSMessage{
    message_length:u32,
    hsms_header:HSMSHeader,
    message_text:Option<Vec<u8>>
}

impl HSMSMessage {
    fn new(hsms_header:HSMSHeader,message_text:&Vec<u8>)->HSMSMessage{
        HSMSMessage{
            message_length:hsms_header.len()+message_text.len() as u32,
            hsms_header: hsms_header,
            message_text:Some(message_text.to_vec())
        }
    }

    fn from_bytes(vec:Vec<u8>)->Result<HSMSMessage,&'static str>{
        if vec.len()<14{
            return Err("Size less than 14");
        }
        let message_length:u32 = bincode::deserialize(&vec[0..4]).unwrap();
        let hsms_header:HSMSHeader = serialize::deserialize_from_bytes(&vec[4..14])
            .expect("Deserialize hsms header fail");
        let mut message_text = None;
        if vec.len()>14{
            message_text = Some(vec[14..].to_vec());
        }
        let hsms_message = HSMSMessage{
            message_length: message_length,
            hsms_header: hsms_header,
            message_text: message_text,
        };
        Ok(hsms_message)
    }

    fn to_bytes(&self)->Vec<u8>{
        let mut vec:Vec<u8>  = bincode::serialize(&self.message_length).unwrap();
        vec.append(&mut serialize::serialize(&self.hsms_header));
        if self.message_text.is_some(){
            vec.append(&mut self.message_text.clone().unwrap());
        }
        vec
    }

}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_session_id_from_direction_stream(){
        let session_id = SessionID{session_id:0x8001};
        let session_id_from_direction_stream = SessionID::from_direction_equip_id(0x8000,0x0001);
        assert_eq!(session_id,session_id_from_direction_stream);
    }
    #[test]
    fn test_header_byte2_from_w_bit_stream(){
        let header_byte2 = HeaderByte2{header_byte2:0x81};
        let header_byte2_from_w_bit_stream = HeaderByte2::from_w_bit_stream(0x80,0x01);
        assert_eq!(header_byte2,header_byte2_from_w_bit_stream);
    }
    #[test]
    fn test_hsms_header_new_secs2(){
        let hsms_header_secs2 = HSMSHeader{
            session_id: SessionID {session_id:0x8001},
            header_byte2: HeaderByte2 {header_byte2:0x81},
            header_byte3: 3,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        let hsms_header_secs2_new=HSMSHeader::new(SessionType::SECS2,
                                                  0,
                                                  0x8000,
                                                  0x0001,
                                                  0,
                                                  0x80,
                                                  0x01,
                                                  3,
                                                  0x11111111);
        assert_eq!(hsms_header_secs2,hsms_header_secs2_new);
    }

    #[test]
    fn test_hsms_header_new_select_req(){
        let hsms_header_select_req = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 1,
            system_bytes: 0x11111111,
        };
        let hsms_header_select_req_new=HSMSHeader::new(SessionType::SelectReq,
                                                       0xFFFF,
                                                       0x8000,
                                                       0x0001,
                                                       0,
                                                       0x80,
                                                       0x01,
                                                       0,
                                                       0x11111111);
        assert_eq!(hsms_header_select_req,hsms_header_select_req_new);
    }
    //todo!

    #[test]
    fn test_get_session_type(){
        let hsms_header_secs2 = HSMSHeader{
            session_id: SessionID {session_id:0x8001},
            header_byte2: HeaderByte2 {header_byte2:0x81},
            header_byte3: 3,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        assert_eq!(hsms_header_secs2.get_session_type().unwrap(),SessionType::SECS2);

        let hsms_header_secs2 = HSMSHeader{
            session_id: SessionID {session_id:0x8001},
            header_byte2: HeaderByte2 {header_byte2:0x81},
            header_byte3: 3,
            p_type: 0,
            s_type: 8,
            system_bytes: 0x11111111,
        };
        let error = Err(num_enum::TryFromPrimitiveError::new(8));
        assert_eq!(hsms_header_secs2.get_session_type(),error);
    }

    #[test]
    fn test_serialize_session_id(){
        let session_id =SessionID{session_id:0x8FFF};
        let session_id_bytes =  serialize::serialize(&session_id);
        assert_eq!(session_id_bytes,vec![0xFF,0x8F]);
    }
    #[test]
    fn test_deserialize_session_id_from_bytes(){
        let session_id =SessionID{session_id:0x8FFF};
        let mut session_vec:Vec<u8> = vec![0xFF,0x8F];
        let session_id_bytes:SessionID =  serialize::deserialize_from_bytes(&mut session_vec).unwrap();
        assert_eq!(session_id_bytes,session_id);
    }

    //I don't know how to test this function in this file
    // #[test]
    // async fn test_deserialize_session_id_from_reader(){
    //     let session_id =SessionID{session_id:0x8FFF};
    //
    //     let f = File::open("test.txt")?;
    //     let mut reader =tokio::io::BufReader::new(f);
    //     let session_id_bytes:SessionID =  serialize::deserialize(&mut reader).await?;
    //     assert_eq!(session_id_bytes,session_id);
    // }

    #[test]
    fn test_serialize_header_byte2(){
        let header_byte2 = HeaderByte2{header_byte2:0x81};
        let header_byte2_bytes =  serialize::serialize(&header_byte2);
        assert_eq!(header_byte2_bytes,vec![0x81]);
    }
    #[test]
    fn test_deserialize_header_byte2_from_bytes(){
        let header_byte2 = HeaderByte2{header_byte2:0x81};
        let mut header_byte2_vec:Vec<u8> = vec![0x81];
        let header_byte2_bytes:HeaderByte2 =  serialize::deserialize_from_bytes(& mut header_byte2_vec).unwrap();
        assert_eq!(header_byte2_bytes,header_byte2);
    }
    #[test]
    fn test_serialize_hsms_header(){
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 1,
            system_bytes: 0x11111111,
        };
        let hsms_header_bytes = serialize::serialize(&hsms_header);
        assert_eq!(hsms_header_bytes,vec![0xFF,0xFF,0x00,0x00,0x00,0x01,0x11,0x11,0x11,0x011]);
    }
    #[test]
    fn test_deserialize_hsms_header(){
        let hsms_header_from_bytes:HSMSHeader = serialize::deserialize_from_bytes(&vec![0xFF,0xFF,0x00,0x00,0x00,0x01,0x11,0x11,0x11,0x011]).unwrap();
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 1,
            system_bytes: 0x11111111,
        };
        assert_eq!(hsms_header_from_bytes,hsms_header);
    }


    #[test]
    fn test_hsms_message_new(){
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 1,
            system_bytes: 0x11111111,
        };
        let hsms_message = HSMSMessage{
            message_length:10,
            hsms_header:hsms_header.clone(),
            message_text:Some(vec![])
        };

        let hsms_message_new = HSMSMessage::new(hsms_header,&vec![]);
        assert_eq!(hsms_message,hsms_message_new);

        let hsms_header_with_text = HSMSHeader{
            session_id: SessionID {session_id:0x8001},
            header_byte2: HeaderByte2 {header_byte2:0x81},
            header_byte3: 3,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        let hsms_message_with_text = HSMSMessage{
            message_length:12,
            hsms_header:hsms_header_with_text.clone(),
            message_text:Some(vec![0x01,0x02])
        };

        let hsms_message_new_with_text = HSMSMessage::new(hsms_header_with_text,&vec![0x01,0x02]);
        assert_eq!(hsms_message_with_text,hsms_message_new_with_text);
    }

    #[test]
    fn test_hsms_message_to_bytes(){
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 1,
            system_bytes: 0x11111111,
        };
        let hsms_message = HSMSMessage{
            message_length:10,
            hsms_header:hsms_header.clone(),
            message_text:None
        };

        let hsms_message_bytes = hsms_message.to_bytes();
        assert_eq!(hsms_message_bytes,vec![0x0A,0x00,0x00,0x00,0xFF,0xFF,0x00,0x00,0x00,0x01,0x11,0x11,0x11,0x011])
    }
    #[test]
    fn test_hsms_message_to_bytes_with_message(){
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        let hsms_message = HSMSMessage{
            message_length:12,
            hsms_header:hsms_header.clone(),
            message_text:Some(vec![0x01,0x02])
        };

        let hsms_message_bytes = hsms_message.to_bytes();
        assert_eq!(hsms_message_bytes,vec![0x0C,0x00,0x00,0x00,0xFF,0xFF,0x00,0x00,0x00,0x00,0x11,0x11,0x11,0x011,0x01,0x02])
    }
    #[test]
    fn test_hsms_message_from_bytes(){
        let hsms_message_from_bytes = HSMSMessage::from_bytes(
            vec![0x0A,0x00,0x00,0x00,0xFF,0xFF,0x00,0x00,0x00,0x00,0x11,0x11,0x11,0x011]);
        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        let hsms_message = HSMSMessage{
            message_length:10,
            hsms_header:hsms_header.clone(),
            message_text:None
        };

        assert_eq!(hsms_message,hsms_message_from_bytes.unwrap());
    }
    #[test]
    fn test_hsms_message_from_bytes_with_message(){
       let hsms_message_from_bytes = HSMSMessage::from_bytes(
           vec![0x0C,0x00,0x00,0x00,0xFF,0xFF,0x00,0x00,0x00,0x00,0x11,0x11,0x11,0x011,0x01,0x02]) ;

        let hsms_header = HSMSHeader{
            session_id: SessionID {session_id:0xFFFF},
            header_byte2: HeaderByte2 {header_byte2:0},
            header_byte3: 0,
            p_type: 0,
            s_type: 0,
            system_bytes: 0x11111111,
        };
        let hsms_message = HSMSMessage{
            message_length:12,
            hsms_header:hsms_header.clone(),
            message_text:Some(vec![0x01,0x02])
        };

        let hsms_message_bytes = hsms_message.to_bytes();
        assert_eq!(hsms_message,hsms_message_from_bytes.unwrap());
    }
}
