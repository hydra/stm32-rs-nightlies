///Register `DIEPEACHMSK1` reader
pub struct R(crate::R<DIEPEACHMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPEACHMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPEACHMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPEACHMSK1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIEPEACHMSK1` writer
pub struct W(crate::W<DIEPEACHMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPEACHMSK1_SPEC>;
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
impl From<crate::W<DIEPEACHMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPEACHMSK1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `XFRCM` reader - Transfer completed interrupt mask
pub type XFRCM_R = crate::BitReader<bool>;
///Field `XFRCM` writer - Transfer completed interrupt mask
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `EPDM` reader - Endpoint disabled interrupt mask
pub type EPDM_R = crate::BitReader<bool>;
///Field `EPDM` writer - Endpoint disabled interrupt mask
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `AHBERRM` reader - AHB error mask
pub type AHBERRM_R = crate::BitReader<bool>;
///Field `AHBERRM` writer - AHB error mask
pub type AHBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `TOM` reader - Timeout condition mask (Non-isochronous endpoints)
pub type TOM_R = crate::BitReader<bool>;
///Field `TOM` writer - Timeout condition mask (Non-isochronous endpoints)
pub type TOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_R = crate::BitReader<bool>;
///Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask
pub type ITTXFEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `INEPNEM` reader - IN endpoint NAK effective mask
pub type INEPNEM_R = crate::BitReader<bool>;
///Field `INEPNEM` writer - IN endpoint NAK effective mask
pub type INEPNEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `TXFURM` reader - FIFO underrun mask
pub type TXFURM_R = crate::BitReader<bool>;
///Field `TXFURM` writer - FIFO underrun mask
pub type TXFURM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `BNAM` reader - BNA interrupt mask
pub type BNAM_R = crate::BitReader<bool>;
///Field `BNAM` writer - BNA interrupt mask
pub type BNAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
///Field `NAKM` reader - NAK interrupt mask
pub type NAKM_R = crate::BitReader<bool>;
///Field `NAKM` writer - NAK interrupt mask
pub type NAKM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIEPEACHMSK1_SPEC, bool, O>;
impl R {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - AHB error mask
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - FIFO underrun mask
    #[inline(always)]
    pub fn txfurm(&self) -> TXFURM_R {
        TXFURM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Transfer completed interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    ///Bit 1 - Endpoint disabled interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    ///Bit 2 - AHB error mask
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<2> {
        AHBERRM_W::new(self)
    }
    ///Bit 3 - Timeout condition mask (Non-isochronous endpoints)
    #[inline(always)]
    #[must_use]
    pub fn tom(&mut self) -> TOM_W<3> {
        TOM_W::new(self)
    }
    ///Bit 4 - IN token received when TxFIFO empty mask
    #[inline(always)]
    #[must_use]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<4> {
        ITTXFEMSK_W::new(self)
    }
    ///Bit 6 - IN endpoint NAK effective mask
    #[inline(always)]
    #[must_use]
    pub fn inepnem(&mut self) -> INEPNEM_W<6> {
        INEPNEM_W::new(self)
    }
    ///Bit 8 - FIFO underrun mask
    #[inline(always)]
    #[must_use]
    pub fn txfurm(&mut self) -> TXFURM_W<8> {
        TXFURM_W::new(self)
    }
    ///Bit 9 - BNA interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn bnam(&mut self) -> BNAM_W<9> {
        BNAM_W::new(self)
    }
    ///Bit 13 - NAK interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn nakm(&mut self) -> NAKM_W<13> {
        NAKM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [diepeachmsk1](index.html) module
pub struct DIEPEACHMSK1_SPEC;
impl crate::RegisterSpec for DIEPEACHMSK1_SPEC {
    type Ux = u32;
}
///`read()` method returns [diepeachmsk1::R](R) reader structure
impl crate::Readable for DIEPEACHMSK1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [diepeachmsk1::W](W) writer structure
impl crate::Writable for DIEPEACHMSK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIEPEACHMSK1 to value 0
impl crate::Resettable for DIEPEACHMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
