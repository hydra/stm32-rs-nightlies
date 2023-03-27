///Register `DMAMR` reader
pub struct R(crate::R<DMAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMAMR` writer
pub struct W(crate::W<DMAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMR_SPEC>;
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
impl From<crate::W<DMAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWR` reader - Software Reset
pub type SWR_R = crate::BitReader<bool>;
///Field `SWR` writer - Software Reset
pub type SWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMR_SPEC, bool, O>;
///Field `DA` reader - DMA Tx or Rx Arbitration Scheme
pub type DA_R = crate::BitReader<bool>;
///Field `DA` writer - DMA Tx or Rx Arbitration Scheme
pub type DA_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMR_SPEC, bool, O>;
///Field `TXPR` reader - Transmit priority
pub type TXPR_R = crate::BitReader<bool>;
///Field `TXPR` writer - Transmit priority
pub type TXPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAMR_SPEC, bool, O>;
///Field `PR` reader - Priority ratio
pub type PR_R = crate::FieldReader<u8, u8>;
///Field `PR` writer - Priority ratio
pub type PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMR_SPEC, u8, u8, 3, O>;
///Field `INTM` reader - Interrupt Mode
pub type INTM_R = crate::FieldReader<u8, u8>;
///Field `INTM` writer - Interrupt Mode
pub type INTM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMAMR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - Software Reset
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - Software Reset
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<0> {
        SWR_W::new(self)
    }
    ///Bit 1 - DMA Tx or Rx Arbitration Scheme
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<1> {
        DA_W::new(self)
    }
    ///Bit 11 - Transmit priority
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<11> {
        TXPR_W::new(self)
    }
    ///Bits 12:14 - Priority ratio
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<12> {
        PR_W::new(self)
    }
    ///Bits 16:17 - Interrupt Mode
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<16> {
        INTM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA mode register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmamr](index.html) module
pub struct DMAMR_SPEC;
impl crate::RegisterSpec for DMAMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmamr::R](R) reader structure
impl crate::Readable for DMAMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmamr::W](W) writer structure
impl crate::Writable for DMAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMAMR to value 0
impl crate::Resettable for DMAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
