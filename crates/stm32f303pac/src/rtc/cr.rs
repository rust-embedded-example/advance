#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `WCKSEL` reader - Wakeup clock selection"]
pub type WckselR = crate::FieldReader;
#[doc = "Field `WCKSEL` writer - Wakeup clock selection"]
pub type WckselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge"]
pub type TsedgeR = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge"]
pub type TsedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)"]
pub type RefckonR = crate::BitReader;
#[doc = "Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)"]
pub type RefckonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers"]
pub type BypshadR = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers"]
pub type BypshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMT` reader - Hour format"]
pub type FmtR = crate::BitReader;
#[doc = "Field `FMT` writer - Hour format"]
pub type FmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type AlraeR = crate::BitReader;
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type AlraeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type AlrbeR = crate::BitReader;
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type AlrbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub type WuteR = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub type WuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type AlraieR = crate::BitReader;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type AlraieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type AlrbieR = crate::BitReader;
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type AlrbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WutieR = crate::BitReader;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` reader - Add 1 hour (summer time change)"]
pub type Add1hR = crate::BitReader;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` reader - Subtract 1 hour (winter time change)"]
pub type Sub1hR = crate::BitReader;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change)"]
pub type Sub1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup"]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Backup"]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection"]
pub type CoselR = crate::BitReader;
#[doc = "Field `COSEL` writer - Calibration output selection"]
pub type CoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Output polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Output polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection"]
pub type OselR = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection"]
pub type OselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - Calibration output enable"]
pub type CoeR = crate::BitReader;
#[doc = "Field `COE` writer - Calibration output enable"]
pub type CoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wcksel(&self) -> WckselR {
        WckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TsedgeR {
        TsedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> RefckonR {
        RefckonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BypshadR {
        BypshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> AlraeR {
        AlraeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> AlrbeR {
        AlrbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WuteR {
        WuteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> AlraieR {
        AlraieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> AlrbieR {
        AlrbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WutieR {
        WutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&self) -> Add1hR {
        Add1hR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&self) -> Sub1hR {
        Sub1hR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> CoselR {
        CoselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> CoeR {
        CoeR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wcksel(&mut self) -> WckselW<'_, CrSpec> {
        WckselW::new(self, 0)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TsedgeW<'_, CrSpec> {
        TsedgeW::new(self, 3)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&mut self) -> RefckonW<'_, CrSpec> {
        RefckonW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BypshadW<'_, CrSpec> {
        BypshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FmtW<'_, CrSpec> {
        FmtW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&mut self) -> AlraeW<'_, CrSpec> {
        AlraeW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> AlrbeW<'_, CrSpec> {
        AlrbeW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&mut self) -> WuteW<'_, CrSpec> {
        WuteW::new(self, 10)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<'_, CrSpec> {
        TseW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&mut self) -> AlraieW<'_, CrSpec> {
        AlraieW::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> AlrbieW<'_, CrSpec> {
        AlrbieW::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WutieW<'_, CrSpec> {
        WutieW::new(self, 14)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TsieW<'_, CrSpec> {
        TsieW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&mut self) -> Add1hW<'_, CrSpec> {
        Add1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&mut self) -> Sub1hW<'_, CrSpec> {
        Sub1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, CrSpec> {
        BkpW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&mut self) -> CoselW<'_, CrSpec> {
        CoselW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, CrSpec> {
        PolW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&mut self) -> OselW<'_, CrSpec> {
        OselW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&mut self) -> CoeW<'_, CrSpec> {
        CoeW::new(self, 23)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
