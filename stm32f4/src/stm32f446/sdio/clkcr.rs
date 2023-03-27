///Register `CLKCR` reader
pub struct R(crate::R<CLKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CLKCR` writer
pub struct W(crate::W<CLKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCR_SPEC>;
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
impl From<crate::W<CLKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CLKDIV` reader - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \[CLKDIV + 2\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
///Field `CLKDIV` writer - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \[CLKDIV + 2\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLKCR_SPEC, u8, u8, 8, O>;
///Field `CLKEN` reader - Clock enable bit
pub type CLKEN_R = crate::BitReader<CLKEN_A>;
///Clock enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKEN_A {
    ///0: Disable clock
    Disabled = 0,
    ///1: Enable clock
    Enabled = 1,
}
impl From<CLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CLKEN_A {
        match self.bits {
            false => CLKEN_A::Disabled,
            true => CLKEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CLKEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CLKEN_A::Enabled
    }
}
///Field `CLKEN` writer - Clock enable bit
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, CLKEN_A, O>;
impl<'a, const O: u8> CLKEN_W<'a, O> {
    ///Disable clock
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Disabled)
    }
    ///Enable clock
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CLKEN_A::Enabled)
    }
}
///Field `PWRSAV` reader - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
pub type PWRSAV_R = crate::BitReader<PWRSAV_A>;
///Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWRSAV_A {
    ///0: SDIO_CK clock is always enabled
    Enabled = 0,
    ///1: SDIO_CK is only enabled when the bus is active
    Disabled = 1,
}
impl From<PWRSAV_A> for bool {
    #[inline(always)]
    fn from(variant: PWRSAV_A) -> Self {
        variant as u8 != 0
    }
}
impl PWRSAV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PWRSAV_A {
        match self.bits {
            false => PWRSAV_A::Enabled,
            true => PWRSAV_A::Disabled,
        }
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWRSAV_A::Enabled
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWRSAV_A::Disabled
    }
}
///Field `PWRSAV` writer - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
pub type PWRSAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, PWRSAV_A, O>;
impl<'a, const O: u8> PWRSAV_W<'a, O> {
    ///SDIO_CK clock is always enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::Enabled)
    }
    ///SDIO_CK is only enabled when the bus is active
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PWRSAV_A::Disabled)
    }
}
///Field `BYPASS` reader - Clock divider bypass enable bit
pub type BYPASS_R = crate::BitReader<BYPASS_A>;
///Clock divider bypass enable bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_A {
    ///0: SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
    Disabled = 0,
    ///1: SDIOCLK directly drives the SDIO_CK output signal
    Enabled = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::Disabled,
            true => BYPASS_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::Enabled
    }
}
///Field `BYPASS` writer - Clock divider bypass enable bit
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, BYPASS_A, O>;
impl<'a, const O: u8> BYPASS_W<'a, O> {
    ///SDIOCLK is divided according to the CLKDIV value before driving the SDIO_CK output signal.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::Disabled)
    }
    ///SDIOCLK directly drives the SDIO_CK output signal
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::Enabled)
    }
}
///Field `WIDBUS` reader - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type WIDBUS_R = crate::FieldReader<u8, WIDBUS_A>;
///Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WIDBUS_A {
    ///0: 1 lane wide bus
    BusWidth1 = 0,
    ///1: 4 lane wide bus
    BusWidth4 = 1,
    ///2: 8 lane wide bus
    BusWidth8 = 2,
}
impl From<WIDBUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WIDBUS_A) -> Self {
        variant as _
    }
}
impl WIDBUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<WIDBUS_A> {
        match self.bits {
            0 => Some(WIDBUS_A::BusWidth1),
            1 => Some(WIDBUS_A::BusWidth4),
            2 => Some(WIDBUS_A::BusWidth8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `BusWidth1`
    #[inline(always)]
    pub fn is_bus_width1(&self) -> bool {
        *self == WIDBUS_A::BusWidth1
    }
    ///Checks if the value of the field is `BusWidth4`
    #[inline(always)]
    pub fn is_bus_width4(&self) -> bool {
        *self == WIDBUS_A::BusWidth4
    }
    ///Checks if the value of the field is `BusWidth8`
    #[inline(always)]
    pub fn is_bus_width8(&self) -> bool {
        *self == WIDBUS_A::BusWidth8
    }
}
///Field `WIDBUS` writer - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
pub type WIDBUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKCR_SPEC, u8, WIDBUS_A, 2, O>;
impl<'a, const O: u8> WIDBUS_W<'a, O> {
    ///1 lane wide bus
    #[inline(always)]
    pub fn bus_width1(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth1)
    }
    ///4 lane wide bus
    #[inline(always)]
    pub fn bus_width4(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth4)
    }
    ///8 lane wide bus
    #[inline(always)]
    pub fn bus_width8(self) -> &'a mut W {
        self.variant(WIDBUS_A::BusWidth8)
    }
}
///Field `NEGEDGE` reader - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
pub type NEGEDGE_R = crate::BitReader<NEGEDGE_A>;
///SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEGEDGE_A {
    ///0: SDIO_CK generated on the rising edge
    Rising = 0,
    ///1: SDIO_CK generated on the falling edge
    Falling = 1,
}
impl From<NEGEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: NEGEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl NEGEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NEGEDGE_A {
        match self.bits {
            false => NEGEDGE_A::Rising,
            true => NEGEDGE_A::Falling,
        }
    }
    ///Checks if the value of the field is `Rising`
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == NEGEDGE_A::Rising
    }
    ///Checks if the value of the field is `Falling`
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == NEGEDGE_A::Falling
    }
}
///Field `NEGEDGE` writer - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
pub type NEGEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, NEGEDGE_A, O>;
impl<'a, const O: u8> NEGEDGE_W<'a, O> {
    ///SDIO_CK generated on the rising edge
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(NEGEDGE_A::Rising)
    }
    ///SDIO_CK generated on the falling edge
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(NEGEDGE_A::Falling)
    }
}
///Field `HWFC_EN` reader - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
pub type HWFC_EN_R = crate::BitReader<HWFC_EN_A>;
///HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HWFC_EN_A {
    ///0: HW Flow Control is disabled
    Disabled = 0,
    ///1: HW Flow Control is enabled
    Enabled = 1,
}
impl From<HWFC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl HWFC_EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HWFC_EN_A {
        match self.bits {
            false => HWFC_EN_A::Disabled,
            true => HWFC_EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_EN_A::Enabled
    }
}
///Field `HWFC_EN` writer - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
pub type HWFC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKCR_SPEC, HWFC_EN_A, O>;
impl<'a, const O: u8> HWFC_EN_W<'a, O> {
    ///HW Flow Control is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::Disabled)
    }
    ///HW Flow Control is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_EN_A::Enabled)
    }
}
impl R {
    ///Bits 0:7 - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \[CLKDIV + 2\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
    #[inline(always)]
    pub fn pwrsav(&self) -> PWRSAV_R {
        PWRSAV_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    pub fn widbus(&self) -> WIDBUS_R {
        WIDBUS_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
    #[inline(always)]
    pub fn hwfc_en(&self) -> HWFC_EN_R {
        HWFC_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Clock divide factor. This field defines the divide factor between the input clock (SDIOCLK) and the output clock (SDIO_CK): SDIO_CK frequency = SDIOCLK / \[CLKDIV + 2\]. While the SD/SDIO card or MultiMediaCard is in identification mode, the SDIO_CK frequency must be less than 400 kHz. The clock frequency can be changed to the maximum card bus frequency when relative card addresses are assigned to all cards
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    ///Bit 8 - Clock enable bit
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> CLKEN_W<8> {
        CLKEN_W::new(self)
    }
    ///Bit 9 - Power saving configuration bit. For power saving, the SDIO_CK clock output can be disabled when the bus is idle by setting PWRSAV
    #[inline(always)]
    #[must_use]
    pub fn pwrsav(&mut self) -> PWRSAV_W<9> {
        PWRSAV_W::new(self)
    }
    ///Bit 10 - Clock divider bypass enable bit
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<10> {
        BYPASS_W::new(self)
    }
    ///Bits 11:12 - Wide bus mode enable bit This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0)
    #[inline(always)]
    #[must_use]
    pub fn widbus(&mut self) -> WIDBUS_W<11> {
        WIDBUS_W::new(self)
    }
    ///Bit 13 - SDIO_CK dephasing selection bit. When BYPASS is active, the data and the command change on SDIOCLK falling edge whatever NEGEDGE value
    #[inline(always)]
    #[must_use]
    pub fn negedge(&mut self) -> NEGEDGE_W<13> {
        NEGEDGE_W::new(self)
    }
    ///Bit 14 - HW Flow Control enable. When HW Flow Control is enabled, the meaning of the TXFIFOE and RXFIFOF interrupt signals, see SDIO Status register definition in Section 29.8.11
    #[inline(always)]
    #[must_use]
    pub fn hwfc_en(&mut self) -> HWFC_EN_W<14> {
        HWFC_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CLKCR register controls the SDIO_CK output clock.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clkcr](index.html) module
pub struct CLKCR_SPEC;
impl crate::RegisterSpec for CLKCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [clkcr::R](R) reader structure
impl crate::Readable for CLKCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [clkcr::W](W) writer structure
impl crate::Writable for CLKCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CLKCR to value 0
impl crate::Resettable for CLKCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
