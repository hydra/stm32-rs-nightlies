///Register `DTS_ITR1` reader
pub struct R(crate::R<DTS_ITR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTS_ITR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTS_ITR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTS_ITR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DTS_ITR1` writer
pub struct W(crate::W<DTS_ITR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTS_ITR1_SPEC>;
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
impl From<crate::W<DTS_ITR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTS_ITR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TS1_LITTHD` reader - TS1_LITTHD
pub type TS1_LITTHD_R = crate::FieldReader<u16, u16>;
///Field `TS1_LITTHD` writer - TS1_LITTHD
pub type TS1_LITTHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_ITR1_SPEC, u16, u16, 16, O>;
///Field `TS1_HITTHD` reader - TS1_HITTHD
pub type TS1_HITTHD_R = crate::FieldReader<u16, u16>;
///Field `TS1_HITTHD` writer - TS1_HITTHD
pub type TS1_HITTHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DTS_ITR1_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - TS1_LITTHD
    #[inline(always)]
    pub fn ts1_litthd(&self) -> TS1_LITTHD_R {
        TS1_LITTHD_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - TS1_HITTHD
    #[inline(always)]
    pub fn ts1_hitthd(&self) -> TS1_HITTHD_R {
        TS1_HITTHD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - TS1_LITTHD
    #[inline(always)]
    #[must_use]
    pub fn ts1_litthd(&mut self) -> TS1_LITTHD_W<0> {
        TS1_LITTHD_W::new(self)
    }
    ///Bits 16:31 - TS1_HITTHD
    #[inline(always)]
    #[must_use]
    pub fn ts1_hitthd(&mut self) -> TS1_HITTHD_W<16> {
        TS1_HITTHD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DTS_ITR1 contains the threshold values for sensor 1.
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dts_itr1](index.html) module
pub struct DTS_ITR1_SPEC;
impl crate::RegisterSpec for DTS_ITR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [dts_itr1::R](R) reader structure
impl crate::Readable for DTS_ITR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dts_itr1::W](W) writer structure
impl crate::Writable for DTS_ITR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DTS_ITR1 to value 0
impl crate::Resettable for DTS_ITR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
