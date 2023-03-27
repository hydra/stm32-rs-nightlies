///Register `TCCR5` reader
pub struct R(crate::R<TCCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TCCR5` writer
pub struct W(crate::W<TCCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR5_SPEC>;
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
impl From<crate::W<TCCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSWR_TOCNT` reader - Low-Power Write Timeout Counter
pub type LSWR_TOCNT_R = crate::FieldReader<u16, u16>;
///Field `LSWR_TOCNT` writer - Low-Power Write Timeout Counter
pub type LSWR_TOCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCCR5_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Low-Power Write Timeout Counter
    #[inline(always)]
    pub fn lswr_tocnt(&self) -> LSWR_TOCNT_R {
        LSWR_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Low-Power Write Timeout Counter
    #[inline(always)]
    #[must_use]
    pub fn lswr_tocnt(&mut self) -> LSWR_TOCNT_W<0> {
        LSWR_TOCNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DSI Host Timeout Counter Configuration Register5
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tccr5](index.html) module
pub struct TCCR5_SPEC;
impl crate::RegisterSpec for TCCR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [tccr5::R](R) reader structure
impl crate::Readable for TCCR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tccr5::W](W) writer structure
impl crate::Writable for TCCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TCCR5 to value 0
impl crate::Resettable for TCCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
