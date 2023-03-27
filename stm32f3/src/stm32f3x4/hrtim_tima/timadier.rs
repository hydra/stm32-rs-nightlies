///Register `TIMADIER` reader
pub struct R(crate::R<TIMADIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMADIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMADIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMADIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMADIER` writer
pub struct W(crate::W<TIMADIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMADIER_SPEC>;
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
impl From<crate::W<TIMADIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMADIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CMP1IE` reader - CMP1IE
pub type CMP1IE_R = crate::BitReader<CMP1IE_A>;
///CMP1IE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1IE_A {
    ///0: Compare interrupt disabled
    Disabled = 0,
    ///1: Compare interrupt enabled
    Enabled = 1,
}
impl From<CMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMP1IE_A {
        match self.bits {
            false => CMP1IE_A::Disabled,
            true => CMP1IE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1IE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1IE_A::Enabled
    }
}
///Field `CMP1IE` writer - CMP1IE
pub type CMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, CMP1IE_A, O>;
impl<'a, const O: u8> CMP1IE_W<'a, O> {
    ///Compare interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP1IE_A::Disabled)
    }
    ///Compare interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP1IE_A::Enabled)
    }
}
///Field `CMP2IE` reader - CMP2IE
pub use CMP1IE_R as CMP2IE_R;
///Field `CMP3IE` reader - CMP3IE
pub use CMP1IE_R as CMP3IE_R;
///Field `CMP4IE` reader - CMP4IE
pub use CMP1IE_R as CMP4IE_R;
///Field `CMP2IE` writer - CMP2IE
pub use CMP1IE_W as CMP2IE_W;
///Field `CMP3IE` writer - CMP3IE
pub use CMP1IE_W as CMP3IE_W;
///Field `CMP4IE` writer - CMP4IE
pub use CMP1IE_W as CMP4IE_W;
///Field `REPIE` reader - REPIE
pub type REPIE_R = crate::BitReader<REPIE_A>;
///REPIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPIE_A {
    ///0: Repetition interrupt disabled
    Disabled = 0,
    ///1: Repetition interrupt enabled
    Enabled = 1,
}
impl From<REPIE_A> for bool {
    #[inline(always)]
    fn from(variant: REPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REPIE_A {
        match self.bits {
            false => REPIE_A::Disabled,
            true => REPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPIE_A::Enabled
    }
}
///Field `REPIE` writer - REPIE
pub type REPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, REPIE_A, O>;
impl<'a, const O: u8> REPIE_W<'a, O> {
    ///Repetition interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPIE_A::Disabled)
    }
    ///Repetition interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPIE_A::Enabled)
    }
}
///Field `UPDIE` reader - UPDIE
pub type UPDIE_R = crate::BitReader<UPDIE_A>;
///UPDIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDIE_A {
    ///0: Update interrupt disabled
    Disabled = 0,
    ///1: Update interrupt enabled
    Enabled = 1,
}
impl From<UPDIE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPDIE_A {
        match self.bits {
            false => UPDIE_A::Disabled,
            true => UPDIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDIE_A::Enabled
    }
}
///Field `UPDIE` writer - UPDIE
pub type UPDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, UPDIE_A, O>;
impl<'a, const O: u8> UPDIE_W<'a, O> {
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDIE_A::Disabled)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDIE_A::Enabled)
    }
}
///Field `CPT1IE` reader - CPT1IE
pub type CPT1IE_R = crate::BitReader<CPT1IE_A>;
///CPT1IE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1IE_A {
    ///0: Capture interrupt disabled
    Disabled = 0,
    ///1: Capture interrupt enabled
    Enabled = 1,
}
impl From<CPT1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CPT1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPT1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPT1IE_A {
        match self.bits {
            false => CPT1IE_A::Disabled,
            true => CPT1IE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1IE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1IE_A::Enabled
    }
}
///Field `CPT1IE` writer - CPT1IE
pub type CPT1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, CPT1IE_A, O>;
impl<'a, const O: u8> CPT1IE_W<'a, O> {
    ///Capture interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT1IE_A::Disabled)
    }
    ///Capture interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT1IE_A::Enabled)
    }
}
///Field `CPT2IE` reader - CPT2IE
pub use CPT1IE_R as CPT2IE_R;
///Field `CPT2IE` writer - CPT2IE
pub use CPT1IE_W as CPT2IE_W;
///Field `SETx1IE` reader - SET1xIE
pub type SETX1IE_R = crate::BitReader<SETX1IE_A>;
///SET1xIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1IE_A {
    ///0: Tx output set interrupt disabled
    Disabled = 0,
    ///1: Tx output set interrupt enabled
    Enabled = 1,
}
impl From<SETX1IE_A> for bool {
    #[inline(always)]
    fn from(variant: SETX1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl SETX1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SETX1IE_A {
        match self.bits {
            false => SETX1IE_A::Disabled,
            true => SETX1IE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX1IE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX1IE_A::Enabled
    }
}
///Field `SETx1IE` writer - SET1xIE
pub type SETX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, SETX1IE_A, O>;
impl<'a, const O: u8> SETX1IE_W<'a, O> {
    ///Tx output set interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX1IE_A::Disabled)
    }
    ///Tx output set interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX1IE_A::Enabled)
    }
}
///Field `RSTx1IE` reader - RSTx1IE
pub type RSTX1IE_R = crate::BitReader<RSTX1IE_A>;
///RSTx1IE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1IE_A {
    ///0: Tx output reset interrupt disabled
    Disabled = 0,
    ///1: Tx output reset interrupt enabled
    Enabled = 1,
}
impl From<RSTX1IE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTX1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTX1IE_A {
        match self.bits {
            false => RSTX1IE_A::Disabled,
            true => RSTX1IE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX1IE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX1IE_A::Enabled
    }
}
///Field `RSTx1IE` writer - RSTx1IE
pub type RSTX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, RSTX1IE_A, O>;
impl<'a, const O: u8> RSTX1IE_W<'a, O> {
    ///Tx output reset interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX1IE_A::Disabled)
    }
    ///Tx output reset interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX1IE_A::Enabled)
    }
}
///Field `RSTx2IE` reader - RSTx2IE
pub use RSTX1IE_R as RSTX2IE_R;
///Field `RSTx2IE` writer - RSTx2IE
pub use RSTX1IE_W as RSTX2IE_W;
///Field `SETx2IE` reader - SETx2IE
pub use SETX1IE_R as SETX2IE_R;
///Field `SETx2IE` writer - SETx2IE
pub use SETX1IE_W as SETX2IE_W;
///Field `RSTIE` reader - RSTIE
pub type RSTIE_R = crate::BitReader<RSTIE_A>;
///RSTIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTIE_A {
    ///0: Timer x counter/reset roll-over interrupt disabled
    Disabled = 0,
    ///1: Timer x counter/reset roll-over interrupt enabled
    Enabled = 1,
}
impl From<RSTIE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTIE_A {
        match self.bits {
            false => RSTIE_A::Disabled,
            true => RSTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTIE_A::Enabled
    }
}
///Field `RSTIE` writer - RSTIE
pub type RSTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, RSTIE_A, O>;
impl<'a, const O: u8> RSTIE_W<'a, O> {
    ///Timer x counter/reset roll-over interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTIE_A::Disabled)
    }
    ///Timer x counter/reset roll-over interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTIE_A::Enabled)
    }
}
///Field `DLYPRTIE` reader - DLYPRTIE
pub type DLYPRTIE_R = crate::BitReader<DLYPRTIE_A>;
///DLYPRTIE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTIE_A {
    ///0: Delayed protection interrupt disabled
    Disabled = 0,
    ///1: Delayed protection interrupt enabled
    Enabled = 1,
}
impl From<DLYPRTIE_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYPRTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTIE_A {
        match self.bits {
            false => DLYPRTIE_A::Disabled,
            true => DLYPRTIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTIE_A::Enabled
    }
}
///Field `DLYPRTIE` writer - DLYPRTIE
pub type DLYPRTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, DLYPRTIE_A, O>;
impl<'a, const O: u8> DLYPRTIE_W<'a, O> {
    ///Delayed protection interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTIE_A::Disabled)
    }
    ///Delayed protection interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTIE_A::Enabled)
    }
}
///Field `CMP1DE` reader - CMP1DE
pub type CMP1DE_R = crate::BitReader<CMP1DE_A>;
///CMP1DE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1DE_A {
    ///0: Compare DMA request disabled
    Disabled = 0,
    ///1: Compare DMA request enabled
    Enabled = 1,
}
impl From<CMP1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP1DE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CMP1DE_A {
        match self.bits {
            false => CMP1DE_A::Disabled,
            true => CMP1DE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMP1DE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMP1DE_A::Enabled
    }
}
///Field `CMP1DE` writer - CMP1DE
pub type CMP1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, CMP1DE_A, O>;
impl<'a, const O: u8> CMP1DE_W<'a, O> {
    ///Compare DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP1DE_A::Disabled)
    }
    ///Compare DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP1DE_A::Enabled)
    }
}
///Field `CMP2DE` reader - CMP2DE
pub use CMP1DE_R as CMP2DE_R;
///Field `CMP3DE` reader - CMP3DE
pub use CMP1DE_R as CMP3DE_R;
///Field `CMP4DE` reader - CMP4DE
pub use CMP1DE_R as CMP4DE_R;
///Field `CMP2DE` writer - CMP2DE
pub use CMP1DE_W as CMP2DE_W;
///Field `CMP3DE` writer - CMP3DE
pub use CMP1DE_W as CMP3DE_W;
///Field `CMP4DE` writer - CMP4DE
pub use CMP1DE_W as CMP4DE_W;
///Field `REPDE` reader - REPDE
pub type REPDE_R = crate::BitReader<REPDE_A>;
///REPDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REPDE_A {
    ///0: Repetition DMA request disabled
    Disabled = 0,
    ///1: Repetition DMA request enabled
    Enabled = 1,
}
impl From<REPDE_A> for bool {
    #[inline(always)]
    fn from(variant: REPDE_A) -> Self {
        variant as u8 != 0
    }
}
impl REPDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> REPDE_A {
        match self.bits {
            false => REPDE_A::Disabled,
            true => REPDE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPDE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPDE_A::Enabled
    }
}
///Field `REPDE` writer - REPDE
pub type REPDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, REPDE_A, O>;
impl<'a, const O: u8> REPDE_W<'a, O> {
    ///Repetition DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPDE_A::Disabled)
    }
    ///Repetition DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPDE_A::Enabled)
    }
}
///Field `UPDDE` reader - UPDDE
pub type UPDDE_R = crate::BitReader<UPDDE_A>;
///UPDDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDDE_A {
    ///0: Update DMA request disabled
    Disabled = 0,
    ///1: Update DMA request enabled
    Enabled = 1,
}
impl From<UPDDE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> UPDDE_A {
        match self.bits {
            false => UPDDE_A::Disabled,
            true => UPDDE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UPDDE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UPDDE_A::Enabled
    }
}
///Field `UPDDE` writer - UPDDE
pub type UPDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, UPDDE_A, O>;
impl<'a, const O: u8> UPDDE_W<'a, O> {
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UPDDE_A::Disabled)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UPDDE_A::Enabled)
    }
}
///Field `CPT1DE` reader - CPT1DE
pub type CPT1DE_R = crate::BitReader<CPT1DE_A>;
///CPT1DE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1DE_A {
    ///0: Capture DMA request disabled
    Disabled = 0,
    ///1: Capture DMA request enabled
    Enabled = 1,
}
impl From<CPT1DE_A> for bool {
    #[inline(always)]
    fn from(variant: CPT1DE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPT1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPT1DE_A {
        match self.bits {
            false => CPT1DE_A::Disabled,
            true => CPT1DE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CPT1DE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CPT1DE_A::Enabled
    }
}
///Field `CPT1DE` writer - CPT1DE
pub type CPT1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, CPT1DE_A, O>;
impl<'a, const O: u8> CPT1DE_W<'a, O> {
    ///Capture DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPT1DE_A::Disabled)
    }
    ///Capture DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPT1DE_A::Enabled)
    }
}
///Field `CPT2DE` reader - CPT2DE
pub use CPT1DE_R as CPT2DE_R;
///Field `CPT2DE` writer - CPT2DE
pub use CPT1DE_W as CPT2DE_W;
///Field `SETx1DE` reader - SET1xDE
pub type SETX1DE_R = crate::BitReader<SETX1DE_A>;
///SET1xDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SETX1DE_A {
    ///0: Tx output set DMA request disabled
    Disabled = 0,
    ///1: Tx output set DMA request enabled
    Enabled = 1,
}
impl From<SETX1DE_A> for bool {
    #[inline(always)]
    fn from(variant: SETX1DE_A) -> Self {
        variant as u8 != 0
    }
}
impl SETX1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SETX1DE_A {
        match self.bits {
            false => SETX1DE_A::Disabled,
            true => SETX1DE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SETX1DE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SETX1DE_A::Enabled
    }
}
///Field `SETx1DE` writer - SET1xDE
pub type SETX1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, SETX1DE_A, O>;
impl<'a, const O: u8> SETX1DE_W<'a, O> {
    ///Tx output set DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SETX1DE_A::Disabled)
    }
    ///Tx output set DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SETX1DE_A::Enabled)
    }
}
///Field `RSTx1DE` reader - RSTx1DE
pub type RSTX1DE_R = crate::BitReader<RSTX1DE_A>;
///RSTx1DE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTX1DE_A {
    ///0: Tx output reset DMA request disabled
    Disabled = 0,
    ///1: Tx output reset DMA request enabled
    Enabled = 1,
}
impl From<RSTX1DE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTX1DE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTX1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTX1DE_A {
        match self.bits {
            false => RSTX1DE_A::Disabled,
            true => RSTX1DE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTX1DE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTX1DE_A::Enabled
    }
}
///Field `RSTx1DE` writer - RSTx1DE
pub type RSTX1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, RSTX1DE_A, O>;
impl<'a, const O: u8> RSTX1DE_W<'a, O> {
    ///Tx output reset DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTX1DE_A::Disabled)
    }
    ///Tx output reset DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTX1DE_A::Enabled)
    }
}
///Field `RSTx2DE` reader - RSTx2DE
pub use RSTX1DE_R as RSTX2DE_R;
///Field `RSTx2DE` writer - RSTx2DE
pub use RSTX1DE_W as RSTX2DE_W;
///Field `SETx2DE` reader - SETx2DE
pub use SETX1DE_R as SETX2DE_R;
///Field `SETx2DE` writer - SETx2DE
pub use SETX1DE_W as SETX2DE_W;
///Field `RSTDE` reader - RSTDE
pub type RSTDE_R = crate::BitReader<RSTDE_A>;
///RSTDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTDE_A {
    ///0: Timer x counter reset/roll-over DMA request disabled
    Disabled = 0,
    ///1: Timer x counter reset/roll-over DMA request enabled
    Enabled = 1,
}
impl From<RSTDE_A> for bool {
    #[inline(always)]
    fn from(variant: RSTDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RSTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RSTDE_A {
        match self.bits {
            false => RSTDE_A::Disabled,
            true => RSTDE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTDE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTDE_A::Enabled
    }
}
///Field `RSTDE` writer - RSTDE
pub type RSTDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, RSTDE_A, O>;
impl<'a, const O: u8> RSTDE_W<'a, O> {
    ///Timer x counter reset/roll-over DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RSTDE_A::Disabled)
    }
    ///Timer x counter reset/roll-over DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RSTDE_A::Enabled)
    }
}
///Field `DLYPRTDE` reader - DLYPRTDE
pub type DLYPRTDE_R = crate::BitReader<DLYPRTDE_A>;
///DLYPRTDE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRTDE_A {
    ///0: Delayed protection DMA request disabled
    Disabled = 0,
    ///1: Delayed protection DMA request enabled
    Enabled = 1,
}
impl From<DLYPRTDE_A> for bool {
    #[inline(always)]
    fn from(variant: DLYPRTDE_A) -> Self {
        variant as u8 != 0
    }
}
impl DLYPRTDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DLYPRTDE_A {
        match self.bits {
            false => DLYPRTDE_A::Disabled,
            true => DLYPRTDE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DLYPRTDE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DLYPRTDE_A::Enabled
    }
}
///Field `DLYPRTDE` writer - DLYPRTDE
pub type DLYPRTDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMADIER_SPEC, DLYPRTDE_A, O>;
impl<'a, const O: u8> DLYPRTDE_W<'a, O> {
    ///Delayed protection DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DLYPRTDE_A::Disabled)
    }
    ///Delayed protection DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DLYPRTDE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - CMP1IE
    #[inline(always)]
    pub fn cmp1ie(&self) -> CMP1IE_R {
        CMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    pub fn cmp2ie(&self) -> CMP2IE_R {
        CMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    pub fn cmp3ie(&self) -> CMP3IE_R {
        CMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    pub fn cmp4ie(&self) -> CMP4IE_R {
        CMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    pub fn repie(&self) -> REPIE_R {
        REPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    pub fn updie(&self) -> UPDIE_R {
        UPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    pub fn cpt1ie(&self) -> CPT1IE_R {
        CPT1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    pub fn cpt2ie(&self) -> CPT2IE_R {
        CPT2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SET1xIE
    #[inline(always)]
    pub fn setx1ie(&self) -> SETX1IE_R {
        SETX1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    pub fn rstx1ie(&self) -> RSTX1IE_R {
        RSTX1IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SETx2IE
    #[inline(always)]
    pub fn setx2ie(&self) -> SETX2IE_R {
        SETX2IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    pub fn rstx2ie(&self) -> RSTX2IE_R {
        RSTX2IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    pub fn dlyprtie(&self) -> DLYPRTIE_R {
        DLYPRTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    pub fn cmp1de(&self) -> CMP1DE_R {
        CMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    pub fn cmp2de(&self) -> CMP2DE_R {
        CMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    pub fn cmp3de(&self) -> CMP3DE_R {
        CMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    pub fn cmp4de(&self) -> CMP4DE_R {
        CMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    pub fn repde(&self) -> REPDE_R {
        REPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    pub fn updde(&self) -> UPDDE_R {
        UPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    pub fn cpt1de(&self) -> CPT1DE_R {
        CPT1DE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    pub fn cpt2de(&self) -> CPT2DE_R {
        CPT2DE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SET1xDE
    #[inline(always)]
    pub fn setx1de(&self) -> SETX1DE_R {
        SETX1DE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    pub fn rstx1de(&self) -> RSTX1DE_R {
        RSTX1DE_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - SETx2DE
    #[inline(always)]
    pub fn setx2de(&self) -> SETX2DE_R {
        SETX2DE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    pub fn rstx2de(&self) -> RSTX2DE_R {
        RSTX2DE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    pub fn rstde(&self) -> RSTDE_R {
        RSTDE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    pub fn dlyprtde(&self) -> DLYPRTDE_R {
        DLYPRTDE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CMP1IE
    #[inline(always)]
    #[must_use]
    pub fn cmp1ie(&mut self) -> CMP1IE_W<0> {
        CMP1IE_W::new(self)
    }
    ///Bit 1 - CMP2IE
    #[inline(always)]
    #[must_use]
    pub fn cmp2ie(&mut self) -> CMP2IE_W<1> {
        CMP2IE_W::new(self)
    }
    ///Bit 2 - CMP3IE
    #[inline(always)]
    #[must_use]
    pub fn cmp3ie(&mut self) -> CMP3IE_W<2> {
        CMP3IE_W::new(self)
    }
    ///Bit 3 - CMP4IE
    #[inline(always)]
    #[must_use]
    pub fn cmp4ie(&mut self) -> CMP4IE_W<3> {
        CMP4IE_W::new(self)
    }
    ///Bit 4 - REPIE
    #[inline(always)]
    #[must_use]
    pub fn repie(&mut self) -> REPIE_W<4> {
        REPIE_W::new(self)
    }
    ///Bit 6 - UPDIE
    #[inline(always)]
    #[must_use]
    pub fn updie(&mut self) -> UPDIE_W<6> {
        UPDIE_W::new(self)
    }
    ///Bit 7 - CPT1IE
    #[inline(always)]
    #[must_use]
    pub fn cpt1ie(&mut self) -> CPT1IE_W<7> {
        CPT1IE_W::new(self)
    }
    ///Bit 8 - CPT2IE
    #[inline(always)]
    #[must_use]
    pub fn cpt2ie(&mut self) -> CPT2IE_W<8> {
        CPT2IE_W::new(self)
    }
    ///Bit 9 - SET1xIE
    #[inline(always)]
    #[must_use]
    pub fn setx1ie(&mut self) -> SETX1IE_W<9> {
        SETX1IE_W::new(self)
    }
    ///Bit 10 - RSTx1IE
    #[inline(always)]
    #[must_use]
    pub fn rstx1ie(&mut self) -> RSTX1IE_W<10> {
        RSTX1IE_W::new(self)
    }
    ///Bit 11 - SETx2IE
    #[inline(always)]
    #[must_use]
    pub fn setx2ie(&mut self) -> SETX2IE_W<11> {
        SETX2IE_W::new(self)
    }
    ///Bit 12 - RSTx2IE
    #[inline(always)]
    #[must_use]
    pub fn rstx2ie(&mut self) -> RSTX2IE_W<12> {
        RSTX2IE_W::new(self)
    }
    ///Bit 13 - RSTIE
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<13> {
        RSTIE_W::new(self)
    }
    ///Bit 14 - DLYPRTIE
    #[inline(always)]
    #[must_use]
    pub fn dlyprtie(&mut self) -> DLYPRTIE_W<14> {
        DLYPRTIE_W::new(self)
    }
    ///Bit 16 - CMP1DE
    #[inline(always)]
    #[must_use]
    pub fn cmp1de(&mut self) -> CMP1DE_W<16> {
        CMP1DE_W::new(self)
    }
    ///Bit 17 - CMP2DE
    #[inline(always)]
    #[must_use]
    pub fn cmp2de(&mut self) -> CMP2DE_W<17> {
        CMP2DE_W::new(self)
    }
    ///Bit 18 - CMP3DE
    #[inline(always)]
    #[must_use]
    pub fn cmp3de(&mut self) -> CMP3DE_W<18> {
        CMP3DE_W::new(self)
    }
    ///Bit 19 - CMP4DE
    #[inline(always)]
    #[must_use]
    pub fn cmp4de(&mut self) -> CMP4DE_W<19> {
        CMP4DE_W::new(self)
    }
    ///Bit 20 - REPDE
    #[inline(always)]
    #[must_use]
    pub fn repde(&mut self) -> REPDE_W<20> {
        REPDE_W::new(self)
    }
    ///Bit 22 - UPDDE
    #[inline(always)]
    #[must_use]
    pub fn updde(&mut self) -> UPDDE_W<22> {
        UPDDE_W::new(self)
    }
    ///Bit 23 - CPT1DE
    #[inline(always)]
    #[must_use]
    pub fn cpt1de(&mut self) -> CPT1DE_W<23> {
        CPT1DE_W::new(self)
    }
    ///Bit 24 - CPT2DE
    #[inline(always)]
    #[must_use]
    pub fn cpt2de(&mut self) -> CPT2DE_W<24> {
        CPT2DE_W::new(self)
    }
    ///Bit 25 - SET1xDE
    #[inline(always)]
    #[must_use]
    pub fn setx1de(&mut self) -> SETX1DE_W<25> {
        SETX1DE_W::new(self)
    }
    ///Bit 26 - RSTx1DE
    #[inline(always)]
    #[must_use]
    pub fn rstx1de(&mut self) -> RSTX1DE_W<26> {
        RSTX1DE_W::new(self)
    }
    ///Bit 27 - SETx2DE
    #[inline(always)]
    #[must_use]
    pub fn setx2de(&mut self) -> SETX2DE_W<27> {
        SETX2DE_W::new(self)
    }
    ///Bit 28 - RSTx2DE
    #[inline(always)]
    #[must_use]
    pub fn rstx2de(&mut self) -> RSTX2DE_W<28> {
        RSTX2DE_W::new(self)
    }
    ///Bit 29 - RSTDE
    #[inline(always)]
    #[must_use]
    pub fn rstde(&mut self) -> RSTDE_W<29> {
        RSTDE_W::new(self)
    }
    ///Bit 30 - DLYPRTDE
    #[inline(always)]
    #[must_use]
    pub fn dlyprtde(&mut self) -> DLYPRTDE_W<30> {
        DLYPRTDE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIMxDIER5
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timadier](index.html) module
pub struct TIMADIER_SPEC;
impl crate::RegisterSpec for TIMADIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [timadier::R](R) reader structure
impl crate::Readable for TIMADIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timadier::W](W) writer structure
impl crate::Writable for TIMADIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMADIER to value 0
impl crate::Resettable for TIMADIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
