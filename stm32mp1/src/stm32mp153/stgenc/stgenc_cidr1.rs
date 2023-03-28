///Register `STGENC_CIDR1` reader
pub struct R(crate::R<STGENC_CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PRMBL_1` reader - PRMBL_1
pub type PRMBL_1_R = crate::FieldReader<u8, u8>;
///Field `CLASS` reader - CLASS
pub type CLASS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - PRMBL_1
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CLASS
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///STGENC component ID1 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenc_cidr1](index.html) module
pub struct STGENC_CIDR1_SPEC;
impl crate::RegisterSpec for STGENC_CIDR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenc_cidr1::R](R) reader structure
impl crate::Readable for STGENC_CIDR1_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENC_CIDR1 to value 0xf0
impl crate::Resettable for STGENC_CIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
