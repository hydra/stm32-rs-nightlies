///Register `RDR` reader
pub struct R(crate::R<RDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RDR` reader - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit.
pub type RDR_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:8 - Receive data value Contains the received data character. The RDR register provides the parallel interface between the input shift register and the internal bus (see ). When receiving with the parity enabled, the value read in the MSB bit is the received parity bit.
    #[inline(always)]
    pub fn rdr(&self) -> RDR_R {
        RDR_R::new((self.bits & 0x01ff) as u16)
    }
}
///USART receive data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rdr](index.html) module
pub struct RDR_SPEC;
impl crate::RegisterSpec for RDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rdr::R](R) reader structure
impl crate::Readable for RDR_SPEC {
    type Reader = R;
}
///`reset()` method sets RDR to value 0
impl crate::Resettable for RDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
