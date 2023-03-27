///Register `MDIOS_DOUTR23` reader
pub struct R(crate::R<MDIOS_DOUTR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_DOUTR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_DOUTR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_DOUTR23_SPEC>) -> Self {
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
///For information about available fields see [mdios_doutr23](index.html) module
pub struct MDIOS_DOUTR23_SPEC;
impl crate::RegisterSpec for MDIOS_DOUTR23_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_doutr23::R](R) reader structure
impl crate::Readable for MDIOS_DOUTR23_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_DOUTR23 to value 0
impl crate::Resettable for MDIOS_DOUTR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
