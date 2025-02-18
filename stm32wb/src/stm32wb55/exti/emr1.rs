///Register `EMR1` reader
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EMR1` writer
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EM0_15` reader - CPU(m) Wakeup with event generation Mask on Event input
pub type EM0_15_R = crate::FieldReader<u16, u16>;
///Field `EM0_15` writer - CPU(m) Wakeup with event generation Mask on Event input
pub type EM0_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR1_SPEC, u16, u16, 16, O>;
///Field `EM17_21` reader - CPU(m) Wakeup with event generation Mask on Event input
pub type EM17_21_R = crate::FieldReader<u8, u8>;
///Field `EM17_21` writer - CPU(m) Wakeup with event generation Mask on Event input
pub type EM17_21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR1_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em0_15(&self) -> EM0_15_R {
        EM0_15_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    pub fn em17_21(&self) -> EM17_21_R {
        EM17_21_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:15 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    #[must_use]
    pub fn em0_15(&mut self) -> EM0_15_W<0> {
        EM0_15_W::new(self)
    }
    ///Bits 17:21 - CPU(m) Wakeup with event generation Mask on Event input
    #[inline(always)]
    #[must_use]
    pub fn em17_21(&mut self) -> EM17_21_W<17> {
        EM17_21_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CPUm wakeup with event mask register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [emr1](index.html) module
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [emr1::R](R) reader structure
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [emr1::W](W) writer structure
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EMR1 to value 0
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
