///Register `AHB2RSTR` reader
pub struct R(crate::R<AHB2RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2RSTR` writer
pub struct W(crate::W<AHB2RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2RSTR_SPEC>;
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
impl From<crate::W<AHB2RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOARST` reader - IO port A reset
pub type GPIOARST_R = crate::BitReader<GPIOARST_A>;
///IO port A reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOARST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset GPIO port x
    Reset = 1,
}
impl From<GPIOARST_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOARST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOARST_A {
        match self.bits {
            false => GPIOARST_A::NoEffect,
            true => GPIOARST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == GPIOARST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == GPIOARST_A::Reset
    }
}
///Field `GPIOARST` writer - IO port A reset
pub type GPIOARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, GPIOARST_A, O>;
impl<'a, const O: u8> GPIOARST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(GPIOARST_A::NoEffect)
    }
    ///Reset GPIO port x
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(GPIOARST_A::Reset)
    }
}
///Field `GPIOBRST` reader - IO port B reset
pub use GPIOARST_R as GPIOBRST_R;
///Field `GPIOCRST` reader - IO port C reset
pub use GPIOARST_R as GPIOCRST_R;
///Field `GPIODRST` reader - IO port D reset
pub use GPIOARST_R as GPIODRST_R;
///Field `GPIOERST` reader - IO port E reset
pub use GPIOARST_R as GPIOERST_R;
///Field `GPIOFRST` reader - IO port F reset
pub use GPIOARST_R as GPIOFRST_R;
///Field `GPIOGRST` reader - IO port G reset
pub use GPIOARST_R as GPIOGRST_R;
///Field `GPIOHRST` reader - IO port H reset
pub use GPIOARST_R as GPIOHRST_R;
///Field `GPIOIRST` reader - IO port I reset
pub use GPIOARST_R as GPIOIRST_R;
///Field `GPIOBRST` writer - IO port B reset
pub use GPIOARST_W as GPIOBRST_W;
///Field `GPIOCRST` writer - IO port C reset
pub use GPIOARST_W as GPIOCRST_W;
///Field `GPIODRST` writer - IO port D reset
pub use GPIOARST_W as GPIODRST_W;
///Field `GPIOERST` writer - IO port E reset
pub use GPIOARST_W as GPIOERST_W;
///Field `GPIOFRST` writer - IO port F reset
pub use GPIOARST_W as GPIOFRST_W;
///Field `GPIOGRST` writer - IO port G reset
pub use GPIOARST_W as GPIOGRST_W;
///Field `GPIOHRST` writer - IO port H reset
pub use GPIOARST_W as GPIOHRST_W;
///Field `GPIOIRST` writer - IO port I reset
pub use GPIOARST_W as GPIOIRST_W;
///Field `OTGFSRST` reader - USB OTG FS reset
pub type OTGFSRST_R = crate::BitReader<OTGFSRST_A>;
///USB OTG FS reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset USB OTG FS
    Reset = 1,
}
impl From<OTGFSRST_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSRST_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OTGFSRST_A {
        match self.bits {
            false => OTGFSRST_A::NoEffect,
            true => OTGFSRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OTGFSRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OTGFSRST_A::Reset
    }
}
///Field `OTGFSRST` writer - USB OTG FS reset
pub type OTGFSRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, OTGFSRST_A, O>;
impl<'a, const O: u8> OTGFSRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OTGFSRST_A::NoEffect)
    }
    ///Reset USB OTG FS
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OTGFSRST_A::Reset)
    }
}
///Field `ADCRST` reader - ADC reset
pub type ADCRST_R = crate::BitReader<ADCRST_A>;
///ADC reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset ADC
    Reset = 1,
}
impl From<ADCRST_A> for bool {
    #[inline(always)]
    fn from(variant: ADCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCRST_A {
        match self.bits {
            false => ADCRST_A::NoEffect,
            true => ADCRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == ADCRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ADCRST_A::Reset
    }
}
///Field `ADCRST` writer - ADC reset
pub type ADCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, ADCRST_A, O>;
impl<'a, const O: u8> ADCRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(ADCRST_A::NoEffect)
    }
    ///Reset ADC
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ADCRST_A::Reset)
    }
}
///Field `DCMIRST` reader - Digital Camera Interface reset
pub type DCMIRST_R = crate::BitReader<DCMIRST_A>;
///Digital Camera Interface reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset DCMI/PSSI interface
    Reset = 1,
}
impl From<DCMIRST_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCMIRST_A {
        match self.bits {
            false => DCMIRST_A::NoEffect,
            true => DCMIRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DCMIRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DCMIRST_A::Reset
    }
}
///Field `DCMIRST` writer - Digital Camera Interface reset
pub type DCMIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, DCMIRST_A, O>;
impl<'a, const O: u8> DCMIRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DCMIRST_A::NoEffect)
    }
    ///Reset DCMI/PSSI interface
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DCMIRST_A::Reset)
    }
}
///Field `PKARST` reader - PKA reset
pub type PKARST_R = crate::BitReader<PKARST_A>;
///PKA reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKARST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset PKA
    Reset = 1,
}
impl From<PKARST_A> for bool {
    #[inline(always)]
    fn from(variant: PKARST_A) -> Self {
        variant as u8 != 0
    }
}
impl PKARST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PKARST_A {
        match self.bits {
            false => PKARST_A::NoEffect,
            true => PKARST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PKARST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PKARST_A::Reset
    }
}
///Field `PKARST` writer - PKA reset
pub type PKARST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, PKARST_A, O>;
impl<'a, const O: u8> PKARST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PKARST_A::NoEffect)
    }
    ///Reset PKA
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(PKARST_A::Reset)
    }
}
///Field `AESRST` reader - AES hardware accelerator reset
pub type AESRST_R = crate::BitReader<AESRST_A>;
///AES hardware accelerator reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset AES
    Reset = 1,
}
impl From<AESRST_A> for bool {
    #[inline(always)]
    fn from(variant: AESRST_A) -> Self {
        variant as u8 != 0
    }
}
impl AESRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AESRST_A {
        match self.bits {
            false => AESRST_A::NoEffect,
            true => AESRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == AESRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == AESRST_A::Reset
    }
}
///Field `AESRST` writer - AES hardware accelerator reset
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, AESRST_A, O>;
impl<'a, const O: u8> AESRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(AESRST_A::NoEffect)
    }
    ///Reset AES
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(AESRST_A::Reset)
    }
}
///Field `HASHRST` reader - Hash reset
pub type HASHRST_R = crate::BitReader<HASHRST_A>;
///Hash reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset HASH
    Reset = 1,
}
impl From<HASHRST_A> for bool {
    #[inline(always)]
    fn from(variant: HASHRST_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HASHRST_A {
        match self.bits {
            false => HASHRST_A::NoEffect,
            true => HASHRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == HASHRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == HASHRST_A::Reset
    }
}
///Field `HASHRST` writer - Hash reset
pub type HASHRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, HASHRST_A, O>;
impl<'a, const O: u8> HASHRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(HASHRST_A::NoEffect)
    }
    ///Reset HASH
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(HASHRST_A::Reset)
    }
}
///Field `RNGRST` reader - Random number generator reset
pub type RNGRST_R = crate::BitReader<RNGRST_A>;
///Random number generator reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset RNG
    Reset = 1,
}
impl From<RNGRST_A> for bool {
    #[inline(always)]
    fn from(variant: RNGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGRST_A {
        match self.bits {
            false => RNGRST_A::NoEffect,
            true => RNGRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RNGRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RNGRST_A::Reset
    }
}
///Field `RNGRST` writer - Random number generator reset
pub type RNGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, RNGRST_A, O>;
impl<'a, const O: u8> RNGRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RNGRST_A::NoEffect)
    }
    ///Reset RNG
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RNGRST_A::Reset)
    }
}
///Field `OSPIMRST` reader - OCTOSPI IO manager reset
pub type OSPIMRST_R = crate::BitReader<OSPIMRST_A>;
///OCTOSPI IO manager reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMRST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset OctoSPI IO manager
    Reset = 1,
}
impl From<OSPIMRST_A> for bool {
    #[inline(always)]
    fn from(variant: OSPIMRST_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPIMRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSPIMRST_A {
        match self.bits {
            false => OSPIMRST_A::NoEffect,
            true => OSPIMRST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == OSPIMRST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == OSPIMRST_A::Reset
    }
}
///Field `OSPIMRST` writer - OCTOSPI IO manager reset
pub type OSPIMRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, OSPIMRST_A, O>;
impl<'a, const O: u8> OSPIMRST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OSPIMRST_A::NoEffect)
    }
    ///Reset OctoSPI IO manager
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(OSPIMRST_A::Reset)
    }
}
///Field `SDMMC1RST` reader - SDMMC1 reset
pub type SDMMC1RST_R = crate::BitReader<SDMMC1RST_A>;
///SDMMC1 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SDMMC1
    Reset = 1,
}
impl From<SDMMC1RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC1RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1RST_A {
        match self.bits {
            false => SDMMC1RST_A::NoEffect,
            true => SDMMC1RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC1RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC1RST_A::Reset
    }
}
///Field `SDMMC1RST` writer - SDMMC1 reset
pub type SDMMC1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, SDMMC1RST_A, O>;
impl<'a, const O: u8> SDMMC1RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SDMMC1RST_A::NoEffect)
    }
    ///Reset SDMMC1
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDMMC1RST_A::Reset)
    }
}
///Field `SDMMC2RST` reader - SDMMC2 reset
pub type SDMMC2RST_R = crate::BitReader<SDMMC2RST_A>;
///SDMMC2 reset
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC2RST_A {
    ///0: No effect
    NoEffect = 0,
    ///1: Reset SDMMC2
    Reset = 1,
}
impl From<SDMMC2RST_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC2RST_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC2RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDMMC2RST_A {
        match self.bits {
            false => SDMMC2RST_A::NoEffect,
            true => SDMMC2RST_A::Reset,
        }
    }
    ///Checks if the value of the field is `NoEffect`
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SDMMC2RST_A::NoEffect
    }
    ///Checks if the value of the field is `Reset`
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SDMMC2RST_A::Reset
    }
}
///Field `SDMMC2RST` writer - SDMMC2 reset
pub type SDMMC2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2RSTR_SPEC, SDMMC2RST_A, O>;
impl<'a, const O: u8> SDMMC2RST_W<'a, O> {
    ///No effect
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::NoEffect)
    }
    ///Reset SDMMC2
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SDMMC2RST_A::Reset)
    }
}
impl R {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    pub fn otgfsrst(&self) -> OTGFSRST_R {
        OTGFSRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    pub fn dcmirst(&self) -> DCMIRST_R {
        DCMIRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKA reset
    #[inline(always)]
    pub fn pkarst(&self) -> PKARST_R {
        PKARST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    pub fn hashrst(&self) -> HASHRST_R {
        HASHRST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    pub fn rngrst(&self) -> RNGRST_R {
        RNGRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    pub fn ospimrst(&self) -> OSPIMRST_R {
        OSPIMRST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> SDMMC1RST_R {
        SDMMC1RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SDMMC2 reset
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> SDMMC2RST_R {
        SDMMC2RST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A reset
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<0> {
        GPIOARST_W::new(self)
    }
    ///Bit 1 - IO port B reset
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<1> {
        GPIOBRST_W::new(self)
    }
    ///Bit 2 - IO port C reset
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<2> {
        GPIOCRST_W::new(self)
    }
    ///Bit 3 - IO port D reset
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<3> {
        GPIODRST_W::new(self)
    }
    ///Bit 4 - IO port E reset
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<4> {
        GPIOERST_W::new(self)
    }
    ///Bit 5 - IO port F reset
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<5> {
        GPIOFRST_W::new(self)
    }
    ///Bit 6 - IO port G reset
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<6> {
        GPIOGRST_W::new(self)
    }
    ///Bit 7 - IO port H reset
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<7> {
        GPIOHRST_W::new(self)
    }
    ///Bit 8 - IO port I reset
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<8> {
        GPIOIRST_W::new(self)
    }
    ///Bit 12 - USB OTG FS reset
    #[inline(always)]
    #[must_use]
    pub fn otgfsrst(&mut self) -> OTGFSRST_W<12> {
        OTGFSRST_W::new(self)
    }
    ///Bit 13 - ADC reset
    #[inline(always)]
    #[must_use]
    pub fn adcrst(&mut self) -> ADCRST_W<13> {
        ADCRST_W::new(self)
    }
    ///Bit 14 - Digital Camera Interface reset
    #[inline(always)]
    #[must_use]
    pub fn dcmirst(&mut self) -> DCMIRST_W<14> {
        DCMIRST_W::new(self)
    }
    ///Bit 15 - PKA reset
    #[inline(always)]
    #[must_use]
    pub fn pkarst(&mut self) -> PKARST_W<15> {
        PKARST_W::new(self)
    }
    ///Bit 16 - AES hardware accelerator reset
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<16> {
        AESRST_W::new(self)
    }
    ///Bit 17 - Hash reset
    #[inline(always)]
    #[must_use]
    pub fn hashrst(&mut self) -> HASHRST_W<17> {
        HASHRST_W::new(self)
    }
    ///Bit 18 - Random number generator reset
    #[inline(always)]
    #[must_use]
    pub fn rngrst(&mut self) -> RNGRST_W<18> {
        RNGRST_W::new(self)
    }
    ///Bit 20 - OCTOSPI IO manager reset
    #[inline(always)]
    #[must_use]
    pub fn ospimrst(&mut self) -> OSPIMRST_W<20> {
        OSPIMRST_W::new(self)
    }
    ///Bit 22 - SDMMC1 reset
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1rst(&mut self) -> SDMMC1RST_W<22> {
        SDMMC1RST_W::new(self)
    }
    ///Bit 23 - SDMMC2 reset
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2rst(&mut self) -> SDMMC2RST_W<23> {
        SDMMC2RST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2rstr](index.html) module
pub struct AHB2RSTR_SPEC;
impl crate::RegisterSpec for AHB2RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2rstr::R](R) reader structure
impl crate::Readable for AHB2RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2rstr::W](W) writer structure
impl crate::Writable for AHB2RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2RSTR to value 0
impl crate::Resettable for AHB2RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
