///Register `HWCFGR1` reader
pub struct R(crate::R<HWCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CFG1` reader - CFG1
pub type CFG1_R = crate::FieldReader<u8, u8>;
///Field `CFG2` reader - CFG2
pub type CFG2_R = crate::FieldReader<u8, u8>;
///Field `CFG3` reader - CFG3
pub type CFG3_R = crate::FieldReader<u8, u8>;
///Field `CFG4` reader - CFG4
pub type CFG4_R = crate::FieldReader<u8, u8>;
///Field `CFG5` reader - CFG5
pub type CFG5_R = crate::FieldReader<u8, u8>;
///Field `CFG6` reader - CFG6
pub type CFG6_R = crate::FieldReader<u8, u8>;
///Field `CFG7` reader - CFG7
pub type CFG7_R = crate::FieldReader<u8, u8>;
///Field `CFG8` reader - CFG8
pub type CFG8_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - CFG1
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - CFG2
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - CFG3
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - CFG4
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - CFG5
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - CFG6
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - CFG7
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - CFG8
    #[inline(always)]
    pub fn cfg8(&self) -> CFG8_R {
        CFG8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
///USART Hardware Configuration register 1
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr1](index.html) module
pub struct HWCFGR1_SPEC;
impl crate::RegisterSpec for HWCFGR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr1::R](R) reader structure
impl crate::Readable for HWCFGR1_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR1 to value 0x14
impl crate::Resettable for HWCFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
