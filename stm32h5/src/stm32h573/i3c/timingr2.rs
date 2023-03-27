///Register `TIMINGR2` reader
pub struct R(crate::R<TIMINGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMINGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMINGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMINGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMINGR2` writer
pub struct W(crate::W<TIMINGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMINGR2_SPEC>;
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
impl From<crate::W<TIMINGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMINGR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `STALLT` reader - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
pub type STALLT_R = crate::BitReader<bool>;
///Field `STALLT` writer - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
pub type STALLT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGR2_SPEC, bool, O>;
///Field `STALLD` reader - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
pub type STALLD_R = crate::BitReader<bool>;
///Field `STALLD` writer - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
pub type STALLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGR2_SPEC, bool, O>;
///Field `STALLC` reader - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
pub type STALLC_R = crate::BitReader<bool>;
///Field `STALLC` writer - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
pub type STALLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGR2_SPEC, bool, O>;
///Field `STALLA` reader - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
pub type STALLA_R = crate::BitReader<bool>;
///Field `STALLA` writer - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
pub type STALLA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMINGR2_SPEC, bool, O>;
///Field `STALL` reader - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
pub type STALL_R = crate::FieldReader<u8, u8>;
///Field `STALL` writer - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
pub type STALL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMINGR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
    #[inline(always)]
    pub fn stallt(&self) -> STALLT_R {
        STALLT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
    #[inline(always)]
    pub fn stalld(&self) -> STALLD_R {
        STALLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
    #[inline(always)]
    pub fn stallc(&self) -> STALLC_R {
        STALLC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
    #[inline(always)]
    pub fn stalla(&self) -> STALLA_R {
        STALLA_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Controller clock stall on T-bit phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to prepare data to be sent.
    #[inline(always)]
    #[must_use]
    pub fn stallt(&mut self) -> STALLT_W<0> {
        STALLT_W::new(self)
    }
    ///Bit 1 - controller clock stall on PAR phase of Data enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase (before 9th bit). This allows the target to read received data.
    #[inline(always)]
    #[must_use]
    pub fn stalld(&mut self) -> STALLD_W<1> {
        STALLD_W::new(self)
    }
    ///Bit 2 - controller clock stall on PAR phase of CCC enable The SCL is stalled during STALL x tSCLL_PP in the T-bit phase of common command code (before 9th bit). This allows the target to decode the command.
    #[inline(always)]
    #[must_use]
    pub fn stallc(&mut self) -> STALLC_W<2> {
        STALLC_W::new(self)
    }
    ///Bit 3 - controller clock stall enable on ACK phase The SCL is stalled (during tSCLL_STALLas defined by STALL) in the address ACK/NACK phase (before 9th bit). This allows the target to prepare data or the controller to respond to target interrupt.
    #[inline(always)]
    #[must_use]
    pub fn stalla(&mut self) -> STALLA_W<3> {
        STALLA_W::new(self)
    }
    ///Bits 8:15 - controller clock stall time, in number of kernel clock cycles tSCLL_STALL = STALL x tI3CCLK
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<8> {
        STALL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C timing register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timingr2](index.html) module
pub struct TIMINGR2_SPEC;
impl crate::RegisterSpec for TIMINGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [timingr2::R](R) reader structure
impl crate::Readable for TIMINGR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timingr2::W](W) writer structure
impl crate::Writable for TIMINGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMINGR2 to value 0
impl crate::Resettable for TIMINGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
