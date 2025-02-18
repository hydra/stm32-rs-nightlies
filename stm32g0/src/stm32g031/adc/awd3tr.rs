///Register `AWD3TR` reader
pub struct R(crate::R<AWD3TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AWD3TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AWD3TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AWD3TR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AWD3TR` writer
pub struct W(crate::W<AWD3TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AWD3TR_SPEC>;
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
impl From<crate::W<AWD3TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AWD3TR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT3` reader - ADC analog watchdog 3 threshold high
pub type LT3_R = crate::FieldReader<u16, u16>;
///Field `LT3` writer - ADC analog watchdog 3 threshold high
pub type LT3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWD3TR_SPEC, u16, u16, 12, O>;
///Field `HT3` reader - ADC analog watchdog 3 threshold high
pub type HT3_R = crate::FieldReader<u16, u16>;
///Field `HT3` writer - ADC analog watchdog 3 threshold high
pub type HT3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, AWD3TR_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<0> {
        LT3_W::new(self)
    }
    ///Bits 16:27 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<16> {
        HT3_W::new(self)
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
///For information about available fields see [awd3tr](index.html) module
pub struct AWD3TR_SPEC;
impl crate::RegisterSpec for AWD3TR_SPEC {
    type Ux = u32;
}
///`read()` method returns [awd3tr::R](R) reader structure
impl crate::Readable for AWD3TR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [awd3tr::W](W) writer structure
impl crate::Writable for AWD3TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AWD3TR to value 0x0fff_0000
impl crate::Resettable for AWD3TR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
