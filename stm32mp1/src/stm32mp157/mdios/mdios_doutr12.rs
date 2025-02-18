///Register `MDIOS_DOUTR12` reader
pub struct R(crate::R<MDIOS_DOUTR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DOUTR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DOUTR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DOUTR12_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DOUT` reader - DOUT
pub type DOUT_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - DOUT
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new((self.bits & 0xffff) as u16)
    }
}
///MDIOS output data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdios_doutr12](index.html) module
pub struct MDIOS_DOUTR12_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR12_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_doutr12::R](R) reader structure
impl crate::Readable for MDIOS_DOUTR12_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_DOUTR12 to value 0
impl crate::Resettable for MDIOS_DOUTR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
