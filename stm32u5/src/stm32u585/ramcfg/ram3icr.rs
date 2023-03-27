///Register `RAM3ICR` reader
pub struct R(crate::R<RAM3ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM3ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM3ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM3ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RAM3ICR` writer
pub struct W(crate::W<RAM3ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM3ICR_SPEC>;
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
impl From<crate::W<RAM3ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM3ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSEDC` reader - CSEDC
pub type CSEDC_R = crate::BitReader<bool>;
///Field `CSEDC` writer - CSEDC
pub type CSEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM3ICR_SPEC, bool, O>;
///Field `CDED` reader - CDED
pub type CDED_R = crate::BitReader<bool>;
///Field `CDED` writer - CDED
pub type CDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM3ICR_SPEC, bool, O>;
impl R {
    ///Bit 0 - CSEDC
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CDED
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CSEDC
    #[inline(always)]
    #[must_use]
    pub fn csedc(&mut self) -> CSEDC_W<0> {
        CSEDC_W::new(self)
    }
    ///Bit 1 - CDED
    #[inline(always)]
    #[must_use]
    pub fn cded(&mut self) -> CDED_W<1> {
        CDED_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG RAM x interrupt clear register x
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram3icr](index.html) module
pub struct RAM3ICR_SPEC;
impl crate::RegisterSpec for RAM3ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram3icr::R](R) reader structure
impl crate::Readable for RAM3ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ram3icr::W](W) writer structure
impl crate::Writable for RAM3ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM3ICR to value 0
impl crate::Resettable for RAM3ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
