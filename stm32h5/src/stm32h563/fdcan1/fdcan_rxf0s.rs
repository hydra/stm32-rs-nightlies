///Register `FDCAN_RXF0S` reader
pub struct R(crate::R<FDCAN_RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
///Field `F0FL` reader - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3.
pub type F0FL_R = crate::FieldReader<u8, u8>;
///Field `F0GI` reader - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2.
pub type F0GI_R = crate::FieldReader<u8, u8>;
///Field `F0PI` reader - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2.
pub type F0PI_R = crate::FieldReader<u8, u8>;
///Field `F0F` reader - Rx FIFO 0 full
pub type F0F_R = crate::BitReader<bool>;
///Field `RF0L` reader - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\[RF0L\]. When IR\[RF0L\]
///is reset, this bit is also reset.
pub type RF0L_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3.
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:9 - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 16:17 - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2.
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Rx FIFO 0 full
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\[RF0L\]. When IR\[RF0L\]
    ///is reset, this bit is also reset.
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
///FDCAN Rx FIFO 0 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fdcan_rxf0s](index.html) module
pub struct FDCAN_RXF0S_SPEC;
impl crate::RegisterSpec for FDCAN_RXF0S_SPEC {
    type Ux = u32;
}
///`read()` method returns [fdcan_rxf0s::R](R) reader structure
impl crate::Readable for FDCAN_RXF0S_SPEC {
    type Reader = R;
}
///`reset()` method sets FDCAN_RXF0S to value 0
impl crate::Resettable for FDCAN_RXF0S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
