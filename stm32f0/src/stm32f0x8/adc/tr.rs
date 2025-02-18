///Register `TR` reader
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TR` writer
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT` reader - Analog watchdog lower threshold
pub type LT_R = crate::FieldReader<u16, u16>;
///Field `LT` writer - Analog watchdog lower threshold
pub type LT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TR_SPEC, u16, u16, 12, O>;
///Field `HT` reader - Analog watchdog higher threshold
pub type HT_R = crate::FieldReader<u16, u16>;
///Field `HT` writer - Analog watchdog higher threshold
pub type HT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog higher threshold
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog lower threshold
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
    }
    ///Bits 16:27 - Analog watchdog higher threshold
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<16> {
        HT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///watchdog threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr](index.html) module
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tr::R](R) reader structure
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tr::W](W) writer structure
impl crate::Writable for TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TR to value 0x0fff
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
