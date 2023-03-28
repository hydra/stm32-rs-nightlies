///Register `CMCR` reader
pub struct R(crate::R<CMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CMCR` writer
pub struct W(crate::W<CMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCR_SPEC>;
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
impl From<crate::W<CMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TEARE` reader - Tearing Effect Acknowledge Request Enable
pub type TEARE_R = crate::BitReader<bool>;
///Field `TEARE` writer - Tearing Effect Acknowledge Request Enable
pub type TEARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `ARE` reader - Acknowledge Request Enable
pub type ARE_R = crate::BitReader<bool>;
///Field `ARE` writer - Acknowledge Request Enable
pub type ARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSW0TX` reader - Generic Short Write Zero parameters Transmission
pub type GSW0TX_R = crate::BitReader<bool>;
///Field `GSW0TX` writer - Generic Short Write Zero parameters Transmission
pub type GSW0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSW1TX` reader - Generic Short Write One parameters Transmission
pub type GSW1TX_R = crate::BitReader<bool>;
///Field `GSW1TX` writer - Generic Short Write One parameters Transmission
pub type GSW1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSW2TX` reader - Generic Short Write Two parameters Transmission
pub type GSW2TX_R = crate::BitReader<bool>;
///Field `GSW2TX` writer - Generic Short Write Two parameters Transmission
pub type GSW2TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSR0TX` reader - Generic Short Read Zero parameters Transmission
pub type GSR0TX_R = crate::BitReader<bool>;
///Field `GSR0TX` writer - Generic Short Read Zero parameters Transmission
pub type GSR0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSR1TX` reader - Generic Short Read One parameters Transmission
pub type GSR1TX_R = crate::BitReader<bool>;
///Field `GSR1TX` writer - Generic Short Read One parameters Transmission
pub type GSR1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GSR2TX` reader - Generic Short Read Two parameters Transmission
pub type GSR2TX_R = crate::BitReader<bool>;
///Field `GSR2TX` writer - Generic Short Read Two parameters Transmission
pub type GSR2TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `GLWTX` reader - Generic Long Write Transmission
pub type GLWTX_R = crate::BitReader<bool>;
///Field `GLWTX` writer - Generic Long Write Transmission
pub type GLWTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `DSW0TX` reader - DCS Short Write Zero parameter Transmission
pub type DSW0TX_R = crate::BitReader<bool>;
///Field `DSW0TX` writer - DCS Short Write Zero parameter Transmission
pub type DSW0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `DSW1TX` reader - DCS Short Read One parameter Transmission
pub type DSW1TX_R = crate::BitReader<bool>;
///Field `DSW1TX` writer - DCS Short Read One parameter Transmission
pub type DSW1TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `DSR0TX` reader - DCS Short Read Zero parameter Transmission
pub type DSR0TX_R = crate::BitReader<bool>;
///Field `DSR0TX` writer - DCS Short Read Zero parameter Transmission
pub type DSR0TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `DLWTX` reader - DCS Long Write Transmission
pub type DLWTX_R = crate::BitReader<bool>;
///Field `DLWTX` writer - DCS Long Write Transmission
pub type DLWTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
///Field `MRDPS` reader - Maximum Read Packet Size
pub type MRDPS_R = crate::BitReader<bool>;
///Field `MRDPS` writer - Maximum Read Packet Size
pub type MRDPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tearing Effect Acknowledge Request Enable
    #[inline(always)]
    pub fn teare(&self) -> TEARE_R {
        TEARE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Acknowledge Request Enable
    #[inline(always)]
    pub fn are(&self) -> ARE_R {
        ARE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - Generic Short Write Zero parameters Transmission
    #[inline(always)]
    pub fn gsw0tx(&self) -> GSW0TX_R {
        GSW0TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic Short Write One parameters Transmission
    #[inline(always)]
    pub fn gsw1tx(&self) -> GSW1TX_R {
        GSW1TX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic Short Write Two parameters Transmission
    #[inline(always)]
    pub fn gsw2tx(&self) -> GSW2TX_R {
        GSW2TX_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic Short Read Zero parameters Transmission
    #[inline(always)]
    pub fn gsr0tx(&self) -> GSR0TX_R {
        GSR0TX_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic Short Read One parameters Transmission
    #[inline(always)]
    pub fn gsr1tx(&self) -> GSR1TX_R {
        GSR1TX_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Generic Short Read Two parameters Transmission
    #[inline(always)]
    pub fn gsr2tx(&self) -> GSR2TX_R {
        GSR2TX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Generic Long Write Transmission
    #[inline(always)]
    pub fn glwtx(&self) -> GLWTX_R {
        GLWTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - DCS Short Write Zero parameter Transmission
    #[inline(always)]
    pub fn dsw0tx(&self) -> DSW0TX_R {
        DSW0TX_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DCS Short Read One parameter Transmission
    #[inline(always)]
    pub fn dsw1tx(&self) -> DSW1TX_R {
        DSW1TX_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - DCS Short Read Zero parameter Transmission
    #[inline(always)]
    pub fn dsr0tx(&self) -> DSR0TX_R {
        DSR0TX_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - DCS Long Write Transmission
    #[inline(always)]
    pub fn dlwtx(&self) -> DLWTX_R {
        DLWTX_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 24 - Maximum Read Packet Size
    #[inline(always)]
    pub fn mrdps(&self) -> MRDPS_R {
        MRDPS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tearing Effect Acknowledge Request Enable
    #[inline(always)]
    #[must_use]
    pub fn teare(&mut self) -> TEARE_W<0> {
        TEARE_W::new(self)
    }
    ///Bit 1 - Acknowledge Request Enable
    #[inline(always)]
    #[must_use]
    pub fn are(&mut self) -> ARE_W<1> {
        ARE_W::new(self)
    }
    ///Bit 8 - Generic Short Write Zero parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsw0tx(&mut self) -> GSW0TX_W<8> {
        GSW0TX_W::new(self)
    }
    ///Bit 9 - Generic Short Write One parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsw1tx(&mut self) -> GSW1TX_W<9> {
        GSW1TX_W::new(self)
    }
    ///Bit 10 - Generic Short Write Two parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsw2tx(&mut self) -> GSW2TX_W<10> {
        GSW2TX_W::new(self)
    }
    ///Bit 11 - Generic Short Read Zero parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsr0tx(&mut self) -> GSR0TX_W<11> {
        GSR0TX_W::new(self)
    }
    ///Bit 12 - Generic Short Read One parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsr1tx(&mut self) -> GSR1TX_W<12> {
        GSR1TX_W::new(self)
    }
    ///Bit 13 - Generic Short Read Two parameters Transmission
    #[inline(always)]
    #[must_use]
    pub fn gsr2tx(&mut self) -> GSR2TX_W<13> {
        GSR2TX_W::new(self)
    }
    ///Bit 14 - Generic Long Write Transmission
    #[inline(always)]
    #[must_use]
    pub fn glwtx(&mut self) -> GLWTX_W<14> {
        GLWTX_W::new(self)
    }
    ///Bit 16 - DCS Short Write Zero parameter Transmission
    #[inline(always)]
    #[must_use]
    pub fn dsw0tx(&mut self) -> DSW0TX_W<16> {
        DSW0TX_W::new(self)
    }
    ///Bit 17 - DCS Short Read One parameter Transmission
    #[inline(always)]
    #[must_use]
    pub fn dsw1tx(&mut self) -> DSW1TX_W<17> {
        DSW1TX_W::new(self)
    }
    ///Bit 18 - DCS Short Read Zero parameter Transmission
    #[inline(always)]
    #[must_use]
    pub fn dsr0tx(&mut self) -> DSR0TX_W<18> {
        DSR0TX_W::new(self)
    }
    ///Bit 19 - DCS Long Write Transmission
    #[inline(always)]
    #[must_use]
    pub fn dlwtx(&mut self) -> DLWTX_W<19> {
        DLWTX_W::new(self)
    }
    ///Bit 24 - Maximum Read Packet Size
    #[inline(always)]
    #[must_use]
    pub fn mrdps(&mut self) -> MRDPS_W<24> {
        MRDPS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Command mode Configuration Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cmcr](index.html) module
pub struct CMCR_SPEC;
impl crate::RegisterSpec for CMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cmcr::R](R) reader structure
impl crate::Readable for CMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cmcr::W](W) writer structure
impl crate::Writable for CMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CMCR to value 0
impl crate::Resettable for CMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
