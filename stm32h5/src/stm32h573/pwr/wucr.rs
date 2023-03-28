///Register `WUCR` reader
pub struct R(crate::R<WUCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WUCR` writer
pub struct W(crate::W<WUCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUCR_SPEC>;
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
impl From<crate::W<WUCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WUPEN1` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN1_R = crate::BitReader<bool>;
///Field `WUPEN1` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN2` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN2_R = crate::BitReader<bool>;
///Field `WUPEN2` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN3` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN3_R = crate::BitReader<bool>;
///Field `WUPEN3` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN4` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN4_R = crate::BitReader<bool>;
///Field `WUPEN4` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN5` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN5_R = crate::BitReader<bool>;
///Field `WUPEN5` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN6` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN6_R = crate::BitReader<bool>;
///Field `WUPEN6` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN7` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN7_R = crate::BitReader<bool>;
///Field `WUPEN7` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPEN8` reader - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN8_R = crate::BitReader<bool>;
///Field `WUPEN8` writer - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
pub type WUPEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP1` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP1_R = crate::BitReader<bool>;
///Field `WUPP1` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP2` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP2_R = crate::BitReader<bool>;
///Field `WUPP2` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP3` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP3_R = crate::BitReader<bool>;
///Field `WUPP3` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP4` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP4_R = crate::BitReader<bool>;
///Field `WUPP4` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP5` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP5_R = crate::BitReader<bool>;
///Field `WUPP5` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP6` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP6_R = crate::BitReader<bool>;
///Field `WUPP6` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP7` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP7_R = crate::BitReader<bool>;
///Field `WUPP7` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPP8` reader - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP8_R = crate::BitReader<bool>;
///Field `WUPP8` writer - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
pub type WUPP8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUCR_SPEC, bool, O>;
///Field `WUPPUPD1` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD1_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD1` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD2` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD2_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD2` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD3` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD3_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD3` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD4` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD4_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD4` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD5` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD5_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD5` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD6` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD6_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD6` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD7` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD7_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD7` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
///Field `WUPPUPD8` reader - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD8_R = crate::FieldReader<u8, u8>;
///Field `WUPPUPD8` writer - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
pub type WUPPUPD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen1(&self) -> WUPEN1_R {
        WUPEN1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen2(&self) -> WUPEN2_R {
        WUPEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen3(&self) -> WUPEN3_R {
        WUPEN3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen4(&self) -> WUPEN4_R {
        WUPEN4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen5(&self) -> WUPEN5_R {
        WUPEN5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen6(&self) -> WUPEN6_R {
        WUPEN6_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen7(&self) -> WUPEN7_R {
        WUPEN7_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    pub fn wupen8(&self) -> WUPEN8_R {
        WUPEN8_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp1(&self) -> WUPP1_R {
        WUPP1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp2(&self) -> WUPP2_R {
        WUPP2_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp3(&self) -> WUPP3_R {
        WUPP3_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp4(&self) -> WUPP4_R {
        WUPP4_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp5(&self) -> WUPP5_R {
        WUPP5_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp6(&self) -> WUPP6_R {
        WUPP6_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp7(&self) -> WUPP7_R {
        WUPP7_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    pub fn wupp8(&self) -> WUPP8_R {
        WUPP8_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd1(&self) -> WUPPUPD1_R {
        WUPPUPD1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd2(&self) -> WUPPUPD2_R {
        WUPPUPD2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd3(&self) -> WUPPUPD3_R {
        WUPPUPD3_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd4(&self) -> WUPPUPD4_R {
        WUPPUPD4_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd5(&self) -> WUPPUPD5_R {
        WUPPUPD5_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd6(&self) -> WUPPUPD6_R {
        WUPPUPD6_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd7(&self) -> WUPPUPD7_R {
        WUPPUPD7_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    pub fn wuppupd8(&self) -> WUPPUPD8_R {
        WUPPUPD8_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen1(&mut self) -> WUPEN1_W<0> {
        WUPEN1_W::new(self)
    }
    ///Bit 1 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen2(&mut self) -> WUPEN2_W<1> {
        WUPEN2_W::new(self)
    }
    ///Bit 2 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen3(&mut self) -> WUPEN3_W<2> {
        WUPEN3_W::new(self)
    }
    ///Bit 3 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen4(&mut self) -> WUPEN4_W<3> {
        WUPEN4_W::new(self)
    }
    ///Bit 4 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen5(&mut self) -> WUPEN5_W<4> {
        WUPEN5_W::new(self)
    }
    ///Bit 5 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen6(&mut self) -> WUPEN6_W<5> {
        WUPEN6_W::new(self)
    }
    ///Bit 6 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen7(&mut self) -> WUPEN7_W<6> {
        WUPEN7_W::new(self)
    }
    ///Bit 7 - enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge.
    #[inline(always)]
    #[must_use]
    pub fn wupen8(&mut self) -> WUPEN8_W<7> {
        WUPEN8_W::new(self)
    }
    ///Bit 8 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp1(&mut self) -> WUPP1_W<8> {
        WUPP1_W::new(self)
    }
    ///Bit 9 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp2(&mut self) -> WUPP2_W<9> {
        WUPP2_W::new(self)
    }
    ///Bit 10 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp3(&mut self) -> WUPP3_W<10> {
        WUPP3_W::new(self)
    }
    ///Bit 11 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp4(&mut self) -> WUPP4_W<11> {
        WUPP4_W::new(self)
    }
    ///Bit 12 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp5(&mut self) -> WUPP5_W<12> {
        WUPP5_W::new(self)
    }
    ///Bit 13 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp6(&mut self) -> WUPP6_W<13> {
        WUPP6_W::new(self)
    }
    ///Bit 14 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp7(&mut self) -> WUPP7_W<14> {
        WUPP7_W::new(self)
    }
    ///Bit 15 - wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin.
    #[inline(always)]
    #[must_use]
    pub fn wupp8(&mut self) -> WUPP8_W<15> {
        WUPP8_W::new(self)
    }
    ///Bits 16:17 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd1(&mut self) -> WUPPUPD1_W<16> {
        WUPPUPD1_W::new(self)
    }
    ///Bits 18:19 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd2(&mut self) -> WUPPUPD2_W<18> {
        WUPPUPD2_W::new(self)
    }
    ///Bits 20:21 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd3(&mut self) -> WUPPUPD3_W<20> {
        WUPPUPD3_W::new(self)
    }
    ///Bits 22:23 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd4(&mut self) -> WUPPUPD4_W<22> {
        WUPPUPD4_W::new(self)
    }
    ///Bits 24:25 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd5(&mut self) -> WUPPUPD5_W<24> {
        WUPPUPD5_W::new(self)
    }
    ///Bits 26:27 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd6(&mut self) -> WUPPUPD6_W<26> {
        WUPPUPD6_W::new(self)
    }
    ///Bits 28:29 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd7(&mut self) -> WUPPUPD7_W<28> {
        WUPPUPD7_W::new(self)
    }
    ///Bits 30:31 - wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode.
    #[inline(always)]
    #[must_use]
    pub fn wuppupd8(&mut self) -> WUPPUPD8_W<30> {
        WUPPUPD8_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///PWR wakeup configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wucr](index.html) module
pub struct WUCR_SPEC;
impl crate::RegisterSpec for WUCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [wucr::R](R) reader structure
impl crate::Readable for WUCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wucr::W](W) writer structure
impl crate::Writable for WUCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WUCR to value 0
impl crate::Resettable for WUCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
