///Register `DTER` reader
pub struct R(crate::R<DTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTER` writer
pub struct W(crate::W<DTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTER_SPEC>;
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
impl From<crate::W<DTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTRx` reader - Deadtime Rising value
pub type DTRX_R = crate::FieldReader<u16, u16>;
///Field `DTRx` writer - Deadtime Rising value
pub type DTRX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTER_SPEC, u16, u16, 9, O>;
///Field `SDTRx` reader - Sign Deadtime Rising value
pub type SDTRX_R = crate::BitReader<bool>;
///Field `SDTRx` writer - Sign Deadtime Rising value
pub type SDTRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
///Field `DTPRSC` reader - Deadtime Prescaler
pub type DTPRSC_R = crate::FieldReader<u8, u8>;
///Field `DTPRSC` writer - Deadtime Prescaler
pub type DTPRSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTER_SPEC, u8, u8, 3, O>;
///Field `DTRSLKx` reader - Deadtime Rising Sign Lock
pub type DTRSLKX_R = crate::BitReader<bool>;
///Field `DTRSLKx` writer - Deadtime Rising Sign Lock
pub type DTRSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
///Field `DTRLKx` reader - Deadtime Rising Lock
pub type DTRLKX_R = crate::BitReader<bool>;
///Field `DTRLKx` writer - Deadtime Rising Lock
pub type DTRLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
///Field `DTFx` reader - Deadtime Falling value
pub type DTFX_R = crate::FieldReader<u16, u16>;
///Field `DTFx` writer - Deadtime Falling value
pub type DTFX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DTER_SPEC, u16, u16, 9, O>;
///Field `SDTFx` reader - Sign Deadtime Falling value
pub type SDTFX_R = crate::BitReader<bool>;
///Field `SDTFx` writer - Sign Deadtime Falling value
pub type SDTFX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
///Field `DTFSLKx` reader - Deadtime Falling Sign Lock
pub type DTFSLKX_R = crate::BitReader<bool>;
///Field `DTFSLKx` writer - Deadtime Falling Sign Lock
pub type DTFSLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
///Field `DTFLKx` reader - Deadtime Falling Lock
pub type DTFLKX_R = crate::BitReader<bool>;
///Field `DTFLKx` writer - Deadtime Falling Lock
pub type DTFLKX_W<'a, const O: u8> = crate::BitWriter<'a, u32, DTER_SPEC, bool, O>;
impl R {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    pub fn dtrx(&self) -> DTRX_R {
        DTRX_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    pub fn sdtrx(&self) -> SDTRX_R {
        SDTRX_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    pub fn dtprsc(&self) -> DTPRSC_R {
        DTPRSC_R::new(((self.bits >> 10) & 7) as u8)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    pub fn dtrslkx(&self) -> DTRSLKX_R {
        DTRSLKX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    pub fn dtrlkx(&self) -> DTRLKX_R {
        DTRLKX_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    pub fn dtfx(&self) -> DTFX_R {
        DTFX_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    pub fn sdtfx(&self) -> SDTFX_R {
        SDTFX_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    pub fn dtfslkx(&self) -> DTFSLKX_R {
        DTFSLKX_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    pub fn dtflkx(&self) -> DTFLKX_R {
        DTFLKX_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:8 - Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn dtrx(&mut self) -> DTRX_W<0> {
        DTRX_W::new(self)
    }
    ///Bit 9 - Sign Deadtime Rising value
    #[inline(always)]
    #[must_use]
    pub fn sdtrx(&mut self) -> SDTRX_W<9> {
        SDTRX_W::new(self)
    }
    ///Bits 10:12 - Deadtime Prescaler
    #[inline(always)]
    #[must_use]
    pub fn dtprsc(&mut self) -> DTPRSC_W<10> {
        DTPRSC_W::new(self)
    }
    ///Bit 14 - Deadtime Rising Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrslkx(&mut self) -> DTRSLKX_W<14> {
        DTRSLKX_W::new(self)
    }
    ///Bit 15 - Deadtime Rising Lock
    #[inline(always)]
    #[must_use]
    pub fn dtrlkx(&mut self) -> DTRLKX_W<15> {
        DTRLKX_W::new(self)
    }
    ///Bits 16:24 - Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn dtfx(&mut self) -> DTFX_W<16> {
        DTFX_W::new(self)
    }
    ///Bit 25 - Sign Deadtime Falling value
    #[inline(always)]
    #[must_use]
    pub fn sdtfx(&mut self) -> SDTFX_W<25> {
        SDTFX_W::new(self)
    }
    ///Bit 30 - Deadtime Falling Sign Lock
    #[inline(always)]
    #[must_use]
    pub fn dtfslkx(&mut self) -> DTFSLKX_W<30> {
        DTFSLKX_W::new(self)
    }
    ///Bit 31 - Deadtime Falling Lock
    #[inline(always)]
    #[must_use]
    pub fn dtflkx(&mut self) -> DTFLKX_W<31> {
        DTFLKX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timerx Deadtime Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dter](index.html) module
pub struct DTER_SPEC;
impl crate::RegisterSpec for DTER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dter::R](R) reader structure
impl crate::Readable for DTER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dter::W](W) writer structure
impl crate::Writable for DTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTER to value 0
impl crate::Resettable for DTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
