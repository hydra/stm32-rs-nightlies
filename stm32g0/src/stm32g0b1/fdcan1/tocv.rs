///Register `TOCV` reader
pub struct R(crate::R<TOCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOCV_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TOCV` writer
pub struct W(crate::W<TOCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOCV_SPEC>;
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
impl From<crate::W<TOCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOCV_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TOC` reader - Timeout counter The timeout counter is decremented in multiples of CAN bit times \[1 â¦ 16\]
///depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS.
pub type TOC_R = crate::FieldReader<u16, u16>;
///Field `TOC` writer - Timeout counter The timeout counter is decremented in multiples of CAN bit times \[1 â¦ 16\]
///depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS.
pub type TOC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TOCV_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Timeout counter The timeout counter is decremented in multiples of CAN bit times \[1 â¦ 16\]
    ///depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS.
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timeout counter The timeout counter is decremented in multiples of CAN bit times \[1 â¦ 16\]
    ///depending on the configuration of TSCC.TCP. When decremented to 0, interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start and reset/restart conditions are configured via TOCC.TOS.
    #[inline(always)]
    #[must_use]
    pub fn toc(&mut self) -> TOC_W<0> {
        TOC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN timeout counter value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tocv](index.html) module
pub struct TOCV_SPEC;
impl crate::RegisterSpec for TOCV_SPEC {
    type Ux = u32;
}
///`read()` method returns [tocv::R](R) reader structure
impl crate::Readable for TOCV_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tocv::W](W) writer structure
impl crate::Writable for TOCV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TOCV to value 0xffff
impl crate::Resettable for TOCV_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
