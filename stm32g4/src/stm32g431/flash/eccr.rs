///Register `ECCR` reader
pub struct R(crate::R<ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ECCR` writer
pub struct W(crate::W<ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCR_SPEC>;
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
impl From<crate::W<ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32, u32>;
///Field `BK_ECC` reader - BK_ECC
pub type BK_ECC_R = crate::BitReader<bool>;
///Field `SYSF_ECC` reader - SYSF_ECC
pub type SYSF_ECC_R = crate::BitReader<bool>;
///Field `ECCIE` reader - ECCIE
pub type ECCIE_R = crate::BitReader<bool>;
///Field `ECCIE` writer - ECCIE
pub type ECCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
///Field `ECCC2` reader - ECC correction
pub type ECCC2_R = crate::BitReader<bool>;
///Field `ECCC2` writer - ECC correction
pub type ECCC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
///Field `ECCD2` reader - ECC2 detection
pub type ECCD2_R = crate::BitReader<bool>;
///Field `ECCD2` writer - ECC2 detection
pub type ECCD2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
///Field `ECCC` reader - ECC correction
pub type ECCC_R = crate::BitReader<bool>;
///Field `ECCC` writer - ECC correction
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
///Field `ECCD` reader - ECC detection
pub type ECCD_R = crate::BitReader<bool>;
///Field `ECCD` writer - ECC detection
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCR_SPEC, bool, O>;
impl R {
    ///Bits 0:18 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x0007_ffff)
    }
    ///Bit 21 - BK_ECC
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SYSF_ECC
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - ECCIE
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    pub fn eccc2(&self) -> ECCC2_R {
        ECCC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    pub fn eccd2(&self) -> ECCD2_R {
        ECCD2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - ECCIE
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<24> {
        ECCIE_W::new(self)
    }
    ///Bit 28 - ECC correction
    #[inline(always)]
    #[must_use]
    pub fn eccc2(&mut self) -> ECCC2_W<28> {
        ECCC2_W::new(self)
    }
    ///Bit 29 - ECC2 detection
    #[inline(always)]
    #[must_use]
    pub fn eccd2(&mut self) -> ECCD2_W<29> {
        ECCD2_W::new(self)
    }
    ///Bit 30 - ECC correction
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    ///Bit 31 - ECC detection
    #[inline(always)]
    #[must_use]
    pub fn eccd(&mut self) -> ECCD_W<31> {
        ECCD_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash ECC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [eccr](index.html) module
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [eccr::R](R) reader structure
impl crate::Readable for ECCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [eccr::W](W) writer structure
impl crate::Writable for ECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ECCR to value 0
impl crate::Resettable for ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
