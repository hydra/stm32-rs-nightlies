///Register `MESR` reader
pub struct R(crate::R<MESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MESR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MESR` writer
pub struct W(crate::W<MESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MESR_SPEC>;
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
impl From<crate::W<MESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MESR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MCLR` reader - MCLR
pub type MCLR_R = crate::BitReader<bool>;
///Field `MCLR` writer - MCLR
pub type MCLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
///Field `IPMEE` reader - IPMEE
pub type IPMEE_R = crate::BitReader<bool>;
///Field `IPMEE` writer - IPMEE
pub type IPMEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MESR_SPEC, bool, O>;
impl R {
    ///Bit 0 - MCLR
    #[inline(always)]
    pub fn mclr(&self) -> MCLR_R {
        MCLR_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - IPMEE
    #[inline(always)]
    pub fn ipmee(&self) -> IPMEE_R {
        IPMEE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - MCLR
    #[inline(always)]
    #[must_use]
    pub fn mclr(&mut self) -> MCLR_W<0> {
        MCLR_W::new(self)
    }
    ///Bit 16 - IPMEE
    #[inline(always)]
    #[must_use]
    pub fn ipmee(&mut self) -> IPMEE_W<16> {
        IPMEE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///memory erase status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mesr](index.html) module
pub struct MESR_SPEC;
impl crate::RegisterSpec for MESR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mesr::R](R) reader structure
impl crate::Readable for MESR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mesr::W](W) writer structure
impl crate::Writable for MESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MESR to value 0
impl crate::Resettable for MESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
