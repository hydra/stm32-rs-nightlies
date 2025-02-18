///Register `RCR` reader
pub struct R(crate::R<RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RCR` writer
pub struct W(crate::W<RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR_SPEC>;
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
impl From<crate::W<RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PAGE0_WP` reader - CCM SRAM page write protection bit
pub type PAGE0_WP_R = crate::BitReader<PAGE0_WP_A>;
///CCM SRAM page write protection bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAGE0_WP_A {
    ///0: Write protection of pagex is disabled
    Disabled = 0,
    ///1: Write protection of pagex is enabled
    Enabled = 1,
}
impl From<PAGE0_WP_A> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP_A) -> Self {
        variant as u8 != 0
    }
}
impl PAGE0_WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> PAGE0_WP_A {
        match self.bits {
            false => PAGE0_WP_A::Disabled,
            true => PAGE0_WP_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAGE0_WP_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAGE0_WP_A::Enabled
    }
}
///Field `PAGE0_WP` writer - CCM SRAM page write protection bit
pub type PAGE0_WP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR_SPEC, PAGE0_WP_A, O>;
impl<'a, const O: u8> PAGE0_WP_W<'a, O> {
    ///Write protection of pagex is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::Disabled)
    }
    ///Write protection of pagex is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PAGE0_WP_A::Enabled)
    }
}
///Field `PAGE1_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE1_WP_R;
///Field `PAGE2_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE2_WP_R;
///Field `PAGE3_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE3_WP_R;
///Field `PAGE1_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE1_WP_W;
///Field `PAGE2_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE2_WP_W;
///Field `PAGE3_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE3_WP_W;
impl R {
    ///Bit 0 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CCM SRAM page write protection bit
    #[inline(always)]
    #[must_use]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W<0> {
        PAGE0_WP_W::new(self)
    }
    ///Bit 1 - CCM SRAM page write protection bit
    #[inline(always)]
    #[must_use]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W<1> {
        PAGE1_WP_W::new(self)
    }
    ///Bit 2 - CCM SRAM page write protection bit
    #[inline(always)]
    #[must_use]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W<2> {
        PAGE2_WP_W::new(self)
    }
    ///Bit 3 - CCM SRAM page write protection bit
    #[inline(always)]
    #[must_use]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W<3> {
        PAGE3_WP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CCM SRAM protection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rcr](index.html) module
pub struct RCR_SPEC;
impl crate::RegisterSpec for RCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rcr::R](R) reader structure
impl crate::Readable for RCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rcr::W](W) writer structure
impl crate::Writable for RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RCR to value 0
impl crate::Resettable for RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
