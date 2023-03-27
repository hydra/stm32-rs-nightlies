///Register `MACFCR` reader
pub struct R(crate::R<MACFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACFCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACFCR` writer
pub struct W(crate::W<MACFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACFCR_SPEC>;
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
impl From<crate::W<MACFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACFCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FCB` reader - Flow control busy/back pressure activate
pub type FCB_R = crate::BitReader<FCB_A>;
///Flow control busy/back pressure activate
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCB_A {
    ///0: In half duplex only, deasserts back pressure
    DisableBackPressure = 0,
    ///1: In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    PauseOrBackPressure = 1,
}
impl From<FCB_A> for bool {
    #[inline(always)]
    fn from(variant: FCB_A) -> Self {
        variant as u8 != 0
    }
}
impl FCB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FCB_A {
        match self.bits {
            false => FCB_A::DisableBackPressure,
            true => FCB_A::PauseOrBackPressure,
        }
    }
    ///Checks if the value of the field is `DisableBackPressure`
    #[inline(always)]
    pub fn is_disable_back_pressure(&self) -> bool {
        *self == FCB_A::DisableBackPressure
    }
    ///Checks if the value of the field is `PauseOrBackPressure`
    #[inline(always)]
    pub fn is_pause_or_back_pressure(&self) -> bool {
        *self == FCB_A::PauseOrBackPressure
    }
}
///Field `FCB` writer - Flow control busy/back pressure activate
pub type FCB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, FCB_A, O>;
impl<'a, const O: u8> FCB_W<'a, O> {
    ///In half duplex only, deasserts back pressure
    #[inline(always)]
    pub fn disable_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::DisableBackPressure)
    }
    ///In full duplex, initiate a Pause control frame. In half duplex, assert back pressure
    #[inline(always)]
    pub fn pause_or_back_pressure(self) -> &'a mut W {
        self.variant(FCB_A::PauseOrBackPressure)
    }
}
///Field `TFCE` reader - Transmit flow control enable
pub type TFCE_R = crate::BitReader<TFCE_A>;
///Transmit flow control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFCE_A {
    ///0: In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    Disabled = 0,
    ///1: In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    Enabled = 1,
}
impl From<TFCE_A> for bool {
    #[inline(always)]
    fn from(variant: TFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TFCE_A {
        match self.bits {
            false => TFCE_A::Disabled,
            true => TFCE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFCE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFCE_A::Enabled
    }
}
///Field `TFCE` writer - Transmit flow control enable
pub type TFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, TFCE_A, O>;
impl<'a, const O: u8> TFCE_W<'a, O> {
    ///In full duplex, flow control is disabled. In half duplex, back pressure is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TFCE_A::Disabled)
    }
    ///In full duplex, flow control is enabled. In half duplex, back pressure is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TFCE_A::Enabled)
    }
}
///Field `RFCE` reader - Receive flow control enable
pub type RFCE_R = crate::BitReader<RFCE_A>;
///Receive flow control enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFCE_A {
    ///0: Pause frames are not decoded
    Disabled = 0,
    ///1: MAC decodes received Pause frames and disables its transmitted for a specified time
    Enabled = 1,
}
impl From<RFCE_A> for bool {
    #[inline(always)]
    fn from(variant: RFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RFCE_A {
        match self.bits {
            false => RFCE_A::Disabled,
            true => RFCE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RFCE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RFCE_A::Enabled
    }
}
///Field `RFCE` writer - Receive flow control enable
pub type RFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, RFCE_A, O>;
impl<'a, const O: u8> RFCE_W<'a, O> {
    ///Pause frames are not decoded
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RFCE_A::Disabled)
    }
    ///MAC decodes received Pause frames and disables its transmitted for a specified time
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RFCE_A::Enabled)
    }
}
///Field `UPFD` reader - Unicast pause frame detect
pub type UPFD_R = crate::BitReader<UPFD_A>;
///Unicast pause frame detect
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPFD_A {
    ///0: MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    Disabled = 0,
    ///1: MAC additionally detects Pause frames with the station's unicast address
    Enabled = 1,
}
impl From<UPFD_A> for bool {
    #[inline(always)]
    fn from(variant: UPFD_A) -> Self {
        variant as u8 != 0
    }
}
impl UPFD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPFD_A {
        match self.bits {
            false => UPFD_A::Disabled,
            true => UPFD_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPFD_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPFD_A::Enabled
    }
}
///Field `UPFD` writer - Unicast pause frame detect
pub type UPFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, UPFD_A, O>;
impl<'a, const O: u8> UPFD_W<'a, O> {
    ///MAC detects only a Pause frame with the multicast address specified in the 802.3x standard
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPFD_A::Disabled)
    }
    ///MAC additionally detects Pause frames with the station's unicast address
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPFD_A::Enabled)
    }
}
///Field `PLT` reader - Pause low threshold
pub type PLT_R = crate::FieldReader<u8, PLT_A>;
///Pause low threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLT_A {
    ///0: Pause time minus 4 slot times
    Plt4 = 0,
    ///1: Pause time minus 28 slot times
    Plt28 = 1,
    ///2: Pause time minus 144 slot times
    Plt144 = 2,
    ///3: Pause time minus 256 slot times
    Plt256 = 3,
}
impl From<PLT_A> for u8 {
    #[inline(always)]
    fn from(variant: PLT_A) -> Self {
        variant as _
    }
}
impl PLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PLT_A {
        match self.bits {
            0 => PLT_A::Plt4,
            1 => PLT_A::Plt28,
            2 => PLT_A::Plt144,
            3 => PLT_A::Plt256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Plt4`
    #[inline(always)]
    pub fn is_plt4(&self) -> bool {
        *self == PLT_A::Plt4
    }
    ///Checks if the value of the field is `Plt28`
    #[inline(always)]
    pub fn is_plt28(&self) -> bool {
        *self == PLT_A::Plt28
    }
    ///Checks if the value of the field is `Plt144`
    #[inline(always)]
    pub fn is_plt144(&self) -> bool {
        *self == PLT_A::Plt144
    }
    ///Checks if the value of the field is `Plt256`
    #[inline(always)]
    pub fn is_plt256(&self) -> bool {
        *self == PLT_A::Plt256
    }
}
///Field `PLT` writer - Pause low threshold
pub type PLT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACFCR_SPEC, u8, PLT_A, 2, O>;
impl<'a, const O: u8> PLT_W<'a, O> {
    ///Pause time minus 4 slot times
    #[inline(always)]
    pub fn plt4(self) -> &'a mut W {
        self.variant(PLT_A::Plt4)
    }
    ///Pause time minus 28 slot times
    #[inline(always)]
    pub fn plt28(self) -> &'a mut W {
        self.variant(PLT_A::Plt28)
    }
    ///Pause time minus 144 slot times
    #[inline(always)]
    pub fn plt144(self) -> &'a mut W {
        self.variant(PLT_A::Plt144)
    }
    ///Pause time minus 256 slot times
    #[inline(always)]
    pub fn plt256(self) -> &'a mut W {
        self.variant(PLT_A::Plt256)
    }
}
///Field `ZQPD` reader - Zero-quanta pause disable
pub type ZQPD_R = crate::BitReader<ZQPD_A>;
///Zero-quanta pause disable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZQPD_A {
    ///0: Normal operation with automatic zero-quanta pause control frame generation
    Enabled = 0,
    ///1: Automatic generation of zero-quanta pause control frames is disabled
    Disabled = 1,
}
impl From<ZQPD_A> for bool {
    #[inline(always)]
    fn from(variant: ZQPD_A) -> Self {
        variant as u8 != 0
    }
}
impl ZQPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ZQPD_A {
        match self.bits {
            false => ZQPD_A::Enabled,
            true => ZQPD_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ZQPD_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ZQPD_A::Disabled
    }
}
///Field `ZQPD` writer - Zero-quanta pause disable
pub type ZQPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACFCR_SPEC, ZQPD_A, O>;
impl<'a, const O: u8> ZQPD_W<'a, O> {
    ///Normal operation with automatic zero-quanta pause control frame generation
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ZQPD_A::Enabled)
    }
    ///Automatic generation of zero-quanta pause control frames is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ZQPD_A::Disabled)
    }
}
///Field `PT` reader - Pause time
pub type PT_R = crate::FieldReader<u16, u16>;
///Field `PT` writer - Pause time
pub type PT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MACFCR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    pub fn tfce(&self) -> TFCE_R {
        TFCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    pub fn rfce(&self) -> RFCE_R {
        RFCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    pub fn upfd(&self) -> UPFD_R {
        UPFD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    pub fn zqpd(&self) -> ZQPD_R {
        ZQPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bit 0 - Flow control busy/back pressure activate
    #[inline(always)]
    #[must_use]
    pub fn fcb(&mut self) -> FCB_W<0> {
        FCB_W::new(self)
    }
    ///Bit 1 - Transmit flow control enable
    #[inline(always)]
    #[must_use]
    pub fn tfce(&mut self) -> TFCE_W<1> {
        TFCE_W::new(self)
    }
    ///Bit 2 - Receive flow control enable
    #[inline(always)]
    #[must_use]
    pub fn rfce(&mut self) -> RFCE_W<2> {
        RFCE_W::new(self)
    }
    ///Bit 3 - Unicast pause frame detect
    #[inline(always)]
    #[must_use]
    pub fn upfd(&mut self) -> UPFD_W<3> {
        UPFD_W::new(self)
    }
    ///Bits 4:5 - Pause low threshold
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<4> {
        PLT_W::new(self)
    }
    ///Bit 7 - Zero-quanta pause disable
    #[inline(always)]
    #[must_use]
    pub fn zqpd(&mut self) -> ZQPD_W<7> {
        ZQPD_W::new(self)
    }
    ///Bits 16:31 - Pause time
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<16> {
        PT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Ethernet MAC flow control register (ETH_MACFCR)
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [macfcr](index.html) module
pub struct MACFCR_SPEC;
impl crate::RegisterSpec for MACFCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [macfcr::R](R) reader structure
impl crate::Readable for MACFCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [macfcr::W](W) writer structure
impl crate::Writable for MACFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACFCR to value 0
impl crate::Resettable for MACFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
