///Register `CCR` reader
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR` writer
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DUAL` reader - Dual ADC mode selection
pub type DUAL_R = crate::FieldReader<u8, DUAL_A>;
///Dual ADC mode selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL_A {
    ///0: Independent mode
    Independent = 0,
    ///1: Dual, combined regular simultaneous + injected simultaneous mode
    DualRj = 1,
    ///2: Dual, combined regular simultaneous + alternate trigger mode
    DualRa = 2,
    ///3: Dual, combined interleaved mode + injected simultaneous mode
    DualIj = 3,
    ///5: Dual, injected simultaneous mode only
    DualJ = 5,
    ///6: Dual, regular simultaneous mode only
    DualR = 6,
    ///7: Dual, interleaved mode only
    DualI = 7,
    ///9: Dual, alternate trigger mode only
    DualA = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
impl DUAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DUAL_A> {
        match self.bits {
            0 => Some(DUAL_A::Independent),
            1 => Some(DUAL_A::DualRj),
            2 => Some(DUAL_A::DualRa),
            3 => Some(DUAL_A::DualIj),
            5 => Some(DUAL_A::DualJ),
            6 => Some(DUAL_A::DualR),
            7 => Some(DUAL_A::DualI),
            9 => Some(DUAL_A::DualA),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Independent`
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL_A::Independent
    }
    ///Checks if the value of the field is `DualRj`
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL_A::DualRj
    }
    ///Checks if the value of the field is `DualRa`
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL_A::DualRa
    }
    ///Checks if the value of the field is `DualIj`
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL_A::DualIj
    }
    ///Checks if the value of the field is `DualJ`
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL_A::DualJ
    }
    ///Checks if the value of the field is `DualR`
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL_A::DualR
    }
    ///Checks if the value of the field is `DualI`
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL_A::DualI
    }
    ///Checks if the value of the field is `DualA`
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL_A::DualA
    }
}
///Field `DUAL` writer - Dual ADC mode selection
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, DUAL_A, 5, O>;
impl<'a, const O: u8> DUAL_W<'a, O> {
    ///Independent mode
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::Independent)
    }
    ///Dual, combined regular simultaneous + injected simultaneous mode
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DualRj)
    }
    ///Dual, combined regular simultaneous + alternate trigger mode
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DualRa)
    }
    ///Dual, combined interleaved mode + injected simultaneous mode
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DualIj)
    }
    ///Dual, injected simultaneous mode only
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DualJ)
    }
    ///Dual, regular simultaneous mode only
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DualR)
    }
    ///Dual, interleaved mode only
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DualI)
    }
    ///Dual, alternate trigger mode only
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DualA)
    }
}
///Field `DELAY` reader - Delay between 2 sampling phases
pub type DELAY_R = crate::FieldReader<u8, u8>;
///Field `DELAY` writer - Delay between 2 sampling phases
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, u8, 4, O>;
///Field `DMACFG` reader - DMA configuration (for multi-ADC mode)
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
///DMA configuration (for multi-ADC mode)
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    ///0: DMA One Shot mode selected
    OneShotMode = 0,
    ///1: DMA Circular mode selected
    CircularMode = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::OneShotMode,
            true => DMACFG_A::CircularMode,
        }
    }
    ///Checks if the value of the field is `OneShotMode`
    #[inline(always)]
    pub fn is_one_shot_mode(&self) -> bool {
        *self == DMACFG_A::OneShotMode
    }
    ///Checks if the value of the field is `CircularMode`
    #[inline(always)]
    pub fn is_circular_mode(&self) -> bool {
        *self == DMACFG_A::CircularMode
    }
}
///Field `DMACFG` writer - DMA configuration (for multi-ADC mode)
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DMACFG_A, O>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    ///DMA One Shot mode selected
    #[inline(always)]
    pub fn one_shot_mode(self) -> &'a mut W {
        self.variant(DMACFG_A::OneShotMode)
    }
    ///DMA Circular mode selected
    #[inline(always)]
    pub fn circular_mode(self) -> &'a mut W {
        self.variant(DMACFG_A::CircularMode)
    }
}
///Field `MDMA` reader - Direct memory access mode for multi ADC mode
pub type MDMA_R = crate::FieldReader<u8, MDMA_A>;
///Direct memory access mode for multi ADC mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDMA_A {
    ///0: MDMA mode disabled
    Disabled = 0,
    ///1: Enable dual interleaved mode to output to the master channel of DFSDM interface both Master and the Slave result (16-bit data width)
    Interleaved = 1,
    ///2: MDMA mode enabled for 12 and 10-bit resolution
    Bits1210 = 2,
    ///3: MDMA mode enabled for 8 and 6-bit resolution
    Bits86 = 3,
}
impl From<MDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: MDMA_A) -> Self {
        variant as _
    }
}
impl MDMA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MDMA_A {
        match self.bits {
            0 => MDMA_A::Disabled,
            1 => MDMA_A::Interleaved,
            2 => MDMA_A::Bits1210,
            3 => MDMA_A::Bits86,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMA_A::Disabled
    }
    ///Checks if the value of the field is `Interleaved`
    #[inline(always)]
    pub fn is_interleaved(&self) -> bool {
        *self == MDMA_A::Interleaved
    }
    ///Checks if the value of the field is `Bits1210`
    #[inline(always)]
    pub fn is_bits12_10(&self) -> bool {
        *self == MDMA_A::Bits1210
    }
    ///Checks if the value of the field is `Bits86`
    #[inline(always)]
    pub fn is_bits8_6(&self) -> bool {
        *self == MDMA_A::Bits86
    }
}
///Field `MDMA` writer - Direct memory access mode for multi ADC mode
pub type MDMA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, MDMA_A, 2, O>;
impl<'a, const O: u8> MDMA_W<'a, O> {
    ///MDMA mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMA_A::Disabled)
    }
    ///Enable dual interleaved mode to output to the master channel of DFSDM interface both Master and the Slave result (16-bit data width)
    #[inline(always)]
    pub fn interleaved(self) -> &'a mut W {
        self.variant(MDMA_A::Interleaved)
    }
    ///MDMA mode enabled for 12 and 10-bit resolution
    #[inline(always)]
    pub fn bits12_10(self) -> &'a mut W {
        self.variant(MDMA_A::Bits1210)
    }
    ///MDMA mode enabled for 8 and 6-bit resolution
    #[inline(always)]
    pub fn bits8_6(self) -> &'a mut W {
        self.variant(MDMA_A::Bits86)
    }
}
///Field `CKMODE` reader - ADC clock mode
pub type CKMODE_R = crate::FieldReader<u8, CKMODE_A>;
///ADC clock mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    ///0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    Asynchronous = 0,
    ///1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    SyncDiv1 = 1,
    ///2: Use AHB clock rcc_hclk3 divided by 2
    SyncDiv2 = 2,
    ///3: Use AHB clock rcc_hclk3 divided by 4
    SyncDiv4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl CKMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::Asynchronous,
            1 => CKMODE_A::SyncDiv1,
            2 => CKMODE_A::SyncDiv2,
            3 => CKMODE_A::SyncDiv4,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Asynchronous`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE_A::Asynchronous
    }
    ///Checks if the value of the field is `SyncDiv1`
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE_A::SyncDiv1
    }
    ///Checks if the value of the field is `SyncDiv2`
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE_A::SyncDiv2
    }
    ///Checks if the value of the field is `SyncDiv4`
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE_A::SyncDiv4
    }
}
///Field `CKMODE` writer - ADC clock mode
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CCR_SPEC, u8, CKMODE_A, 2, O>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    ///Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::Asynchronous)
    }
    ///Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv1)
    }
    ///Use AHB clock rcc_hclk3 divided by 2
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv2)
    }
    ///Use AHB clock rcc_hclk3 divided by 4
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SyncDiv4)
    }
}
///Field `PRESC` reader - ADC prescaler
pub type PRESC_R = crate::FieldReader<u8, PRESC_A>;
///ADC prescaler
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESC_A {
    ///0: Input ADC clock not divided
    Div1 = 0,
    ///1: Input ADC clock divided by 2
    Div2 = 1,
    ///2: Input ADC clock divided by 4
    Div4 = 2,
    ///3: Input ADC clock divided by 6
    Div6 = 3,
    ///4: Input ADC clock divided by 8
    Div8 = 4,
    ///5: Input ADC clock divided by 10
    Div10 = 5,
    ///6: Input ADC clock divided by 12
    Div12 = 6,
    ///7: Input ADC clock divided by 16
    Div16 = 7,
    ///8: Input ADC clock divided by 32
    Div32 = 8,
    ///9: Input ADC clock divided by 64
    Div64 = 9,
    ///10: Input ADC clock divided by 128
    Div128 = 10,
    ///11: Input ADC clock divided by 256
    Div256 = 11,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
impl PRESC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESC_A> {
        match self.bits {
            0 => Some(PRESC_A::Div1),
            1 => Some(PRESC_A::Div2),
            2 => Some(PRESC_A::Div4),
            3 => Some(PRESC_A::Div6),
            4 => Some(PRESC_A::Div8),
            5 => Some(PRESC_A::Div10),
            6 => Some(PRESC_A::Div12),
            7 => Some(PRESC_A::Div16),
            8 => Some(PRESC_A::Div32),
            9 => Some(PRESC_A::Div64),
            10 => Some(PRESC_A::Div128),
            11 => Some(PRESC_A::Div256),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Div1`
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::Div1
    }
    ///Checks if the value of the field is `Div2`
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::Div2
    }
    ///Checks if the value of the field is `Div4`
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::Div4
    }
    ///Checks if the value of the field is `Div6`
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PRESC_A::Div6
    }
    ///Checks if the value of the field is `Div8`
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::Div8
    }
    ///Checks if the value of the field is `Div10`
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PRESC_A::Div10
    }
    ///Checks if the value of the field is `Div12`
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PRESC_A::Div12
    }
    ///Checks if the value of the field is `Div16`
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::Div16
    }
    ///Checks if the value of the field is `Div32`
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::Div32
    }
    ///Checks if the value of the field is `Div64`
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::Div64
    }
    ///Checks if the value of the field is `Div128`
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::Div128
    }
    ///Checks if the value of the field is `Div256`
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESC_A::Div256
    }
}
///Field `PRESC` writer - ADC prescaler
pub type PRESC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR_SPEC, u8, PRESC_A, 4, O>;
impl<'a, const O: u8> PRESC_W<'a, O> {
    ///Input ADC clock not divided
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::Div1)
    }
    ///Input ADC clock divided by 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::Div2)
    }
    ///Input ADC clock divided by 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::Div4)
    }
    ///Input ADC clock divided by 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(PRESC_A::Div6)
    }
    ///Input ADC clock divided by 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::Div8)
    }
    ///Input ADC clock divided by 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(PRESC_A::Div10)
    }
    ///Input ADC clock divided by 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(PRESC_A::Div12)
    }
    ///Input ADC clock divided by 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::Div16)
    }
    ///Input ADC clock divided by 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::Div32)
    }
    ///Input ADC clock divided by 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::Div64)
    }
    ///Input ADC clock divided by 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::Div128)
    }
    ///Input ADC clock divided by 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESC_A::Div256)
    }
}
///Field `VREFEN` reader - VREFINT enable
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
///VREFINT enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    ///0: V_REFINT channel disabled
    Disabled = 0,
    ///1: V_REFINT channel enabled
    Enabled = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::Disabled,
            true => VREFEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::Enabled
    }
}
///Field `VREFEN` writer - VREFINT enable
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VREFEN_A, O>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    ///V_REFINT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Disabled)
    }
    ///V_REFINT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::Enabled)
    }
}
///Field `VSENSEEN` reader - Temperature sensor selection
pub type VSENSEEN_R = crate::BitReader<VSENSEEN_A>;
///Temperature sensor selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VSENSEEN_A {
    ///0: Temperature sensor channel disabled
    Disabled = 0,
    ///1: Temperature sensor channel enabled
    Enabled = 1,
}
impl From<VSENSEEN_A> for bool {
    #[inline(always)]
    fn from(variant: VSENSEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VSENSEEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VSENSEEN_A {
        match self.bits {
            false => VSENSEEN_A::Disabled,
            true => VSENSEEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VSENSEEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VSENSEEN_A::Enabled
    }
}
///Field `VSENSEEN` writer - Temperature sensor selection
pub type VSENSEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VSENSEEN_A, O>;
impl<'a, const O: u8> VSENSEEN_W<'a, O> {
    ///Temperature sensor channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VSENSEEN_A::Disabled)
    }
    ///Temperature sensor channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VSENSEEN_A::Enabled)
    }
}
///Field `VBATEN` reader - VBAT selection
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
///VBAT selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    ///0: VBAT channel disabled
    Disabled = 0,
    ///1: VBAT channel enabled
    Enabled = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::Disabled,
            true => VBATEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN_A::Enabled
    }
}
///Field `VBATEN` writer - VBAT selection
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, VBATEN_A, O>;
impl<'a, const O: u8> VBATEN_W<'a, O> {
    ///VBAT channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Disabled)
    }
    ///VBAT channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEN_A::Enabled)
    }
}
impl R {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Temperature sensor selection
    #[inline(always)]
    pub fn vsenseen(&self) -> VSENSEEN_R {
        VSENSEEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:4 - Dual ADC mode selection
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    ///Bits 8:11 - Delay between 2 sampling phases
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    ///Bit 13 - DMA configuration (for multi-ADC mode)
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<13> {
        DMACFG_W::new(self)
    }
    ///Bits 14:15 - Direct memory access mode for multi ADC mode
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<14> {
        MDMA_W::new(self)
    }
    ///Bits 16:17 - ADC clock mode
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<16> {
        CKMODE_W::new(self)
    }
    ///Bits 18:21 - ADC prescaler
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<18> {
        PRESC_W::new(self)
    }
    ///Bit 22 - VREFINT enable
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    ///Bit 23 - Temperature sensor selection
    #[inline(always)]
    #[must_use]
    pub fn vsenseen(&mut self) -> VSENSEEN_W<23> {
        VSENSEEN_W::new(self)
    }
    ///Bit 24 - VBAT selection
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC common control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr](index.html) module
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr::R](R) reader structure
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr::W](W) writer structure
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR to value 0
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
