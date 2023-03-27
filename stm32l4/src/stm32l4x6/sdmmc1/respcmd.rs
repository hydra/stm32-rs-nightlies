///Register `RESPCMD` reader
pub struct R(crate::R<RESPCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESPCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESPCMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESPCMD_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RESPCMD` reader - Response command index
pub type RESPCMD_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:5 - Response command index
    #[inline(always)]
    pub fn respcmd(&self) -> RESPCMD_R {
        RESPCMD_R::new((self.bits & 0x3f) as u8)
    }
}
///command response register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [respcmd](index.html) module
pub struct RESPCMD_SPEC;
impl crate::RegisterSpec for RESPCMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [respcmd::R](R) reader structure
impl crate::Readable for RESPCMD_SPEC {
    type Reader = R;
}
///`reset()` method sets RESPCMD to value 0
impl crate::Resettable for RESPCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
