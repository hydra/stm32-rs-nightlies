///Register `DINR3` reader
pub struct R(crate::R<DINR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR3_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN3` reader - Input data received from MDIO Master during write frames
pub type DIN3_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din3(&self) -> DIN3_R {
        DIN3_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 3
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr3](index.html) module
pub struct DINR3_SPEC;
impl crate::RegisterSpec for DINR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr3::R](R) reader structure
impl crate::Readable for DINR3_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR3 to value 0
impl crate::Resettable for DINR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
