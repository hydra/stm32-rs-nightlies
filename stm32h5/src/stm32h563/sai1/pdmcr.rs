///Register `PDMCR` reader
pub struct R(crate::R<PDMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDMCR` writer
pub struct W(crate::W<PDMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMCR_SPEC>;
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
impl From<crate::W<PDMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PDMEN` reader - PDM enable This bit is set and cleared by software. This bit allows to control the state of the PDM interface block. Make sure that the SAI in already operating in TDM master mode before enabling the PDM interface.
pub type PDMEN_R = crate::BitReader<bool>;
///Field `PDMEN` writer - PDM enable This bit is set and cleared by software. This bit allows to control the state of the PDM interface block. Make sure that the SAI in already operating in TDM master mode before enabling the PDM interface.
pub type PDMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDMCR_SPEC, bool, O>;
///Field `MICNBR` reader - Number of microphones This bit is set and cleared by software. Note: It is not recommended to configure this field when PDMEN = 1.* The complete set of data lines might not be available for all SAI instances. Refer to for details.
pub type MICNBR_R = crate::FieldReader<u8, u8>;
///Field `MICNBR` writer - Number of microphones This bit is set and cleared by software. Note: It is not recommended to configure this field when PDMEN = 1.* The complete set of data lines might not be available for all SAI instances. Refer to for details.
pub type MICNBR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDMCR_SPEC, u8, u8, 2, O>;
///Field `CKEN1` reader - Clock enable of bitstream clock number 1 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK1 might not be available for all SAI instances. Refer to implementation for details.
pub type CKEN1_R = crate::BitReader<bool>;
///Field `CKEN1` writer - Clock enable of bitstream clock number 1 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK1 might not be available for all SAI instances. Refer to implementation for details.
pub type CKEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDMCR_SPEC, bool, O>;
///Field `CKEN2` reader - Clock enable of bitstream clock number 2 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK2 might not be available for all SAI instances. Refer to implementation for details.
pub type CKEN2_R = crate::BitReader<bool>;
///Field `CKEN2` writer - Clock enable of bitstream clock number 2 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK2 might not be available for all SAI instances. Refer to implementation for details.
pub type CKEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDMCR_SPEC, bool, O>;
impl R {
    ///Bit 0 - PDM enable This bit is set and cleared by software. This bit allows to control the state of the PDM interface block. Make sure that the SAI in already operating in TDM master mode before enabling the PDM interface.
    #[inline(always)]
    pub fn pdmen(&self) -> PDMEN_R {
        PDMEN_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:5 - Number of microphones This bit is set and cleared by software. Note: It is not recommended to configure this field when PDMEN = 1.* The complete set of data lines might not be available for all SAI instances. Refer to for details.
    #[inline(always)]
    pub fn micnbr(&self) -> MICNBR_R {
        MICNBR_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 8 - Clock enable of bitstream clock number 1 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK1 might not be available for all SAI instances. Refer to implementation for details.
    #[inline(always)]
    pub fn cken1(&self) -> CKEN1_R {
        CKEN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock enable of bitstream clock number 2 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK2 might not be available for all SAI instances. Refer to implementation for details.
    #[inline(always)]
    pub fn cken2(&self) -> CKEN2_R {
        CKEN2_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - PDM enable This bit is set and cleared by software. This bit allows to control the state of the PDM interface block. Make sure that the SAI in already operating in TDM master mode before enabling the PDM interface.
    #[inline(always)]
    #[must_use]
    pub fn pdmen(&mut self) -> PDMEN_W<0> {
        PDMEN_W::new(self)
    }
    ///Bits 4:5 - Number of microphones This bit is set and cleared by software. Note: It is not recommended to configure this field when PDMEN = 1.* The complete set of data lines might not be available for all SAI instances. Refer to for details.
    #[inline(always)]
    #[must_use]
    pub fn micnbr(&mut self) -> MICNBR_W<4> {
        MICNBR_W::new(self)
    }
    ///Bit 8 - Clock enable of bitstream clock number 1 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK1 might not be available for all SAI instances. Refer to implementation for details.
    #[inline(always)]
    #[must_use]
    pub fn cken1(&mut self) -> CKEN1_W<8> {
        CKEN1_W::new(self)
    }
    ///Bit 9 - Clock enable of bitstream clock number 2 This bit is set and cleared by software. Note: It is not recommended to configure this bit when PDMEN = 1. SAI_CK2 might not be available for all SAI instances. Refer to implementation for details.
    #[inline(always)]
    #[must_use]
    pub fn cken2(&mut self) -> CKEN2_W<9> {
        CKEN2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SAI PDM control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdmcr](index.html) module
pub struct PDMCR_SPEC;
impl crate::RegisterSpec for PDMCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdmcr::R](R) reader structure
impl crate::Readable for PDMCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdmcr::W](W) writer structure
impl crate::Writable for PDMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDMCR to value 0
impl crate::Resettable for PDMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
