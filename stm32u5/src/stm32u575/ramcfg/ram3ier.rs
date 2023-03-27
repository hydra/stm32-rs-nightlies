///Register `RAM3IER` reader
pub struct R(crate::R<RAM3IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM3IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM3IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM3IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RAM3IER` writer
pub struct W(crate::W<RAM3IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM3IER_SPEC>;
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
impl From<crate::W<RAM3IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM3IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEIE` reader - SEIE
pub type SEIE_R = crate::BitReader<bool>;
///Field `SEIE` writer - SEIE
pub type SEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM3IER_SPEC, bool, O>;
///Field `DEIE` reader - DEIE
pub type DEIE_R = crate::BitReader<bool>;
///Field `DEIE` writer - DEIE
pub type DEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM3IER_SPEC, bool, O>;
///Field `ECCNMI` reader - ECCNMI
pub type ECCNMI_R = crate::BitReader<bool>;
///Field `ECCNMI` writer - ECCNMI
pub type ECCNMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAM3IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - SEIE
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DEIE
    #[inline(always)]
    pub fn deie(&self) -> DEIE_R {
        DEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - ECCNMI
    #[inline(always)]
    pub fn eccnmi(&self) -> ECCNMI_R {
        ECCNMI_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SEIE
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<0> {
        SEIE_W::new(self)
    }
    ///Bit 1 - DEIE
    #[inline(always)]
    #[must_use]
    pub fn deie(&mut self) -> DEIE_W<1> {
        DEIE_W::new(self)
    }
    ///Bit 3 - ECCNMI
    #[inline(always)]
    #[must_use]
    pub fn eccnmi(&mut self) -> ECCNMI_W<3> {
        ECCNMI_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///RAMCFG SRAM x interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram3ier](index.html) module
pub struct RAM3IER_SPEC;
impl crate::RegisterSpec for RAM3IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ram3ier::R](R) reader structure
impl crate::Readable for RAM3IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ram3ier::W](W) writer structure
impl crate::Writable for RAM3IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM3IER to value 0
impl crate::Resettable for RAM3IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
