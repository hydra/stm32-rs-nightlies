///Register `CHPAR` reader
pub struct R(crate::R<CHPAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPAR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CHPAR` writer
pub struct W(crate::W<CHPAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPAR_SPEC>;
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
impl From<crate::W<CHPAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPAR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CARFRQ` reader - Timerx carrier frequency value
pub type CARFRQ_R = crate::FieldReader<u8, u8>;
///Field `CARFRQ` writer - Timerx carrier frequency value
pub type CARFRQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPAR_SPEC, u8, u8, 4, O>;
///Field `CARDTY` reader - Timerx chopper duty cycle value
pub type CARDTY_R = crate::FieldReader<u8, u8>;
///Field `CARDTY` writer - Timerx chopper duty cycle value
pub type CARDTY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPAR_SPEC, u8, u8, 3, O>;
///Field `STRTPW` reader - STRTPW
pub type STRTPW_R = crate::FieldReader<u8, u8>;
///Field `STRTPW` writer - STRTPW
pub type STRTPW_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CHPAR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - Timerx carrier frequency value
    #[inline(always)]
    pub fn carfrq(&self) -> CARFRQ_R {
        CARFRQ_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Timerx chopper duty cycle value
    #[inline(always)]
    pub fn cardty(&self) -> CARDTY_R {
        CARDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 7:10 - STRTPW
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Timerx carrier frequency value
    #[inline(always)]
    #[must_use]
    pub fn carfrq(&mut self) -> CARFRQ_W<0> {
        CARFRQ_W::new(self)
    }
    ///Bits 4:6 - Timerx chopper duty cycle value
    #[inline(always)]
    #[must_use]
    pub fn cardty(&mut self) -> CARDTY_W<4> {
        CARDTY_W::new(self)
    }
    ///Bits 7:10 - STRTPW
    #[inline(always)]
    #[must_use]
    pub fn strtpw(&mut self) -> STRTPW_W<7> {
        STRTPW_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Chopper Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [chpar](index.html) module
pub struct CHPAR_SPEC;
impl crate::RegisterSpec for CHPAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [chpar::R](R) reader structure
impl crate::Readable for CHPAR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [chpar::W](W) writer structure
impl crate::Writable for CHPAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CHPAR to value 0
impl crate::Resettable for CHPAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
