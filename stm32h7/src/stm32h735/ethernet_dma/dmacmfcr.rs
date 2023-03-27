///Register `DMACMFCR` reader
pub struct R(crate::R<DMACMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACMFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DMACMFCR` writer
pub struct W(crate::W<DMACMFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACMFCR_SPEC>;
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
impl From<crate::W<DMACMFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACMFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MFC` reader - Dropped Packet Counters
pub type MFC_R = crate::FieldReader<u16, u16>;
///Field `MFC` writer - Dropped Packet Counters
pub type MFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMACMFCR_SPEC, u16, u16, 11, O>;
///Field `MFCO` reader - Overflow status of the MFC Counter
pub type MFCO_R = crate::BitReader<bool>;
///Field `MFCO` writer - Overflow status of the MFC Counter
pub type MFCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACMFCR_SPEC, bool, O>;
impl R {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    pub fn mfc(&self) -> MFC_R {
        MFC_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    pub fn mfco(&self) -> MFCO_R {
        MFCO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:10 - Dropped Packet Counters
    #[inline(always)]
    #[must_use]
    pub fn mfc(&mut self) -> MFC_W<0> {
        MFC_W::new(self)
    }
    ///Bit 15 - Overflow status of the MFC Counter
    #[inline(always)]
    #[must_use]
    pub fn mfco(&mut self) -> MFCO_W<15> {
        MFCO_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Channel missed frame count register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dmacmfcr](index.html) module
pub struct DMACMFCR_SPEC;
impl crate::RegisterSpec for DMACMFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dmacmfcr::R](R) reader structure
impl crate::Readable for DMACMFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dmacmfcr::W](W) writer structure
impl crate::Writable for DMACMFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DMACMFCR to value 0
impl crate::Resettable for DMACMFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
