///Register `ETH_MACL3A30` reader
pub struct R(crate::R<ETH_MACL3A30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL3A30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL3A30_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL3A30_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACL3A30` writer
pub struct W(crate::W<ETH_MACL3A30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL3A30_SPEC>;
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
impl From<crate::W<ETH_MACL3A30_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL3A30_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L3A30` reader - L3A30
pub type L3A30_R = crate::FieldReader<u32, u32>;
///Field `L3A30` writer - L3A30
pub type L3A30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL3A30_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - L3A30
    #[inline(always)]
    pub fn l3a30(&self) -> L3A30_R {
        L3A30_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - L3A30
    #[inline(always)]
    #[must_use]
    pub fn l3a30(&mut self) -> L3A30_W<0> {
        L3A30_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The Layer 3 Address 3 Register 0 register is reserved for IPv4 packets. For IPv6 packets, it contains Bits\[127:96\]
///of 128-bit IP Source Address or Destination Address field.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macl3a30](index.html) module
pub struct ETH_MACL3A30_SPEC;
impl crate::RegisterSpec for ETH_MACL3A30_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macl3a30::R](R) reader structure
impl crate::Readable for ETH_MACL3A30_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macl3a30::W](W) writer structure
impl crate::Writable for ETH_MACL3A30_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACL3A30 to value 0
impl crate::Resettable for ETH_MACL3A30_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
