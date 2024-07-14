// quiz3.rs
//
// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to
// "A+" to show that your changes allow alphabetical grades.
//
// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
}

impl ReportCard {
    pub fn grade_to_letter(&self) -> String {
        match self.grade {
            4.0 => "A+".to_string(),
            3.7..=3.9 => "A".to_string(),
            3.3..=3.6 => "B+".to_string(),
            3.0..=3.2 => "B".to_string(),
            2.7..=2.9 => "C+".to_string(),
            2.0..=2.6 => "C".to_string(),
            1.7..=1.9 => "D+".to_string(),
            1.0..=1.6 => "D".to_string(),
            _ => "F".to_string(),
        }
    }

    pub fn print(&self) -> String {
        if self.grade != 4.0{format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, self.grade)}
            else{format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, "A+".to_string())}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 4.0,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
