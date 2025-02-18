///Register `OTG_HAINT` reader
pub struct R(crate::R<OTG_HAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HAINT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `HAINT` reader - HAINT
pub type HAINT_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - HAINT
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xffff) as u16)
    }
}
///When a significant event occurs on a channel, the host all channels interrupt register interrupts the application using the host channels interrupt bit of the core interrupt register (HCINT bit in OTG_GINTSTS). This is shown in Figure724. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding host channel-x interrupt register.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otg_haint](index.html) module
pub struct OTG_HAINT_SPEC;
impl crate::RegisterSpec for OTG_HAINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [otg_haint::R](R) reader structure
impl crate::Readable for OTG_HAINT_SPEC {
    type Reader = R;
}
///`reset()` method sets OTG_HAINT to value 0
impl crate::Resettable for OTG_HAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
