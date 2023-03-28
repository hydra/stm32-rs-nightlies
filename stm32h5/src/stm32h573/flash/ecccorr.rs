///Register `ECCCORR` reader
pub struct R(crate::R<ECCCORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCCORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCCORR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ECCCORR` writer
pub struct W(crate::W<ECCCORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCCORR_SPEC>;
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
impl From<crate::W<ECCCORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCCORR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDR_ECC` reader - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area).
pub type ADDR_ECC_R = crate::FieldReader<u16, u16>;
///Field `OBK_ECC` reader - Single ECC error corrected in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error.
pub type OBK_ECC_R = crate::BitReader<bool>;
///Field `EDATA_ECC` reader - ECC fail for corrected ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error.
pub type EDATA_ECC_R = crate::BitReader<bool>;
///Field `BK_ECC` reader - ECC fail bank for corrected ECC error It indicates which bank is concerned by ECC error
pub type BK_ECC_R = crate::BitReader<bool>;
///Field `SYSF_ECC` reader - ECC fail for corrected ECC error in system flash memory It indicates if system flash memory is concerned by ECC error.
pub type SYSF_ECC_R = crate::BitReader<bool>;
///Field `OTP_ECC` reader - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield.
pub type OTP_ECC_R = crate::BitReader<bool>;
///Field `ECCCIE` reader - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.
pub type ECCCIE_R = crate::BitReader<bool>;
///Field `ECCCIE` writer - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.
pub type ECCCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCCORR_SPEC, bool, O>;
///Field `ECCC` reader - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1.
pub type ECCC_R = crate::BitReader<bool>;
///Field `ECCC` writer - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1.
pub type ECCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECCCORR_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The flash interface programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the flash memory area where the error occurred (user flash memory, system flash memory, data area, read-only/OTP area).
    #[inline(always)]
    pub fn addr_ecc(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 20 - Single ECC error corrected in flash OB Keys storage area. It indicates the OBK storage concerned by ECC error.
    #[inline(always)]
    pub fn obk_ecc(&self) -> OBK_ECC_R {
        OBK_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - ECC fail for corrected ECC error in flash high-cycle data area It indicates if flash high-cycle data area is concerned by ECC error.
    #[inline(always)]
    pub fn edata_ecc(&self) -> EDATA_ECC_R {
        EDATA_ECC_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - ECC fail bank for corrected ECC error It indicates which bank is concerned by ECC error
    #[inline(always)]
    pub fn bk_ecc(&self) -> BK_ECC_R {
        BK_ECC_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - ECC fail for corrected ECC error in system flash memory It indicates if system flash memory is concerned by ECC error.
    #[inline(always)]
    pub fn sysf_ecc(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield.
    #[inline(always)]
    pub fn otp_ecc(&self) -> OTP_ECC_R {
        OTP_ECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.
    #[inline(always)]
    pub fn ecccie(&self) -> ECCCIE_R {
        ECCCIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1.
    #[inline(always)]
    pub fn eccc(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    ///Bit 25 - ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation.
    #[inline(always)]
    #[must_use]
    pub fn ecccie(&mut self) -> ECCCIE_W<25> {
        ECCCIE_W::new(self)
    }
    ///Bit 30 - ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1.
    #[inline(always)]
    #[must_use]
    pub fn eccc(&mut self) -> ECCC_W<30> {
        ECCC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///FLASH ECC correction register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ecccorr](index.html) module
pub struct ECCCORR_SPEC;
impl crate::RegisterSpec for ECCCORR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ecccorr::R](R) reader structure
impl crate::Readable for ECCCORR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ecccorr::W](W) writer structure
impl crate::Writable for ECCCORR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ECCCORR to value 0
impl crate::Resettable for ECCCORR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
