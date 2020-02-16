use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MerchantOffersPacket {
    pub containerId: i32,
    pub offers: Vec<MerchantOffer>,
    pub villagerLevel: i32,
    pub villagerXp: i32,
    pub showProgress: bool,
    pub canRestock: bool,
}

impl CodablePacket for MerchantOffersPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.containerId);
        buf.set_mc_u8(self.offers.len() as u8);
        for offer in self.offers {
            buf.set_mc_item_stack(offer.base_cost_a);
            buf.set_mc_item_stack(offer.result);
            match offer.cost_b.is_empty() {
                true => {
                    buf.set_mc_bool(false);
                }
                false => {
                    buf.set_mc_bool(true);
                    buf.set_mc_item_stack(offer.cost_b);
                }
            }
            buf.set_mc_bool(offer.uses >= offer.maxUses);
            buf.set_mc_i32(offer.uses);
            buf.set_mc_i32(offer.maxUses);
            buf.set_mc_i32(offer.xp);
            buf.set_mc_i32(offer.specialPriceDiff);
            buf.set_mc_f32(offer.priceMultiplier);
            buf.set_mc_i32(offer.demand);
        }
        buf.set_mc_var_int(self.villagerLevel);
        buf.set_mc_var_int(self.villagerXp);
        buf.set_mc_bool(self.showProgress);
        buf.set_mc_bool(self.canRestock);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_var_int()?;
        let mut offers: Vec<MerchantOffer> = vec![];
        let offer_count = buf.get_mc_u8()?;
        for _ in 0..offer_count {
            let base_cost_a = buf.get_mc_item_stack()?;
            let result = buf.get_mc_item_stack()?;
            let has_cost_b = buf.get_mc_bool()?;
            let cost_b = if has_cost_b {
                buf.get_mc_item_stack()?
            } else {
                ItemStack::empty()
            };
            buf.get_mc_bool()?; // unused
            let uses = buf.get_mc_i32()?;
            let maxUses = buf.get_mc_i32()?;
            let xp = buf.get_mc_i32()?;
            let specialPriceDiff = buf.get_mc_i32()?;
            let priceMultiplier = buf.get_mc_f32()?;
            let demand = buf.get_mc_i32()?;
            offers.push(MerchantOffer {
                base_cost_a,
                result,
                cost_b,
                uses,
                maxUses,
                xp,
                specialPriceDiff,
                priceMultiplier,
                demand,
                rewardExp: true,
            });
        }
        let villagerLevel = buf.get_mc_var_int()?;
        let villagerXp = buf.get_mc_var_int()?;
        let showProgress = buf.get_mc_bool()?;
        let canRestock = buf.get_mc_bool()?;
        return Ok(MerchantOffersPacket {
            containerId,
            offers,
            villagerLevel,
            villagerXp,
            showProgress,
            canRestock,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MerchantOffersPacket {
            containerId: 234,
            offers: vec![MerchantOffer {
                base_cost_a: ItemStack::empty(),
                result: ItemStack::empty(),
                cost_b: ItemStack::empty(),
                uses: 128,
                maxUses: 192,
                rewardExp: true,
                xp: 345345,
                specialPriceDiff: 1,
                priceMultiplier: 2.0,
                demand: 145353,
            }],
            villagerLevel: 45353,
            villagerXp: 233,
            showProgress: true,
            canRestock: false,
        })
    }
}