///Register `DDRPHYC_DX0GCR` reader
pub struct R(crate::R<DDRPHYC_DX0GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX0GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX0GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX0GCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_DX0GCR` writer
pub struct W(crate::W<DDRPHYC_DX0GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX0GCR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX0GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX0GCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DXEN` reader - DXEN
pub type DXEN_R = crate::BitReader<bool>;
///Field `DXEN` writer - DXEN
pub type DXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DQSODT` reader - DQSODT
pub type DQSODT_R = crate::BitReader<bool>;
///Field `DQSODT` writer - DQSODT
pub type DQSODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DQODT` reader - DQODT
pub type DQODT_R = crate::BitReader<bool>;
///Field `DQODT` writer - DQODT
pub type DQODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DXIOM` reader - DXIOM
pub type DXIOM_R = crate::BitReader<bool>;
///Field `DXIOM` writer - DXIOM
pub type DXIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DXPDD` reader - DXPDD
pub type DXPDD_R = crate::BitReader<bool>;
///Field `DXPDD` writer - DXPDD
pub type DXPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DXPDR` reader - DXPDR
pub type DXPDR_R = crate::BitReader<bool>;
///Field `DXPDR` writer - DXPDR
pub type DXPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DQSRPD` reader - DQSRPD
pub type DQSRPD_R = crate::BitReader<bool>;
///Field `DQSRPD` writer - DQSRPD
pub type DQSRPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DSEN` reader - DSEN
pub type DSEN_R = crate::FieldReader<u8, u8>;
///Field `DSEN` writer - DSEN
pub type DSEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, u8, u8, 2, O>;
///Field `DQSRTT` reader - DQSRTT
pub type DQSRTT_R = crate::BitReader<bool>;
///Field `DQSRTT` writer - DQSRTT
pub type DQSRTT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `DQRTT` reader - DQRTT
pub type DQRTT_R = crate::BitReader<bool>;
///Field `DQRTT` writer - DQRTT
pub type DQRTT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `RTTOH` reader - RTTOH
pub type RTTOH_R = crate::FieldReader<u8, u8>;
///Field `RTTOH` writer - RTTOH
pub type RTTOH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, u8, u8, 2, O>;
///Field `RTTOAL` reader - RTTOAL
pub type RTTOAL_R = crate::BitReader<bool>;
///Field `RTTOAL` writer - RTTOAL
pub type RTTOAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, bool, O>;
///Field `R0RVSL` reader - R0RVSL
pub type R0RVSL_R = crate::FieldReader<u8, u8>;
///Field `R0RVSL` writer - R0RVSL
pub type R0RVSL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_DX0GCR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - DXEN
    #[inline(always)]
    pub fn dxen(&self) -> DXEN_R {
        DXEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DQSODT
    #[inline(always)]
    pub fn dqsodt(&self) -> DQSODT_R {
        DQSODT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DQODT
    #[inline(always)]
    pub fn dqodt(&self) -> DQODT_R {
        DQODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DXIOM
    #[inline(always)]
    pub fn dxiom(&self) -> DXIOM_R {
        DXIOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - DXPDD
    #[inline(always)]
    pub fn dxpdd(&self) -> DXPDD_R {
        DXPDD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - DXPDR
    #[inline(always)]
    pub fn dxpdr(&self) -> DXPDR_R {
        DXPDR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DQSRPD
    #[inline(always)]
    pub fn dqsrpd(&self) -> DQSRPD_R {
        DQSRPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:8 - DSEN
    #[inline(always)]
    pub fn dsen(&self) -> DSEN_R {
        DSEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - DQSRTT
    #[inline(always)]
    pub fn dqsrtt(&self) -> DQSRTT_R {
        DQSRTT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - DQRTT
    #[inline(always)]
    pub fn dqrtt(&self) -> DQRTT_R {
        DQRTT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - RTTOH
    #[inline(always)]
    pub fn rttoh(&self) -> RTTOH_R {
        RTTOH_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - RTTOAL
    #[inline(always)]
    pub fn rttoal(&self) -> RTTOAL_R {
        RTTOAL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:16 - R0RVSL
    #[inline(always)]
    pub fn r0rvsl(&self) -> R0RVSL_R {
        R0RVSL_R::new(((self.bits >> 14) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - DXEN
    #[inline(always)]
    #[must_use]
    pub fn dxen(&mut self) -> DXEN_W<0> {
        DXEN_W::new(self)
    }
    ///Bit 1 - DQSODT
    #[inline(always)]
    #[must_use]
    pub fn dqsodt(&mut self) -> DQSODT_W<1> {
        DQSODT_W::new(self)
    }
    ///Bit 2 - DQODT
    #[inline(always)]
    #[must_use]
    pub fn dqodt(&mut self) -> DQODT_W<2> {
        DQODT_W::new(self)
    }
    ///Bit 3 - DXIOM
    #[inline(always)]
    #[must_use]
    pub fn dxiom(&mut self) -> DXIOM_W<3> {
        DXIOM_W::new(self)
    }
    ///Bit 4 - DXPDD
    #[inline(always)]
    #[must_use]
    pub fn dxpdd(&mut self) -> DXPDD_W<4> {
        DXPDD_W::new(self)
    }
    ///Bit 5 - DXPDR
    #[inline(always)]
    #[must_use]
    pub fn dxpdr(&mut self) -> DXPDR_W<5> {
        DXPDR_W::new(self)
    }
    ///Bit 6 - DQSRPD
    #[inline(always)]
    #[must_use]
    pub fn dqsrpd(&mut self) -> DQSRPD_W<6> {
        DQSRPD_W::new(self)
    }
    ///Bits 7:8 - DSEN
    #[inline(always)]
    #[must_use]
    pub fn dsen(&mut self) -> DSEN_W<7> {
        DSEN_W::new(self)
    }
    ///Bit 9 - DQSRTT
    #[inline(always)]
    #[must_use]
    pub fn dqsrtt(&mut self) -> DQSRTT_W<9> {
        DQSRTT_W::new(self)
    }
    ///Bit 10 - DQRTT
    #[inline(always)]
    #[must_use]
    pub fn dqrtt(&mut self) -> DQRTT_W<10> {
        DQRTT_W::new(self)
    }
    ///Bits 11:12 - RTTOH
    #[inline(always)]
    #[must_use]
    pub fn rttoh(&mut self) -> RTTOH_W<11> {
        RTTOH_W::new(self)
    }
    ///Bit 13 - RTTOAL
    #[inline(always)]
    #[must_use]
    pub fn rttoal(&mut self) -> RTTOAL_W<13> {
        RTTOAL_W::new(self)
    }
    ///Bits 14:16 - R0RVSL
    #[inline(always)]
    #[must_use]
    pub fn r0rvsl(&mut self) -> R0RVSL_W<14> {
        R0RVSL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC byte lane 0 GC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_dx0gcr](index.html) module
pub struct DDRPHYC_DX0GCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX0GCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_dx0gcr::R](R) reader structure
impl crate::Readable for DDRPHYC_DX0GCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_dx0gcr::W](W) writer structure
impl crate::Writable for DDRPHYC_DX0GCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_DX0GCR to value 0xee81
impl crate::Resettable for DDRPHYC_DX0GCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xee81;
}
