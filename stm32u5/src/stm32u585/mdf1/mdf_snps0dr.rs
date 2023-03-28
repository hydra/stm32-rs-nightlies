///Register `MDF_SNPS0DR` reader
pub struct R(crate::R<MDF_SNPS0DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDF_SNPS0DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDF_SNPS0DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDF_SNPS0DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MCICDC` reader - Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)
pub type MCICDC_R = crate::FieldReader<u16, u16>;
///Field `EXTSDR` reader - Extended data size If SNPSFMT = 0 , EXTSDR\[6:0\]
///contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT).
pub type EXTSDR_R = crate::FieldReader<u8, u8>;
///Field `SDR` reader - Contains the 16 MSB of the last valid data processed by the digital filter.
pub type SDR_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:8 - Contains the MCIC decimation counter value at the moment of the last trigger event occurs (MCIC_CNT)
    #[inline(always)]
    pub fn mcicdc(&self) -> MCICDC_R {
        MCICDC_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bits 9:15 - Extended data size If SNPSFMT = 0 , EXTSDR\[6:0\]
    ///contains the bit 7 to 1 of the last valid data processed by the digital filter, If SNPSFMT = 1 , this field contains the INT accumulator counter value at the moment of the last trigger event occurs (INT_CNT).
    #[inline(always)]
    pub fn extsdr(&self) -> EXTSDR_R {
        EXTSDR_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    ///Bits 16:31 - Contains the 16 MSB of the last valid data processed by the digital filter.
    #[inline(always)]
    pub fn sdr(&self) -> SDR_R {
        SDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///This register is used to read the data processed by each digital filter in snapshot mode.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mdf_snps0dr](index.html) module
pub struct MDF_SNPS0DR_SPEC;
impl crate::RegisterSpec for MDF_SNPS0DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mdf_snps0dr::R](R) reader structure
impl crate::Readable for MDF_SNPS0DR_SPEC {
    type Reader = R;
}
///`reset()` method sets MDF_SNPS0DR to value 0
impl crate::Resettable for MDF_SNPS0DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
