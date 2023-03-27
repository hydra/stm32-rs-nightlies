///Register `CFGR2` reader
pub struct R(crate::R<CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CFGR2` writer
pub struct W(crate::W<CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR2_SPEC>;
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
impl From<crate::W<CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IN1SEL` reader - LPTIM input 1 selection
pub type IN1SEL_R = crate::FieldReader<u8, u8>;
///Field `IN1SEL` writer - LPTIM input 1 selection
pub type IN1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
///Field `IN2SEL` reader - LPTIM input 2 selection
pub type IN2SEL_R = crate::FieldReader<u8, u8>;
///Field `IN2SEL` writer - LPTIM input 2 selection
pub type IN2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
///Field `IC1SEL` reader - LPTIM input capture 1 selection
pub type IC1SEL_R = crate::FieldReader<u8, u8>;
///Field `IC1SEL` writer - LPTIM input capture 1 selection
pub type IC1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
///Field `IC2SEL` reader - LPTIM input capture 2 selection
pub type IC2SEL_R = crate::FieldReader<u8, u8>;
///Field `IC2SEL` writer - LPTIM input capture 2 selection
pub type IC2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - LPTIM input 1 selection
    #[inline(always)]
    pub fn in1sel(&self) -> IN1SEL_R {
        IN1SEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - LPTIM input 2 selection
    #[inline(always)]
    pub fn in2sel(&self) -> IN2SEL_R {
        IN2SEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:17 - LPTIM input capture 1 selection
    #[inline(always)]
    pub fn ic1sel(&self) -> IC1SEL_R {
        IC1SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 20:21 - LPTIM input capture 2 selection
    #[inline(always)]
    pub fn ic2sel(&self) -> IC2SEL_R {
        IC2SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - LPTIM input 1 selection
    #[inline(always)]
    #[must_use]
    pub fn in1sel(&mut self) -> IN1SEL_W<0> {
        IN1SEL_W::new(self)
    }
    ///Bits 4:5 - LPTIM input 2 selection
    #[inline(always)]
    #[must_use]
    pub fn in2sel(&mut self) -> IN2SEL_W<4> {
        IN2SEL_W::new(self)
    }
    ///Bits 16:17 - LPTIM input capture 1 selection
    #[inline(always)]
    #[must_use]
    pub fn ic1sel(&mut self) -> IC1SEL_W<16> {
        IC1SEL_W::new(self)
    }
    ///Bits 20:21 - LPTIM input capture 2 selection
    #[inline(always)]
    #[must_use]
    pub fn ic2sel(&mut self) -> IC2SEL_W<20> {
        IC2SEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPTIM configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cfgr2](index.html) module
pub struct CFGR2_SPEC;
impl crate::RegisterSpec for CFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cfgr2::R](R) reader structure
impl crate::Readable for CFGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cfgr2::W](W) writer structure
impl crate::Writable for CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CFGR2 to value 0
impl crate::Resettable for CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
