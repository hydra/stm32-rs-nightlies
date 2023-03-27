///Register `HCINTMSK4` reader
pub struct R(crate::R<HCINTMSK4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `HCINTMSK4` writer
pub struct W(crate::W<HCINTMSK4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK4_SPEC>;
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
impl From<crate::W<HCINTMSK4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRCM` reader - Transfer completed mask
pub type XFRCM_R = crate::BitReader<bool>;
///Field `XFRCM` writer - Transfer completed mask
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `CHHM` reader - Channel halted mask
pub type CHHM_R = crate::BitReader<bool>;
///Field `CHHM` writer - Channel halted mask
pub type CHHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `STALLM` reader - STALL response received interrupt mask
pub type STALLM_R = crate::BitReader<bool>;
///Field `STALLM` writer - STALL response received interrupt mask
pub type STALLM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `NAKM` reader - NAK response received interrupt mask
pub type NAKM_R = crate::BitReader<bool>;
///Field `NAKM` writer - NAK response received interrupt mask
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `ACKM` reader - ACK response received/transmitted interrupt mask
pub type ACKM_R = crate::BitReader<bool>;
///Field `ACKM` writer - ACK response received/transmitted interrupt mask
pub type ACKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `TXERRM` reader - Transaction error mask
pub type TXERRM_R = crate::BitReader<bool>;
///Field `TXERRM` writer - Transaction error mask
pub type TXERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `BBERRM` reader - Babble error mask
pub type BBERRM_R = crate::BitReader<bool>;
///Field `BBERRM` writer - Babble error mask
pub type BBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `FRMORM` reader - Frame overrun mask
pub type FRMORM_R = crate::BitReader<bool>;
///Field `FRMORM` writer - Frame overrun mask
pub type FRMORM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
///Field `DTERRM` reader - Data toggle error mask
pub type DTERRM_R = crate::BitReader<bool>;
///Field `DTERRM` writer - Data toggle error mask
pub type DTERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCINTMSK4_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transfer completed mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Channel halted mask
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - STALL response received interrupt mask
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NAK response received interrupt mask
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ACK response received/transmitted interrupt mask
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Transaction error mask
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Babble error mask
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame overrun mask
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data toggle error mask
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer completed mask
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    ///Bit 1 - Channel halted mask
    #[inline(always)]
    #[must_use]
    pub fn chhm(&mut self) -> CHHM_W<1> {
        CHHM_W::new(self)
    }
    ///Bit 3 - STALL response received interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn stallm(&mut self) -> STALLM_W<3> {
        STALLM_W::new(self)
    }
    ///Bit 4 - NAK response received interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<4> {
        NAKM_W::new(self)
    }
    ///Bit 5 - ACK response received/transmitted interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn ackm(&mut self) -> ACKM_W<5> {
        ACKM_W::new(self)
    }
    ///Bit 7 - Transaction error mask
    #[inline(always)]
    #[must_use]
    pub fn txerrm(&mut self) -> TXERRM_W<7> {
        TXERRM_W::new(self)
    }
    ///Bit 8 - Babble error mask
    #[inline(always)]
    #[must_use]
    pub fn bberrm(&mut self) -> BBERRM_W<8> {
        BBERRM_W::new(self)
    }
    ///Bit 9 - Frame overrun mask
    #[inline(always)]
    #[must_use]
    pub fn frmorm(&mut self) -> FRMORM_W<9> {
        FRMORM_W::new(self)
    }
    ///Bit 10 - Data toggle error mask
    #[inline(always)]
    #[must_use]
    pub fn dterrm(&mut self) -> DTERRM_W<10> {
        DTERRM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///OTG_FS host channel-4 mask register (OTG_FS_HCINTMSK4)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hcintmsk4](index.html) module
pub struct HCINTMSK4_SPEC;
impl crate::RegisterSpec for HCINTMSK4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hcintmsk4::R](R) reader structure
impl crate::Readable for HCINTMSK4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [hcintmsk4::W](W) writer structure
impl crate::Writable for HCINTMSK4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets HCINTMSK4 to value 0
impl crate::Resettable for HCINTMSK4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
