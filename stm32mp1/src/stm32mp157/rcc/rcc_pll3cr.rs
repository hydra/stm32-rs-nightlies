///Register `RCC_PLL3CR` reader
pub struct R(crate::R<RCC_PLL3CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_PLL3CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_PLL3CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_PLL3CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCC_PLL3CR` writer
pub struct W(crate::W<RCC_PLL3CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_PLL3CR_SPEC>;
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
impl From<crate::W<RCC_PLL3CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_PLL3CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PLLON` reader - PLLON
pub type PLLON_R = crate::BitReader<bool>;
///Field `PLLON` writer - PLLON
pub type PLLON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CR_SPEC, bool, O>;
///Field `PLL3RDY` reader - PLL3RDY
pub type PLL3RDY_R = crate::BitReader<bool>;
///Field `SSCG_CTRL` reader - SSCG_CTRL
pub type SSCG_CTRL_R = crate::BitReader<bool>;
///Field `SSCG_CTRL` writer - SSCG_CTRL
pub type SSCG_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CR_SPEC, bool, O>;
///Field `DIVPEN` reader - DIVPEN
pub type DIVPEN_R = crate::BitReader<bool>;
///Field `DIVPEN` writer - DIVPEN
pub type DIVPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CR_SPEC, bool, O>;
///Field `DIVQEN` reader - DIVQEN
pub type DIVQEN_R = crate::BitReader<bool>;
///Field `DIVQEN` writer - DIVQEN
pub type DIVQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CR_SPEC, bool, O>;
///Field `DIVREN` reader - DIVREN
pub type DIVREN_R = crate::BitReader<bool>;
///Field `DIVREN` writer - DIVREN
pub type DIVREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_PLL3CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - PLLON
    #[inline(always)]
    pub fn pllon(&self) -> PLLON_R {
        PLLON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL3RDY
    #[inline(always)]
    pub fn pll3rdy(&self) -> PLL3RDY_R {
        PLL3RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SSCG_CTRL
    #[inline(always)]
    pub fn sscg_ctrl(&self) -> SSCG_CTRL_R {
        SSCG_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - DIVPEN
    #[inline(always)]
    pub fn divpen(&self) -> DIVPEN_R {
        DIVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DIVQEN
    #[inline(always)]
    pub fn divqen(&self) -> DIVQEN_R {
        DIVQEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DIVREN
    #[inline(always)]
    pub fn divren(&self) -> DIVREN_R {
        DIVREN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PLLON
    #[inline(always)]
    #[must_use]
    pub fn pllon(&mut self) -> PLLON_W<0> {
        PLLON_W::new(self)
    }
    ///Bit 2 - SSCG_CTRL
    #[inline(always)]
    #[must_use]
    pub fn sscg_ctrl(&mut self) -> SSCG_CTRL_W<2> {
        SSCG_CTRL_W::new(self)
    }
    ///Bit 4 - DIVPEN
    #[inline(always)]
    #[must_use]
    pub fn divpen(&mut self) -> DIVPEN_W<4> {
        DIVPEN_W::new(self)
    }
    ///Bit 5 - DIVQEN
    #[inline(always)]
    #[must_use]
    pub fn divqen(&mut self) -> DIVQEN_W<5> {
        DIVQEN_W::new(self)
    }
    ///Bit 6 - DIVREN
    #[inline(always)]
    #[must_use]
    pub fn divren(&mut self) -> DIVREN_W<6> {
        DIVREN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to control the PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcc_pll3cr](index.html) module
pub struct RCC_PLL3CR_SPEC;
impl crate::RegisterSpec for RCC_PLL3CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcc_pll3cr::R](R) reader structure
impl crate::Readable for RCC_PLL3CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcc_pll3cr::W](W) writer structure
impl crate::Writable for RCC_PLL3CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCC_PLL3CR to value 0
impl crate::Resettable for RCC_PLL3CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
