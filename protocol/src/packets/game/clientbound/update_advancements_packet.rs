use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct UpdateAdvancementsPacket {
    pub reset: bool,
    pub added: Vec<(ResourceLocation, Advancement)>,
    pub removed: Vec<ResourceLocation>,
    pub progress: Vec<(ResourceLocation, Option<i64>)>,
}

impl CodablePacket for UpdateAdvancementsPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_bool(self.reset);
        buf.set_mc_var_int(self.added.len() as i32);
        for (key, value) in self.added {
            buf.set_mc_string(key);
            match value.parentId {
                Some(parentId) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_string(parentId);
                }
                None => {
                    buf.set_mc_bool(false);
                }
            }
            match value.display {
                Some(display) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_chat_component(display.title);
                    buf.set_mc_chat_component(display.description);
                    buf.set_mc_item_stack(display.icon);
                    buf.set_mc_var_int(display.frame as i32);
                    let mut flags: i32 = 0;
                    if display.background.is_some() {
                        flags |= 1;
                    }
                    if display.showToast {
                        flags |= 2;
                    }
                    if display.hidden {
                        flags |= 4;
                    }
                    buf.set_mc_i32(flags);
                    if display.background.is_some() {
                        buf.set_mc_string(display.background.unwrap());
                    }
                    buf.set_mc_f32(display.x);
                    buf.set_mc_f32(display.y);
                }
                None => {
                    buf.set_mc_bool(false);
                }
            }
            buf.set_mc_var_int(value.criterion.len() as i32);
            for critereon in value.criterion {
                buf.set_mc_string(critereon);
            }
            buf.set_mc_var_int(value.requirements.len() as i32);
            for requirement_set in value.requirements {
                buf.set_mc_var_int(requirement_set.len() as i32);
                for requirement in requirement_set {
                    buf.set_mc_string(requirement);
                }
            }
        }
        buf.set_mc_var_int(self.removed.len() as i32);
        for key in self.removed {
            buf.set_mc_string(key);
        }
        buf.set_mc_var_int(self.progress.len() as i32);
        for (key, value) in self.progress {
            buf.set_mc_string(key);
            match value {
                Some(timestamp) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_i64(timestamp);
                }
                None => {
                    buf.set_mc_bool(false);
                }
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let reset = buf.get_mc_bool()?;
        let added_count = buf.get_mc_var_int()?;
        let mut added: Vec<(ResourceLocation, Advancement)> = vec![];
        for _ in 0..added_count {
            let key = buf.get_mc_string(32767)?;
            let parentId = if buf.get_mc_bool()? {
                Some(buf.get_mc_chat_component()?)
            } else {
                None
            };
            let display = if buf.get_mc_bool()? {
                let title = buf.get_mc_chat_component()?;
                let description = buf.get_mc_chat_component()?;
                let icon = buf.get_mc_item_stack()?;
                let frame: FrameType = buf.get_mc_enum()?;
                let flags = buf.get_mc_i32()?;
                let showToast = (flags & 2) > 0;
                let hidden = (flags & 4) > 0;
                let background = if (flags & 1) > 0 {
                    Some(buf.get_mc_string(32767)?)
                } else {
                    None
                };
                let x = buf.get_mc_f32()?;
                let y = buf.get_mc_f32()?;
                Some(AdvancementDisplayInfo {
                    title,
                    description,
                    icon,
                    frame,
                    showToast,
                    hidden,
                    background,
                    x,
                    y,
                })
            } else {
                None
            };
            let criterion_count = buf.get_mc_var_int()?;
            let mut criterion: Vec<String> = vec![];
            for _ in 0..criterion_count {
                criterion.push(buf.get_mc_string(32767)?)
            }
            let mut requirements: Vec<Vec<String>> = vec![];
            let requirements_count = buf.get_mc_var_int()?;
            for _ in 0..requirements_count {
                let requirement_set_count = buf.get_mc_var_int()?;
                let mut requirement_set: Vec<String> = vec![];
                for _ in 0..requirement_set_count {
                    requirement_set.push(buf.get_mc_string(32767)?);
                }
                requirements.push(requirement_set);
            }
            let advancement = Advancement {
                parentId,
                display,
                criterion,
                requirements,
            };
            added.push((key, advancement));
        }
        let removed_count = buf.get_mc_var_int()?;
        let mut removed: Vec<ResourceLocation> = vec![];
        for _ in 0..removed_count {
            removed.push(buf.get_mc_string(32767)?);
        }
        let progress_count = buf.get_mc_var_int()?;
        let mut progress: Vec<(ResourceLocation, Option<i64>)> = vec![];
        for _ in 0..progress_count {
            let key = buf.get_mc_string(32767)?;
            let timestamp = if buf.get_mc_bool()? {
                Some(buf.get_mc_i64()?)
            } else {
                None
            };
            progress.push((key, timestamp));
        }
        return Ok(UpdateAdvancementsPacket {
            reset,
            added,
            removed,
            progress,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(UpdateAdvancementsPacket {
            reset: false,
            added: vec![(
                "a name".to_string(),
                Advancement {
                    parentId: Some("another advancement".to_string()),
                    display: Some(AdvancementDisplayInfo {
                        title: "display title".to_string(),
                        description: "description".to_string(),
                        icon: ItemStack::empty(),
                        frame: FrameType::Task,
                        background: Some("background".to_string()),
                        showToast: true,
                        hidden: false,
                        x: 120.0,
                        y: 19.0,
                    }),
                    criterion: vec!["criterion1".to_string(), "criterion2".to_string()],
                    requirements: vec![
                        vec!["req11".to_string(), "req12".to_string()],
                        vec!["req21".to_string(), "req22".to_string()],
                    ],
                },
            )],
            removed: vec!["removed_advancement1".to_string()],
            progress: vec![("progress_advancement1".to_string(), Some(12))],
        })
    }
}
