///Register `DTXFSTS3` reader
pub struct R(crate::R<DTXFSTS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTXFSTS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTXFSTS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTXFSTS3_SPEC>) -> Self {
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
///For information about available fields see [dtxfsts3](index.html) module
pub struct DTXFSTS3_SPEC;
impl crate::RegisterSpec for DTXFSTS3_SPEC {
    type Ux = u32;
}
///`read()` method returns [dtxfsts3::R](R) reader structure
impl crate::Readable for DTXFSTS3_SPEC {
    type Reader = R;
}
///`reset()` method sets DTXFSTS3 to value 0x0200
impl crate::Resettable for DTXFSTS3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
