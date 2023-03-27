///Register `MACVIR` reader
pub struct R(crate::R<MACVIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACVIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACVIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACVIR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACVIR` writer
pub struct W(crate::W<MACVIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACVIR_SPEC>;
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
impl From<crate::W<MACVIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACVIR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `VLT` reader - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
pub type VLT_R = crate::FieldReader<u16, u16>;
///Field `VLT` writer - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
pub type VLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVIR_SPEC, u16, u16, 16, O>;
///Field `VLC` reader - VLAN Tag Control in Transmit Packets Note: Changes to this field take effect only on the start of a packet. If you write this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
pub type VLC_R = crate::FieldReader<u8, u8>;
///Field `VLC` writer - VLAN Tag Control in Transmit Packets Note: Changes to this field take effect only on the start of a packet. If you write this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
pub type VLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACVIR_SPEC, u8, u8, 2, O>;
///Field `VLP` reader - VLAN Priority Control When this bit is set, the control bits\[17:16\]
///are used for VLAN deletion, insertion, or replacement. When this bit is reset, bits\[17:16\]
///are ignored.
pub type VLP_R = crate::BitReader<bool>;
///Field `VLP` writer - VLAN Priority Control When this bit is set, the control bits\[17:16\]
///are used for VLAN deletion, insertion, or replacement. When this bit is reset, bits\[17:16\]
///are ignored.
pub type VLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVIR_SPEC, bool, O>;
///Field `CSVL` reader - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
pub type CSVL_R = crate::BitReader<bool>;
///Field `CSVL` writer - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
pub type CSVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVIR_SPEC, bool, O>;
///Field `VLTI` reader - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor.
pub type VLTI_R = crate::BitReader<bool>;
///Field `VLTI` writer - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor.
pub type VLTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACVIR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
    #[inline(always)]
    pub fn vlt(&self) -> VLT_R {
        VLT_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - VLAN Tag Control in Transmit Packets Note: Changes to this field take effect only on the start of a packet. If you write this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
    #[inline(always)]
    pub fn vlc(&self) -> VLC_R {
        VLC_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - VLAN Priority Control When this bit is set, the control bits\[17:16\]
    ///are used for VLAN deletion, insertion, or replacement. When this bit is reset, bits\[17:16\]
    ///are ignored.
    #[inline(always)]
    pub fn vlp(&self) -> VLP_R {
        VLP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
    #[inline(always)]
    pub fn csvl(&self) -> CSVL_R {
        CSVL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor.
    #[inline(always)]
    pub fn vlti(&self) -> VLTI_R {
        VLTI_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - VLAN Tag for Transmit Packets This field contains the value of the VLAN tag to be inserted or replaced. The value must only be changed when the transmit lines are inactive or during the initialization phase. The following list describes the bits of this field: Bits\[15:13\]: User Priority Bit 12: Canonical Format Indicator (CFI) or Drop Eligible Indicator (DEI) Bits\[11:0\]: VLAN Identifier (VID) field of VLAN tag
    #[inline(always)]
    #[must_use]
    pub fn vlt(&mut self) -> VLT_W<0> {
        VLT_W::new(self)
    }
    ///Bits 16:17 - VLAN Tag Control in Transmit Packets Note: Changes to this field take effect only on the start of a packet. If you write this register field when a packet is being transmitted, only the subsequent packet can use the updated value, that is, the current packet does not use the updated value.
    #[inline(always)]
    #[must_use]
    pub fn vlc(&mut self) -> VLC_W<16> {
        VLC_W::new(self)
    }
    ///Bit 18 - VLAN Priority Control When this bit is set, the control bits\[17:16\]
    ///are used for VLAN deletion, insertion, or replacement. When this bit is reset, bits\[17:16\]
    ///are ignored.
    #[inline(always)]
    #[must_use]
    pub fn vlp(&mut self) -> VLP_W<18> {
        VLP_W::new(self)
    }
    ///Bit 19 - C-VLAN or S-VLAN When this bit is set, S-VLAN type (0x88A8) is inserted or replaced in the 13th and 14th bytes of transmitted packets. When this bit is reset, C-VLAN type (0x8100) is inserted or replaced in the 13th and 14th bytes of transmitted packets.
    #[inline(always)]
    #[must_use]
    pub fn csvl(&mut self) -> CSVL_W<19> {
        CSVL_W::new(self)
    }
    ///Bit 20 - VLAN Tag Input When this bit is set, it indicates that the VLAN tag to be inserted or replaced in Tx packet should be taken from the Tx descriptor.
    #[inline(always)]
    #[must_use]
    pub fn vlti(&mut self) -> VLTI_W<20> {
        VLTI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///VLAN inclusion register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macvir](index.html) module
pub struct MACVIR_SPEC;
impl crate::RegisterSpec for MACVIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macvir::R](R) reader structure
impl crate::Readable for MACVIR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macvir::W](W) writer structure
impl crate::Writable for MACVIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACVIR to value 0
impl crate::Resettable for MACVIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
