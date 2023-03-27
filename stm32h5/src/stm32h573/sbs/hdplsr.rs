///Register `HDPLSR` reader
pub struct R(crate::R<HDPLSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDPLSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDPLSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDPLSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HDPL` reader - temporal isolation level This bitfield returns the current temporal isolation level.
pub type HDPL_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - temporal isolation level This bitfield returns the current temporal isolation level.
    #[inline(always)]
    pub fn hdpl(&self) -> HDPL_R {
        HDPL_R::new((self.bits & 0xff) as u8)
    }
}
///SBS temporal isolation status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdplsr](index.html) module
pub struct HDPLSR_SPEC;
impl crate::RegisterSpec for HDPLSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdplsr::R](R) reader structure
impl crate::Readable for HDPLSR_SPEC {
    type Reader = R;
}
///`reset()` method sets HDPLSR to value 0
impl crate::Resettable for HDPLSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
