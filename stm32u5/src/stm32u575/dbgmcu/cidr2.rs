///Register `CIDR2` reader
pub struct R(crate::R<CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PREAMBLE` reader - component identification bits \[23:16\]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - component identification bits \[23:16\]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
///Debug MCU CoreSight component identity register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cidr2](index.html) module
pub struct CIDR2_SPEC;
impl crate::RegisterSpec for CIDR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cidr2::R](R) reader structure
impl crate::Readable for CIDR2_SPEC {
    type Reader = R;
}
///`reset()` method sets CIDR2 to value 0x05
impl crate::Resettable for CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
