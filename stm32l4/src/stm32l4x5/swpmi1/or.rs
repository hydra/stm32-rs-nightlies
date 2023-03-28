///Register `OR` reader
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OR` writer
pub struct W(crate::W<OR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OR_SPEC>;
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
impl From<crate::W<OR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWP_TBYP` reader - SWP transceiver bypass
pub type SWP_TBYP_R = crate::BitReader<bool>;
///Field `SWP_TBYP` writer - SWP transceiver bypass
pub type SWP_TBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
///Field `SWP_CLASS` reader - SWP class selection
pub type SWP_CLASS_R = crate::BitReader<bool>;
///Field `SWP_CLASS` writer - SWP class selection
pub type SWP_CLASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    pub fn swp_tbyp(&self) -> SWP_TBYP_R {
        SWP_TBYP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    pub fn swp_class(&self) -> SWP_CLASS_R {
        SWP_CLASS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SWP transceiver bypass
    #[inline(always)]
    #[must_use]
    pub fn swp_tbyp(&mut self) -> SWP_TBYP_W<0> {
        SWP_TBYP_W::new(self)
    }
    ///Bit 1 - SWP class selection
    #[inline(always)]
    #[must_use]
    pub fn swp_class(&mut self) -> SWP_CLASS_W<1> {
        SWP_CLASS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SWPMI Option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [or](index.html) module
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
///`read()` method returns [or::R](R) reader structure
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [or::W](W) writer structure
impl crate::Writable for OR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
