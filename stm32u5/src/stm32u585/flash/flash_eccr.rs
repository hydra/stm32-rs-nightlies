///Register `FLASH_ECCR` reader
pub struct R(crate::R<FLASH_ECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_ECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_ECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_ECCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FLASH_ECCR` writer
pub struct W(crate::W<FLASH_ECCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_ECCR_SPEC>;
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
impl From<crate::W<FLASH_ECCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_ECCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDR_ECC` reader - ECC fail address
pub type ADDR_ECC_R = crate::FieldReader<u32, u32>;
///Field `BK_ECC` reader - ECC fail bank
pub type BK_ECC_R = crate::BitReader<bool>;
///Field `SYSF_ECC` reader - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory.
pub type SYSF_ECC_R = crate::BitReader<bool>;
///Field `ECCIE` reader - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
pub type ECCIE_R = crate::BitReader<bool>;
///Field `ECCIE` writer - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
pub type ECCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ECCR_SPEC, bool, O>;
///Field `ECCC` reader - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
pub type ECCC_R = crate::BitReader<bool>;
///Field `ECCC` writer - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ECCR_SPEC, bool, O>;
///Field `ECCD` reader - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
pub type ECCD_R = crate::BitReader<bool>;
///Field `ECCD` writer - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
pub type ECCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_ECCR_SPEC, bool, O>;
impl R {
    ///Bits 0:19 - ECC fail address
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 21 - ECC fail bank
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory.
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
    #[inline(always)]
    pub fn eccie(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
    #[inline(always)]
    pub fn eccd(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 24 - ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set.
    #[inline(always)]
    #[must_use]
    pub fn eccie(&mut self) -> ECCIE_W<24> {
        ECCIE_W::new(self)
    }
    ///Bit 30 - ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1.
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    ///Bit 31 - ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1.
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
///FLASH ECC register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [flash_eccr](index.html) module
pub struct FLASH_ECCR_SPEC;
impl crate::RegisterSpec for FLASH_ECCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [flash_eccr::R](R) reader structure
impl crate::Readable for FLASH_ECCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [flash_eccr::W](W) writer structure
impl crate::Writable for FLASH_ECCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets FLASH_ECCR to value 0
impl crate::Resettable for FLASH_ECCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
