///Register `DOEPINT0` reader
pub struct R(crate::R<DOEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DOEPINT0` writer
pub struct W(crate::W<DOEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT0_SPEC>;
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
impl From<crate::W<DOEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRC` reader - XFRC
pub type XFRC_R = crate::BitReader<bool>;
///Field `XFRC` writer - XFRC
pub type XFRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `EPDISD` reader - EPDISD
pub type EPDISD_R = crate::BitReader<bool>;
///Field `EPDISD` writer - EPDISD
pub type EPDISD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `STUP` reader - STUP
pub type STUP_R = crate::BitReader<bool>;
///Field `STUP` writer - STUP
pub type STUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `OTEPDIS` reader - OTEPDIS
pub type OTEPDIS_R = crate::BitReader<bool>;
///Field `OTEPDIS` writer - OTEPDIS
pub type OTEPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `STSPHSRX` reader - Status phase received for control write
pub type STSPHSRX_R = crate::BitReader<bool>;
///Field `STSPHSRX` writer - Status phase received for control write
pub type STSPHSRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `BERR` reader - Babble error interrupt
pub type BERR_R = crate::BitReader<bool>;
///Field `BERR` writer - Babble error interrupt
pub type BERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
///Field `NAK` reader - NAK input
pub type NAK_R = crate::BitReader<bool>;
///Field `NAK` writer - NAK input
pub type NAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT0_SPEC, bool, O>;
impl R {
    ///Bit 0 - XFRC
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    pub fn stsphsrx(&self) -> STSPHSRX_R {
        STSPHSRX_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    pub fn nak(&self) -> NAK_R {
        NAK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - XFRC
    #[inline(always)]
    #[must_use]
    pub fn xfrc(&mut self) -> XFRC_W<0> {
        XFRC_W::new(self)
    }
    ///Bit 1 - EPDISD
    #[inline(always)]
    #[must_use]
    pub fn epdisd(&mut self) -> EPDISD_W<1> {
        EPDISD_W::new(self)
    }
    ///Bit 3 - STUP
    #[inline(always)]
    #[must_use]
    pub fn stup(&mut self) -> STUP_W<3> {
        STUP_W::new(self)
    }
    ///Bit 4 - OTEPDIS
    #[inline(always)]
    #[must_use]
    pub fn otepdis(&mut self) -> OTEPDIS_W<4> {
        OTEPDIS_W::new(self)
    }
    ///Bit 5 - Status phase received for control write
    #[inline(always)]
    #[must_use]
    pub fn stsphsrx(&mut self) -> STSPHSRX_W<5> {
        STSPHSRX_W::new(self)
    }
    ///Bit 12 - Babble error interrupt
    #[inline(always)]
    #[must_use]
    pub fn berr(&mut self) -> BERR_W<12> {
        BERR_W::new(self)
    }
    ///Bit 13 - NAK input
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NAK_W<13> {
        NAK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device endpoint-0 interrupt register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doepint0](index.html) module
pub struct DOEPINT0_SPEC;
impl crate::RegisterSpec for DOEPINT0_SPEC {
    type Ux = u32;
}
///`read()` method returns [doepint0::R](R) reader structure
impl crate::Readable for DOEPINT0_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [doepint0::W](W) writer structure
impl crate::Writable for DOEPINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DOEPINT0 to value 0x80
impl crate::Resettable for DOEPINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
