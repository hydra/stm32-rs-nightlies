///Register `IPCC_C1CR` reader
pub struct R(crate::R<IPCC_C1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCC_C1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCC_C1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCC_C1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IPCC_C1CR` writer
pub struct W(crate::W<IPCC_C1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCC_C1CR_SPEC>;
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
impl From<crate::W<IPCC_C1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCC_C1CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RXOIE` reader - RXOIE
pub type RXOIE_R = crate::BitReader<bool>;
///Field `RXOIE` writer - RXOIE
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCC_C1CR_SPEC, bool, O>;
///Field `TXFIE` reader - TXFIE
pub type TXFIE_R = crate::BitReader<bool>;
///Field `TXFIE` writer - TXFIE
pub type TXFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCC_C1CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - RXOIE
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - RXOIE
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<0> {
        RXOIE_W::new(self)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<16> {
        TXFIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///IPCC Processor 1 control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ipcc_c1cr](index.html) module
pub struct IPCC_C1CR_SPEC;
impl crate::RegisterSpec for IPCC_C1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ipcc_c1cr::R](R) reader structure
impl crate::Readable for IPCC_C1CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ipcc_c1cr::W](W) writer structure
impl crate::Writable for IPCC_C1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IPCC_C1CR to value 0
impl crate::Resettable for IPCC_C1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
