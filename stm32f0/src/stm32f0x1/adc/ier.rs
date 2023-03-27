///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADRDYIE` reader - ADC ready interrupt enable
pub type ADRDYIE_R = crate::BitReader<ADRDYIE_A>;
///ADC ready interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYIE_A {
    ///0: ADC ready interrupt disabled
    Disabled = 0,
    ///1: ADC ready interrupt enabled
    Enabled = 1,
}
impl From<ADRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> ADRDYIE_A {
        match self.bits {
            false => ADRDYIE_A::Disabled,
            true => ADRDYIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADRDYIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADRDYIE_A::Enabled
    }
}
///Field `ADRDYIE` writer - ADC ready interrupt enable
pub type ADRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ADRDYIE_A, O>;
impl<'a, const O: u8> ADRDYIE_W<'a, O> {
    ///ADC ready interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Disabled)
    }
    ///ADC ready interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADRDYIE_A::Enabled)
    }
}
///Field `EOSMPIE` reader - End of sampling flag interrupt enable
pub type EOSMPIE_R = crate::BitReader<EOSMPIE_A>;
///End of sampling flag interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPIE_A {
    ///0: End of sampling interrupt disabled
    Disabled = 0,
    ///1: End of sampling interrupt enabled
    Enabled = 1,
}
impl From<EOSMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSMPIE_A {
        match self.bits {
            false => EOSMPIE_A::Disabled,
            true => EOSMPIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSMPIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSMPIE_A::Enabled
    }
}
///Field `EOSMPIE` writer - End of sampling flag interrupt enable
pub type EOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSMPIE_A, O>;
impl<'a, const O: u8> EOSMPIE_W<'a, O> {
    ///End of sampling interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Disabled)
    }
    ///End of sampling interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSMPIE_A::Enabled)
    }
}
///Field `EOCIE` reader - End of conversion interrupt enable
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
///End of conversion interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    ///0: End of conversion interrupt disabled
    Disabled = 0,
    ///1: End of conversion interrupt enabled
    Enabled = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::Disabled,
            true => EOCIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOCIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOCIE_A::Enabled
    }
}
///Field `EOCIE` writer - End of conversion interrupt enable
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOCIE_A, O>;
impl<'a, const O: u8> EOCIE_W<'a, O> {
    ///End of conversion interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Disabled)
    }
    ///End of conversion interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOCIE_A::Enabled)
    }
}
///Field `EOSEQIE` reader - End of conversion sequence interrupt enable
pub type EOSEQIE_R = crate::BitReader<EOSEQIE_A>;
///End of conversion sequence interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSEQIE_A {
    ///0: End of conversion sequence interrupt disabled
    Disabled = 0,
    ///1: End of conversion sequence interrupt enabled
    Enabled = 1,
}
impl From<EOSEQIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOSEQIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSEQIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EOSEQIE_A {
        match self.bits {
            false => EOSEQIE_A::Disabled,
            true => EOSEQIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOSEQIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOSEQIE_A::Enabled
    }
}
///Field `EOSEQIE` writer - End of conversion sequence interrupt enable
pub type EOSEQIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, EOSEQIE_A, O>;
impl<'a, const O: u8> EOSEQIE_W<'a, O> {
    ///End of conversion sequence interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOSEQIE_A::Disabled)
    }
    ///End of conversion sequence interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOSEQIE_A::Enabled)
    }
}
///Field `OVRIE` reader - Overrun interrupt enable
pub type OVRIE_R = crate::BitReader<OVRIE_A>;
///Overrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRIE_A {
    ///0: Overrun interrupt disabled
    Disabled = 0,
    ///1: Overrun interrupt enabled
    Enabled = 1,
}
impl From<OVRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRIE_A {
        match self.bits {
            false => OVRIE_A::Disabled,
            true => OVRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRIE_A::Enabled
    }
}
///Field `OVRIE` writer - Overrun interrupt enable
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVRIE_A, O>;
impl<'a, const O: u8> OVRIE_W<'a, O> {
    ///Overrun interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Disabled)
    }
    ///Overrun interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRIE_A::Enabled)
    }
}
///Field `AWDIE` reader - Analog watchdog interrupt enable
pub type AWDIE_R = crate::BitReader<AWDIE_A>;
///Analog watchdog interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDIE_A {
    ///0: Analog watchdog interrupt disabled
    Disabled = 0,
    ///1: Analog watchdog interrupt enabled
    Enabled = 1,
}
impl From<AWDIE_A> for bool {
    #[inline(always)]
    fn from(variant: AWDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AWDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AWDIE_A {
        match self.bits {
            false => AWDIE_A::Disabled,
            true => AWDIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWDIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWDIE_A::Enabled
    }
}
///Field `AWDIE` writer - Analog watchdog interrupt enable
pub type AWDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, AWDIE_A, O>;
impl<'a, const O: u8> AWDIE_W<'a, O> {
    ///Analog watchdog interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Disabled)
    }
    ///Analog watchdog interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWDIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag interrupt enable
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion interrupt enable
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of conversion sequence interrupt enable
    #[inline(always)]
    pub fn eoseqie(&self) -> EOSEQIE_R {
        EOSEQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog interrupt enable
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<0> {
        ADRDYIE_W::new(self)
    }
    ///Bit 1 - End of sampling flag interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<1> {
        EOSMPIE_W::new(self)
    }
    ///Bit 2 - End of conversion interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<2> {
        EOCIE_W::new(self)
    }
    ///Bit 3 - End of conversion sequence interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eoseqie(&mut self) -> EOSEQIE_W<3> {
        EOSEQIE_W::new(self)
    }
    ///Bit 4 - Overrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<4> {
        OVRIE_W::new(self)
    }
    ///Bit 7 - Analog watchdog interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<7> {
        AWDIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
