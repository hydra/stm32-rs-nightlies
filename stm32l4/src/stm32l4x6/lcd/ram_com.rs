///Register `RAM_COM%s` reader
pub struct R(crate::R<RAM_COM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_COM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAM_COM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAM_COM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RAM_COM%s` writer
pub struct W(crate::W<RAM_COM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAM_COM_SPEC>;
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
impl From<crate::W<RAM_COM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAM_COM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SEGS` reader - Segment states, one bit per segment, LSB: S00, MSB: S39
pub type SEGS_R = crate::FieldReader<u64, u64>;
///Field `SEGS` writer - Segment states, one bit per segment, LSB: S00, MSB: S39
pub type SEGS_W<'a, const O: u8> = crate::FieldWriter<'a, u64, RAM_COM_SPEC, u64, u64, 40, O>;
impl R {
    ///Bits 0:39 - Segment states, one bit per segment, LSB: S00, MSB: S39
    #[inline(always)]
    pub fn segs(&self) -> SEGS_R {
        SEGS_R::new(self.bits & 0x00ff_ffff_ffff)
    }
}
impl W {
    ///Bits 0:39 - Segment states, one bit per segment, LSB: S00, MSB: S39
    #[inline(always)]
    #[must_use]
    pub fn segs(&mut self) -> SEGS_W<0> {
        SEGS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u64) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///display memory
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ram_com](index.html) module
pub struct RAM_COM_SPEC;
impl crate::RegisterSpec for RAM_COM_SPEC {
    type Ux = u64;
}
///`read()` method returns [ram_com::R](R) reader structure
impl crate::Readable for RAM_COM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ram_com::W](W) writer structure
impl crate::Writable for RAM_COM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RAM_COM%s to value 0
impl crate::Resettable for RAM_COM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
