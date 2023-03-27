///Register `M2ICR` reader
pub struct R(crate::R<M2ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M2ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M2ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M2ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `M2ICR` writer
pub struct W(crate::W<M2ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M2ICR_SPEC>;
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
impl From<crate::W<M2ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M2ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CSEDC` reader - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value.
pub type CSEDC_R = crate::BitReader<bool>;
///Field `CSEDC` writer - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value.
pub type CSEDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, M2ICR_SPEC, bool, O>;
///Field `CDED` reader - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value.
pub type CDED_R = crate::BitReader<bool>;
///Field `CDED` writer - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value.
pub type CDED_W<'a, const O: u8> = crate::BitWriter<'a, u32, M2ICR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value.
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value.
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value.
    #[inline(always)]
    #[must_use]
    pub fn csedc(&mut self) -> CSEDC_W<0> {
        CSEDC_W::new(self)
    }
    ///Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value.
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
///RAMCFG memory 2 interrupt clear register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [m2icr](index.html) module
pub struct M2ICR_SPEC;
impl crate::RegisterSpec for M2ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [m2icr::R](R) reader structure
impl crate::Readable for M2ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [m2icr::W](W) writer structure
impl crate::Writable for M2ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets M2ICR to value 0
impl crate::Resettable for M2ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
