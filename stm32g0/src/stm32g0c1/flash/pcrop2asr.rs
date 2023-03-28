///Register `PCROP2ASR` reader
pub struct R(crate::R<PCROP2ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCROP2ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCROP2ASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCROP2ASR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PCROP2ASR` writer
pub struct W(crate::W<PCROP2ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCROP2ASR_SPEC>;
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
impl From<crate::W<PCROP2ASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCROP2ASR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PCROP2A_STRT` reader - PCROP2A area start offset, bank2
pub type PCROP2A_STRT_R = crate::FieldReader<u16, u16>;
///Field `PCROP2A_STRT` writer - PCROP2A area start offset, bank2
pub type PCROP2A_STRT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PCROP2ASR_SPEC, u16, u16, 9, O>;
impl R {
    ///Bits 0:8 - PCROP2A area start offset, bank2
    #[inline(always)]
    pub fn pcrop2a_strt(&self) -> PCROP2A_STRT_R {
        PCROP2A_STRT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    ///Bits 0:8 - PCROP2A area start offset, bank2
    #[inline(always)]
    #[must_use]
    pub fn pcrop2a_strt(&mut self) -> PCROP2A_STRT_W<0> {
        PCROP2A_STRT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash PCROP2 area A start address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pcrop2asr](index.html) module
pub struct PCROP2ASR_SPEC;
impl crate::RegisterSpec for PCROP2ASR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pcrop2asr::R](R) reader structure
impl crate::Readable for PCROP2ASR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pcrop2asr::W](W) writer structure
impl crate::Writable for PCROP2ASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PCROP2ASR to value 0xffff_ffff
impl crate::Resettable for PCROP2ASR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
