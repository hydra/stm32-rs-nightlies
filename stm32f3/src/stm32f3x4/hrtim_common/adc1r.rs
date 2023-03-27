///Register `ADC1R` reader
pub struct R(crate::R<ADC1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ADC1R` writer
pub struct W(crate::W<ADC1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1R_SPEC>;
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
impl From<crate::W<ADC1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AD1MC1` reader - ADC trigger 1 on Master Compare 1
pub type AD1MC1_R = crate::BitReader<AD1MC1_A>;
///ADC trigger 1 on Master Compare 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MC1_A {
    ///0: No generation of ADC trigger on master compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on master compare event
    Enabled = 1,
}
impl From<AD1MC1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MC1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1MC1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AD1MC1_A {
        match self.bits {
            false => AD1MC1_A::Disabled,
            true => AD1MC1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MC1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MC1_A::Enabled
    }
}
///Field `AD1MC1` writer - ADC trigger 1 on Master Compare 1
pub type AD1MC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1R_SPEC, AD1MC1_A, O>;
impl<'a, const O: u8> AD1MC1_W<'a, O> {
    ///No generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::Disabled)
    }
    ///Generation of ADC trigger on master compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MC1_A::Enabled)
    }
}
///Field `AD1MC2` reader - ADC trigger 1 on Master Compare 2
pub use AD1MC1_R as AD1MC2_R;
///Field `AD1MC3` reader - ADC trigger 1 on Master Compare 3
pub use AD1MC1_R as AD1MC3_R;
///Field `AD1MC4` reader - ADC trigger 1 on Master Compare 4
pub use AD1MC1_R as AD1MC4_R;
///Field `AD1MC2` writer - ADC trigger 1 on Master Compare 2
pub use AD1MC1_W as AD1MC2_W;
///Field `AD1MC3` writer - ADC trigger 1 on Master Compare 3
pub use AD1MC1_W as AD1MC3_W;
///Field `AD1MC4` writer - ADC trigger 1 on Master Compare 4
pub use AD1MC1_W as AD1MC4_W;
///Field `AD1MPER` reader - ADC trigger 1 on Master Period
pub type AD1MPER_R = crate::BitReader<AD1MPER_A>;
///ADC trigger 1 on Master Period
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1MPER_A {
    ///0: No generation of ADC trigger on timer period event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer period event
    Enabled = 1,
}
impl From<AD1MPER_A> for bool {
    #[inline(always)]
    fn from(variant: AD1MPER_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1MPER_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AD1MPER_A {
        match self.bits {
            false => AD1MPER_A::Disabled,
            true => AD1MPER_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1MPER_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1MPER_A::Enabled
    }
}
///Field `AD1MPER` writer - ADC trigger 1 on Master Period
pub type AD1MPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1R_SPEC, AD1MPER_A, O>;
impl<'a, const O: u8> AD1MPER_W<'a, O> {
    ///No generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::Disabled)
    }
    ///Generation of ADC trigger on timer period event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1MPER_A::Enabled)
    }
}
///Field `AD1EEV1` reader - ADC trigger 1 on External Event 1
pub type AD1EEV1_R = crate::BitReader<AD1EEV1_A>;
///ADC trigger 1 on External Event 1
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1EEV1_A {
    ///0: No generation of ADC trigger on external event
    Disabled = 0,
    ///1: Generation of ADC trigger on external event
    Enabled = 1,
}
impl From<AD1EEV1_A> for bool {
    #[inline(always)]
    fn from(variant: AD1EEV1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1EEV1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AD1EEV1_A {
        match self.bits {
            false => AD1EEV1_A::Disabled,
            true => AD1EEV1_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1EEV1_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1EEV1_A::Enabled
    }
}
///Field `AD1EEV1` writer - ADC trigger 1 on External Event 1
pub type AD1EEV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1R_SPEC, AD1EEV1_A, O>;
impl<'a, const O: u8> AD1EEV1_W<'a, O> {
    ///No generation of ADC trigger on external event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::Disabled)
    }
    ///Generation of ADC trigger on external event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1EEV1_A::Enabled)
    }
}
///Field `AD1EEV2` reader - ADC trigger 1 on External Event 2
pub use AD1EEV1_R as AD1EEV2_R;
///Field `AD1EEV3` reader - ADC trigger 1 on External Event 3
pub use AD1EEV1_R as AD1EEV3_R;
///Field `AD1EEV4` reader - ADC trigger 1 on External Event 4
pub use AD1EEV1_R as AD1EEV4_R;
///Field `AD1EEV5` reader - ADC trigger 1 on External Event 5
pub use AD1EEV1_R as AD1EEV5_R;
///Field `AD1EEV2` writer - ADC trigger 1 on External Event 2
pub use AD1EEV1_W as AD1EEV2_W;
///Field `AD1EEV3` writer - ADC trigger 1 on External Event 3
pub use AD1EEV1_W as AD1EEV3_W;
///Field `AD1EEV4` writer - ADC trigger 1 on External Event 4
pub use AD1EEV1_W as AD1EEV4_W;
///Field `AD1EEV5` writer - ADC trigger 1 on External Event 5
pub use AD1EEV1_W as AD1EEV5_W;
///Field `AD1TAC2` reader - ADC trigger 1 on Timer A compare 2
pub type AD1TAC2_R = crate::BitReader<AD1TAC2_A>;
///ADC trigger 1 on Timer A compare 2
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TAC2_A {
    ///0: No generation of ADC trigger on timer compare event
    Disabled = 0,
    ///1: Generation of ADC trigger on timer compare event
    Enabled = 1,
}
impl From<AD1TAC2_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TAC2_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1TAC2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AD1TAC2_A {
        match self.bits {
            false => AD1TAC2_A::Disabled,
            true => AD1TAC2_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TAC2_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TAC2_A::Enabled
    }
}
///Field `AD1TAC2` writer - ADC trigger 1 on Timer A compare 2
pub type AD1TAC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1R_SPEC, AD1TAC2_A, O>;
impl<'a, const O: u8> AD1TAC2_W<'a, O> {
    ///No generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::Disabled)
    }
    ///Generation of ADC trigger on timer compare event
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TAC2_A::Enabled)
    }
}
///Field `AD1TAPER` reader - ADC trigger 1 on Timer A Period
pub use AD1MPER_R as AD1TAPER_R;
///Field `AD1TAPER` writer - ADC trigger 1 on Timer A Period
pub use AD1MPER_W as AD1TAPER_W;
///Field `AD1TAC3` reader - ADC trigger 1 on Timer A compare 3
pub use AD1TAC2_R as AD1TAC3_R;
///Field `AD1TAC4` reader - ADC trigger 1 on Timer A compare 4
pub use AD1TAC2_R as AD1TAC4_R;
///Field `AD1TAC3` writer - ADC trigger 1 on Timer A compare 3
pub use AD1TAC2_W as AD1TAC3_W;
///Field `AD1TAC4` writer - ADC trigger 1 on Timer A compare 4
pub use AD1TAC2_W as AD1TAC4_W;
///Field `AD1TARST` reader - ADC trigger 1 on Timer A Reset
pub type AD1TARST_R = crate::BitReader<AD1TARST_A>;
///ADC trigger 1 on Timer A Reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AD1TARST_A {
    ///0: No generation of ADC trigger on timer reset and roll-over
    Disabled = 0,
    ///1: Generation of ADC trigger on timer reset and roll-over
    Enabled = 1,
}
impl From<AD1TARST_A> for bool {
    #[inline(always)]
    fn from(variant: AD1TARST_A) -> Self {
        variant as u8 != 0
    }
}
impl AD1TARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AD1TARST_A {
        match self.bits {
            false => AD1TARST_A::Disabled,
            true => AD1TARST_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD1TARST_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD1TARST_A::Enabled
    }
}
///Field `AD1TARST` writer - ADC trigger 1 on Timer A Reset
pub type AD1TARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC1R_SPEC, AD1TARST_A, O>;
impl<'a, const O: u8> AD1TARST_W<'a, O> {
    ///No generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::Disabled)
    }
    ///Generation of ADC trigger on timer reset and roll-over
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD1TARST_A::Enabled)
    }
}
///Field `AD1TBPER` reader - ADC trigger 1 on Timer B Period
pub use AD1MPER_R as AD1TBPER_R;
///Field `AD1TCPER` reader - ADC trigger 1 on Timer C Period
pub use AD1MPER_R as AD1TCPER_R;
///Field `AD1TDPER` reader - ADC trigger 1 on Timer D Period
pub use AD1MPER_R as AD1TDPER_R;
///Field `AD1TEPER` reader - ADC trigger 1 on Timer E Period
pub use AD1MPER_R as AD1TEPER_R;
///Field `AD1TBPER` writer - ADC trigger 1 on Timer B Period
pub use AD1MPER_W as AD1TBPER_W;
///Field `AD1TCPER` writer - ADC trigger 1 on Timer C Period
pub use AD1MPER_W as AD1TCPER_W;
///Field `AD1TDPER` writer - ADC trigger 1 on Timer D Period
pub use AD1MPER_W as AD1TDPER_W;
///Field `AD1TEPER` writer - ADC trigger 1 on Timer E Period
pub use AD1MPER_W as AD1TEPER_W;
///Field `AD1TBC2` reader - ADC trigger 1 on Timer B compare 2
pub use AD1TAC2_R as AD1TBC2_R;
///Field `AD1TBC3` reader - ADC trigger 1 on Timer B compare 3
pub use AD1TAC2_R as AD1TBC3_R;
///Field `AD1TBC4` reader - ADC trigger 1 on Timer B compare 4
pub use AD1TAC2_R as AD1TBC4_R;
///Field `AD1TCC2` reader - ADC trigger 1 on Timer C compare 2
pub use AD1TAC2_R as AD1TCC2_R;
///Field `AD1TCC3` reader - ADC trigger 1 on Timer C compare 3
pub use AD1TAC2_R as AD1TCC3_R;
///Field `AD1TCC4` reader - ADC trigger 1 on Timer C compare 4
pub use AD1TAC2_R as AD1TCC4_R;
///Field `AD1TDC2` reader - ADC trigger 1 on Timer D compare 2
pub use AD1TAC2_R as AD1TDC2_R;
///Field `AD1TDC3` reader - ADC trigger 1 on Timer D compare 3
pub use AD1TAC2_R as AD1TDC3_R;
///Field `AD1TDC4` reader - ADC trigger 1 on Timer D compare 4
pub use AD1TAC2_R as AD1TDC4_R;
///Field `AD1TEC2` reader - ADC trigger 1 on Timer E compare 2
pub use AD1TAC2_R as AD1TEC2_R;
///Field `AD1TEC3` reader - ADC trigger 1 on Timer E compare 3
pub use AD1TAC2_R as AD1TEC3_R;
///Field `AD1TEC4` reader - ADC trigger 1 on Timer E compare 4
pub use AD1TAC2_R as AD1TEC4_R;
///Field `AD1TBC2` writer - ADC trigger 1 on Timer B compare 2
pub use AD1TAC2_W as AD1TBC2_W;
///Field `AD1TBC3` writer - ADC trigger 1 on Timer B compare 3
pub use AD1TAC2_W as AD1TBC3_W;
///Field `AD1TBC4` writer - ADC trigger 1 on Timer B compare 4
pub use AD1TAC2_W as AD1TBC4_W;
///Field `AD1TCC2` writer - ADC trigger 1 on Timer C compare 2
pub use AD1TAC2_W as AD1TCC2_W;
///Field `AD1TCC3` writer - ADC trigger 1 on Timer C compare 3
pub use AD1TAC2_W as AD1TCC3_W;
///Field `AD1TCC4` writer - ADC trigger 1 on Timer C compare 4
pub use AD1TAC2_W as AD1TCC4_W;
///Field `AD1TDC2` writer - ADC trigger 1 on Timer D compare 2
pub use AD1TAC2_W as AD1TDC2_W;
///Field `AD1TDC3` writer - ADC trigger 1 on Timer D compare 3
pub use AD1TAC2_W as AD1TDC3_W;
///Field `AD1TDC4` writer - ADC trigger 1 on Timer D compare 4
pub use AD1TAC2_W as AD1TDC4_W;
///Field `AD1TEC2` writer - ADC trigger 1 on Timer E compare 2
pub use AD1TAC2_W as AD1TEC2_W;
///Field `AD1TEC3` writer - ADC trigger 1 on Timer E compare 3
pub use AD1TAC2_W as AD1TEC3_W;
///Field `AD1TEC4` writer - ADC trigger 1 on Timer E compare 4
pub use AD1TAC2_W as AD1TEC4_W;
///Field `AD1TBRST` reader - ADC trigger 1 on Timer B Reset
pub use AD1TARST_R as AD1TBRST_R;
///Field `AD1TBRST` writer - ADC trigger 1 on Timer B Reset
pub use AD1TARST_W as AD1TBRST_W;
impl R {
    ///Bit 0 - ADC trigger 1 on Master Compare 1
    #[inline(always)]
    pub fn ad1mc1(&self) -> AD1MC1_R {
        AD1MC1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC trigger 1 on Master Compare 2
    #[inline(always)]
    pub fn ad1mc2(&self) -> AD1MC2_R {
        AD1MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC trigger 1 on Master Compare 3
    #[inline(always)]
    pub fn ad1mc3(&self) -> AD1MC3_R {
        AD1MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC trigger 1 on Master Compare 4
    #[inline(always)]
    pub fn ad1mc4(&self) -> AD1MC4_R {
        AD1MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC trigger 1 on Master Period
    #[inline(always)]
    pub fn ad1mper(&self) -> AD1MPER_R {
        AD1MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC trigger 1 on External Event 1
    #[inline(always)]
    pub fn ad1eev1(&self) -> AD1EEV1_R {
        AD1EEV1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC trigger 1 on External Event 2
    #[inline(always)]
    pub fn ad1eev2(&self) -> AD1EEV2_R {
        AD1EEV2_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC trigger 1 on External Event 3
    #[inline(always)]
    pub fn ad1eev3(&self) -> AD1EEV3_R {
        AD1EEV3_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC trigger 1 on External Event 4
    #[inline(always)]
    pub fn ad1eev4(&self) -> AD1EEV4_R {
        AD1EEV4_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC trigger 1 on External Event 5
    #[inline(always)]
    pub fn ad1eev5(&self) -> AD1EEV5_R {
        AD1EEV5_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC trigger 1 on Timer A compare 2
    #[inline(always)]
    pub fn ad1tac2(&self) -> AD1TAC2_R {
        AD1TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - ADC trigger 1 on Timer A compare 3
    #[inline(always)]
    pub fn ad1tac3(&self) -> AD1TAC3_R {
        AD1TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - ADC trigger 1 on Timer A compare 4
    #[inline(always)]
    pub fn ad1tac4(&self) -> AD1TAC4_R {
        AD1TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC trigger 1 on Timer A Period
    #[inline(always)]
    pub fn ad1taper(&self) -> AD1TAPER_R {
        AD1TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - ADC trigger 1 on Timer A Reset
    #[inline(always)]
    pub fn ad1tarst(&self) -> AD1TARST_R {
        AD1TARST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - ADC trigger 1 on Timer B compare 2
    #[inline(always)]
    pub fn ad1tbc2(&self) -> AD1TBC2_R {
        AD1TBC2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - ADC trigger 1 on Timer B compare 3
    #[inline(always)]
    pub fn ad1tbc3(&self) -> AD1TBC3_R {
        AD1TBC3_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - ADC trigger 1 on Timer B compare 4
    #[inline(always)]
    pub fn ad1tbc4(&self) -> AD1TBC4_R {
        AD1TBC4_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ADC trigger 1 on Timer B Period
    #[inline(always)]
    pub fn ad1tbper(&self) -> AD1TBPER_R {
        AD1TBPER_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ADC trigger 1 on Timer B Reset
    #[inline(always)]
    pub fn ad1tbrst(&self) -> AD1TBRST_R {
        AD1TBRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - ADC trigger 1 on Timer C compare 2
    #[inline(always)]
    pub fn ad1tcc2(&self) -> AD1TCC2_R {
        AD1TCC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ADC trigger 1 on Timer C compare 3
    #[inline(always)]
    pub fn ad1tcc3(&self) -> AD1TCC3_R {
        AD1TCC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ADC trigger 1 on Timer C compare 4
    #[inline(always)]
    pub fn ad1tcc4(&self) -> AD1TCC4_R {
        AD1TCC4_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ADC trigger 1 on Timer C Period
    #[inline(always)]
    pub fn ad1tcper(&self) -> AD1TCPER_R {
        AD1TCPER_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - ADC trigger 1 on Timer D compare 2
    #[inline(always)]
    pub fn ad1tdc2(&self) -> AD1TDC2_R {
        AD1TDC2_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ADC trigger 1 on Timer D compare 3
    #[inline(always)]
    pub fn ad1tdc3(&self) -> AD1TDC3_R {
        AD1TDC3_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - ADC trigger 1 on Timer D compare 4
    #[inline(always)]
    pub fn ad1tdc4(&self) -> AD1TDC4_R {
        AD1TDC4_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - ADC trigger 1 on Timer D Period
    #[inline(always)]
    pub fn ad1tdper(&self) -> AD1TDPER_R {
        AD1TDPER_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - ADC trigger 1 on Timer E compare 2
    #[inline(always)]
    pub fn ad1tec2(&self) -> AD1TEC2_R {
        AD1TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC trigger 1 on Timer E compare 3
    #[inline(always)]
    pub fn ad1tec3(&self) -> AD1TEC3_R {
        AD1TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADC trigger 1 on Timer E compare 4
    #[inline(always)]
    pub fn ad1tec4(&self) -> AD1TEC4_R {
        AD1TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC trigger 1 on Timer E Period
    #[inline(always)]
    pub fn ad1teper(&self) -> AD1TEPER_R {
        AD1TEPER_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADC trigger 1 on Master Compare 1
    #[inline(always)]
    #[must_use]
    pub fn ad1mc1(&mut self) -> AD1MC1_W<0> {
        AD1MC1_W::new(self)
    }
    ///Bit 1 - ADC trigger 1 on Master Compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1mc2(&mut self) -> AD1MC2_W<1> {
        AD1MC2_W::new(self)
    }
    ///Bit 2 - ADC trigger 1 on Master Compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1mc3(&mut self) -> AD1MC3_W<2> {
        AD1MC3_W::new(self)
    }
    ///Bit 3 - ADC trigger 1 on Master Compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1mc4(&mut self) -> AD1MC4_W<3> {
        AD1MC4_W::new(self)
    }
    ///Bit 4 - ADC trigger 1 on Master Period
    #[inline(always)]
    #[must_use]
    pub fn ad1mper(&mut self) -> AD1MPER_W<4> {
        AD1MPER_W::new(self)
    }
    ///Bit 5 - ADC trigger 1 on External Event 1
    #[inline(always)]
    #[must_use]
    pub fn ad1eev1(&mut self) -> AD1EEV1_W<5> {
        AD1EEV1_W::new(self)
    }
    ///Bit 6 - ADC trigger 1 on External Event 2
    #[inline(always)]
    #[must_use]
    pub fn ad1eev2(&mut self) -> AD1EEV2_W<6> {
        AD1EEV2_W::new(self)
    }
    ///Bit 7 - ADC trigger 1 on External Event 3
    #[inline(always)]
    #[must_use]
    pub fn ad1eev3(&mut self) -> AD1EEV3_W<7> {
        AD1EEV3_W::new(self)
    }
    ///Bit 8 - ADC trigger 1 on External Event 4
    #[inline(always)]
    #[must_use]
    pub fn ad1eev4(&mut self) -> AD1EEV4_W<8> {
        AD1EEV4_W::new(self)
    }
    ///Bit 9 - ADC trigger 1 on External Event 5
    #[inline(always)]
    #[must_use]
    pub fn ad1eev5(&mut self) -> AD1EEV5_W<9> {
        AD1EEV5_W::new(self)
    }
    ///Bit 10 - ADC trigger 1 on Timer A compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1tac2(&mut self) -> AD1TAC2_W<10> {
        AD1TAC2_W::new(self)
    }
    ///Bit 11 - ADC trigger 1 on Timer A compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1tac3(&mut self) -> AD1TAC3_W<11> {
        AD1TAC3_W::new(self)
    }
    ///Bit 12 - ADC trigger 1 on Timer A compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1tac4(&mut self) -> AD1TAC4_W<12> {
        AD1TAC4_W::new(self)
    }
    ///Bit 13 - ADC trigger 1 on Timer A Period
    #[inline(always)]
    #[must_use]
    pub fn ad1taper(&mut self) -> AD1TAPER_W<13> {
        AD1TAPER_W::new(self)
    }
    ///Bit 14 - ADC trigger 1 on Timer A Reset
    #[inline(always)]
    #[must_use]
    pub fn ad1tarst(&mut self) -> AD1TARST_W<14> {
        AD1TARST_W::new(self)
    }
    ///Bit 15 - ADC trigger 1 on Timer B compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc2(&mut self) -> AD1TBC2_W<15> {
        AD1TBC2_W::new(self)
    }
    ///Bit 16 - ADC trigger 1 on Timer B compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc3(&mut self) -> AD1TBC3_W<16> {
        AD1TBC3_W::new(self)
    }
    ///Bit 17 - ADC trigger 1 on Timer B compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1tbc4(&mut self) -> AD1TBC4_W<17> {
        AD1TBC4_W::new(self)
    }
    ///Bit 18 - ADC trigger 1 on Timer B Period
    #[inline(always)]
    #[must_use]
    pub fn ad1tbper(&mut self) -> AD1TBPER_W<18> {
        AD1TBPER_W::new(self)
    }
    ///Bit 19 - ADC trigger 1 on Timer B Reset
    #[inline(always)]
    #[must_use]
    pub fn ad1tbrst(&mut self) -> AD1TBRST_W<19> {
        AD1TBRST_W::new(self)
    }
    ///Bit 20 - ADC trigger 1 on Timer C compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc2(&mut self) -> AD1TCC2_W<20> {
        AD1TCC2_W::new(self)
    }
    ///Bit 21 - ADC trigger 1 on Timer C compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc3(&mut self) -> AD1TCC3_W<21> {
        AD1TCC3_W::new(self)
    }
    ///Bit 22 - ADC trigger 1 on Timer C compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1tcc4(&mut self) -> AD1TCC4_W<22> {
        AD1TCC4_W::new(self)
    }
    ///Bit 23 - ADC trigger 1 on Timer C Period
    #[inline(always)]
    #[must_use]
    pub fn ad1tcper(&mut self) -> AD1TCPER_W<23> {
        AD1TCPER_W::new(self)
    }
    ///Bit 24 - ADC trigger 1 on Timer D compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc2(&mut self) -> AD1TDC2_W<24> {
        AD1TDC2_W::new(self)
    }
    ///Bit 25 - ADC trigger 1 on Timer D compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc3(&mut self) -> AD1TDC3_W<25> {
        AD1TDC3_W::new(self)
    }
    ///Bit 26 - ADC trigger 1 on Timer D compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1tdc4(&mut self) -> AD1TDC4_W<26> {
        AD1TDC4_W::new(self)
    }
    ///Bit 27 - ADC trigger 1 on Timer D Period
    #[inline(always)]
    #[must_use]
    pub fn ad1tdper(&mut self) -> AD1TDPER_W<27> {
        AD1TDPER_W::new(self)
    }
    ///Bit 28 - ADC trigger 1 on Timer E compare 2
    #[inline(always)]
    #[must_use]
    pub fn ad1tec2(&mut self) -> AD1TEC2_W<28> {
        AD1TEC2_W::new(self)
    }
    ///Bit 29 - ADC trigger 1 on Timer E compare 3
    #[inline(always)]
    #[must_use]
    pub fn ad1tec3(&mut self) -> AD1TEC3_W<29> {
        AD1TEC3_W::new(self)
    }
    ///Bit 30 - ADC trigger 1 on Timer E compare 4
    #[inline(always)]
    #[must_use]
    pub fn ad1tec4(&mut self) -> AD1TEC4_W<30> {
        AD1TEC4_W::new(self)
    }
    ///Bit 31 - ADC trigger 1 on Timer E Period
    #[inline(always)]
    #[must_use]
    pub fn ad1teper(&mut self) -> AD1TEPER_W<31> {
        AD1TEPER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC Trigger 1 Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [adc1r](index.html) module
pub struct ADC1R_SPEC;
impl crate::RegisterSpec for ADC1R_SPEC {
    type Ux = u32;
}
///`read()` method returns [adc1r::R](R) reader structure
impl crate::Readable for ADC1R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [adc1r::W](W) writer structure
impl crate::Writable for ADC1R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ADC1R to value 0
impl crate::Resettable for ADC1R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
