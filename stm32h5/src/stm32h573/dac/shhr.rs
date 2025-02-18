///Register `SHHR` reader
pub struct R(crate::R<SHHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHHR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SHHR` writer
pub struct W(crate::W<SHHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHHR_SPEC>;
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
impl From<crate::W<SHHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHHR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `THOLD1` reader - DAC channel1 hold time (only valid in Sample and hold mode) Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN1 = 0.
pub type THOLD1_R = crate::FieldReader<u16, u16>;
///Field `THOLD1` writer - DAC channel1 hold time (only valid in Sample and hold mode) Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN1 = 0.
pub type THOLD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHHR_SPEC, u16, u16, 10, O>;
///Field `THOLD2` reader - DAC channel2 hold time (only valid in Sample and hold mode). Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN2 = 0. These bits are available only on dual-channel DACs. Refer to implementation.
pub type THOLD2_R = crate::FieldReader<u16, u16>;
///Field `THOLD2` writer - DAC channel2 hold time (only valid in Sample and hold mode). Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN2 = 0. These bits are available only on dual-channel DACs. Refer to implementation.
pub type THOLD2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHHR_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode) Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    pub fn thold1(&self) -> THOLD1_R {
        THOLD1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 16:25 - DAC channel2 hold time (only valid in Sample and hold mode). Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN2 = 0. These bits are available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    pub fn thold2(&self) -> THOLD2_R {
        THOLD2_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    ///Bits 0:9 - DAC channel1 hold time (only valid in Sample and hold mode) Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN1 = 0.
    #[inline(always)]
    #[must_use]
    pub fn thold1(&mut self) -> THOLD1_W<0> {
        THOLD1_W::new(self)
    }
    ///Bits 16:25 - DAC channel2 hold time (only valid in Sample and hold mode). Hold time = (THOLD\[9:0\]) x LSI/LSE clock period Note: This register can be modified only when EN2 = 0. These bits are available only on dual-channel DACs. Refer to implementation.
    #[inline(always)]
    #[must_use]
    pub fn thold2(&mut self) -> THOLD2_W<16> {
        THOLD2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DAC sample and hold time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [shhr](index.html) module
pub struct SHHR_SPEC;
impl crate::RegisterSpec for SHHR_SPEC {
    type Ux = u32;
}
///`read()` method returns [shhr::R](R) reader structure
impl crate::Readable for SHHR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [shhr::W](W) writer structure
impl crate::Writable for SHHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SHHR to value 0x0001_0001
impl crate::Resettable for SHHR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
