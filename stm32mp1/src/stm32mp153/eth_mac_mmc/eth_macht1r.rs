///Register `ETH_MACHT1R` reader
pub struct R(crate::R<ETH_MACHT1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACHT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACHT1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACHT1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACHT1R` writer
pub struct W(crate::W<ETH_MACHT1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACHT1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ETH_MACHT1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACHT1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `HT63T32` reader - HT63T32
pub type HT63T32_R = crate::FieldReader<u32, u32>;
///Field `HT63T32` writer - HT63T32
pub type HT63T32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_MACHT1R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - HT63T32
    #[inline(always)]
    pub fn ht63t32(&self) -> HT63T32_R {
        HT63T32_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - HT63T32
    #[inline(always)]
    #[must_use]
    pub fn ht63t32(&mut self) -> HT63T32_W<0> {
        HT63T32_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The Hash Table Register 1contains the last 32 bits of the Hash table (64 bits). For Hash filtering, the content of the destination address in the incoming packet is passed through the CRC logic and the upper six bits of the CRC register are used to index the content of the Hash table. The most significant bits determines the register to be used (Hash Table Register 0 or 1). The Hash value of the destination address is calculated in the following way: Calculate the 32-bit CRC for the DA (See IEEE 802.3, Section 3.2.8 for the steps to calculate CRC32). Perform bitwise reversal for the value obtained in Step 1. Take the upper 7 or 8 bits from the value obtained in Step 2. If the corresponding bit value of the register is 1, the packet is accepted. Otherwise, it is rejected. If the PM bit is set in ETH_MACPFR, all multicast packets are accepted regardless of the multicast Hash values. If the Hash Table register is configured to be double-synchronized to the GMII clock domain, the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the Hash Table Register X registers are written.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macht1r](index.html) module
pub struct ETH_MACHT1R_SPEC;
impl crate::RegisterSpec for ETH_MACHT1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macht1r::R](R) reader structure
impl crate::Readable for ETH_MACHT1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macht1r::W](W) writer structure
impl crate::Writable for ETH_MACHT1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACHT1R to value 0
impl crate::Resettable for ETH_MACHT1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
