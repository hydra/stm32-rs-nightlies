///Register `DINR6` reader
pub struct R(crate::R<DINR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR6_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN6` reader - Input data received from MDIO Master during write frames
pub type DIN6_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din6(&self) -> DIN6_R {
        DIN6_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 6
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr6](index.html) module
pub struct DINR6_SPEC;
impl crate::RegisterSpec for DINR6_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr6::R](R) reader structure
impl crate::Readable for DINR6_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR6 to value 0
impl crate::Resettable for DINR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
