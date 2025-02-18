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
///Field `C1VAL` reader - COMP channel 1 output status bit
pub type C1VAL_R = crate::BitReader<bool>;
///Field `C2VAL` reader - COMP channel 2 output status bit
pub type C2VAL_R = crate::BitReader<bool>;
///Field `C1IF` reader - COMP channel 1 Interrupt Flag
pub type C1IF_R = crate::BitReader<bool>;
///Field `C2IF` reader - COMP channel 2 Interrupt Flag
pub type C2IF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - COMP channel 1 output status bit
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COMP channel 2 output status bit
    #[inline(always)]
    pub fn c2val(&self) -> C2VAL_R {
        C2VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 16 - COMP channel 1 Interrupt Flag
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - COMP channel 2 Interrupt Flag
    #[inline(always)]
    pub fn c2if(&self) -> C2IF_R {
        C2IF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
///Comparator status register
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
