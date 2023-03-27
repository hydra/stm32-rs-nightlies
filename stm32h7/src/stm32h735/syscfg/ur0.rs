///Register `UR0` reader
pub struct R(crate::R<UR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR0_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDP` reader - Readout protection
pub type RDP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 16:23 - Readout protection
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
///SYSCFG user register 0
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ur0](index.html) module
pub struct UR0_SPEC;
impl crate::RegisterSpec for UR0_SPEC {
    type Ux = u32;
}
///`read()` method returns [ur0::R](R) reader structure
impl crate::Readable for UR0_SPEC {
    type Reader = R;
}
///`reset()` method sets UR0 to value 0
impl crate::Resettable for UR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
