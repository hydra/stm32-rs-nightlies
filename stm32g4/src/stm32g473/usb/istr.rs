///Register `ISTR` reader
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISTR` writer
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
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
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EP_ID` reader - EP_ID
pub type EP_ID_R = crate::FieldReader<u8, u8>;
///Field `EP_ID` writer - EP_ID
pub type EP_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISTR_SPEC, u8, u8, 4, O>;
///Field `DIR` reader - DIR
pub type DIR_R = crate::BitReader<bool>;
///Field `DIR` writer - DIR
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `L1REQ` reader - L1REQ
pub type L1REQ_R = crate::BitReader<bool>;
///Field `L1REQ` writer - L1REQ
pub type L1REQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `ESOF` reader - ESOF
pub type ESOF_R = crate::BitReader<bool>;
///Field `ESOF` writer - ESOF
pub type ESOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `SOF` reader - SOF
pub type SOF_R = crate::BitReader<bool>;
///Field `SOF` writer - SOF
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `RESET` reader - RESET
pub type RESET_R = crate::BitReader<bool>;
///Field `RESET` writer - RESET
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `SUSP` reader - SUSP
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - SUSP
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `WKUP` reader - WKUP
pub type WKUP_R = crate::BitReader<bool>;
///Field `WKUP` writer - WKUP
pub type WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `ERR` reader - ERR
pub type ERR_R = crate::BitReader<bool>;
///Field `ERR` writer - ERR
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `PMAOVR` reader - PMAOVR
pub type PMAOVR_R = crate::BitReader<bool>;
///Field `PMAOVR` writer - PMAOVR
pub type PMAOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
///Field `CTR` reader - CTR
pub type CTR_R = crate::BitReader<bool>;
///Field `CTR` writer - CTR
pub type CTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISTR_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - EP_ID
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - L1REQ
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ESOF
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SOF
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RESET
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - SUSP
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - WKUP
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - ERR
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PMAOVR
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CTR
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - EP_ID
    #[inline(always)]
    #[must_use]
    pub fn ep_id(&mut self) -> EP_ID_W<0> {
        EP_ID_W::new(self)
    }
    ///Bit 4 - DIR
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    ///Bit 7 - L1REQ
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<7> {
        L1REQ_W::new(self)
    }
    ///Bit 8 - ESOF
    #[inline(always)]
    #[must_use]
    pub fn esof(&mut self) -> ESOF_W<8> {
        ESOF_W::new(self)
    }
    ///Bit 9 - SOF
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<9> {
        SOF_W::new(self)
    }
    ///Bit 10 - RESET
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<10> {
        RESET_W::new(self)
    }
    ///Bit 11 - SUSP
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<11> {
        SUSP_W::new(self)
    }
    ///Bit 12 - WKUP
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<12> {
        WKUP_W::new(self)
    }
    ///Bit 13 - ERR
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<13> {
        ERR_W::new(self)
    }
    ///Bit 14 - PMAOVR
    #[inline(always)]
    #[must_use]
    pub fn pmaovr(&mut self) -> PMAOVR_W<14> {
        PMAOVR_W::new(self)
    }
    ///Bit 15 - CTR
    #[inline(always)]
    #[must_use]
    pub fn ctr(&mut self) -> CTR_W<15> {
        CTR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///USB interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [istr](index.html) module
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [istr::R](R) reader structure
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [istr::W](W) writer structure
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISTR to value 0
impl crate::Resettable for ISTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
