///Register `MDIOS_RDFR` reader
pub struct R(crate::R<MDIOS_RDFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIOS_RDFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIOS_RDFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIOS_RDFR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDF` reader - RDF
pub type RDF_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - RDF
    #[inline(always)]
    pub fn rdf(&self) -> RDF_R {
        RDF_R::new(self.bits)
    }
}
///MDIOS read flag register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdios_rdfr](index.html) module
pub struct MDIOS_RDFR_SPEC;
impl crate::RegisterSpec for MDIOS_RDFR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdios_rdfr::R](R) reader structure
impl crate::Readable for MDIOS_RDFR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDIOS_RDFR to value 0
impl crate::Resettable for MDIOS_RDFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
