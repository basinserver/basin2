
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

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
        // TODO: EXTRA: this.offers.writeToStream(var1);
        buf.set_mc_var_int(self.villagerLevel);
        buf.set_mc_var_int(self.villagerXp);
        buf.set_mc_bool(self.showProgress);
        buf.set_mc_bool(self.canRestock);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_var_int()?;
        // TODO: EXTRA: this.offers = MerchantOffers.createFromStream(var1);
        let villagerLevel = buf.get_mc_var_int()?;
        let villagerXp = buf.get_mc_var_int()?;
        let showProgress = buf.get_mc_bool()?;
        let canRestock = buf.get_mc_bool()?;
        return Ok(MerchantOffersPacket { containerId, offers, villagerLevel, villagerXp, showProgress, canRestock });
    }
}
