///Register `MDF_OEC1CR` reader
pub struct R(crate::R<MDF_OEC1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_OEC1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_OEC1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_OEC1CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MDF_OEC1CR` writer
pub struct W(crate::W<MDF_OEC1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDF_OEC1CR_SPEC>;
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
impl From<crate::W<MDF_OEC1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDF_OEC1CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET` reader - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
///field will inform the application on the current offset value. OFFSET\[25:0\]
///represents the value to be subtracted to the signal before going to the SCALE.
pub type OFFSET_R = crate::FieldReader<u32, u32>;
///Field `OFFSET` writer - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
///field will inform the application on the current offset value. OFFSET\[25:0\]
///represents the value to be subtracted to the signal before going to the SCALE.
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDF_OEC1CR_SPEC, u32, u32, 26, O>;
impl R {
    ///Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
    ///field will inform the application on the current offset value. OFFSET\[25:0\]
    ///represents the value to be subtracted to the signal before going to the SCALE.
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    ///Bits 0:25 - Offset error compensation Set and cleared by software. If the application attempts to write a new offset value while the previous one is not yet applied, this new offset value is ignored. Reading back the OFFSET\[25:0\]
    ///field will inform the application on the current offset value. OFFSET\[25:0\]
    ///represents the value to be subtracted to the signal before going to the SCALE.
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///This register contains the offset compensation value.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_oec1cr](index.html) module
pub struct MDF_OEC1CR_SPEC;
impl crate::RegisterSpec for MDF_OEC1CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_oec1cr::R](R) reader structure
impl crate::Readable for MDF_OEC1CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mdf_oec1cr::W](W) writer structure
impl crate::Writable for MDF_OEC1CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MDF_OEC1CR to value 0
impl crate::Resettable for MDF_OEC1CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
