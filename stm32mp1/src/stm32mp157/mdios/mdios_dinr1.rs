///Register `MDIOS_DINR1` reader
pub struct R(crate::R<MDIOS_DINR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DINR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DINR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DINR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DIN` reader - DIN
pub type DIN_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - DIN
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS input data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdios_dinr1](index.html) module
pub struct MDIOS_DINR1_SPEC;
impl crate::RegisterSpec for MDIOS_DINR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_dinr1::R](R) reader structure
impl crate::Readable for MDIOS_DINR1_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_DINR1 to value 0
impl crate::Resettable for MDIOS_DINR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
