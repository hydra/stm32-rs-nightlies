///Register `DINR2` reader
pub struct R(crate::R<DINR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN2` reader - Input data received from MDIO Master during write frames
pub type DIN2_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din2(&self) -> DIN2_R {
        DIN2_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr2](index.html) module
pub struct DINR2_SPEC;
impl crate::RegisterSpec for DINR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr2::R](R) reader structure
impl crate::Readable for DINR2_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR2 to value 0
impl crate::Resettable for DINR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
