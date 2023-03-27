///Register `DAINT` reader
pub struct R(crate::R<DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINT_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IEPINT` reader - IEPINT
pub type IEPINT_R = crate::FieldReader<u16, u16>;
///Field `OEPINT` reader - OEPINT
pub type OEPINT_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - IEPINT
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - OEPINT
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///When a significant event occurs on an endpoint, a DAINT register interrupts the application using the device OUT endpoints interrupt bit or device IN endpoints interrupt bit of the GINTSTS register (OEPINT or IEPINT in GINTSTS, respectively). There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding device endpoint-x interrupt register (DIEPINTx/DOEPINTx).
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daint](index.html) module
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [daint::R](R) reader structure
impl crate::Readable for DAINT_SPEC {
    type Reader = R;
}
///`reset()` method sets DAINT to value 0
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
