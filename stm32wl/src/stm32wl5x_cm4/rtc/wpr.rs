///Register `WPR` writer
pub struct W(crate::W<WPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPR_SPEC>;
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
impl From<crate::W<WPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPR_SPEC>) -> Self {
        W(writer)
    }
}
///Write protection key
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEY_AW {
    ///0: Activate write protection (any value that is not the keys)
    Activate = 0,
    ///83: Key 2
    Deactivate2 = 83,
    ///202: Key 1
    Deactivate1 = 202,
}
impl From<KEY_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEY_AW) -> Self {
        variant as _
    }
}
///Field `KEY` writer - Write protection key
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WPR_SPEC, u8, KEY_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    ///Activate write protection (any value that is not the keys)
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(KEY_AW::Activate)
    }
    ///Key 2
    #[inline(always)]
    pub fn deactivate2(self) -> &'a mut W {
        self.variant(KEY_AW::Deactivate2)
    }
    ///Key 1
    #[inline(always)]
    pub fn deactivate1(self) -> &'a mut W {
        self.variant(KEY_AW::Deactivate1)
    }
}
impl W {
    ///Bits 0:7 - Write protection key
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Write protection register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [wpr](index.html) module
pub struct WPR_SPEC;
impl crate::RegisterSpec for WPR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [wpr::W](W) writer structure
impl crate::Writable for WPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets WPR to value 0
impl crate::Resettable for WPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
