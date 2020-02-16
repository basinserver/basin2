use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MapItemDataPacket {
    pub mapId: i32,
    pub scale: u8,
    pub trackingPosition: bool,
    pub locked: bool,
    pub decorations: Vec<MapDecoration>,
    pub startX: u8,
    pub startY: u8,
    pub width: u8,
    pub height: u8,
    pub mapColors: Vec<u8>,
}

impl CodablePacket for MapItemDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.mapId);
        buf.set_mc_u8(self.scale);
        buf.set_mc_bool(self.trackingPosition);
        buf.set_mc_bool(self.locked);
        buf.set_mc_var_int(self.decorations.len() as i32);
        for decoration in self.decorations {
            buf.set_mc_var_int(decoration.decoration_type as i32);
            buf.set_mc_u8(decoration.x);
            buf.set_mc_u8(decoration.y);
            buf.set_mc_u8(decoration.rot & 15);
            match decoration.component {
                Some(component) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_chat_component(component);
                }
                None => {
                    buf.set_mc_bool(false);
                }
            }
        }
        buf.set_mc_u8(self.width);
        if self.width > 0 {
            buf.set_mc_u8(self.height);
            buf.set_mc_u8(self.startX);
            buf.set_mc_u8(self.startY);
            buf.set_mc_byte_array(self.mapColors);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let mapId = buf.get_mc_var_int()?;
        let scale = buf.get_mc_u8()?;
        let trackingPosition = buf.get_mc_bool()?;
        let locked = buf.get_mc_bool()?;
        let decorations_count = buf.get_mc_var_int()?;
        let mut decorations: Vec<MapDecoration> = vec![];
        for _ in 0..decorations_count {
            decorations.push(MapDecoration {
                decoration_type: buf.get_mc_enum()?,
                x: buf.get_mc_u8()?,
                y: buf.get_mc_u8()?,
                rot: buf.get_mc_u8()? & 15,
                component: match buf.get_mc_bool()? {
                    true => Some(buf.get_mc_chat_component()?),
                    false => None,
                },
            });
        }
        let width = buf.get_mc_u8()?;
        let (height, startX, startY, mapColors) = match width {
            x if x > 0 => (
                buf.get_mc_u8()?,
                buf.get_mc_u8()?,
                buf.get_mc_u8()?,
                buf.get_mc_byte_array_bounded(2048)?,
            ),
            _ => (0, 0, 0, vec![]),
        };
        return Ok(MapItemDataPacket {
            mapId,
            scale,
            trackingPosition,
            locked,
            decorations,
            startX,
            startY,
            width,
            height,
            mapColors,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MapItemDataPacket {
            mapId: 234,
            scale: 8,
            trackingPosition: true,
            locked: false,
            decorations: vec![MapDecoration {
                decoration_type: MapDecorationType::BannerMagenta,
                x: 23,
                y: 19,
                rot: 1,
                component: Some("test".to_string()),
            }],
            startX: 12,
            startY: 25,
            width: 90,
            height: 90,
            mapColors: vec![12, 96],
        })
    }
}
