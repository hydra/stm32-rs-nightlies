///Register `M1FDRL` reader
pub struct R(crate::R<M1FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M1FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M1FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M1FDRL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `M1FDRL` writer
pub struct W(crate::W<M1FDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M1FDRL_SPEC>;
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
impl From<crate::W<M1FDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M1FDRL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FDATAL` reader - ECC failing data low
pub type FDATAL_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - ECC failing data low
    #[inline(always)]
    pub fn fdatal(&self) -> FDATAL_R {
        FDATAL_R::new(self.bits)
    }
}
impl W {
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMECC monitor 1 failing data low register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m1fdrl](index.html) module
pub struct M1FDRL_SPEC;
impl crate::RegisterSpec for M1FDRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [m1fdrl::R](R) reader structure
impl crate::Readable for M1FDRL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [m1fdrl::W](W) writer structure
impl crate::Writable for M1FDRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets M1FDRL to value 0
impl crate::Resettable for M1FDRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
