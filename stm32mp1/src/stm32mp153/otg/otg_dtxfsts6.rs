///Register `OTG_DTXFSTS6` reader
pub struct R(crate::R<OTG_DTXFSTS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DTXFSTS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DTXFSTS6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DTXFSTS6_SPEC>) -> Self {
        R(reader)
    }
}
///Field `INEPTFSAV` reader - INEPTFSAV
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - INEPTFSAV
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
///This read-only register contains the free space information for the device IN endpoint Tx FIFO.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_dtxfsts6](index.html) module
pub struct OTG_DTXFSTS6_SPEC;
impl crate::RegisterSpec for OTG_DTXFSTS6_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_dtxfsts6::R](R) reader structure
impl crate::Readable for OTG_DTXFSTS6_SPEC {
    type Reader = R;
}
///`reset()` method sets OTG_DTXFSTS6 to value 0x0200
impl crate::Resettable for OTG_DTXFSTS6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
