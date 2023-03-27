///Register `AHB1ENR` reader
pub struct R(crate::R<AHB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1ENR` writer
pub struct W(crate::W<AHB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1ENR_SPEC>;
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
impl From<crate::W<AHB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1EN` reader - DMA1 clock enable
pub type DMA1EN_R = crate::BitReader<DMA1EN_A>;
///DMA1 clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA1EN_A {
    ///0: DMAx clock disabled
    Disabled = 0,
    ///1: DMAx clock enabled
    Enabled = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::Disabled,
            true => DMA1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::Enabled
    }
}
///Field `DMA1EN` writer - DMA1 clock enable
pub type DMA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, DMA1EN_A, O>;
impl<'a, const O: u8> DMA1EN_W<'a, O> {
    ///DMAx clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Disabled)
    }
    ///DMAx clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::Enabled)
    }
}
///Field `DMA2EN` reader - DMA2 clock enable
pub use DMA1EN_R as DMA2EN_R;
///Field `DMA2EN` writer - DMA2 clock enable
pub use DMA1EN_W as DMA2EN_W;
///Field `DMAMUX1EN` reader - DMAMUX clock enable
pub type DMAMUX1EN_R = crate::BitReader<DMAMUX1EN_A>;
///DMAMUX clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX1EN_A {
    ///0: DMAMUX1 clock disabled
    Disabled = 0,
    ///1: DMAMUX1 clock enabled
    Enabled = 1,
}
impl From<DMAMUX1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX1EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX1EN_A {
        match self.bits {
            false => DMAMUX1EN_A::Disabled,
            true => DMAMUX1EN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAMUX1EN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAMUX1EN_A::Enabled
    }
}
///Field `DMAMUX1EN` writer - DMAMUX clock enable
pub type DMAMUX1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, DMAMUX1EN_A, O>;
impl<'a, const O: u8> DMAMUX1EN_W<'a, O> {
    ///DMAMUX1 clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::Disabled)
    }
    ///DMAMUX1 clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAMUX1EN_A::Enabled)
    }
}
///Field `FLASHEN` reader - Flash memory interface clock enable
pub type FLASHEN_R = crate::BitReader<FLASHEN_A>;
///Flash memory interface clock enable
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHEN_A {
    ///0: Flash memory interface clock disabled
    Disabled = 0,
    ///1: Flash memory interface clock enabled
    Enabled = 1,
}
impl From<FLASHEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FLASHEN_A {
        match self.bits {
            false => FLASHEN_A::Disabled,
            true => FLASHEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FLASHEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FLASHEN_A::Enabled
    }
}
///Field `FLASHEN` writer - Flash memory interface clock enable
pub type FLASHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, FLASHEN_A, O>;
impl<'a, const O: u8> FLASHEN_W<'a, O> {
    ///Flash memory interface clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHEN_A::Disabled)
    }
    ///Flash memory interface clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHEN_A::Enabled)
    }
}
///Field `CRCEN` reader - CRC clock enable
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
///CRC clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    ///0: CRC clock disabled
    Disabled = 0,
    ///1: CRC clock enabled
    Enabled = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::Disabled,
            true => CRCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::Enabled
    }
}
///Field `CRCEN` writer - CRC clock enable
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, CRCEN_A, O>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    ///CRC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Disabled)
    }
    ///CRC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::Enabled)
    }
}
///Field `TSCEN` reader - Touch Sensing Controller clock enable
pub type TSCEN_R = crate::BitReader<TSCEN_A>;
///Touch Sensing Controller clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSCEN_A {
    ///0: TSC clock disabled
    Disabled = 0,
    ///1: TSC clock enabled
    Enabled = 1,
}
impl From<TSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> TSCEN_A {
        match self.bits {
            false => TSCEN_A::Disabled,
            true => TSCEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSCEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSCEN_A::Enabled
    }
}
///Field `TSCEN` writer - Touch Sensing Controller clock enable
pub type TSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, TSCEN_A, O>;
impl<'a, const O: u8> TSCEN_W<'a, O> {
    ///TSC clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSCEN_A::Disabled)
    }
    ///TSC clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSCEN_A::Enabled)
    }
}
///Field `DMA2DEN` reader - DMA2D clock enable
pub type DMA2DEN_R = crate::BitReader<DMA2DEN_A>;
///DMA2D clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA2DEN_A {
    ///0: DMA2D clock disabled
    Disabled = 0,
    ///1: DMA2D clock enabled
    Enabled = 1,
}
impl From<DMA2DEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA2DEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA2DEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMA2DEN_A {
        match self.bits {
            false => DMA2DEN_A::Disabled,
            true => DMA2DEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA2DEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA2DEN_A::Enabled
    }
}
///Field `DMA2DEN` writer - DMA2D clock enable
pub type DMA2DEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, DMA2DEN_A, O>;
impl<'a, const O: u8> DMA2DEN_W<'a, O> {
    ///DMA2D clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA2DEN_A::Disabled)
    }
    ///DMA2D clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA2DEN_A::Enabled)
    }
}
///Field `GFXMMUEN` reader - Graphic MMU clock enable
pub type GFXMMUEN_R = crate::BitReader<GFXMMUEN_A>;
///Graphic MMU clock enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GFXMMUEN_A {
    ///0: GFXMMU clock disabled
    Disabled = 0,
    ///1: GFXMMU clock enabled
    Enabled = 1,
}
impl From<GFXMMUEN_A> for bool {
    #[inline(always)]
    fn from(variant: GFXMMUEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GFXMMUEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> GFXMMUEN_A {
        match self.bits {
            false => GFXMMUEN_A::Disabled,
            true => GFXMMUEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GFXMMUEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GFXMMUEN_A::Enabled
    }
}
///Field `GFXMMUEN` writer - Graphic MMU clock enable
pub type GFXMMUEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1ENR_SPEC, GFXMMUEN_A, O>;
impl<'a, const O: u8> GFXMMUEN_W<'a, O> {
    ///GFXMMU clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GFXMMUEN_A::Disabled)
    }
    ///GFXMMU clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GFXMMUEN_A::Enabled)
    }
}
impl R {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX clock enable
    #[inline(always)]
    pub fn dmamux1en(&self) -> DMAMUX1EN_R {
        DMAMUX1EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    pub fn flashen(&self) -> FLASHEN_R {
        FLASHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    pub fn tscen(&self) -> TSCEN_R {
        TSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA2D clock enable
    #[inline(always)]
    pub fn dma2den(&self) -> DMA2DEN_R {
        DMA2DEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Graphic MMU clock enable
    #[inline(always)]
    pub fn gfxmmuen(&self) -> GFXMMUEN_R {
        GFXMMUEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma1en(&mut self) -> DMA1EN_W<0> {
        DMA1EN_W::new(self)
    }
    ///Bit 1 - DMA2 clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma2en(&mut self) -> DMA2EN_W<1> {
        DMA2EN_W::new(self)
    }
    ///Bit 2 - DMAMUX clock enable
    #[inline(always)]
    #[must_use]
    pub fn dmamux1en(&mut self) -> DMAMUX1EN_W<2> {
        DMAMUX1EN_W::new(self)
    }
    ///Bit 8 - Flash memory interface clock enable
    #[inline(always)]
    #[must_use]
    pub fn flashen(&mut self) -> FLASHEN_W<8> {
        FLASHEN_W::new(self)
    }
    ///Bit 12 - CRC clock enable
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<12> {
        CRCEN_W::new(self)
    }
    ///Bit 16 - Touch Sensing Controller clock enable
    #[inline(always)]
    #[must_use]
    pub fn tscen(&mut self) -> TSCEN_W<16> {
        TSCEN_W::new(self)
    }
    ///Bit 17 - DMA2D clock enable
    #[inline(always)]
    #[must_use]
    pub fn dma2den(&mut self) -> DMA2DEN_W<17> {
        DMA2DEN_W::new(self)
    }
    ///Bit 18 - Graphic MMU clock enable
    #[inline(always)]
    #[must_use]
    pub fn gfxmmuen(&mut self) -> GFXMMUEN_W<18> {
        GFXMMUEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB1 peripheral clock enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1enr](index.html) module
pub struct AHB1ENR_SPEC;
impl crate::RegisterSpec for AHB1ENR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1enr::R](R) reader structure
impl crate::Readable for AHB1ENR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1enr::W](W) writer structure
impl crate::Writable for AHB1ENR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1ENR to value 0x0100
impl crate::Resettable for AHB1ENR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
