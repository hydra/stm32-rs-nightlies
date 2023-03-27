///Register `HPTXSTS` reader
pub struct R(crate::R<HPTXSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTXSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTXSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTXSTS_SPEC>) -> Self {
        R(reader)
    }
}
///Field `PTXFSAVL` reader - PTXFSAVL
pub type PTXFSAVL_R = crate::FieldReader<u16, u16>;
///Field `PTXQSAV` reader - PTXQSAV
pub type PTXQSAV_R = crate::FieldReader<u8, u8>;
///Field `PTXQTOP` reader - PTXQTOP
pub type PTXQTOP_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:15 - PTXFSAVL
    #[inline(always)]
    pub fn ptxfsavl(&self) -> PTXFSAVL_R {
        PTXFSAVL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:23 - PTXQSAV
    #[inline(always)]
    pub fn ptxqsav(&self) -> PTXQSAV_R {
        PTXQSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - PTXQTOP
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///This read-only register contains the free space information for the periodic Tx FIFO and the periodic transmit request queue.
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hptxsts](index.html) module
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
///`read()` method returns [hptxsts::R](R) reader structure
impl crate::Readable for HPTXSTS_SPEC {
    type Reader = R;
}
///`reset()` method sets HPTXSTS to value 0x0008_0100
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0100;
}
