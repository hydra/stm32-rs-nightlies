///Register `EP0R` reader
pub struct R(crate::R<EP0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EP0R` writer
pub struct W(crate::W<EP0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EP0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EA` reader - EA
pub type EA_R = crate::FieldReader<u8, u8>;
///Field `EA` writer - EA
pub type EA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0R_SPEC, u8, u8, 4, O>;
///Field `STAT_TX` reader - STAT_TX
pub type STAT_TX_R = crate::FieldReader<u8, u8>;
///Field `STAT_TX` writer - STAT_TX
pub type STAT_TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0R_SPEC, u8, u8, 2, O>;
///Field `DTOG_TX` reader - DTOG_TX
pub type DTOG_TX_R = crate::BitReader<bool>;
///Field `DTOG_TX` writer - DTOG_TX
pub type DTOG_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
///Field `CTR_TX` reader - CTR_TX
pub type CTR_TX_R = crate::BitReader<bool>;
///Field `CTR_TX` writer - CTR_TX
pub type CTR_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
///Field `EP_KIND` reader - EP_KIND
pub type EP_KIND_R = crate::BitReader<bool>;
///Field `EP_KIND` writer - EP_KIND
pub type EP_KIND_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
///Field `EP_TYPE` reader - EP_TYPE
pub type EP_TYPE_R = crate::FieldReader<u8, u8>;
///Field `EP_TYPE` writer - EP_TYPE
pub type EP_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0R_SPEC, u8, u8, 2, O>;
///Field `SETUP` reader - SETUP
pub type SETUP_R = crate::BitReader<bool>;
///Field `SETUP` writer - SETUP
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
///Field `STAT_RX` reader - STAT_RX
pub type STAT_RX_R = crate::FieldReader<u8, u8>;
///Field `STAT_RX` writer - STAT_RX
pub type STAT_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EP0R_SPEC, u8, u8, 2, O>;
///Field `DTOG_RX` reader - DTOG_RX
pub type DTOG_RX_R = crate::BitReader<bool>;
///Field `DTOG_RX` writer - DTOG_RX
pub type DTOG_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
///Field `CTR_RX` reader - CTR_RX
pub type CTR_RX_R = crate::BitReader<bool>;
///Field `CTR_RX` writer - CTR_RX
pub type CTR_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, EP0R_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - EA
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - STAT_TX
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - DTOG_TX
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CTR_TX
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - EP_KIND
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - EP_TYPE
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - SETUP
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - STAT_RX
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - DTOG_RX
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTR_RX
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - EA
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<0> {
        EA_W::new(self)
    }
    ///Bits 4:5 - STAT_TX
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<4> {
        STAT_TX_W::new(self)
    }
    ///Bit 6 - DTOG_TX
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<6> {
        DTOG_TX_W::new(self)
    }
    ///Bit 7 - CTR_TX
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<7> {
        CTR_TX_W::new(self)
    }
    ///Bit 8 - EP_KIND
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<8> {
        EP_KIND_W::new(self)
    }
    ///Bits 9:10 - EP_TYPE
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<9> {
        EP_TYPE_W::new(self)
    }
    ///Bit 11 - SETUP
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<11> {
        SETUP_W::new(self)
    }
    ///Bits 12:13 - STAT_RX
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<12> {
        STAT_RX_W::new(self)
    }
    ///Bit 14 - DTOG_RX
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<14> {
        DTOG_RX_W::new(self)
    }
    ///Bit 15 - CTR_RX
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<15> {
        CTR_RX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB endpoint n register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep0r](index.html) module
pub struct EP0R_SPEC;
impl crate::RegisterSpec for EP0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [ep0r::R](R) reader structure
impl crate::Readable for EP0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ep0r::W](W) writer structure
impl crate::Writable for EP0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EP0R to value 0
impl crate::Resettable for EP0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
