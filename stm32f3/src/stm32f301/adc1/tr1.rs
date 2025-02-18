///Register `TR1` reader
pub struct R(crate::R<TR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TR1` writer
pub struct W(crate::W<TR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR1_SPEC>;
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
impl From<crate::W<TR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT1` reader - Analog watchdog 1 lower threshold
pub type LT1_R = crate::FieldReader<u16, u16>;
///Field `LT1` writer - Analog watchdog 1 lower threshold
pub type LT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TR1_SPEC, u16, u16, 12, O>;
///Field `HT1` reader - Analog watchdog 1 higher threshold
pub type HT1_R = crate::FieldReader<u16, u16>;
///Field `HT1` writer - Analog watchdog 1 higher threshold
pub type HT1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TR1_SPEC, u16, u16, 12, O>;
impl R {
    ///Bits 0:11 - Analog watchdog 1 lower threshold
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Analog watchdog 1 lower threshold
    #[inline(always)]
    #[must_use]
    pub fn lt1(&mut self) -> LT1_W<0> {
        LT1_W::new(self)
    }
    ///Bits 16:27 - Analog watchdog 1 higher threshold
    #[inline(always)]
    #[must_use]
    pub fn ht1(&mut self) -> HT1_W<16> {
        HT1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC watchdog threshold register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr1](index.html) module
pub struct TR1_SPEC;
impl crate::RegisterSpec for TR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [tr1::R](R) reader structure
impl crate::Readable for TR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tr1::W](W) writer structure
impl crate::Writable for TR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TR1 to value 0x0fff_0000
impl crate::Resettable for TR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
