///Register `WIER` reader
pub struct R(crate::R<WIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `WIER` writer
pub struct W(crate::W<WIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIER_SPEC>;
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
impl From<crate::W<WIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TEIE` reader - Tearing effect interrupt enable
pub type TEIE_R = crate::BitReader<bool>;
///Field `TEIE` writer - Tearing effect interrupt enable
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
///Field `ERIE` reader - End of refresh interrupt enable
pub type ERIE_R = crate::BitReader<bool>;
///Field `ERIE` writer - End of refresh interrupt enable
pub type ERIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
///Field `PLLLIE` reader - PLL lock interrupt enable
pub type PLLLIE_R = crate::BitReader<bool>;
///Field `PLLLIE` writer - PLL lock interrupt enable
pub type PLLLIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
///Field `PLLUIE` reader - PLL unlock interrupt enable
pub type PLLUIE_R = crate::BitReader<bool>;
///Field `PLLUIE` writer - PLL unlock interrupt enable
pub type PLLUIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
///Field `RRIE` reader - Regulator ready interrupt enable
pub type RRIE_R = crate::BitReader<bool>;
///Field `RRIE` writer - Regulator ready interrupt enable
pub type RRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Tearing effect interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of refresh interrupt enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - PLL lock interrupt enable
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL unlock interrupt enable
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Regulator ready interrupt enable
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Tearing effect interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<0> {
        TEIE_W::new(self)
    }
    ///Bit 1 - End of refresh interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn erie(&mut self) -> ERIE_W<1> {
        ERIE_W::new(self)
    }
    ///Bit 9 - PLL lock interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn plllie(&mut self) -> PLLLIE_W<9> {
        PLLLIE_W::new(self)
    }
    ///Bit 10 - PLL unlock interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn plluie(&mut self) -> PLLUIE_W<10> {
        PLLUIE_W::new(self)
    }
    ///Bit 13 - Regulator ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rrie(&mut self) -> RRIE_W<13> {
        RRIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI wrapper interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wier](index.html) module
pub struct WIER_SPEC;
impl crate::RegisterSpec for WIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [wier::R](R) reader structure
impl crate::Readable for WIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [wier::W](W) writer structure
impl crate::Writable for WIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WIER to value 0
impl crate::Resettable for WIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
