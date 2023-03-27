///Register `AHB2ENR` reader
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB2ENR` writer
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `GPIOAEN` reader - IO port A clock enable
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
///IO port A clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPIOAEN_A {
    ///0: IO port x clock disabled
    Disabled = 0,
    ///1: IO port x clock enabled
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
///Field `GPIOAEN` writer - IO port A clock enable
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    ///IO port x clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    ///IO port x clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
///Field `GPIOBEN` reader - IO port B clock enable
pub use GPIOAEN_R as GPIOBEN_R;
///Field `GPIOCEN` reader - IO port C clock enable
pub use GPIOAEN_R as GPIOCEN_R;
///Field `GPIODEN` reader - IO port D clock enable
pub use GPIOAEN_R as GPIODEN_R;
///Field `GPIOEEN` reader - IO port E clock enable
pub use GPIOAEN_R as GPIOEEN_R;
///Field `GPIOFEN` reader - IO port F clock enable
pub use GPIOAEN_R as GPIOFEN_R;
///Field `GPIOGEN` reader - IO port G clock enable
pub use GPIOAEN_R as GPIOGEN_R;
///Field `GPIOHEN` reader - IO port H clock enable
pub use GPIOAEN_R as GPIOHEN_R;
///Field `GPIOIEN` reader - IO port I clock enable
pub use GPIOAEN_R as GPIOIEN_R;
///Field `GPIOBEN` writer - IO port B clock enable
pub use GPIOAEN_W as GPIOBEN_W;
///Field `GPIOCEN` writer - IO port C clock enable
pub use GPIOAEN_W as GPIOCEN_W;
///Field `GPIODEN` writer - IO port D clock enable
pub use GPIOAEN_W as GPIODEN_W;
///Field `GPIOEEN` writer - IO port E clock enable
pub use GPIOAEN_W as GPIOEEN_W;
///Field `GPIOFEN` writer - IO port F clock enable
pub use GPIOAEN_W as GPIOFEN_W;
///Field `GPIOGEN` writer - IO port G clock enable
pub use GPIOAEN_W as GPIOGEN_W;
///Field `GPIOHEN` writer - IO port H clock enable
pub use GPIOAEN_W as GPIOHEN_W;
///Field `GPIOIEN` writer - IO port I clock enable
pub use GPIOAEN_W as GPIOIEN_W;
///Field `OTGFSEN` reader - OTG full speed clock enable
pub type OTGFSEN_R = crate::BitReader<OTGFSEN_A>;
///OTG full speed clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGFSEN_A {
    ///0: USB OTG full speed clock disabled
    Disabled = 0,
    ///1: USB OTG full speed clock enabled
    Enabled = 1,
}
impl From<OTGFSEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGFSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGFSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OTGFSEN_A {
        match self.bits {
            false => OTGFSEN_A::Disabled,
            true => OTGFSEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OTGFSEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OTGFSEN_A::Enabled
    }
}
///Field `OTGFSEN` writer - OTG full speed clock enable
pub type OTGFSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, OTGFSEN_A, O>;
impl<'a, const O: u8> OTGFSEN_W<'a, O> {
    ///USB OTG full speed clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::Disabled)
    }
    ///USB OTG full speed clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OTGFSEN_A::Enabled)
    }
}
///Field `ADCEN` reader - ADC clock enable
pub type ADCEN_R = crate::BitReader<ADCEN_A>;
///ADC clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCEN_A {
    ///0: ADC clock disabled
    Disabled = 0,
    ///1: ADC clock enabled
    Enabled = 1,
}
impl From<ADCEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADCEN_A {
        match self.bits {
            false => ADCEN_A::Disabled,
            true => ADCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADCEN_A::Enabled
    }
}
///Field `ADCEN` writer - ADC clock enable
pub type ADCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, ADCEN_A, O>;
impl<'a, const O: u8> ADCEN_W<'a, O> {
    ///ADC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Disabled)
    }
    ///ADC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADCEN_A::Enabled)
    }
}
///Field `DCMIEN` reader - DCMI clock enable
pub type DCMIEN_R = crate::BitReader<DCMIEN_A>;
///DCMI clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCMIEN_A {
    ///0: DCMI/PSSI clock disabled
    Disabled = 0,
    ///1: DCMI/PSSI clock enabled
    Enabled = 1,
}
impl From<DCMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCMIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DCMIEN_A {
        match self.bits {
            false => DCMIEN_A::Disabled,
            true => DCMIEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCMIEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCMIEN_A::Enabled
    }
}
///Field `DCMIEN` writer - DCMI clock enable
pub type DCMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, DCMIEN_A, O>;
impl<'a, const O: u8> DCMIEN_W<'a, O> {
    ///DCMI/PSSI clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Disabled)
    }
    ///DCMI/PSSI clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCMIEN_A::Enabled)
    }
}
///Field `PKAEN` reader - PKA clock enable
pub type PKAEN_R = crate::BitReader<PKAEN_A>;
///PKA clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PKAEN_A {
    ///0: PKA clock disabled
    Disabled = 0,
    ///1: PKA clock enabled
    Enabled = 1,
}
impl From<PKAEN_A> for bool {
    #[inline(always)]
    fn from(variant: PKAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PKAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PKAEN_A {
        match self.bits {
            false => PKAEN_A::Disabled,
            true => PKAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PKAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PKAEN_A::Enabled
    }
}
///Field `PKAEN` writer - PKA clock enable
pub type PKAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, PKAEN_A, O>;
impl<'a, const O: u8> PKAEN_W<'a, O> {
    ///PKA clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Disabled)
    }
    ///PKA clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PKAEN_A::Enabled)
    }
}
///Field `AESEN` reader - AES accelerator clock enable
pub type AESEN_R = crate::BitReader<AESEN_A>;
///AES accelerator clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AESEN_A {
    ///0: AES clock disabled
    Disabled = 0,
    ///1: AES clock enabled
    Enabled = 1,
}
impl From<AESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AESEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AESEN_A {
        match self.bits {
            false => AESEN_A::Disabled,
            true => AESEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AESEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AESEN_A::Enabled
    }
}
///Field `AESEN` writer - AES accelerator clock enable
pub type AESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, AESEN_A, O>;
impl<'a, const O: u8> AESEN_W<'a, O> {
    ///AES clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AESEN_A::Disabled)
    }
    ///AES clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AESEN_A::Enabled)
    }
}
///Field `HASHEN` reader - HASH clock enable
pub type HASHEN_R = crate::BitReader<HASHEN_A>;
///HASH clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HASHEN_A {
    ///0: HASH clock disabled
    Disabled = 0,
    ///1: HASH clock enabled
    Enabled = 1,
}
impl From<HASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: HASHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HASHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HASHEN_A {
        match self.bits {
            false => HASHEN_A::Disabled,
            true => HASHEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HASHEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HASHEN_A::Enabled
    }
}
///Field `HASHEN` writer - HASH clock enable
pub type HASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, HASHEN_A, O>;
impl<'a, const O: u8> HASHEN_W<'a, O> {
    ///HASH clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HASHEN_A::Disabled)
    }
    ///HASH clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HASHEN_A::Enabled)
    }
}
///Field `RNGEN` reader - Random Number Generator clock enable
pub type RNGEN_R = crate::BitReader<RNGEN_A>;
///Random Number Generator clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGEN_A {
    ///0: Random Number Generator clock disabled
    Disabled = 0,
    ///1: Random Number Generator clock enabled
    Enabled = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::Disabled,
            true => RNGEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RNGEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RNGEN_A::Enabled
    }
}
///Field `RNGEN` writer - Random Number Generator clock enable
pub type RNGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, RNGEN_A, O>;
impl<'a, const O: u8> RNGEN_W<'a, O> {
    ///Random Number Generator clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Disabled)
    }
    ///Random Number Generator clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RNGEN_A::Enabled)
    }
}
///Field `OSPIMEN` reader - OctoSPI IO manager clock enable
pub type OSPIMEN_R = crate::BitReader<OSPIMEN_A>;
///OctoSPI IO manager clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OSPIMEN_A {
    ///0: OctoSPI IO manager clock disabled
    Disabled = 0,
    ///1: OctoSPI IO manager clock enabled
    Enabled = 1,
}
impl From<OSPIMEN_A> for bool {
    #[inline(always)]
    fn from(variant: OSPIMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OSPIMEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OSPIMEN_A {
        match self.bits {
            false => OSPIMEN_A::Disabled,
            true => OSPIMEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSPIMEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OSPIMEN_A::Enabled
    }
}
///Field `OSPIMEN` writer - OctoSPI IO manager clock enable
pub type OSPIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, OSPIMEN_A, O>;
impl<'a, const O: u8> OSPIMEN_W<'a, O> {
    ///OctoSPI IO manager clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSPIMEN_A::Disabled)
    }
    ///OctoSPI IO manager clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OSPIMEN_A::Enabled)
    }
}
///Field `SDMMC1EN` reader - SDMMC1 clock enable
pub type SDMMC1EN_R = crate::BitReader<SDMMC1EN_A>;
///SDMMC1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDMMC1EN_A {
    ///0: SDMMCx clock disabled
    Disabled = 0,
    ///1: SDMMCx clock enabled
    Enabled = 1,
}
impl From<SDMMC1EN_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMC1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMC1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SDMMC1EN_A {
        match self.bits {
            false => SDMMC1EN_A::Disabled,
            true => SDMMC1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDMMC1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDMMC1EN_A::Enabled
    }
}
///Field `SDMMC1EN` writer - SDMMC1 clock enable
pub type SDMMC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, SDMMC1EN_A, O>;
impl<'a, const O: u8> SDMMC1EN_W<'a, O> {
    ///SDMMCx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::Disabled)
    }
    ///SDMMCx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDMMC1EN_A::Enabled)
    }
}
///Field `SDMMC2EN` reader - SDMMC2 clock enable
pub use SDMMC1EN_R as SDMMC2EN_R;
///Field `SDMMC2EN` writer - SDMMC2 clock enable
pub use SDMMC1EN_W as SDMMC2EN_W;
impl R {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    pub fn otgfsen(&self) -> OTGFSEN_R {
        OTGFSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    pub fn dcmien(&self) -> DCMIEN_R {
        DCMIEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - PKA clock enable
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    pub fn ospimen(&self) -> OSPIMEN_R {
        OSPIMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - SDMMC2 clock enable
    #[inline(always)]
    pub fn sdmmc2en(&self) -> SDMMC2EN_R {
        SDMMC2EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - IO port A clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    ///Bit 1 - IO port B clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    ///Bit 2 - IO port C clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    ///Bit 3 - IO port D clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    ///Bit 4 - IO port E clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    ///Bit 5 - IO port F clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    ///Bit 6 - IO port G clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    ///Bit 7 - IO port H clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    ///Bit 8 - IO port I clock enable
    #[inline(always)]
    #[must_use]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    ///Bit 12 - OTG full speed clock enable
    #[inline(always)]
    #[must_use]
    pub fn otgfsen(&mut self) -> OTGFSEN_W<12> {
        OTGFSEN_W::new(self)
    }
    ///Bit 13 - ADC clock enable
    #[inline(always)]
    #[must_use]
    pub fn adcen(&mut self) -> ADCEN_W<13> {
        ADCEN_W::new(self)
    }
    ///Bit 14 - DCMI clock enable
    #[inline(always)]
    #[must_use]
    pub fn dcmien(&mut self) -> DCMIEN_W<14> {
        DCMIEN_W::new(self)
    }
    ///Bit 15 - PKA clock enable
    #[inline(always)]
    #[must_use]
    pub fn pkaen(&mut self) -> PKAEN_W<15> {
        PKAEN_W::new(self)
    }
    ///Bit 16 - AES accelerator clock enable
    #[inline(always)]
    #[must_use]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    ///Bit 17 - HASH clock enable
    #[inline(always)]
    #[must_use]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    ///Bit 18 - Random Number Generator clock enable
    #[inline(always)]
    #[must_use]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    ///Bit 20 - OctoSPI IO manager clock enable
    #[inline(always)]
    #[must_use]
    pub fn ospimen(&mut self) -> OSPIMEN_W<20> {
        OSPIMEN_W::new(self)
    }
    ///Bit 22 - SDMMC1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<22> {
        SDMMC1EN_W::new(self)
    }
    ///Bit 23 - SDMMC2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn sdmmc2en(&mut self) -> SDMMC2EN_W<23> {
        SDMMC2EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB2 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb2enr](index.html) module
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb2enr::R](R) reader structure
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb2enr::W](W) writer structure
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB2ENR to value 0
impl crate::Resettable for AHB2ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
