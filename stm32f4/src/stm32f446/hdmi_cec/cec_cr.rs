///Register `CEC_CR` reader
pub struct R(crate::R<CEC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEC_CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CEC_CR` writer
pub struct W(crate::W<CEC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEC_CR_SPEC>;
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
impl From<crate::W<CEC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEC_CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CECEN` reader - CEC Enable
pub type CECEN_R = crate::BitReader<bool>;
///Field `CECEN` writer - CEC Enable
pub type CECEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CEC_CR_SPEC, bool, O>;
///Field `TXSOM` reader - Tx Start Of Message
pub type TXSOM_R = crate::BitReader<bool>;
///Field `TXEOM` reader - Tx End Of Message
pub type TXEOM_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx Start Of Message
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tx End Of Message
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    #[must_use]
    pub fn cecen(&mut self) -> CECEN_W<0> {
        CECEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CEC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cec_cr](index.html) module
pub struct CEC_CR_SPEC;
impl crate::RegisterSpec for CEC_CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cec_cr::R](R) reader structure
impl crate::Readable for CEC_CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cec_cr::W](W) writer structure
impl crate::Writable for CEC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CEC_CR to value 0
impl crate::Resettable for CEC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
