///Register `GETCAPR` reader
pub struct R(crate::R<GETCAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GETCAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GETCAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GETCAPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `GETCAPR` writer
pub struct W(crate::W<GETCAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GETCAPR_SPEC>;
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
impl From<crate::W<GETCAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GETCAPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAPPEND` reader - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
///value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.
pub type CAPPEND_R = crate::BitReader<bool>;
///Field `CAPPEND` writer - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
///value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.
pub type CAPPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, GETCAPR_SPEC, bool, O>;
impl R {
    ///Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
    ///value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.
    #[inline(always)]
    pub fn cappend(&self) -> CAPPEND_R {
        CAPPEND_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    ///Bit 14 - IBI MDB support for pending read notification This bit is written by software during bus initialization (i.e. I3C_CFGR.EN=0) and indicates the support (or not) of the pending read notification via the IBI MDB\[7:0\]
    ///value. This bit is used to return the GETCAP3 byte in response to the GETCAPS CCC format 1.
    #[inline(always)]
    #[must_use]
    pub fn cappend(&mut self) -> CAPPEND_W<14> {
        CAPPEND_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I3C get capability register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [getcapr](index.html) module
pub struct GETCAPR_SPEC;
impl crate::RegisterSpec for GETCAPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [getcapr::R](R) reader structure
impl crate::Readable for GETCAPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [getcapr::W](W) writer structure
impl crate::Writable for GETCAPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets GETCAPR to value 0
impl crate::Resettable for GETCAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
