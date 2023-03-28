///Register `MACL4A0R` reader
pub struct R(crate::R<MACL4A0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACL4A0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACL4A0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACL4A0R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACL4A0R` writer
pub struct W(crate::W<MACL4A0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACL4A0R_SPEC>;
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
impl From<crate::W<MACL4A0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACL4A0R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `L4SP0` reader - Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets.
pub type L4SP0_R = crate::FieldReader<u16, u16>;
///Field `L4SP0` writer - Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets.
pub type L4SP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL4A0R_SPEC, u16, u16, 16, O>;
///Field `L4DP0` reader - Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets.
pub type L4DP0_R = crate::FieldReader<u16, u16>;
///Field `L4DP0` writer - Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets.
pub type L4DP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACL4A0R_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets.
    #[inline(always)]
    pub fn l4sp0(&self) -> L4SP0_R {
        L4SP0_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets.
    #[inline(always)]
    pub fn l4dp0(&self) -> L4DP0_R {
        L4DP0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Layer 4 Source Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Source Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Source Port Number field in the IPv4 or IPv6 packets.
    #[inline(always)]
    #[must_use]
    pub fn l4sp0(&mut self) -> L4SP0_W<0> {
        L4SP0_W::new(self)
    }
    ///Bits 16:31 - Layer 4 Destination Port Number Field When the L4PEN0 bit is reset and the L4DPM0 bit is set in the L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the TCP Destination Port Number field in the IPv4 or IPv6 packets. When the L4PEN0 and L4DPM0 bits are set in L3 and L4 control 0 register (ETH_MACL3L4C0R), this field contains the value to be matched with the UDP Destination Port Number field in the IPv4 or IPv6 packets.
    #[inline(always)]
    #[must_use]
    pub fn l4dp0(&mut self) -> L4DP0_W<16> {
        L4DP0_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Layer4 Address filter 0 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macl4a0r](index.html) module
pub struct MACL4A0R_SPEC;
impl crate::RegisterSpec for MACL4A0R_SPEC {
    type Ux = u32;
}
///`read()` method returns [macl4a0r::R](R) reader structure
impl crate::Readable for MACL4A0R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macl4a0r::W](W) writer structure
impl crate::Writable for MACL4A0R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACL4A0R to value 0
impl crate::Resettable for MACL4A0R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
