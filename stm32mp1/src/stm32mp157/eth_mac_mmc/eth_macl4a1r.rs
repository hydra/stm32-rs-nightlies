///Register `ETH_MACL4A1R` reader
pub struct R(crate::R<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_MACL4A1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_MACL4A1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_MACL4A1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ETH_MACL4A1R` writer
pub struct W(crate::W<ETH_MACL4A1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_MACL4A1R_SPEC>;
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
impl From<crate::W<ETH_MACL4A1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_MACL4A1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L4SP1` reader - L4SP1
pub type L4SP1_R = crate::FieldReader<u16, u16>;
///Field `L4SP1` writer - L4SP1
pub type L4SP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A1R_SPEC, u16, u16, 16, O>;
///Field `L4DP1` reader - L4DP1
pub type L4DP1_R = crate::FieldReader<u16, u16>;
///Field `L4DP1` writer - L4DP1
pub type L4DP1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_MACL4A1R_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - L4SP1
    #[inline(always)]
    pub fn l4sp1(&self) -> L4SP1_R {
        L4SP1_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - L4DP1
    #[inline(always)]
    pub fn l4dp1(&self) -> L4DP1_R {
        L4DP1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - L4SP1
    #[inline(always)]
    #[must_use]
    pub fn l4sp1(&mut self) -> L4SP1_W<0> {
        L4SP1_W::new(self)
    }
    ///Bits 16:31 - L4DP1
    #[inline(always)]
    #[must_use]
    pub fn l4dp1(&mut self) -> L4DP1_W<16> {
        L4DP1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///The Layer 4 Address 0 register and registers 580 through 667 are reserved (RO with default value) if Enable Layer 3 and Layer 4 Packet Filter option is not selected while configuring the core. You can configure the Layer 3 and Layer 4 Address Registers to be double-synchronized by selecting the Synchronize Layer 3 and Layer 4 Address Registers to Rx Clock Domain option while configuring the core. When you select this option, the synchronization is triggered only when Bits\[31:24\]
///(in little-endian mode) or Bits\[7:0\]
///(in big-endian mode) of the Layer 3 and Layer 4 Address Registers are written. For proper synchronization updates, you should perform consecutive writes to same Layer 3 and Layer 4 Address Registers after at least four clock cycles delay of the destination clock.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eth_macl4a1r](index.html) module
pub struct ETH_MACL4A1R_SPEC;
impl crate::RegisterSpec for ETH_MACL4A1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [eth_macl4a1r::R](R) reader structure
impl crate::Readable for ETH_MACL4A1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eth_macl4a1r::W](W) writer structure
impl crate::Writable for ETH_MACL4A1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ETH_MACL4A1R to value 0
impl crate::Resettable for ETH_MACL4A1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
