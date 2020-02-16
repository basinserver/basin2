use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct UpdateTagsPacket {
    pub blocks: Vec<(String, Vec<i32>)>,
    pub items: Vec<(String, Vec<i32>)>,
    pub fluids: Vec<(String, Vec<i32>)>,
    pub entityTypes: Vec<(String, Vec<i32>)>,
}

fn encode_tags(buf: &mut BytesMut, data: Vec<(ResourceLocation, Vec<i32>)>) {
    buf.set_mc_var_int(data.len() as i32);
    for (key, values) in data {
        buf.set_mc_string(key);
        buf.set_mc_var_int(values.len() as i32);
        for value in values {
            buf.set_mc_var_int(value);
        }
    }
}

fn decode_tags(buf: &mut BytesMut) -> Result<Vec<(ResourceLocation, Vec<i32>)>> {
    let mut output: Vec<(ResourceLocation, Vec<i32>)> = vec![];
    let key_count = buf.get_mc_var_int()?;
    for _ in 0..key_count {
        let key = buf.get_mc_string(32767)?;
        let value_count = buf.get_mc_var_int()?;
        let mut values: Vec<i32> = vec![];
        for _ in 0..value_count {
            values.push(buf.get_mc_var_int()?);
        }
        output.push((key, values));
    }
    Ok(output)
}

impl CodablePacket for UpdateTagsPacket {
    fn encode(self, buf: &mut BytesMut) {
        encode_tags(buf, self.blocks);
        encode_tags(buf, self.items);
        encode_tags(buf, self.fluids);
        encode_tags(buf, self.entityTypes);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let blocks = decode_tags(buf)?;
        let items = decode_tags(buf)?;
        let fluids = decode_tags(buf)?;
        let entityTypes = decode_tags(buf)?;
        return Ok(UpdateTagsPacket {
            blocks,
            items,
            fluids,
            entityTypes,
        });
    }
}
