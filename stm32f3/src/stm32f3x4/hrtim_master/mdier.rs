///Register `MDIER` reader
pub struct R(crate::R<MDIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDIER` writer
pub struct W(crate::W<MDIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDIER_SPEC>;
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
impl From<crate::W<MDIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCMP1IE` reader - MCMP1IE
pub type MCMP1IE_R = crate::BitReader<MCMP1IE_A>;
///MCMP1IE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1IE_A {
    ///0: Interrupt disabled
    Disabled = 0,
    ///1: Interrupt enabled
    Enabled = 1,
}
impl From<MCMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCMP1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCMP1IE_A {
        match self.bits {
            false => MCMP1IE_A::Disabled,
            true => MCMP1IE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCMP1IE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCMP1IE_A::Enabled
    }
}
///Field `MCMP1IE` writer - MCMP1IE
pub type MCMP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDIER_SPEC, MCMP1IE_A, O>;
impl<'a, const O: u8> MCMP1IE_W<'a, O> {
    ///Interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP1IE_A::Disabled)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP1IE_A::Enabled)
    }
}
///Field `MCMP2IE` reader - MCMP2IE
pub use MCMP1IE_R as MCMP2IE_R;
///Field `MCMP3IE` reader - MCMP3IE
pub use MCMP1IE_R as MCMP3IE_R;
///Field `MCMP4IE` reader - MCMP4IE
pub use MCMP1IE_R as MCMP4IE_R;
///Field `MREPIE` reader - MREPIE
pub use MCMP1IE_R as MREPIE_R;
///Field `SYNCIE` reader - SYNCIE
pub use MCMP1IE_R as SYNCIE_R;
///Field `MUPDIE` reader - MUPDIE
pub use MCMP1IE_R as MUPDIE_R;
///Field `MCMP2IE` writer - MCMP2IE
pub use MCMP1IE_W as MCMP2IE_W;
///Field `MCMP3IE` writer - MCMP3IE
pub use MCMP1IE_W as MCMP3IE_W;
///Field `MCMP4IE` writer - MCMP4IE
pub use MCMP1IE_W as MCMP4IE_W;
///Field `MREPIE` writer - MREPIE
pub use MCMP1IE_W as MREPIE_W;
///Field `SYNCIE` writer - SYNCIE
pub use MCMP1IE_W as SYNCIE_W;
///Field `MUPDIE` writer - MUPDIE
pub use MCMP1IE_W as MUPDIE_W;
///Field `MCMP1DE` reader - MCMP1DE
pub type MCMP1DE_R = crate::BitReader<MCMP1DE_A>;
///MCMP1DE
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCMP1DE_A {
    ///0: DMA request disabled
    Disabled = 0,
    ///1: DMA request enabled
    Enabled = 1,
}
impl From<MCMP1DE_A> for bool {
    #[inline(always)]
    fn from(variant: MCMP1DE_A) -> Self {
        variant as u8 != 0
    }
}
impl MCMP1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MCMP1DE_A {
        match self.bits {
            false => MCMP1DE_A::Disabled,
            true => MCMP1DE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCMP1DE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCMP1DE_A::Enabled
    }
}
///Field `MCMP1DE` writer - MCMP1DE
pub type MCMP1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDIER_SPEC, MCMP1DE_A, O>;
impl<'a, const O: u8> MCMP1DE_W<'a, O> {
    ///DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCMP1DE_A::Disabled)
    }
    ///DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCMP1DE_A::Enabled)
    }
}
///Field `MCMP2DE` reader - MCMP2DE
pub use MCMP1DE_R as MCMP2DE_R;
///Field `MCMP3DE` reader - MCMP3DE
pub use MCMP1DE_R as MCMP3DE_R;
///Field `MCMP4DE` reader - MCMP4DE
pub use MCMP1DE_R as MCMP4DE_R;
///Field `MREPDE` reader - MREPDE
pub use MCMP1DE_R as MREPDE_R;
///Field `SYNCDE` reader - SYNCDE
pub use MCMP1DE_R as SYNCDE_R;
///Field `MUPDDE` reader - MUPDDE
pub use MCMP1DE_R as MUPDDE_R;
///Field `MCMP2DE` writer - MCMP2DE
pub use MCMP1DE_W as MCMP2DE_W;
///Field `MCMP3DE` writer - MCMP3DE
pub use MCMP1DE_W as MCMP3DE_W;
///Field `MCMP4DE` writer - MCMP4DE
pub use MCMP1DE_W as MCMP4DE_W;
///Field `MREPDE` writer - MREPDE
pub use MCMP1DE_W as MREPDE_W;
///Field `SYNCDE` writer - SYNCDE
pub use MCMP1DE_W as SYNCDE_W;
///Field `MUPDDE` writer - MUPDDE
pub use MCMP1DE_W as MUPDDE_W;
impl R {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    pub fn mcmp1ie(&self) -> MCMP1IE_R {
        MCMP1IE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    pub fn mcmp2ie(&self) -> MCMP2IE_R {
        MCMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    pub fn mcmp3ie(&self) -> MCMP3IE_R {
        MCMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    pub fn mcmp4ie(&self) -> MCMP4IE_R {
        MCMP4IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    pub fn mrepie(&self) -> MREPIE_R {
        MREPIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    pub fn syncie(&self) -> SYNCIE_R {
        SYNCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    pub fn mupdie(&self) -> MUPDIE_R {
        MUPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    pub fn mcmp1de(&self) -> MCMP1DE_R {
        MCMP1DE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    pub fn mcmp2de(&self) -> MCMP2DE_R {
        MCMP2DE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    pub fn mcmp3de(&self) -> MCMP3DE_R {
        MCMP3DE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    pub fn mcmp4de(&self) -> MCMP4DE_R {
        MCMP4DE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    pub fn mrepde(&self) -> MREPDE_R {
        MREPDE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    pub fn syncde(&self) -> SYNCDE_R {
        SYNCDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    pub fn mupdde(&self) -> MUPDDE_R {
        MUPDDE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MCMP1IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1ie(&mut self) -> MCMP1IE_W<0> {
        MCMP1IE_W::new(self)
    }
    ///Bit 1 - MCMP2IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2ie(&mut self) -> MCMP2IE_W<1> {
        MCMP2IE_W::new(self)
    }
    ///Bit 2 - MCMP3IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3ie(&mut self) -> MCMP3IE_W<2> {
        MCMP3IE_W::new(self)
    }
    ///Bit 3 - MCMP4IE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4ie(&mut self) -> MCMP4IE_W<3> {
        MCMP4IE_W::new(self)
    }
    ///Bit 4 - MREPIE
    #[inline(always)]
    #[must_use]
    pub fn mrepie(&mut self) -> MREPIE_W<4> {
        MREPIE_W::new(self)
    }
    ///Bit 5 - SYNCIE
    #[inline(always)]
    #[must_use]
    pub fn syncie(&mut self) -> SYNCIE_W<5> {
        SYNCIE_W::new(self)
    }
    ///Bit 6 - MUPDIE
    #[inline(always)]
    #[must_use]
    pub fn mupdie(&mut self) -> MUPDIE_W<6> {
        MUPDIE_W::new(self)
    }
    ///Bit 16 - MCMP1DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp1de(&mut self) -> MCMP1DE_W<16> {
        MCMP1DE_W::new(self)
    }
    ///Bit 17 - MCMP2DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp2de(&mut self) -> MCMP2DE_W<17> {
        MCMP2DE_W::new(self)
    }
    ///Bit 18 - MCMP3DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp3de(&mut self) -> MCMP3DE_W<18> {
        MCMP3DE_W::new(self)
    }
    ///Bit 19 - MCMP4DE
    #[inline(always)]
    #[must_use]
    pub fn mcmp4de(&mut self) -> MCMP4DE_W<19> {
        MCMP4DE_W::new(self)
    }
    ///Bit 20 - MREPDE
    #[inline(always)]
    #[must_use]
    pub fn mrepde(&mut self) -> MREPDE_W<20> {
        MREPDE_W::new(self)
    }
    ///Bit 21 - SYNCDE
    #[inline(always)]
    #[must_use]
    pub fn syncde(&mut self) -> SYNCDE_W<21> {
        SYNCDE_W::new(self)
    }
    ///Bit 22 - MUPDDE
    #[inline(always)]
    #[must_use]
    pub fn mupdde(&mut self) -> MUPDDE_W<22> {
        MUPDDE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///MDIER4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdier](index.html) module
pub struct MDIER_SPEC;
impl crate::RegisterSpec for MDIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdier::R](R) reader structure
impl crate::Readable for MDIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdier::W](W) writer structure
impl crate::Writable for MDIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDIER to value 0
impl crate::Resettable for MDIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
