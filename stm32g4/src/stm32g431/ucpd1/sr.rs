///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TXIS` reader - TXIS
pub type TXIS_R = crate::BitReader<bool>;
///Field `TXIS` writer - TXIS
pub type TXIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TXMSGDISC` reader - TXMSGDISC
pub type TXMSGDISC_R = crate::BitReader<bool>;
///Field `TXMSGDISC` writer - TXMSGDISC
pub type TXMSGDISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TXMSGSENT` reader - TXMSGSENT
pub type TXMSGSENT_R = crate::BitReader<bool>;
///Field `TXMSGSENT` writer - TXMSGSENT
pub type TXMSGSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TXMSGABT` reader - TXMSGABT
pub type TXMSGABT_R = crate::BitReader<bool>;
///Field `TXMSGABT` writer - TXMSGABT
pub type TXMSGABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `HRSTDISC` reader - HRSTDISC
pub type HRSTDISC_R = crate::BitReader<bool>;
///Field `HRSTDISC` writer - HRSTDISC
pub type HRSTDISC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `HRSTSENT` reader - HRSTSENT
pub type HRSTSENT_R = crate::BitReader<bool>;
///Field `HRSTSENT` writer - HRSTSENT
pub type HRSTSENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TXUND` reader - TXUND
pub type TXUND_R = crate::BitReader<bool>;
///Field `TXUND` writer - TXUND
pub type TXUND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXNE` reader - RXNE
pub type RXNE_R = crate::BitReader<bool>;
///Field `RXNE` writer - RXNE
pub type RXNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXORDDET` reader - RXORDDET
pub type RXORDDET_R = crate::BitReader<bool>;
///Field `RXORDDET` writer - RXORDDET
pub type RXORDDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXHRSTDET` reader - RXHRSTDET
pub type RXHRSTDET_R = crate::BitReader<bool>;
///Field `RXHRSTDET` writer - RXHRSTDET
pub type RXHRSTDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXOVR` reader - RXOVR
pub type RXOVR_R = crate::BitReader<bool>;
///Field `RXOVR` writer - RXOVR
pub type RXOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXMSGEND` reader - RXMSGEND
pub type RXMSGEND_R = crate::BitReader<bool>;
///Field `RXMSGEND` writer - RXMSGEND
pub type RXMSGEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `RXERR` reader - RXERR
pub type RXERR_R = crate::BitReader<bool>;
///Field `RXERR` writer - RXERR
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TYPECEVT1` reader - TYPECEVT1
pub type TYPECEVT1_R = crate::BitReader<bool>;
///Field `TYPECEVT1` writer - TYPECEVT1
pub type TYPECEVT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TYPECEVT2` reader - TYPECEVT2
pub type TYPECEVT2_R = crate::BitReader<bool>;
///Field `TYPECEVT2` writer - TYPECEVT2
pub type TYPECEVT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `TYPEC_VSTATE_CC1` reader - TYPEC_VSTATE_CC1
pub type TYPEC_VSTATE_CC1_R = crate::FieldReader<u8, u8>;
///Field `TYPEC_VSTATE_CC1` writer - TYPEC_VSTATE_CC1
pub type TYPEC_VSTATE_CC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
///Field `TYPEC_VSTATE_CC2` reader - TYPEC_VSTATE_CC2
pub type TYPEC_VSTATE_CC2_R = crate::FieldReader<u8, u8>;
///Field `TYPEC_VSTATE_CC2` writer - TYPEC_VSTATE_CC2
pub type TYPEC_VSTATE_CC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SR_SPEC, u8, u8, 2, O>;
///Field `FRSEVT` reader - FRSEVT
pub type FRSEVT_R = crate::BitReader<bool>;
///Field `FRSEVT` writer - FRSEVT
pub type FRSEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
impl R {
    ///Bit 0 - TXIS
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    pub fn txmsgdisc(&self) -> TXMSGDISC_R {
        TXMSGDISC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    pub fn txmsgsent(&self) -> TXMSGSENT_R {
        TXMSGSENT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    pub fn txmsgabt(&self) -> TXMSGABT_R {
        TXMSGABT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    pub fn hrstdisc(&self) -> HRSTDISC_R {
        HRSTDISC_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    pub fn hrstsent(&self) -> HRSTSENT_R {
        HRSTSENT_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    pub fn txund(&self) -> TXUND_R {
        TXUND_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    pub fn rxorddet(&self) -> RXORDDET_R {
        RXORDDET_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    pub fn rxhrstdet(&self) -> RXHRSTDET_R {
        RXHRSTDET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    pub fn rxovr(&self) -> RXOVR_R {
        RXOVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    pub fn rxmsgend(&self) -> RXMSGEND_R {
        RXMSGEND_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    pub fn typecevt1(&self) -> TYPECEVT1_R {
        TYPECEVT1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    pub fn typecevt2(&self) -> TYPECEVT2_R {
        TYPECEVT2_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    pub fn typec_vstate_cc1(&self) -> TYPEC_VSTATE_CC1_R {
        TYPEC_VSTATE_CC1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    pub fn typec_vstate_cc2(&self) -> TYPEC_VSTATE_CC2_R {
        TYPEC_VSTATE_CC2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    pub fn frsevt(&self) -> FRSEVT_R {
        FRSEVT_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - TXIS
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<0> {
        TXIS_W::new(self)
    }
    ///Bit 1 - TXMSGDISC
    #[inline(always)]
    #[must_use]
    pub fn txmsgdisc(&mut self) -> TXMSGDISC_W<1> {
        TXMSGDISC_W::new(self)
    }
    ///Bit 2 - TXMSGSENT
    #[inline(always)]
    #[must_use]
    pub fn txmsgsent(&mut self) -> TXMSGSENT_W<2> {
        TXMSGSENT_W::new(self)
    }
    ///Bit 3 - TXMSGABT
    #[inline(always)]
    #[must_use]
    pub fn txmsgabt(&mut self) -> TXMSGABT_W<3> {
        TXMSGABT_W::new(self)
    }
    ///Bit 4 - HRSTDISC
    #[inline(always)]
    #[must_use]
    pub fn hrstdisc(&mut self) -> HRSTDISC_W<4> {
        HRSTDISC_W::new(self)
    }
    ///Bit 5 - HRSTSENT
    #[inline(always)]
    #[must_use]
    pub fn hrstsent(&mut self) -> HRSTSENT_W<5> {
        HRSTSENT_W::new(self)
    }
    ///Bit 6 - TXUND
    #[inline(always)]
    #[must_use]
    pub fn txund(&mut self) -> TXUND_W<6> {
        TXUND_W::new(self)
    }
    ///Bit 8 - RXNE
    #[inline(always)]
    #[must_use]
    pub fn rxne(&mut self) -> RXNE_W<8> {
        RXNE_W::new(self)
    }
    ///Bit 9 - RXORDDET
    #[inline(always)]
    #[must_use]
    pub fn rxorddet(&mut self) -> RXORDDET_W<9> {
        RXORDDET_W::new(self)
    }
    ///Bit 10 - RXHRSTDET
    #[inline(always)]
    #[must_use]
    pub fn rxhrstdet(&mut self) -> RXHRSTDET_W<10> {
        RXHRSTDET_W::new(self)
    }
    ///Bit 11 - RXOVR
    #[inline(always)]
    #[must_use]
    pub fn rxovr(&mut self) -> RXOVR_W<11> {
        RXOVR_W::new(self)
    }
    ///Bit 12 - RXMSGEND
    #[inline(always)]
    #[must_use]
    pub fn rxmsgend(&mut self) -> RXMSGEND_W<12> {
        RXMSGEND_W::new(self)
    }
    ///Bit 13 - RXERR
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<13> {
        RXERR_W::new(self)
    }
    ///Bit 14 - TYPECEVT1
    #[inline(always)]
    #[must_use]
    pub fn typecevt1(&mut self) -> TYPECEVT1_W<14> {
        TYPECEVT1_W::new(self)
    }
    ///Bit 15 - TYPECEVT2
    #[inline(always)]
    #[must_use]
    pub fn typecevt2(&mut self) -> TYPECEVT2_W<15> {
        TYPECEVT2_W::new(self)
    }
    ///Bits 16:17 - TYPEC_VSTATE_CC1
    #[inline(always)]
    #[must_use]
    pub fn typec_vstate_cc1(&mut self) -> TYPEC_VSTATE_CC1_W<16> {
        TYPEC_VSTATE_CC1_W::new(self)
    }
    ///Bits 18:19 - TYPEC_VSTATE_CC2
    #[inline(always)]
    #[must_use]
    pub fn typec_vstate_cc2(&mut self) -> TYPEC_VSTATE_CC2_W<18> {
        TYPEC_VSTATE_CC2_W::new(self)
    }
    ///Bit 20 - FRSEVT
    #[inline(always)]
    #[must_use]
    pub fn frsevt(&mut self) -> FRSEVT_W<20> {
        FRSEVT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///UCPD Status Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
