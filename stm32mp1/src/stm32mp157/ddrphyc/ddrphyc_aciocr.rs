///Register `DDRPHYC_ACIOCR` reader
pub struct R(crate::R<DDRPHYC_ACIOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_ACIOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_ACIOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_ACIOCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DDRPHYC_ACIOCR` writer
pub struct W(crate::W<DDRPHYC_ACIOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_ACIOCR_SPEC>;
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
impl From<crate::W<DDRPHYC_ACIOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_ACIOCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ACIOM` reader - ACIOM
pub type ACIOM_R = crate::BitReader<bool>;
///Field `ACIOM` writer - ACIOM
pub type ACIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `ACOE` reader - ACOE
pub type ACOE_R = crate::BitReader<bool>;
///Field `ACOE` writer - ACOE
pub type ACOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `ACODT` reader - ACODT
pub type ACODT_R = crate::BitReader<bool>;
///Field `ACODT` writer - ACODT
pub type ACODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `ACPDD` reader - ACPDD
pub type ACPDD_R = crate::BitReader<bool>;
///Field `ACPDD` writer - ACPDD
pub type ACPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `ACPDR` reader - ACPDR
pub type ACPDR_R = crate::BitReader<bool>;
///Field `ACPDR` writer - ACPDR
pub type ACPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `CKODT` reader - CKODT
pub type CKODT_R = crate::FieldReader<u8, u8>;
///Field `CKODT` writer - CKODT
pub type CKODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, u8, u8, 3, O>;
///Field `CKPDD` reader - CKPDD
pub type CKPDD_R = crate::FieldReader<u8, u8>;
///Field `CKPDD` writer - CKPDD
pub type CKPDD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, u8, u8, 3, O>;
///Field `CKPDR` reader - CKPDR
pub type CKPDR_R = crate::FieldReader<u8, u8>;
///Field `CKPDR` writer - CKPDR
pub type CKPDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, u8, u8, 3, O>;
///Field `RANKODT` reader - RANKODT
pub type RANKODT_R = crate::BitReader<bool>;
///Field `RANKODT` writer - RANKODT
pub type RANKODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `CSPDD` reader - CSPDD
pub type CSPDD_R = crate::BitReader<bool>;
///Field `CSPDD` writer - CSPDD
pub type CSPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `RANKPDR` reader - RANKPDR
pub type RANKPDR_R = crate::BitReader<bool>;
///Field `RANKPDR` writer - RANKPDR
pub type RANKPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `RSTODT` reader - RSTODT
pub type RSTODT_R = crate::BitReader<bool>;
///Field `RSTODT` writer - RSTODT
pub type RSTODT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `RSTPDD` reader - RSTPDD
pub type RSTPDD_R = crate::BitReader<bool>;
///Field `RSTPDD` writer - RSTPDD
pub type RSTPDD_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `RSTPDR` reader - RSTPDR
pub type RSTPDR_R = crate::BitReader<bool>;
///Field `RSTPDR` writer - RSTPDR
pub type RSTPDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `RSTIOM` reader - RSTIOM
pub type RSTIOM_R = crate::BitReader<bool>;
///Field `RSTIOM` writer - RSTIOM
pub type RSTIOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, bool, O>;
///Field `ACSR` reader - ACSR
pub type ACSR_R = crate::FieldReader<u8, u8>;
///Field `ACSR` writer - ACSR
pub type ACSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DDRPHYC_ACIOCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - ACIOM
    #[inline(always)]
    pub fn aciom(&self) -> ACIOM_R {
        ACIOM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACOE
    #[inline(always)]
    pub fn acoe(&self) -> ACOE_R {
        ACOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ACODT
    #[inline(always)]
    pub fn acodt(&self) -> ACODT_R {
        ACODT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ACPDD
    #[inline(always)]
    pub fn acpdd(&self) -> ACPDD_R {
        ACPDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ACPDR
    #[inline(always)]
    pub fn acpdr(&self) -> ACPDR_R {
        ACPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - CKODT
    #[inline(always)]
    pub fn ckodt(&self) -> CKODT_R {
        CKODT_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - CKPDD
    #[inline(always)]
    pub fn ckpdd(&self) -> CKPDD_R {
        CKPDD_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:13 - CKPDR
    #[inline(always)]
    pub fn ckpdr(&self) -> CKPDR_R {
        CKPDR_R::new(((self.bits >> 11) & 7) as u8)
    }
    ///Bit 14 - RANKODT
    #[inline(always)]
    pub fn rankodt(&self) -> RANKODT_R {
        RANKODT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 18 - CSPDD
    #[inline(always)]
    pub fn cspdd(&self) -> CSPDD_R {
        CSPDD_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - RANKPDR
    #[inline(always)]
    pub fn rankpdr(&self) -> RANKPDR_R {
        RANKPDR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 26 - RSTODT
    #[inline(always)]
    pub fn rstodt(&self) -> RSTODT_R {
        RSTODT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - RSTPDD
    #[inline(always)]
    pub fn rstpdd(&self) -> RSTPDD_R {
        RSTPDD_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - RSTPDR
    #[inline(always)]
    pub fn rstpdr(&self) -> RSTPDR_R {
        RSTPDR_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - RSTIOM
    #[inline(always)]
    pub fn rstiom(&self) -> RSTIOM_R {
        RSTIOM_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bits 30:31 - ACSR
    #[inline(always)]
    pub fn acsr(&self) -> ACSR_R {
        ACSR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - ACIOM
    #[inline(always)]
    #[must_use]
    pub fn aciom(&mut self) -> ACIOM_W<0> {
        ACIOM_W::new(self)
    }
    ///Bit 1 - ACOE
    #[inline(always)]
    #[must_use]
    pub fn acoe(&mut self) -> ACOE_W<1> {
        ACOE_W::new(self)
    }
    ///Bit 2 - ACODT
    #[inline(always)]
    #[must_use]
    pub fn acodt(&mut self) -> ACODT_W<2> {
        ACODT_W::new(self)
    }
    ///Bit 3 - ACPDD
    #[inline(always)]
    #[must_use]
    pub fn acpdd(&mut self) -> ACPDD_W<3> {
        ACPDD_W::new(self)
    }
    ///Bit 4 - ACPDR
    #[inline(always)]
    #[must_use]
    pub fn acpdr(&mut self) -> ACPDR_W<4> {
        ACPDR_W::new(self)
    }
    ///Bits 5:7 - CKODT
    #[inline(always)]
    #[must_use]
    pub fn ckodt(&mut self) -> CKODT_W<5> {
        CKODT_W::new(self)
    }
    ///Bits 8:10 - CKPDD
    #[inline(always)]
    #[must_use]
    pub fn ckpdd(&mut self) -> CKPDD_W<8> {
        CKPDD_W::new(self)
    }
    ///Bits 11:13 - CKPDR
    #[inline(always)]
    #[must_use]
    pub fn ckpdr(&mut self) -> CKPDR_W<11> {
        CKPDR_W::new(self)
    }
    ///Bit 14 - RANKODT
    #[inline(always)]
    #[must_use]
    pub fn rankodt(&mut self) -> RANKODT_W<14> {
        RANKODT_W::new(self)
    }
    ///Bit 18 - CSPDD
    #[inline(always)]
    #[must_use]
    pub fn cspdd(&mut self) -> CSPDD_W<18> {
        CSPDD_W::new(self)
    }
    ///Bit 22 - RANKPDR
    #[inline(always)]
    #[must_use]
    pub fn rankpdr(&mut self) -> RANKPDR_W<22> {
        RANKPDR_W::new(self)
    }
    ///Bit 26 - RSTODT
    #[inline(always)]
    #[must_use]
    pub fn rstodt(&mut self) -> RSTODT_W<26> {
        RSTODT_W::new(self)
    }
    ///Bit 27 - RSTPDD
    #[inline(always)]
    #[must_use]
    pub fn rstpdd(&mut self) -> RSTPDD_W<27> {
        RSTPDD_W::new(self)
    }
    ///Bit 28 - RSTPDR
    #[inline(always)]
    #[must_use]
    pub fn rstpdr(&mut self) -> RSTPDR_W<28> {
        RSTPDR_W::new(self)
    }
    ///Bit 29 - RSTIOM
    #[inline(always)]
    #[must_use]
    pub fn rstiom(&mut self) -> RSTIOM_W<29> {
        RSTIOM_W::new(self)
    }
    ///Bits 30:31 - ACSR
    #[inline(always)]
    #[must_use]
    pub fn acsr(&mut self) -> ACSR_W<30> {
        ACSR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DDRPHYC ACIOC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ddrphyc_aciocr](index.html) module
pub struct DDRPHYC_ACIOCR_SPEC;
impl crate::RegisterSpec for DDRPHYC_ACIOCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ddrphyc_aciocr::R](R) reader structure
impl crate::Readable for DDRPHYC_ACIOCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ddrphyc_aciocr::W](W) writer structure
impl crate::Writable for DDRPHYC_ACIOCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DDRPHYC_ACIOCR to value 0x33c0_3812
impl crate::Resettable for DDRPHYC_ACIOCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x33c0_3812;
}
