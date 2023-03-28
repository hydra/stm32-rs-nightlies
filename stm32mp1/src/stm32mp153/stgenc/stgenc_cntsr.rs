///Register `STGENC_CNTSR` reader
pub struct R(crate::R<STGENC_CNTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CNTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CNTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CNTSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EN` reader - EN
pub type EN_R = crate::BitReader<bool>;
///Field `HLTDBG` reader - HLTDBG
pub type HLTDBG_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HLTDBG
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
///STGENC status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenc_cntsr](index.html) module
pub struct STGENC_CNTSR_SPEC;
impl crate::RegisterSpec for STGENC_CNTSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenc_cntsr::R](R) reader structure
impl crate::Readable for STGENC_CNTSR_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENC_CNTSR to value 0
impl crate::Resettable for STGENC_CNTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
