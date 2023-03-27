///Register `BDTR` reader
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDTR` writer
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader<u8, u8>;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, u8, 8, O>;
///Field `LOCK` reader - Lock configuration
pub type LOCK_R = crate::FieldReader<u8, LOCK_A>;
///Lock configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCK_A {
    ///0: No write protection
    Off = 0,
    ///1: Level 1 write protection
    Level1 = 1,
    ///2: Level 2 write protection
    Level2 = 2,
    ///3: Level 3 write protection
    Level3 = 3,
}
impl From<LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as _
    }
}
impl LOCK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            0 => LOCK_A::Off,
            1 => LOCK_A::Level1,
            2 => LOCK_A::Level2,
            3 => LOCK_A::Level3,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `Off`
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LOCK_A::Off
    }
    ///Checks if the value of the field is `Level1`
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == LOCK_A::Level1
    }
    ///Checks if the value of the field is `Level2`
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == LOCK_A::Level2
    }
    ///Checks if the value of the field is `Level3`
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == LOCK_A::Level3
    }
}
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, LOCK_A, 2, O>;
impl<'a, const O: u8> LOCK_W<'a, O> {
    ///No write protection
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LOCK_A::Off)
    }
    ///Level 1 write protection
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(LOCK_A::Level1)
    }
    ///Level 2 write protection
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(LOCK_A::Level2)
    }
    ///Level 3 write protection
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(LOCK_A::Level3)
    }
}
///Field `OSSI` reader - Off-state selection for Idle mode
pub type OSSI_R = crate::BitReader<OSSI_A>;
///Off-state selection for Idle mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSI_A {
    ///0: OC/OCN outputs are disabled when inactive
    Disabled = 0,
    ///1: OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    Enabled = 1,
}
impl From<OSSI_A> for bool {
    #[inline(always)]
    fn from(variant: OSSI_A) -> Self {
        variant as u8 != 0
    }
}
impl OSSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSI_A {
        match self.bits {
            false => OSSI_A::Disabled,
            true => OSSI_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSI_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSI_A::Enabled
    }
}
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSI_A, O>;
impl<'a, const O: u8> OSSI_W<'a, O> {
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSI_A::Disabled)
    }
    ///OC/OCN outputs are first forced with their inactive level then forced to their idle level after the deadtime
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSI_A::Enabled)
    }
}
///Field `OSSR` reader - Off-state selection for Run mode
pub type OSSR_R = crate::BitReader<OSSR_A>;
///Off-state selection for Run mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSSR_A {
    ///0: OC/OCN outputs are disabled when inactive
    Disabled = 0,
    ///1: OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    Enabled = 1,
}
impl From<OSSR_A> for bool {
    #[inline(always)]
    fn from(variant: OSSR_A) -> Self {
        variant as u8 != 0
    }
}
impl OSSR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSSR_A {
        match self.bits {
            false => OSSR_A::Disabled,
            true => OSSR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSSR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSSR_A::Enabled
    }
}
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, OSSR_A, O>;
impl<'a, const O: u8> OSSR_W<'a, O> {
    ///OC/OCN outputs are disabled when inactive
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSSR_A::Disabled)
    }
    ///OC/OCN outputs are enabled with their inactive level as soon as CCxE=1 or CCxNE=1
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSSR_A::Enabled)
    }
}
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader<BKE_A>;
///Break enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKE_A {
    ///0: Break function disabled
    Disabled = 0,
    ///1: Break function enabled
    Enabled = 1,
}
impl From<BKE_A> for bool {
    #[inline(always)]
    fn from(variant: BKE_A) -> Self {
        variant as u8 != 0
    }
}
impl BKE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKE_A {
        match self.bits {
            false => BKE_A::Disabled,
            true => BKE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BKE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BKE_A::Enabled
    }
}
///Field `BKE` writer - Break enable
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKE_A, O>;
impl<'a, const O: u8> BKE_W<'a, O> {
    ///Break function disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BKE_A::Disabled)
    }
    ///Break function enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BKE_A::Enabled)
    }
}
///Field `BKP` reader - Break polarity
pub type BKP_R = crate::BitReader<BKP_A>;
///Break polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP_A {
    ///0: Break input BRK is active low
    ActiveLow = 0,
    ///1: Break input BRK is active high
    ActiveHigh = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
impl BKP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::ActiveLow,
            true => BKP_A::ActiveHigh,
        }
    }
    ///Checks if the value of the field is `ActiveLow`
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BKP_A::ActiveLow
    }
    ///Checks if the value of the field is `ActiveHigh`
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BKP_A::ActiveHigh
    }
}
///Field `BKP` writer - Break polarity
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKP_A, O>;
impl<'a, const O: u8> BKP_W<'a, O> {
    ///Break input BRK is active low
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(BKP_A::ActiveLow)
    }
    ///Break input BRK is active high
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(BKP_A::ActiveHigh)
    }
}
///Field `AOE` reader - Automatic output enable
pub type AOE_R = crate::BitReader<AOE_A>;
///Automatic output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOE_A {
    ///0: MOE can be set only by software
    Disabled = 0,
    ///1: MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    Enabled = 1,
}
impl From<AOE_A> for bool {
    #[inline(always)]
    fn from(variant: AOE_A) -> Self {
        variant as u8 != 0
    }
}
impl AOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AOE_A {
        match self.bits {
            false => AOE_A::Disabled,
            true => AOE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AOE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AOE_A::Enabled
    }
}
///Field `AOE` writer - Automatic output enable
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, AOE_A, O>;
impl<'a, const O: u8> AOE_W<'a, O> {
    ///MOE can be set only by software
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AOE_A::Disabled)
    }
    ///MOE can be set by software or automatically at the next update event (if none of the break inputs BRK and BRK2 is active)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AOE_A::Enabled)
    }
}
///Field `MOE` reader - Main output enable
pub type MOE_R = crate::BitReader<MOE_A>;
///Main output enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE_A {
    ///0: In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    Disabled = 0,
    ///1: OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    Enabled = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::Disabled,
            true => MOE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MOE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MOE_A::Enabled
    }
}
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, MOE_A, O>;
impl<'a, const O: u8> MOE_W<'a, O> {
    ///In response to a break 2 event OC and OCN outputs are disabled - In response to a break event or if MOE is written to 0 OC and OCN outputs are disabled or forced to idle state depending on the OSSI bit
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MOE_A::Disabled)
    }
    ///OC and OCN outputs are enabled if their respective enable bits are set (CCxE, CCxNE in TIMx_CCER register)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MOE_A::Enabled)
    }
}
///Field `BKF` reader - Break filter
pub type BKF_R = crate::FieldReader<u8, BKF_A>;
///Break filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKF_A {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FdtsDiv32N8 = 15,
}
impl From<BKF_A> for u8 {
    #[inline(always)]
    fn from(variant: BKF_A) -> Self {
        variant as _
    }
}
impl BKF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKF_A {
        match self.bits {
            0 => BKF_A::NoFilter,
            1 => BKF_A::FckIntN2,
            2 => BKF_A::FckIntN4,
            3 => BKF_A::FckIntN8,
            4 => BKF_A::FdtsDiv2N6,
            5 => BKF_A::FdtsDiv2N8,
            6 => BKF_A::FdtsDiv4N6,
            7 => BKF_A::FdtsDiv4N8,
            8 => BKF_A::FdtsDiv8N6,
            9 => BKF_A::FdtsDiv8N8,
            10 => BKF_A::FdtsDiv16N5,
            11 => BKF_A::FdtsDiv16N6,
            12 => BKF_A::FdtsDiv16N8,
            13 => BKF_A::FdtsDiv32N5,
            14 => BKF_A::FdtsDiv32N6,
            15 => BKF_A::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BKF_A::NoFilter
    }
    ///Checks if the value of the field is `FckIntN2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BKF_A::FckIntN2
    }
    ///Checks if the value of the field is `FckIntN4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BKF_A::FckIntN4
    }
    ///Checks if the value of the field is `FckIntN8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BKF_A::FckIntN8
    }
    ///Checks if the value of the field is `FdtsDiv2N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BKF_A::FdtsDiv2N6
    }
    ///Checks if the value of the field is `FdtsDiv2N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BKF_A::FdtsDiv2N8
    }
    ///Checks if the value of the field is `FdtsDiv4N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BKF_A::FdtsDiv4N6
    }
    ///Checks if the value of the field is `FdtsDiv4N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BKF_A::FdtsDiv4N8
    }
    ///Checks if the value of the field is `FdtsDiv8N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BKF_A::FdtsDiv8N6
    }
    ///Checks if the value of the field is `FdtsDiv8N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BKF_A::FdtsDiv8N8
    }
    ///Checks if the value of the field is `FdtsDiv16N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BKF_A::FdtsDiv16N5
    }
    ///Checks if the value of the field is `FdtsDiv16N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BKF_A::FdtsDiv16N6
    }
    ///Checks if the value of the field is `FdtsDiv16N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BKF_A::FdtsDiv16N8
    }
    ///Checks if the value of the field is `FdtsDiv32N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BKF_A::FdtsDiv32N5
    }
    ///Checks if the value of the field is `FdtsDiv32N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BKF_A::FdtsDiv32N6
    }
    ///Checks if the value of the field is `FdtsDiv32N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BKF_A::FdtsDiv32N8
    }
}
///Field `BKF` writer - Break filter
pub type BKF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, BKF_A, 4, O>;
impl<'a, const O: u8> BKF_W<'a, O> {
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(BKF_A::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(BKF_A::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(BKF_A::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(BKF_A::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(BKF_A::FdtsDiv32N8)
    }
}
///Field `BK2F` reader - Break 2 filter
pub type BK2F_R = crate::FieldReader<u8, BK2F_A>;
///Break 2 filter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BK2F_A {
    ///0: No filter, sampling is done at fDTS
    NoFilter = 0,
    ///1: fSAMPLING=fCK_INT, N=2
    FckIntN2 = 1,
    ///2: fSAMPLING=fCK_INT, N=4
    FckIntN4 = 2,
    ///3: fSAMPLING=fCK_INT, N=8
    FckIntN8 = 3,
    ///4: fSAMPLING=fDTS/2, N=6
    FdtsDiv2N6 = 4,
    ///5: fSAMPLING=fDTS/2, N=8
    FdtsDiv2N8 = 5,
    ///6: fSAMPLING=fDTS/4, N=6
    FdtsDiv4N6 = 6,
    ///7: fSAMPLING=fDTS/4, N=8
    FdtsDiv4N8 = 7,
    ///8: fSAMPLING=fDTS/8, N=6
    FdtsDiv8N6 = 8,
    ///9: fSAMPLING=fDTS/8, N=8
    FdtsDiv8N8 = 9,
    ///10: fSAMPLING=fDTS/16, N=5
    FdtsDiv16N5 = 10,
    ///11: fSAMPLING=fDTS/16, N=6
    FdtsDiv16N6 = 11,
    ///12: fSAMPLING=fDTS/16, N=8
    FdtsDiv16N8 = 12,
    ///13: fSAMPLING=fDTS/32, N=5
    FdtsDiv32N5 = 13,
    ///14: fSAMPLING=fDTS/32, N=6
    FdtsDiv32N6 = 14,
    ///15: fSAMPLING=fDTS/32, N=8
    FdtsDiv32N8 = 15,
}
impl From<BK2F_A> for u8 {
    #[inline(always)]
    fn from(variant: BK2F_A) -> Self {
        variant as _
    }
}
impl BK2F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK2F_A {
        match self.bits {
            0 => BK2F_A::NoFilter,
            1 => BK2F_A::FckIntN2,
            2 => BK2F_A::FckIntN4,
            3 => BK2F_A::FckIntN8,
            4 => BK2F_A::FdtsDiv2N6,
            5 => BK2F_A::FdtsDiv2N8,
            6 => BK2F_A::FdtsDiv4N6,
            7 => BK2F_A::FdtsDiv4N8,
            8 => BK2F_A::FdtsDiv8N6,
            9 => BK2F_A::FdtsDiv8N8,
            10 => BK2F_A::FdtsDiv16N5,
            11 => BK2F_A::FdtsDiv16N6,
            12 => BK2F_A::FdtsDiv16N8,
            13 => BK2F_A::FdtsDiv32N5,
            14 => BK2F_A::FdtsDiv32N6,
            15 => BK2F_A::FdtsDiv32N8,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `NoFilter`
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == BK2F_A::NoFilter
    }
    ///Checks if the value of the field is `FckIntN2`
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == BK2F_A::FckIntN2
    }
    ///Checks if the value of the field is `FckIntN4`
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == BK2F_A::FckIntN4
    }
    ///Checks if the value of the field is `FckIntN8`
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == BK2F_A::FckIntN8
    }
    ///Checks if the value of the field is `FdtsDiv2N6`
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == BK2F_A::FdtsDiv2N6
    }
    ///Checks if the value of the field is `FdtsDiv2N8`
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == BK2F_A::FdtsDiv2N8
    }
    ///Checks if the value of the field is `FdtsDiv4N6`
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == BK2F_A::FdtsDiv4N6
    }
    ///Checks if the value of the field is `FdtsDiv4N8`
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == BK2F_A::FdtsDiv4N8
    }
    ///Checks if the value of the field is `FdtsDiv8N6`
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == BK2F_A::FdtsDiv8N6
    }
    ///Checks if the value of the field is `FdtsDiv8N8`
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == BK2F_A::FdtsDiv8N8
    }
    ///Checks if the value of the field is `FdtsDiv16N5`
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == BK2F_A::FdtsDiv16N5
    }
    ///Checks if the value of the field is `FdtsDiv16N6`
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == BK2F_A::FdtsDiv16N6
    }
    ///Checks if the value of the field is `FdtsDiv16N8`
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == BK2F_A::FdtsDiv16N8
    }
    ///Checks if the value of the field is `FdtsDiv32N5`
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == BK2F_A::FdtsDiv32N5
    }
    ///Checks if the value of the field is `FdtsDiv32N6`
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == BK2F_A::FdtsDiv32N6
    }
    ///Checks if the value of the field is `FdtsDiv32N8`
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == BK2F_A::FdtsDiv32N8
    }
}
///Field `BK2F` writer - Break 2 filter
pub type BK2F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BDTR_SPEC, u8, BK2F_A, 4, O>;
impl<'a, const O: u8> BK2F_W<'a, O> {
    ///No filter, sampling is done at fDTS
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(BK2F_A::NoFilter)
    }
    ///fSAMPLING=fCK_INT, N=2
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(BK2F_A::FckIntN2)
    }
    ///fSAMPLING=fCK_INT, N=4
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(BK2F_A::FckIntN4)
    }
    ///fSAMPLING=fCK_INT, N=8
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FckIntN8)
    }
    ///fSAMPLING=fDTS/2, N=6
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv2N6)
    }
    ///fSAMPLING=fDTS/2, N=8
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv2N8)
    }
    ///fSAMPLING=fDTS/4, N=6
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv4N6)
    }
    ///fSAMPLING=fDTS/4, N=8
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv4N8)
    }
    ///fSAMPLING=fDTS/8, N=6
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv8N6)
    }
    ///fSAMPLING=fDTS/8, N=8
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv8N8)
    }
    ///fSAMPLING=fDTS/16, N=5
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv16N5)
    }
    ///fSAMPLING=fDTS/16, N=6
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv16N6)
    }
    ///fSAMPLING=fDTS/16, N=8
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv16N8)
    }
    ///fSAMPLING=fDTS/32, N=5
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv32N5)
    }
    ///fSAMPLING=fDTS/32, N=6
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv32N6)
    }
    ///fSAMPLING=fDTS/32, N=8
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(BK2F_A::FdtsDiv32N8)
    }
}
///Field `BK2E` reader - Break 2 enable
pub type BK2E_R = crate::BitReader<BK2E_A>;
///Break 2 enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2E_A {
    ///0: Break function disabled
    Disabled = 0,
    ///1: Break function enabled
    Enabled = 1,
}
impl From<BK2E_A> for bool {
    #[inline(always)]
    fn from(variant: BK2E_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK2E_A {
        match self.bits {
            false => BK2E_A::Disabled,
            true => BK2E_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BK2E_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BK2E_A::Enabled
    }
}
///Field `BK2E` writer - Break 2 enable
pub type BK2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BK2E_A, O>;
impl<'a, const O: u8> BK2E_W<'a, O> {
    ///Break function disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BK2E_A::Disabled)
    }
    ///Break function enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BK2E_A::Enabled)
    }
}
///Field `BK2P` reader - Break 2 polarity
pub type BK2P_R = crate::BitReader<BK2P_A>;
///Break 2 polarity
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2P_A {
    ///0: Break input BRK2 is active low
    Low = 0,
    ///1: Break input BRK2 is active high
    High = 1,
}
impl From<BK2P_A> for bool {
    #[inline(always)]
    fn from(variant: BK2P_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2P_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK2P_A {
        match self.bits {
            false => BK2P_A::Low,
            true => BK2P_A::High,
        }
    }
    ///Checks if the value of the field is `Low`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == BK2P_A::Low
    }
    ///Checks if the value of the field is `High`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == BK2P_A::High
    }
}
///Field `BK2P` writer - Break 2 polarity
pub type BK2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BK2P_A, O>;
impl<'a, const O: u8> BK2P_W<'a, O> {
    ///Break input BRK2 is active low
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(BK2P_A::Low)
    }
    ///Break input BRK2 is active high
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(BK2P_A::High)
    }
}
///Field `BKDSRM` reader - BKDSRM
pub type BKDSRM_R = crate::BitReader<BKDSRM_A>;
///BKDSRM
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKDSRM_A {
    ///0: Break input BRK is armed
    Armed = 0,
    ///1: Break input BRK is disarmed
    Disarmed = 1,
}
impl From<BKDSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BKDSRM_A) -> Self {
        variant as u8 != 0
    }
}
impl BKDSRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKDSRM_A {
        match self.bits {
            false => BKDSRM_A::Armed,
            true => BKDSRM_A::Disarmed,
        }
    }
    ///Checks if the value of the field is `Armed`
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BKDSRM_A::Armed
    }
    ///Checks if the value of the field is `Disarmed`
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BKDSRM_A::Disarmed
    }
}
///Field `BKDSRM` writer - BKDSRM
pub type BKDSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKDSRM_A, O>;
impl<'a, const O: u8> BKDSRM_W<'a, O> {
    ///Break input BRK is armed
    #[inline(always)]
    pub fn armed(self) -> &'a mut W {
        self.variant(BKDSRM_A::Armed)
    }
    ///Break input BRK is disarmed
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut W {
        self.variant(BKDSRM_A::Disarmed)
    }
}
///Field `BK2DSRM` reader - Break2 Disarm
pub type BK2DSRM_R = crate::BitReader<BK2DSRM_A>;
///Break2 Disarm
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2DSRM_A {
    ///0: Break input BRK2 is armed
    Armed = 0,
    ///1: Break input BRK2 is disarmed
    Disarmed = 1,
}
impl From<BK2DSRM_A> for bool {
    #[inline(always)]
    fn from(variant: BK2DSRM_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2DSRM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK2DSRM_A {
        match self.bits {
            false => BK2DSRM_A::Armed,
            true => BK2DSRM_A::Disarmed,
        }
    }
    ///Checks if the value of the field is `Armed`
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == BK2DSRM_A::Armed
    }
    ///Checks if the value of the field is `Disarmed`
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == BK2DSRM_A::Disarmed
    }
}
///Field `BK2DSRM` writer - Break2 Disarm
pub type BK2DSRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BK2DSRM_A, O>;
impl<'a, const O: u8> BK2DSRM_W<'a, O> {
    ///Break input BRK2 is armed
    #[inline(always)]
    pub fn armed(self) -> &'a mut W {
        self.variant(BK2DSRM_A::Armed)
    }
    ///Break input BRK2 is disarmed
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut W {
        self.variant(BK2DSRM_A::Disarmed)
    }
}
///Field `BKBID` reader - BKBID
pub type BKBID_R = crate::BitReader<BKBID_A>;
///BKBID
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKBID_A {
    ///0: Break input BRK in input mode
    Input = 0,
    ///1: Break input BRK in bidirectional mode
    Bidirectional = 1,
}
impl From<BKBID_A> for bool {
    #[inline(always)]
    fn from(variant: BKBID_A) -> Self {
        variant as u8 != 0
    }
}
impl BKBID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BKBID_A {
        match self.bits {
            false => BKBID_A::Input,
            true => BKBID_A::Bidirectional,
        }
    }
    ///Checks if the value of the field is `Input`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BKBID_A::Input
    }
    ///Checks if the value of the field is `Bidirectional`
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BKBID_A::Bidirectional
    }
}
///Field `BKBID` writer - BKBID
pub type BKBID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BKBID_A, O>;
impl<'a, const O: u8> BKBID_W<'a, O> {
    ///Break input BRK in input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(BKBID_A::Input)
    }
    ///Break input BRK in bidirectional mode
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BKBID_A::Bidirectional)
    }
}
///Field `BK2BID` reader - Break2 bidirectional
pub type BK2BID_R = crate::BitReader<BK2BID_A>;
///Break2 bidirectional
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BK2BID_A {
    ///0: Break input BRK2 in input mode
    Input = 0,
    ///1: Break input BRK2 in bidirectional mode
    Bidirectional = 1,
}
impl From<BK2BID_A> for bool {
    #[inline(always)]
    fn from(variant: BK2BID_A) -> Self {
        variant as u8 != 0
    }
}
impl BK2BID_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> BK2BID_A {
        match self.bits {
            false => BK2BID_A::Input,
            true => BK2BID_A::Bidirectional,
        }
    }
    ///Checks if the value of the field is `Input`
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == BK2BID_A::Input
    }
    ///Checks if the value of the field is `Bidirectional`
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BK2BID_A::Bidirectional
    }
}
///Field `BK2BID` writer - Break2 bidirectional
pub type BK2BID_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, BK2BID_A, O>;
impl<'a, const O: u8> BK2BID_W<'a, O> {
    ///Break input BRK2 in input mode
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(BK2BID_A::Input)
    }
    ///Break input BRK2 in bidirectional mode
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BK2BID_A::Bidirectional)
    }
}
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    pub fn bkdsrm(&self) -> BKDSRM_R {
        BKDSRM_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    pub fn bk2dsrm(&self) -> BK2DSRM_R {
        BK2DSRM_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    pub fn bkbid(&self) -> BKBID_R {
        BKBID_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    pub fn bk2bid(&self) -> BK2BID_R {
        BK2BID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<16> {
        BKF_W::new(self)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<20> {
        BK2F_W::new(self)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<24> {
        BK2E_W::new(self)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<25> {
        BK2P_W::new(self)
    }
    ///Bit 26 - BKDSRM
    #[inline(always)]
    #[must_use]
    pub fn bkdsrm(&mut self) -> BKDSRM_W<26> {
        BKDSRM_W::new(self)
    }
    ///Bit 27 - Break2 Disarm
    #[inline(always)]
    #[must_use]
    pub fn bk2dsrm(&mut self) -> BK2DSRM_W<27> {
        BK2DSRM_W::new(self)
    }
    ///Bit 28 - BKBID
    #[inline(always)]
    #[must_use]
    pub fn bkbid(&mut self) -> BKBID_W<28> {
        BKBID_W::new(self)
    }
    ///Bit 29 - Break2 bidirectional
    #[inline(always)]
    #[must_use]
    pub fn bk2bid(&mut self) -> BK2BID_W<29> {
        BK2BID_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///break and dead-time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtr](index.html) module
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdtr::R](R) reader structure
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdtr::W](W) writer structure
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
