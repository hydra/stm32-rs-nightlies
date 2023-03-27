///Register `RTSR2` reader
pub struct R(crate::R<RTSR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTSR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTSR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTSR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RTSR2` writer
pub struct W(crate::W<RTSR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTSR2_SPEC>;
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
impl From<crate::W<RTSR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTSR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RT35` reader - Rising trigger event configuration bit of configurable event input x
pub type RT35_R = crate::BitReader<bool>;
///Field `RT35` writer - Rising trigger event configuration bit of configurable event input x
pub type RT35_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, bool, O>;
///Field `RT36` reader - Rising trigger event configuration bit of configurable event input x
pub type RT36_R = crate::BitReader<bool>;
///Field `RT36` writer - Rising trigger event configuration bit of configurable event input x
pub type RT36_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, bool, O>;
///Field `RT37` reader - Rising trigger event configuration bit of configurable event input x
pub type RT37_R = crate::BitReader<bool>;
///Field `RT37` writer - Rising trigger event configuration bit of configurable event input x
pub type RT37_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, bool, O>;
///Field `RT38` reader - Rising trigger event configuration bit of configurable event input x
pub type RT38_R = crate::BitReader<bool>;
///Field `RT38` writer - Rising trigger event configuration bit of configurable event input x
pub type RT38_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTSR2_SPEC, bool, O>;
impl R {
    ///Bit 3 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt35(&self) -> RT35_R {
        RT35_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt36(&self) -> RT36_R {
        RT36_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt37(&self) -> RT37_R {
        RT37_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    pub fn rt38(&self) -> RT38_R {
        RT38_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 3 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    #[must_use]
    pub fn rt35(&mut self) -> RT35_W<3> {
        RT35_W::new(self)
    }
    ///Bit 4 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    #[must_use]
    pub fn rt36(&mut self) -> RT36_W<4> {
        RT36_W::new(self)
    }
    ///Bit 5 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    #[must_use]
    pub fn rt37(&mut self) -> RT37_W<5> {
        RT37_W::new(self)
    }
    ///Bit 6 - Rising trigger event configuration bit of configurable event input x
    #[inline(always)]
    #[must_use]
    pub fn rt38(&mut self) -> RT38_W<6> {
        RT38_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///EXTI rising trigger selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rtsr2](index.html) module
pub struct RTSR2_SPEC;
impl crate::RegisterSpec for RTSR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [rtsr2::R](R) reader structure
impl crate::Readable for RTSR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rtsr2::W](W) writer structure
impl crate::Writable for RTSR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RTSR2 to value 0
impl crate::Resettable for RTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
