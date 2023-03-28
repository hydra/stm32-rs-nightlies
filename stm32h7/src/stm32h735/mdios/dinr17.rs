///Register `DINR17` reader
pub struct R(crate::R<DINR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR17_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN17` reader - Input data received from MDIO Master during write frames
pub type DIN17_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Input data received from MDIO Master during write frames
    #[inline(always)]
    pub fn din17(&self) -> DIN17_R {
        DIN17_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register 17
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dinr17](index.html) module
pub struct DINR17_SPEC;
impl crate::RegisterSpec for DINR17_SPEC {
    type Ux = u32;
}
///`read()` method returns [dinr17::R](R) reader structure
impl crate::Readable for DINR17_SPEC {
    type Reader = R;
}
///`reset()` method sets DINR17 to value 0
impl crate::Resettable for DINR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
