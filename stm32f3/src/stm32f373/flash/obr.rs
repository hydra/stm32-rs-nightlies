///Register `OBR` reader
pub struct R(crate::R<OBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OBR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `OPTERR` reader - Option byte error
pub type OPTERR_R = crate::BitReader<OPTERR_A>;
///Option byte error
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPTERR_A {
    ///1: The loaded option byte and its complement do not match
    OptionByteError = 1,
}
impl From<OPTERR_A> for bool {
    #[inline(always)]
    fn from(variant: OPTERR_A) -> Self {
        variant as u8 != 0
    }
}
impl OPTERR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OPTERR_A> {
        match self.bits {
            true => Some(OPTERR_A::OptionByteError),
            _ => None,
        }
    }
    ///Checks if the value of the field is `OptionByteError`
    #[inline(always)]
    pub fn is_option_byte_error(&self) -> bool {
        *self == OPTERR_A::OptionByteError
    }
}
///Field `RDPRT` reader - Read protection Level status
pub type RDPRT_R = crate::FieldReader<u8, RDPRT_A>;
///Read protection Level status
///
///Value on reset: 3
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDPRT_A {
    ///0: Level 0
    Level0 = 0,
    ///1: Level 1
    Level1 = 1,
    ///3: Level 2
    Level2 = 3,
}
impl From<RDPRT_A> for u8 {
    #[inline(always)]
    fn from(variant: RDPRT_A) -> Self {
        variant as _
    }
}
impl RDPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<RDPRT_A> {
        match self.bits {
            0 => Some(RDPRT_A::Level0),
            1 => Some(RDPRT_A::Level1),
            3 => Some(RDPRT_A::Level2),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Level0`
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == RDPRT_A::Level0
    }
    ///Checks if the value of the field is `Level1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == RDPRT_A::Level1
    }
    ///Checks if the value of the field is `Level2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == RDPRT_A::Level2
    }
}
///Field `WDG_SW` reader - WDG_SW
pub type WDG_SW_R = crate::BitReader<WDG_SW_A>;
///WDG_SW
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDG_SW_A {
    ///0: Hardware watchdog
    Hardware = 0,
    ///1: Software watchdog
    Software = 1,
}
impl From<WDG_SW_A> for bool {
    #[inline(always)]
    fn from(variant: WDG_SW_A) -> Self {
        variant as u8 != 0
    }
}
impl WDG_SW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WDG_SW_A {
        match self.bits {
            false => WDG_SW_A::Hardware,
            true => WDG_SW_A::Software,
        }
    }
    ///Checks if the value of the field is `Hardware`
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == WDG_SW_A::Hardware
    }
    ///Checks if the value of the field is `Software`
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == WDG_SW_A::Software
    }
}
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<N_RST_STOP_A>;
///nRST_STOP
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STOP_A {
    ///0: Reset generated when entering Stop mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STOP_A {
        match self.bits {
            false => N_RST_STOP_A::Reset,
            true => N_RST_STOP_A::NoReset,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STOP_A::Reset
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STOP_A::NoReset
    }
}
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<N_RST_STDBY_A>;
///nRST_STDBY
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_RST_STDBY_A {
    ///0: Reset generated when entering Standby mode
    Reset = 0,
    ///1: No reset generated
    NoReset = 1,
}
impl From<N_RST_STDBY_A> for bool {
    #[inline(always)]
    fn from(variant: N_RST_STDBY_A) -> Self {
        variant as u8 != 0
    }
}
impl N_RST_STDBY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_RST_STDBY_A {
        match self.bits {
            false => N_RST_STDBY_A::Reset,
            true => N_RST_STDBY_A::NoReset,
        }
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == N_RST_STDBY_A::Reset
    }
    ///Checks if the value of the field is `NoReset`
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == N_RST_STDBY_A::NoReset
    }
}
///Field `nBOOT1` reader - BOOT1
pub type N_BOOT1_R = crate::BitReader<N_BOOT1_A>;
///BOOT1
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum N_BOOT1_A {
    ///0: Together with BOOT0, select the device boot mode
    Disabled = 0,
    ///1: Together with BOOT0, select the device boot mode
    Enabled = 1,
}
impl From<N_BOOT1_A> for bool {
    #[inline(always)]
    fn from(variant: N_BOOT1_A) -> Self {
        variant as u8 != 0
    }
}
impl N_BOOT1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> N_BOOT1_A {
        match self.bits {
            false => N_BOOT1_A::Disabled,
            true => N_BOOT1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == N_BOOT1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == N_BOOT1_A::Enabled
    }
}
///Field `VDDA_MONITOR` reader - VDDA_MONITOR
pub type VDDA_MONITOR_R = crate::BitReader<VDDA_MONITOR_A>;
///VDDA_MONITOR
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VDDA_MONITOR_A {
    ///0: VDDA power supply supervisor disabled
    Disabled = 0,
    ///1: VDDA power supply supervisor enabled
    Enabled = 1,
}
impl From<VDDA_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: VDDA_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
impl VDDA_MONITOR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VDDA_MONITOR_A {
        match self.bits {
            false => VDDA_MONITOR_A::Disabled,
            true => VDDA_MONITOR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VDDA_MONITOR_A::Enabled
    }
}
///Field `SRAM_PARITY_CHECK` reader - SRAM_PARITY_CHECK
pub type SRAM_PARITY_CHECK_R = crate::BitReader<SRAM_PARITY_CHECK_A>;
///SRAM_PARITY_CHECK
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRAM_PARITY_CHECK_A {
    ///0: RAM parity check disabled
    Disabled = 0,
    ///1: RAM parity check enabled
    Enabled = 1,
}
impl From<SRAM_PARITY_CHECK_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_PARITY_CHECK_A) -> Self {
        variant as u8 != 0
    }
}
impl SRAM_PARITY_CHECK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SRAM_PARITY_CHECK_A {
        match self.bits {
            false => SRAM_PARITY_CHECK_A::Disabled,
            true => SRAM_PARITY_CHECK_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SRAM_PARITY_CHECK_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SRAM_PARITY_CHECK_A::Enabled
    }
}
///Field `SDADC12_VDD_MONITOR` reader - SDADC12_VDD_MONITOR
pub type SDADC12_VDD_MONITOR_R = crate::BitReader<SDADC12_VDD_MONITOR_A>;
///SDADC12_VDD_MONITOR
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDADC12_VDD_MONITOR_A {
    ///0: VDDSD12 monitoring disabled
    Disabled = 0,
    ///1: VDDSD12 monitoring enabled
    Enabled = 1,
}
impl From<SDADC12_VDD_MONITOR_A> for bool {
    #[inline(always)]
    fn from(variant: SDADC12_VDD_MONITOR_A) -> Self {
        variant as u8 != 0
    }
}
impl SDADC12_VDD_MONITOR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDADC12_VDD_MONITOR_A {
        match self.bits {
            false => SDADC12_VDD_MONITOR_A::Disabled,
            true => SDADC12_VDD_MONITOR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDADC12_VDD_MONITOR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDADC12_VDD_MONITOR_A::Enabled
    }
}
///Field `Data0` reader - Data0
pub type DATA0_R = crate::FieldReader<u8, u8>;
///Field `Data1` reader - Data1
pub type DATA1_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - Option byte error
    #[inline(always)]
    pub fn opterr(&self) -> OPTERR_R {
        OPTERR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Read protection Level status
    #[inline(always)]
    pub fn rdprt(&self) -> RDPRT_R {
        RDPRT_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 8 - WDG_SW
    #[inline(always)]
    pub fn wdg_sw(&self) -> WDG_SW_R {
        WDG_SW_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - BOOT1
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - VDDA_MONITOR
    #[inline(always)]
    pub fn vdda_monitor(&self) -> VDDA_MONITOR_R {
        VDDA_MONITOR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SRAM_PARITY_CHECK
    #[inline(always)]
    pub fn sram_parity_check(&self) -> SRAM_PARITY_CHECK_R {
        SRAM_PARITY_CHECK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SDADC12_VDD_MONITOR
    #[inline(always)]
    pub fn sdadc12_vdd_monitor(&self) -> SDADC12_VDD_MONITOR_R {
        SDADC12_VDD_MONITOR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Data0
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Data1
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///Option byte register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [obr](index.html) module
pub struct OBR_SPEC;
impl crate::RegisterSpec for OBR_SPEC {
    type Ux = u32;
}
///`read()` method returns [obr::R](R) reader structure
impl crate::Readable for OBR_SPEC {
    type Reader = R;
}
///`reset()` method sets OBR to value 0xffff_ff0f
impl crate::Resettable for OBR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff0f;
}
