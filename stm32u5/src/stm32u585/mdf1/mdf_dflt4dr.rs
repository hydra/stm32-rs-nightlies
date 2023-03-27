///Register `MDF_DFLT4DR` reader
pub struct R(crate::R<MDF_DFLT4DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT4DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT4DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT4DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DR` reader - Data processed by digital filter.
pub type DR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 8:31 - Data processed by digital filter.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
///This register is used to read the data processed by each digital filter.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt4dr](index.html) module
pub struct MDF_DFLT4DR_SPEC;
impl crate::RegisterSpec for MDF_DFLT4DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt4dr::R](R) reader structure
impl crate::Readable for MDF_DFLT4DR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDF_DFLT4DR to value 0
impl crate::Resettable for MDF_DFLT4DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
