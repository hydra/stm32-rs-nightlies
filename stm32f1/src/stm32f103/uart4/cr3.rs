///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader<EIE_A>;
///Error interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE_A {
    ///0: Error interrupt disabled
    Disabled = 0,
    ///1: Error interrupt enabled
    Enabled = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::Disabled,
            true => EIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE_A::Enabled
    }
}
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, EIE_A, O>;
impl<'a, const O: u8> EIE_W<'a, O> {
    ///Error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::Disabled)
    }
    ///Error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::Enabled)
    }
}
///Field `IREN` reader - IrDA mode enable
pub type IREN_R = crate::BitReader<IREN_A>;
///IrDA mode enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    ///0: IrDA disabled
    Disabled = 0,
    ///1: IrDA enabled
    Enabled = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::Disabled,
            true => IREN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::Enabled
    }
}
///Field `IREN` writer - IrDA mode enable
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    ///IrDA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::Disabled)
    }
    ///IrDA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::Enabled)
    }
}
///Field `IRLP` reader - IrDA low-power
pub type IRLP_R = crate::BitReader<IRLP_A>;
///IrDA low-power
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP_A {
    ///0: Normal mode
    Normal = 0,
    ///1: Low-power mode
    LowPower = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
impl IRLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::Normal,
            true => IRLP_A::LowPower,
        }
    }
    ///Checks if the value of the field is `Normal`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP_A::Normal
    }
    ///Checks if the value of the field is `LowPower`
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP_A::LowPower
    }
}
///Field `IRLP` writer - IrDA low-power
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, IRLP_A, O>;
impl<'a, const O: u8> IRLP_W<'a, O> {
    ///Normal mode
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::Normal)
    }
    ///Low-power mode
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LowPower)
    }
}
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader<HDSEL_A>;
///Half-duplex selection
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL_A {
    ///0: Half duplex mode is not selected
    FullDuplex = 0,
    ///1: Half duplex mode is selected
    HalfDuplex = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::FullDuplex,
            true => HDSEL_A::HalfDuplex,
        }
    }
    ///Checks if the value of the field is `FullDuplex`
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == HDSEL_A::FullDuplex
    }
    ///Checks if the value of the field is `HalfDuplex`
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == HDSEL_A::HalfDuplex
    }
}
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, HDSEL_A, O>;
impl<'a, const O: u8> HDSEL_W<'a, O> {
    ///Half duplex mode is not selected
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(HDSEL_A::FullDuplex)
    }
    ///Half duplex mode is selected
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut W {
        self.variant(HDSEL_A::HalfDuplex)
    }
}
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader<DMAR_A>;
///DMA enable receiver
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR_A {
    ///0: DMA mode is disabled for reception
    Disabled = 0,
    ///1: DMA mode is enabled for reception
    Enabled = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::Disabled,
            true => DMAR_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR_A::Enabled
    }
}
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAR_A, O>;
impl<'a, const O: u8> DMAR_W<'a, O> {
    ///DMA mode is disabled for reception
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::Disabled)
    }
    ///DMA mode is enabled for reception
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::Enabled)
    }
}
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader<DMAT_A>;
///DMA enable transmitter
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT_A {
    ///0: DMA mode is disabled for transmission
    Disabled = 0,
    ///1: DMA mode is enabled for transmission
    Enabled = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::Disabled,
            true => DMAT_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT_A::Enabled
    }
}
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAT_A, O>;
impl<'a, const O: u8> DMAT_W<'a, O> {
    ///DMA mode is disabled for transmission
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::Disabled)
    }
    ///DMA mode is enabled for transmission
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    ///Bit 1 - IrDA mode enable
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    ///Bit 2 - IrDA low-power
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UART4_CR3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
