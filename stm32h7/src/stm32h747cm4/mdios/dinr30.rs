///Register `DINR30` reader
pub struct R(crate::R<DINR30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR30_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN30` reader - Input data received from MDIO Master during write frames
pub type DIN30_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din30(&self) -> DIN30_R {
        DIN30_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 30
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr30](index.html) module
pub struct DINR30_SPEC;
impl crate::RegisterSpec for DINR30_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr30::R](R) reader structure
impl crate::Readable for DINR30_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR30 to value 0
impl crate::Resettable for DINR30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
