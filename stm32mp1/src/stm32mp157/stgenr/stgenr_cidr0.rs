///Register `STGENR_CIDR0` reader
pub struct R(crate::R<STGENR_CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_CIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PRMBL_0` reader - PRMBL_0
pub type PRMBL_0_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - PRMBL_0
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
///STGENR component ID0 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [stgenr_cidr0](index.html) module
pub struct STGENR_CIDR0_SPEC;
impl crate::RegisterSpec for STGENR_CIDR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [stgenr_cidr0::R](R) reader structure
impl crate::Readable for STGENR_CIDR0_SPEC {
    type Reader = R;
}
///`reset()` method sets STGENR_CIDR0 to value 0x0d
impl crate::Resettable for STGENR_CIDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0d;
}
