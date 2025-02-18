///Register `JCHGR` reader
pub struct R(crate::R<JCHGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JCHGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JCHGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JCHGR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `JCHGR` writer
pub struct W(crate::W<JCHGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JCHGR_SPEC>;
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
impl From<crate::W<JCHGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JCHGR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `JCHG` reader - Injected channel group selection JCHG\[y\]=0: channel y is not part of the injected group JCHG\[y\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.
pub type JCHG_R = crate::FieldReader<u8, u8>;
///Field `JCHG` writer - Injected channel group selection JCHG\[y\]=0: channel y is not part of the injected group JCHG\[y\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.
pub type JCHG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, JCHGR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - Injected channel group selection JCHG\[y\]=0: channel y is not part of the injected group JCHG\[y\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.
    #[inline(always)]
    pub fn jchg(&self) -> JCHG_R {
        JCHG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Injected channel group selection JCHG\[y\]=0: channel y is not part of the injected group JCHG\[y\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored.
    #[inline(always)]
    #[must_use]
    pub fn jchg(&mut self) -> JCHG_W<0> {
        JCHG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jchgr](index.html) module
pub struct JCHGR_SPEC;
impl crate::RegisterSpec for JCHGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [jchgr::R](R) reader structure
impl crate::Readable for JCHGR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [jchgr::W](W) writer structure
impl crate::Writable for JCHGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets JCHGR to value 0x01
impl crate::Resettable for JCHGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
