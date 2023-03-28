///Register `VSCR` reader
pub struct R(crate::R<VSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VSCR` writer
pub struct W(crate::W<VSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VSCR_SPEC>;
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
impl From<crate::W<VSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - Enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VSCR_SPEC, bool, O>;
///Field `UR` reader - Update register
pub type UR_R = crate::BitReader<bool>;
///Field `UR` writer - Update register
pub type UR_W<'a, const O: u8> = crate::BitWriter<'a, u32, VSCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Update register
    #[inline(always)]
    pub fn ur(&self) -> UR_R {
        UR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 8 - Update register
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UR_W<8> {
        UR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host video shadow control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [vscr](index.html) module
pub struct VSCR_SPEC;
impl crate::RegisterSpec for VSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [vscr::R](R) reader structure
impl crate::Readable for VSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [vscr::W](W) writer structure
impl crate::Writable for VSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VSCR to value 0
impl crate::Resettable for VSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
