///Register `TTTS` reader
pub struct R(crate::R<TTTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTTS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TTTS` writer
pub struct W(crate::W<TTTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTTS_SPEC>;
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
impl From<crate::W<TTTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTTS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWTDEL` reader - Stop watch trigger input selection
pub type SWTDEL_R = crate::FieldReader<u8, u8>;
///Field `SWTDEL` writer - Stop watch trigger input selection
pub type SWTDEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTS_SPEC, u8, u8, 2, O>;
///Field `EVTSEL` reader - Event trigger input selection
pub type EVTSEL_R = crate::FieldReader<u8, u8>;
///Field `EVTSEL` writer - Event trigger input selection
pub type EVTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTTS_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Stop watch trigger input selection
    #[inline(always)]
    pub fn swtdel(&self) -> SWTDEL_R {
        SWTDEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Event trigger input selection
    #[inline(always)]
    pub fn evtsel(&self) -> EVTSEL_R {
        EVTSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Stop watch trigger input selection
    #[inline(always)]
    #[must_use]
    pub fn swtdel(&mut self) -> SWTDEL_W<0> {
        SWTDEL_W::new(self)
    }
    ///Bits 4:5 - Event trigger input selection
    #[inline(always)]
    #[must_use]
    pub fn evtsel(&mut self) -> EVTSEL_W<4> {
        EVTSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FDCAN TT Trigger Select Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ttts](index.html) module
pub struct TTTS_SPEC;
impl crate::RegisterSpec for TTTS_SPEC {
    type Ux = u32;
}
///`read()` method returns [ttts::R](R) reader structure
impl crate::Readable for TTTS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ttts::W](W) writer structure
impl crate::Writable for TTTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TTTS to value 0
impl crate::Resettable for TTTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
