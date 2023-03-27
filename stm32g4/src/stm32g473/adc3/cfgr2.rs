///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ROVSE` reader - Regular Oversampling Enable
pub type ROVSE_R = crate::BitReader<ROVSE_A>;
///Regular Oversampling Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSE_A {
    ///0: Regular oversampling disabled
    Disabled = 0,
    ///1: Regular oversampling enabled
    Enabled = 1,
}
impl From<ROVSE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROVSE_A {
        match self.bits {
            false => ROVSE_A::Disabled,
            true => ROVSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ROVSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ROVSE_A::Enabled
    }
}
///Field `ROVSE` writer - Regular Oversampling Enable
pub type ROVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSE_A, O>;
impl<'a, const O: u8> ROVSE_W<'a, O> {
    ///Regular oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Disabled)
    }
    ///Regular oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROVSE_A::Enabled)
    }
}
///Field `JOVSE` reader - Injected Oversampling Enable
pub type JOVSE_R = crate::BitReader<JOVSE_A>;
///Injected Oversampling Enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JOVSE_A {
    ///0: Injected oversampling disabled
    Disabled = 0,
    ///1: Injected oversampling enabled
    Enabled = 1,
}
impl From<JOVSE_A> for bool {
    #[inline(always)]
    fn from(variant: JOVSE_A) -> Self {
        variant as u8 != 0
    }
}
impl JOVSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> JOVSE_A {
        match self.bits {
            false => JOVSE_A::Disabled,
            true => JOVSE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JOVSE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JOVSE_A::Enabled
    }
}
///Field `JOVSE` writer - Injected Oversampling Enable
pub type JOVSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, JOVSE_A, O>;
impl<'a, const O: u8> JOVSE_W<'a, O> {
    ///Injected oversampling disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Disabled)
    }
    ///Injected oversampling enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JOVSE_A::Enabled)
    }
}
///Field `OVSR` reader - Oversampling ratio
pub type OVSR_R = crate::FieldReader<u8, OVSR_A>;
///Oversampling ratio
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSR_A {
    ///0: Oversampling ratio of 2
    Os2 = 0,
    ///1: Oversampling ratio of 4
    Os4 = 1,
    ///2: Oversampling ratio of 8
    Os8 = 2,
    ///3: Oversampling ratio of 16
    Os16 = 3,
    ///4: Oversampling ratio of 32
    Os32 = 4,
    ///5: Oversampling ratio of 64
    Os64 = 5,
    ///6: Oversampling ratio of 128
    Os128 = 6,
    ///7: Oversampling ratio of 256
    Os256 = 7,
}
impl From<OVSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSR_A) -> Self {
        variant as _
    }
}
impl OVSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVSR_A {
        match self.bits {
            0 => OVSR_A::Os2,
            1 => OVSR_A::Os4,
            2 => OVSR_A::Os8,
            3 => OVSR_A::Os16,
            4 => OVSR_A::Os32,
            5 => OVSR_A::Os64,
            6 => OVSR_A::Os128,
            7 => OVSR_A::Os256,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Os2`
    #[inline(always)]
    pub fn is_os2(&self) -> bool {
        *self == OVSR_A::Os2
    }
    ///Checks if the value of the field is `Os4`
    #[inline(always)]
    pub fn is_os4(&self) -> bool {
        *self == OVSR_A::Os4
    }
    ///Checks if the value of the field is `Os8`
    #[inline(always)]
    pub fn is_os8(&self) -> bool {
        *self == OVSR_A::Os8
    }
    ///Checks if the value of the field is `Os16`
    #[inline(always)]
    pub fn is_os16(&self) -> bool {
        *self == OVSR_A::Os16
    }
    ///Checks if the value of the field is `Os32`
    #[inline(always)]
    pub fn is_os32(&self) -> bool {
        *self == OVSR_A::Os32
    }
    ///Checks if the value of the field is `Os64`
    #[inline(always)]
    pub fn is_os64(&self) -> bool {
        *self == OVSR_A::Os64
    }
    ///Checks if the value of the field is `Os128`
    #[inline(always)]
    pub fn is_os128(&self) -> bool {
        *self == OVSR_A::Os128
    }
    ///Checks if the value of the field is `Os256`
    #[inline(always)]
    pub fn is_os256(&self) -> bool {
        *self == OVSR_A::Os256
    }
}
///Field `OVSR` writer - Oversampling ratio
pub type OVSR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFGR2_SPEC, u8, OVSR_A, 3, O>;
impl<'a, const O: u8> OVSR_W<'a, O> {
    ///Oversampling ratio of 2
    #[inline(always)]
    pub fn os2(self) -> &'a mut W {
        self.variant(OVSR_A::Os2)
    }
    ///Oversampling ratio of 4
    #[inline(always)]
    pub fn os4(self) -> &'a mut W {
        self.variant(OVSR_A::Os4)
    }
    ///Oversampling ratio of 8
    #[inline(always)]
    pub fn os8(self) -> &'a mut W {
        self.variant(OVSR_A::Os8)
    }
    ///Oversampling ratio of 16
    #[inline(always)]
    pub fn os16(self) -> &'a mut W {
        self.variant(OVSR_A::Os16)
    }
    ///Oversampling ratio of 32
    #[inline(always)]
    pub fn os32(self) -> &'a mut W {
        self.variant(OVSR_A::Os32)
    }
    ///Oversampling ratio of 64
    #[inline(always)]
    pub fn os64(self) -> &'a mut W {
        self.variant(OVSR_A::Os64)
    }
    ///Oversampling ratio of 128
    #[inline(always)]
    pub fn os128(self) -> &'a mut W {
        self.variant(OVSR_A::Os128)
    }
    ///Oversampling ratio of 256
    #[inline(always)]
    pub fn os256(self) -> &'a mut W {
        self.variant(OVSR_A::Os256)
    }
}
///Field `OVSS` reader - Oversampling shift
pub type OVSS_R = crate::FieldReader<u8, OVSS_A>;
///Oversampling shift
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OVSS_A {
    ///0: No right shift applied to oversampling result
    NoShift = 0,
    ///1: Shift oversampling result right by 1 bit
    Shift1 = 1,
    ///2: Shift oversampling result right by 2 bits
    Shift2 = 2,
    ///3: Shift oversampling result right by 3 bits
    Shift3 = 3,
    ///4: Shift oversampling result right by 4 bits
    Shift4 = 4,
    ///5: Shift oversampling result right by 5 bits
    Shift5 = 5,
    ///6: Shift oversampling result right by 6 bits
    Shift6 = 6,
    ///7: Shift oversampling result right by 7 bits
    Shift7 = 7,
    ///8: Shift oversampling result right by 8 bits
    Shift8 = 8,
}
impl From<OVSS_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSS_A) -> Self {
        variant as _
    }
}
impl OVSS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<OVSS_A> {
        match self.bits {
            0 => Some(OVSS_A::NoShift),
            1 => Some(OVSS_A::Shift1),
            2 => Some(OVSS_A::Shift2),
            3 => Some(OVSS_A::Shift3),
            4 => Some(OVSS_A::Shift4),
            5 => Some(OVSS_A::Shift5),
            6 => Some(OVSS_A::Shift6),
            7 => Some(OVSS_A::Shift7),
            8 => Some(OVSS_A::Shift8),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoShift`
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == OVSS_A::NoShift
    }
    ///Checks if the value of the field is `Shift1`
    #[inline(always)]
    pub fn is_shift1(&self) -> bool {
        *self == OVSS_A::Shift1
    }
    ///Checks if the value of the field is `Shift2`
    #[inline(always)]
    pub fn is_shift2(&self) -> bool {
        *self == OVSS_A::Shift2
    }
    ///Checks if the value of the field is `Shift3`
    #[inline(always)]
    pub fn is_shift3(&self) -> bool {
        *self == OVSS_A::Shift3
    }
    ///Checks if the value of the field is `Shift4`
    #[inline(always)]
    pub fn is_shift4(&self) -> bool {
        *self == OVSS_A::Shift4
    }
    ///Checks if the value of the field is `Shift5`
    #[inline(always)]
    pub fn is_shift5(&self) -> bool {
        *self == OVSS_A::Shift5
    }
    ///Checks if the value of the field is `Shift6`
    #[inline(always)]
    pub fn is_shift6(&self) -> bool {
        *self == OVSS_A::Shift6
    }
    ///Checks if the value of the field is `Shift7`
    #[inline(always)]
    pub fn is_shift7(&self) -> bool {
        *self == OVSS_A::Shift7
    }
    ///Checks if the value of the field is `Shift8`
    #[inline(always)]
    pub fn is_shift8(&self) -> bool {
        *self == OVSS_A::Shift8
    }
}
///Field `OVSS` writer - Oversampling shift
pub type OVSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, OVSS_A, 4, O>;
impl<'a, const O: u8> OVSS_W<'a, O> {
    ///No right shift applied to oversampling result
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut W {
        self.variant(OVSS_A::NoShift)
    }
    ///Shift oversampling result right by 1 bit
    #[inline(always)]
    pub fn shift1(self) -> &'a mut W {
        self.variant(OVSS_A::Shift1)
    }
    ///Shift oversampling result right by 2 bits
    #[inline(always)]
    pub fn shift2(self) -> &'a mut W {
        self.variant(OVSS_A::Shift2)
    }
    ///Shift oversampling result right by 3 bits
    #[inline(always)]
    pub fn shift3(self) -> &'a mut W {
        self.variant(OVSS_A::Shift3)
    }
    ///Shift oversampling result right by 4 bits
    #[inline(always)]
    pub fn shift4(self) -> &'a mut W {
        self.variant(OVSS_A::Shift4)
    }
    ///Shift oversampling result right by 5 bits
    #[inline(always)]
    pub fn shift5(self) -> &'a mut W {
        self.variant(OVSS_A::Shift5)
    }
    ///Shift oversampling result right by 6 bits
    #[inline(always)]
    pub fn shift6(self) -> &'a mut W {
        self.variant(OVSS_A::Shift6)
    }
    ///Shift oversampling result right by 7 bits
    #[inline(always)]
    pub fn shift7(self) -> &'a mut W {
        self.variant(OVSS_A::Shift7)
    }
    ///Shift oversampling result right by 8 bits
    #[inline(always)]
    pub fn shift8(self) -> &'a mut W {
        self.variant(OVSS_A::Shift8)
    }
}
///Field `TROVS` reader - Triggered Regular Oversampling
pub type TROVS_R = crate::BitReader<TROVS_A>;
///Triggered Regular Oversampling
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TROVS_A {
    ///0: All oversampled conversions for a channel are run following a trigger
    Automatic = 0,
    ///1: Each oversampled conversion for a channel needs a new trigger
    Triggered = 1,
}
impl From<TROVS_A> for bool {
    #[inline(always)]
    fn from(variant: TROVS_A) -> Self {
        variant as u8 != 0
    }
}
impl TROVS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TROVS_A {
        match self.bits {
            false => TROVS_A::Automatic,
            true => TROVS_A::Triggered,
        }
    }
    ///Checks if the value of the field is `Automatic`
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == TROVS_A::Automatic
    }
    ///Checks if the value of the field is `Triggered`
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == TROVS_A::Triggered
    }
}
///Field `TROVS` writer - Triggered Regular Oversampling
pub type TROVS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, TROVS_A, O>;
impl<'a, const O: u8> TROVS_W<'a, O> {
    ///All oversampled conversions for a channel are run following a trigger
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(TROVS_A::Automatic)
    }
    ///Each oversampled conversion for a channel needs a new trigger
    #[inline(always)]
    pub fn triggered(self) -> &'a mut W {
        self.variant(TROVS_A::Triggered)
    }
}
///Field `ROVSM` reader - Regular Oversampling mode
pub type ROVSM_R = crate::BitReader<ROVSM_A>;
///Regular Oversampling mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVSM_A {
    ///0: Oversampling is temporary stopped and continued after injection sequence
    Continued = 0,
    ///1: Oversampling is aborted and resumed from start after injection sequence
    Resumed = 1,
}
impl From<ROVSM_A> for bool {
    #[inline(always)]
    fn from(variant: ROVSM_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ROVSM_A {
        match self.bits {
            false => ROVSM_A::Continued,
            true => ROVSM_A::Resumed,
        }
    }
    ///Checks if the value of the field is `Continued`
    #[inline(always)]
    pub fn is_continued(&self) -> bool {
        *self == ROVSM_A::Continued
    }
    ///Checks if the value of the field is `Resumed`
    #[inline(always)]
    pub fn is_resumed(&self) -> bool {
        *self == ROVSM_A::Resumed
    }
}
///Field `ROVSM` writer - Regular Oversampling mode
pub type ROVSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, ROVSM_A, O>;
impl<'a, const O: u8> ROVSM_W<'a, O> {
    ///Oversampling is temporary stopped and continued after injection sequence
    #[inline(always)]
    pub fn continued(self) -> &'a mut W {
        self.variant(ROVSM_A::Continued)
    }
    ///Oversampling is aborted and resumed from start after injection sequence
    #[inline(always)]
    pub fn resumed(self) -> &'a mut W {
        self.variant(ROVSM_A::Resumed)
    }
}
///Field `GCOMP` reader - Gain compensation mode
pub type GCOMP_R = crate::BitReader<GCOMP_A>;
///Gain compensation mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCOMP_A {
    ///0: Regular ADC operating mode
    Disabled = 0,
    ///1: Gain compensation enabled and applies to all channels
    Enabled = 1,
}
impl From<GCOMP_A> for bool {
    #[inline(always)]
    fn from(variant: GCOMP_A) -> Self {
        variant as u8 != 0
    }
}
impl GCOMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GCOMP_A {
        match self.bits {
            false => GCOMP_A::Disabled,
            true => GCOMP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCOMP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCOMP_A::Enabled
    }
}
///Field `GCOMP` writer - Gain compensation mode
pub type GCOMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, GCOMP_A, O>;
impl<'a, const O: u8> GCOMP_W<'a, O> {
    ///Regular ADC operating mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCOMP_A::Disabled)
    }
    ///Gain compensation enabled and applies to all channels
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCOMP_A::Enabled)
    }
}
///Field `SWTRIG` reader - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_R = crate::BitReader<SWTRIG_A>;
///Software trigger bit for sampling time control trigger mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWTRIG_A {
    ///0: End sampling period and start conversion
    Disabled = 0,
    ///1: Start sampling period
    Enabled = 1,
}
impl From<SWTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SWTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SWTRIG_A {
        match self.bits {
            false => SWTRIG_A::Disabled,
            true => SWTRIG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SWTRIG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SWTRIG_A::Enabled
    }
}
///Field `SWTRIG` writer - Software trigger bit for sampling time control trigger mode
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SWTRIG_A, O>;
impl<'a, const O: u8> SWTRIG_W<'a, O> {
    ///End sampling period and start conversion
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::Disabled)
    }
    ///Start sampling period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG_A::Enabled)
    }
}
///Field `BULB` reader - Bulb sampling mode
pub type BULB_R = crate::BitReader<BULB_A>;
///Bulb sampling mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BULB_A {
    ///0: Bulb sampling mode disabled
    Disabled = 0,
    ///1: Bulb sampling mode enabled. Immediately start sampling after last conversion finishes.
    Enabled = 1,
}
impl From<BULB_A> for bool {
    #[inline(always)]
    fn from(variant: BULB_A) -> Self {
        variant as u8 != 0
    }
}
impl BULB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BULB_A {
        match self.bits {
            false => BULB_A::Disabled,
            true => BULB_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BULB_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BULB_A::Enabled
    }
}
///Field `BULB` writer - Bulb sampling mode
pub type BULB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, BULB_A, O>;
impl<'a, const O: u8> BULB_W<'a, O> {
    ///Bulb sampling mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BULB_A::Disabled)
    }
    ///Bulb sampling mode enabled. Immediately start sampling after last conversion finishes.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BULB_A::Enabled)
    }
}
///Field `SMPTRIG` reader - Sampling time control trigger mode
pub type SMPTRIG_R = crate::BitReader<SMPTRIG_A>;
///Sampling time control trigger mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMPTRIG_A {
    ///0: Sampling time control trigger mode disabled
    Disabled = 0,
    ///1: Sampling time control trigger mode enabled
    Enabled = 1,
}
impl From<SMPTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SMPTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SMPTRIG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SMPTRIG_A {
        match self.bits {
            false => SMPTRIG_A::Disabled,
            true => SMPTRIG_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMPTRIG_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMPTRIG_A::Enabled
    }
}
///Field `SMPTRIG` writer - Sampling time control trigger mode
pub type SMPTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR2_SPEC, SMPTRIG_A, O>;
impl<'a, const O: u8> SMPTRIG_W<'a, O> {
    ///Sampling time control trigger mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::Disabled)
    }
    ///Sampling time control trigger mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMPTRIG_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Regular Oversampling Enable
    #[inline(always)]
    pub fn rovse(&self) -> ROVSE_R {
        ROVSE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Injected Oversampling Enable
    #[inline(always)]
    pub fn jovse(&self) -> JOVSE_R {
        JOVSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    pub fn ovsr(&self) -> OVSR_R {
        OVSR_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    pub fn ovss(&self) -> OVSS_R {
        OVSS_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    ///Bit 9 - Triggered Regular Oversampling
    #[inline(always)]
    pub fn trovs(&self) -> TROVS_R {
        TROVS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    pub fn rovsm(&self) -> ROVSM_R {
        ROVSM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 16 - Gain compensation mode
    #[inline(always)]
    pub fn gcomp(&self) -> GCOMP_R {
        GCOMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 25 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    pub fn swtrig(&self) -> SWTRIG_R {
        SWTRIG_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Bulb sampling mode
    #[inline(always)]
    pub fn bulb(&self) -> BULB_R {
        BULB_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Sampling time control trigger mode
    #[inline(always)]
    pub fn smptrig(&self) -> SMPTRIG_R {
        SMPTRIG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Regular Oversampling Enable
    #[inline(always)]
    #[must_use]
    pub fn rovse(&mut self) -> ROVSE_W<0> {
        ROVSE_W::new(self)
    }
    ///Bit 1 - Injected Oversampling Enable
    #[inline(always)]
    #[must_use]
    pub fn jovse(&mut self) -> JOVSE_W<1> {
        JOVSE_W::new(self)
    }
    ///Bits 2:4 - Oversampling ratio
    #[inline(always)]
    #[must_use]
    pub fn ovsr(&mut self) -> OVSR_W<2> {
        OVSR_W::new(self)
    }
    ///Bits 5:8 - Oversampling shift
    #[inline(always)]
    #[must_use]
    pub fn ovss(&mut self) -> OVSS_W<5> {
        OVSS_W::new(self)
    }
    ///Bit 9 - Triggered Regular Oversampling
    #[inline(always)]
    #[must_use]
    pub fn trovs(&mut self) -> TROVS_W<9> {
        TROVS_W::new(self)
    }
    ///Bit 10 - Regular Oversampling mode
    #[inline(always)]
    #[must_use]
    pub fn rovsm(&mut self) -> ROVSM_W<10> {
        ROVSM_W::new(self)
    }
    ///Bit 16 - Gain compensation mode
    #[inline(always)]
    #[must_use]
    pub fn gcomp(&mut self) -> GCOMP_W<16> {
        GCOMP_W::new(self)
    }
    ///Bit 25 - Software trigger bit for sampling time control trigger mode
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<25> {
        SWTRIG_W::new(self)
    }
    ///Bit 26 - Bulb sampling mode
    #[inline(always)]
    #[must_use]
    pub fn bulb(&mut self) -> BULB_W<26> {
        BULB_W::new(self)
    }
    ///Bit 27 - Sampling time control trigger mode
    #[inline(always)]
    #[must_use]
    pub fn smptrig(&mut self) -> SMPTRIG_W<27> {
        SMPTRIG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///configuration register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
