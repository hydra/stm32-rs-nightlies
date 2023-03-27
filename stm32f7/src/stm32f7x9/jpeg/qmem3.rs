///Register `QMEM3%s` reader
pub struct R(crate::R<QMEM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QMEM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QMEM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QMEM3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `QMEM3%s` writer
pub struct W(crate::W<QMEM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QMEM3_SPEC>;
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
impl From<crate::W<QMEM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QMEM3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `QMem_RAM` reader - QMem RAM
pub type QMEM_RAM_R = crate::FieldReader<u32, u32>;
///Field `QMem_RAM` writer - QMem RAM
pub type QMEM_RAM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, QMEM3_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    pub fn qmem_ram(&self) -> QMEM_RAM_R {
        QMEM_RAM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - QMem RAM
    #[inline(always)]
    #[must_use]
    pub fn qmem_ram(&mut self) -> QMEM_RAM_W<0> {
        QMEM_RAM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///JPEG quantization tables
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [qmem3](index.html) module
pub struct QMEM3_SPEC;
impl crate::RegisterSpec for QMEM3_SPEC {
    type Ux = u32;
}
///`read()` method returns [qmem3::R](R) reader structure
impl crate::Readable for QMEM3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [qmem3::W](W) writer structure
impl crate::Writable for QMEM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets QMEM3%s to value 0
impl crate::Resettable for QMEM3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
