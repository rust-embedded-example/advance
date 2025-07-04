#[doc = "Register `BKP25R` reader"]
pub type R = crate::R<Bkp25rSpec>;
#[doc = "Register `BKP25R` writer"]
pub type W = crate::W<Bkp25rSpec>;
#[doc = "Field `BKP` reader - BKP"]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<'_, Bkp25rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "backup register\n\nYou can [`read`](crate::Reg::read) this register and get [`bkp25r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkp25r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bkp25rSpec;
impl crate::RegisterSpec for Bkp25rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bkp25r::R`](R) reader structure"]
impl crate::Readable for Bkp25rSpec {}
#[doc = "`write(|w| ..)` method takes [`bkp25r::W`](W) writer structure"]
impl crate::Writable for Bkp25rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BKP25R to value 0"]
impl crate::Resettable for Bkp25rSpec {}
