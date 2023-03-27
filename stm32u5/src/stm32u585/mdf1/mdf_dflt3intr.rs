///Register `MDF_DFLT3INTR` reader
pub struct R(crate::R<MDF_DFLT3INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_DFLT3INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_DFLT3INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_DFLT3INTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_DFLT3INTR` writer
pub struct W(crate::W<MDF_DFLT3INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_DFLT3INTR_SPEC>;
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
impl From<crate::W<MDF_DFLT3INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_DFLT3INTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INTDIV` reader - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTDIV_R = crate::FieldReader<u8, u8>;
///Field `INTDIV` writer - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3INTR_SPEC, u8, u8, 2, O>;
///Field `INTVAL` reader - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTVAL_R = crate::FieldReader<u8, u8>;
///Field `INTVAL` writer - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type INTVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_DFLT3INTR_SPEC, u8, u8, 7, O>;
impl R {
    ///Bits 0:1 - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intdiv(&self) -> INTDIV_R {
        INTDIV_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:10 - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn intval(&self) -> INTVAL_R {
        INTVAL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
impl W {
    ///Bits 0:1 - Integrator output division Set and cleared by software. This bit is used to rescale the signal at the integrator output in order keep the data width lower than 24 bits. - 00: The integrator data outputs are divided by 128 (Default value) - 01: The integrator data outputs are divided by 32 - 10: The integrator data outputs are divided by 4 - 11: The integrator data outputs are not divided This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn intdiv(&mut self) -> INTDIV_W<0> {
        INTDIV_W::new(self)
    }
    ///Bits 4:10 - Integration value selection Set and cleared by software. This field is used to select the integration value. - 0: The integration value is 1, meaning bypass mode (default after reset) - 1: The integration value is 2 - 2: The integration value is 3 ... - 127: The integration value is 128 This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    #[must_use]
    pub fn intval(&mut self) -> INTVAL_W<4> {
        INTVAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register is used to the integrator (INT) settings.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_dflt3intr](index.html) module
pub struct MDF_DFLT3INTR_SPEC;
impl crate::RegisterSpec for MDF_DFLT3INTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_dflt3intr::R](R) reader structure
impl crate::Readable for MDF_DFLT3INTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_dflt3intr::W](W) writer structure
impl crate::Writable for MDF_DFLT3INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_DFLT3INTR to value 0
impl crate::Resettable for MDF_DFLT3INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
