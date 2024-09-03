use super::traits;
use traits::{Speak, ClinicalSkills, AdvanceMedical, PatientRole};

/// 患者
pub struct Patient {
    pub(crate) name: String,
}

/// 护士
pub struct Nurse {
    pub(crate) name: String,
}

/// 执业护士
pub struct NursePractitioner {
    name: String,
}

/// 高级执业护士
pub struct AdvanceNursePractitioner {
    name: String,
}

/// 医生
pub struct Doctor {
    pub(crate) name: String,
}

/// 为所有的人员结构体实现Speak trait
impl Speak for Patient {
    fn introduce(&self) -> () {
        println!("I'm a Patient and my name is {}", self.name)
    }
}

impl Speak for Nurse {
    fn introduce(&self) -> () {
        println!("I'm a Nurse and my name is {}", self.name)
    }
}

impl Speak for NursePractitioner {
    fn introduce(&self) -> () {
        println!("I'm a NursePractitioner and my name is {}", self.name)
    }
}

impl Speak for AdvanceNursePractitioner {
    fn introduce(&self) -> () {
        println!("I'm a AdvanceNursePractitioner and my name is {}", self.name)
    }
}

impl Speak for Doctor {
    fn introduce(&self) -> () {
        println!("I'm a Doctor and my name is {}", self.name)
    }
}

/// 为患者实现患者角色trait
impl PatientRole for Patient {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

/// 实现临床技能trait
impl ClinicalSkills for Nurse {}  // ClinicalSkills的方法有默认实现，所以可以不重写

impl ClinicalSkills for NursePractitioner {
    // 执业护士能开处方
    fn can_prescribe(&self) -> bool {
        true
    }
}

/// 高级trait的用法：
/// 医生和高级执业护士 都可以开处方和诊断
impl AdvanceMedical for AdvanceNursePractitioner {}
impl AdvanceMedical for Doctor {}

impl<T> ClinicalSkills for T
where
    T: AdvanceMedical,
{
    fn can_prescribe(&self) -> bool {
        true
    }
    fn can_diagnose(&self) -> bool {
        true
    }
}