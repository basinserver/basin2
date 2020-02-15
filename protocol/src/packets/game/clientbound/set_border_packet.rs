use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub enum SetBorderPacketData {
    SetSize(f64),
    LerpSize(f64, f64, i64),
    SetCenter(f64, f64),
    Initialize(f64, f64, f64, f64, i64, i32, i32, i32),
    SetWarningTime(i32),
    SetWarningBlocks(i32),
}

pub struct SetBorderPacket {
    pub borderType: SetBorderPacketType,
    pub borderData: SetBorderPacketData,
}

impl CodablePacket for SetBorderPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.borderType as i32);
        match (self.borderType, self.borderData) {
            (SetBorderPacketType::SetSize, SetBorderPacketData::SetSize(newSize)) => {
                buf.set_mc_f64(newSize);
            }
            (
                SetBorderPacketType::LerpSize,
                SetBorderPacketData::LerpSize(oldSize, newSize, lerpTime),
            ) => {
                buf.set_mc_f64(oldSize);
                buf.set_mc_f64(newSize);
                buf.set_mc_var_long(lerpTime);
            }
            (
                SetBorderPacketType::SetCenter,
                SetBorderPacketData::SetCenter(newCenterX, newCenterZ),
            ) => {
                buf.set_mc_f64(newCenterX);
                buf.set_mc_f64(newCenterZ);
            }
            (
                SetBorderPacketType::Initialize,
                SetBorderPacketData::Initialize(
                    newCenterX,
                    newCenterZ,
                    oldSize,
                    newSize,
                    lerpTime,
                    newAbsoluteMaxSize,
                    warningBlocks,
                    warningTime,
                ),
            ) => {
                buf.set_mc_f64(newCenterX);
                buf.set_mc_f64(newCenterZ);
                buf.set_mc_f64(oldSize);
                buf.set_mc_f64(newSize);
                buf.set_mc_var_long(lerpTime);
                buf.set_mc_var_int(newAbsoluteMaxSize);
                buf.set_mc_var_int(warningBlocks);
                buf.set_mc_var_int(warningTime);
            }
            (
                SetBorderPacketType::SetWarningTime,
                SetBorderPacketData::SetWarningTime(warningTime),
            ) => {
                buf.set_mc_var_int(warningTime);
            }
            (
                SetBorderPacketType::SetWarningBlocks,
                SetBorderPacketData::SetWarningBlocks(warningBlocks),
            ) => {
                buf.set_mc_var_int(warningBlocks);
            }
            _ => panic!("invalid formed outgoing set_border_packet, mismatched types"),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use SetBorderPacketType::*;

        let borderType: SetBorderPacketType = buf.get_mc_enum()?;
        let borderData = match borderType {
            SetSize => SetBorderPacketData::SetSize(buf.get_mc_f64()?),
            LerpSize => SetBorderPacketData::LerpSize(
                buf.get_mc_f64()?,
                buf.get_mc_f64()?,
                buf.get_mc_var_long()?,
            ),
            SetCenter => SetBorderPacketData::SetCenter(buf.get_mc_f64()?, buf.get_mc_f64()?),
            Initialize => SetBorderPacketData::Initialize(
                buf.get_mc_f64()?,
                buf.get_mc_f64()?,
                buf.get_mc_f64()?,
                buf.get_mc_f64()?,
                buf.get_mc_var_long()?,
                buf.get_mc_var_int()?,
                buf.get_mc_var_int()?,
                buf.get_mc_var_int()?,
            ),
            SetWarningTime => SetBorderPacketData::SetWarningTime(buf.get_mc_var_int()?),
            SetWarningBlocks => SetBorderPacketData::SetWarningBlocks(buf.get_mc_var_int()?),
        };
        return Ok(SetBorderPacket {
            borderType,
            borderData,
        });
    }
}
