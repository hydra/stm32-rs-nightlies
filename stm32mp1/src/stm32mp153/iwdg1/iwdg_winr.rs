///Register `IWDG_WINR` reader
pub struct R(crate::R<IWDG_WINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWDG_WINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWDG_WINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWDG_WINR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IWDG_WINR` writer
pub struct W(crate::W<IWDG_WINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWDG_WINR_SPEC>;
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
impl From<crate::W<IWDG_WINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWDG_WINR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WIN` reader - WIN
pub type WIN_R = crate::FieldReader<u16, u16>;
///Field `WIN` writer - WIN
pub type WIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IWDG_WINR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - WIN
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - WIN
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<0> {
        WIN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Window register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iwdg_winr](index.html) module
pub struct IWDG_WINR_SPEC;
impl crate::RegisterSpec for IWDG_WINR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iwdg_winr::R](R) reader structure
impl crate::Readable for IWDG_WINR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iwdg_winr::W](W) writer structure
impl crate::Writable for IWDG_WINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IWDG_WINR to value 0x0fff
impl crate::Resettable for IWDG_WINR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
