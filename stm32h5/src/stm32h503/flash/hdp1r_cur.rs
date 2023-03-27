///Register `HDP1R_CUR` reader
pub struct R(crate::R<HDP1R_CUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HDP1R_CUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HDP1R_CUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HDP1R_CUR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HDP1_STRT` reader - HDPL barrier start set in number of 8 Kbytes sectors
pub type HDP1_STRT_R = crate::FieldReader<u8, u8>;
///Field `HDP1_END` reader - HDPL barrier end set in number of 8 Kbytes sectors
pub type HDP1_END_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:2 - HDPL barrier start set in number of 8 Kbytes sectors
    #[inline(always)]
    pub fn hdp1_strt(&self) -> HDP1_STRT_R {
        HDP1_STRT_R::new((self.bits & 7) as u8)
    }
    ///Bits 16:18 - HDPL barrier end set in number of 8 Kbytes sectors
    #[inline(always)]
    pub fn hdp1_end(&self) -> HDP1_END_R {
        HDP1_END_R::new(((self.bits >> 16) & 7) as u8)
    }
}
///FLASH HDP Bank1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hdp1r_cur](index.html) module
pub struct HDP1R_CUR_SPEC;
impl crate::RegisterSpec for HDP1R_CUR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hdp1r_cur::R](R) reader structure
impl crate::Readable for HDP1R_CUR_SPEC {
    type Reader = R;
}
///`reset()` method sets HDP1R_CUR to value 0
impl crate::Resettable for HDP1R_CUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
