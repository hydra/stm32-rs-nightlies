///Register `MACL3A11R` reader
pub struct R(crate::R<MACL3A11R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL3A11R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL3A11R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL3A11R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACL3A11R` writer
pub struct W(crate::W<MACL3A11R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL3A11R_SPEC>;
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
impl From<crate::W<MACL3A11R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL3A11R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L3A11` reader - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
///of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
///of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
pub type L3A11_R = crate::FieldReader<u32, u32>;
///Field `L3A11` writer - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
///of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
///of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
pub type L3A11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL3A11R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
    ///of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
    ///of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
    #[inline(always)]
    pub fn l3a11(&self) -> L3A11_R {
        L3A11_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Layer 3 Address 1 Field When the L3PEN1 and L3SAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
    ///of the IP Source Address field in the IPv6 packets. When the L3PEN1 and L3DAM1 bits are set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with Bits\[63:32\]
    ///of the IP Destination Address field in the IPv6 packets. When the L3PEN1 bit is reset and the L3SAM1 bit is set in the L3 and L4 control 1 register (ETH_MACL3L4C1R), this field contains the value to be matched with the IP Destination Address field in the IPv4 packets.
    #[inline(always)]
    #[must_use]
    pub fn l3a11(&mut self) -> L3A11_W<0> {
        L3A11_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Layer3 address 1 filter 1 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macl3a11r](index.html) module
pub struct MACL3A11R_SPEC;
impl crate::RegisterSpec for MACL3A11R_SPEC {
    type Ux = u32;
}
///`read()` method returns [macl3a11r::R](R) reader structure
impl crate::Readable for MACL3A11R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macl3a11r::W](W) writer structure
impl crate::Writable for MACL3A11R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACL3A11R to value 0
impl crate::Resettable for MACL3A11R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
