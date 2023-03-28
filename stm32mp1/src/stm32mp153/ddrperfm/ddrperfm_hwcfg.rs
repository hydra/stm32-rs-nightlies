///Register `DDRPERFM_HWCFG` reader
pub struct R(crate::R<DDRPERFM_HWCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPERFM_HWCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPERFM_HWCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPERFM_HWCFG_SPEC>) -> Self {
        R(reader)
    }
}
///Field `NCNT` reader - NCNT
pub type NCNT_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - NCNT
    #[inline(always)]
    pub fn ncnt(&self) -> NCNT_R {
        NCNT_R::new((self.bits & 0x0f) as u8)
    }
}
///DDRPERFM hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrperfm_hwcfg](index.html) module
pub struct DDRPERFM_HWCFG_SPEC;
impl crate::RegisterSpec for DDRPERFM_HWCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrperfm_hwcfg::R](R) reader structure
impl crate::Readable for DDRPERFM_HWCFG_SPEC {
    type Reader = R;
}
///`reset()` method sets DDRPERFM_HWCFG to value 0x04
impl crate::Resettable for DDRPERFM_HWCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
