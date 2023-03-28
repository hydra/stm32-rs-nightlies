///Register `MACTSAR` reader
pub struct R(crate::R<MACTSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACTSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACTSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACTSAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MACTSAR` writer
pub struct W(crate::W<MACTSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACTSAR_SPEC>;
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
impl From<crate::W<MACTSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACTSAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSAR` reader - Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization.
pub type TSAR_R = crate::FieldReader<u32, u32>;
///Field `TSAR` writer - Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization.
pub type TSAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACTSAR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization.
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Timestamp Addend Register This field indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization.
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<0> {
        TSAR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timestamp addend register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mactsar](index.html) module
pub struct MACTSAR_SPEC;
impl crate::RegisterSpec for MACTSAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mactsar::R](R) reader structure
impl crate::Readable for MACTSAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mactsar::W](W) writer structure
impl crate::Writable for MACTSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets MACTSAR to value 0
impl crate::Resettable for MACTSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
