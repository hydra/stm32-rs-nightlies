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
///Field `CCF` reader - Computation completed flag This bit mirrors the CCF bit of the SAES_ISR register.
pub type CCF_R = crate::BitReader<bool>;
///Field `RDERR` reader - Read error flag This flag indicates the detection of an unexpected read operation from the SAES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected read returns zero.
pub type RDERR_R = crate::BitReader<bool>;
///Field `WRERR` reader - Write error This flag indicates the detection of an unexpected write operation to the SAES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected write is ignored.
pub type WRERR_R = crate::BitReader<bool>;
///Field `BUSY` reader - Busy This flag indicates whether SAES is idle or busy during GCM payload encryption phase: The flag is set upon SAES initialization, upon fetching random number from the RNG, or upon transferring a shared key to a target peripheral. When GCM encryption is selected, the flag must be at zero before selecting the GCM final phase.
pub type BUSY_R = crate::BitReader<bool>;
///Field `KEYVALID` reader - Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in SAES_CR has been loaded in SAES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the SAES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by SAES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the SAES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the SAES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to .
pub type KEYVALID_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Computation completed flag This bit mirrors the CCF bit of the SAES_ISR register.
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Read error flag This flag indicates the detection of an unexpected read operation from the SAES_DOUTR register (during computation or data input phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected read returns zero.
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Write error This flag indicates the detection of an unexpected write operation to the SAES_DINR register (during computation or data output phase): The flag is set by hardware. It is cleared by software upon setting the RWEIF bit of the SAES_ICR register. Upon the flag setting, an interrupt is generated if enabled through the RWEIE bit of the SAES_ICR register. The flag setting has no impact on the SAES operation. Unexpected write is ignored.
    #[inline(always)]
    pub fn wrerr(&self) -> WRERR_R {
        WRERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Busy This flag indicates whether SAES is idle or busy during GCM payload encryption phase: The flag is set upon SAES initialization, upon fetching random number from the RNG, or upon transferring a shared key to a target peripheral. When GCM encryption is selected, the flag must be at zero before selecting the GCM final phase.
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 7 - Key Valid flag This bit is set by hardware when the amount of key information defined by KEYSIZE in SAES_CR has been loaded in SAES_KEYx key registers. In normal mode when KEYSEL equals to zero, the application must write the key registers in the correct sequence, otherwise the KEIF flag of the SAES_ISR register is set and KEYVALID stays at zero. When KEYSEL is different from zero the BUSY flag is automatically set by SAES. When key is loaded successfully, the BUSY flag is cleared and KEYVALID set. Upon an error, the KEIF flag of the SAES_ISR register is set, the BUSY flag cleared and KEYVALID kept at zero. When the KEIF flag is set, the application must clear it through the SAES_ICR register, otherwise KEYVALID cannot be set. See the KEIF bit description for more details. For more information on key loading please refer to .
    #[inline(always)]
    pub fn keyvalid(&self) -> KEYVALID_R {
        KEYVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
///SAES status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
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
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
