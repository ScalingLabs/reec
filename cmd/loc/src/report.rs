use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
pub struct LinesOfCodeReporterOptions {
    #[arg(short, long, value_name = "SUMMARY", default_value = "false")]
    pub summary: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct LinesOfCodeReport {
    pub reec: usize,
    pub reec_l1: usize,
    pub reec_l2: usize,
    pub levm: usize,
}

pub fn slack_message(old_report: LinesOfCodeReport, new_report: LinesOfCodeReport) -> String {
    let reec_l1_diff = new_report.reec_l1.abs_diff(old_report.reec_l1);
    let reec_l2_diff = new_report.reec_l2.abs_diff(old_report.reec_l2);
    let levm_diff = new_report.levm.abs_diff(old_report.levm);
    let reec_diff_total = reec_l1_diff + reec_l2_diff + levm_diff;

    format!(
        r#"{{
    "blocks": [
        {{
            "type": "header",
            "text": {{
                "type": "plain_text",
                "text": "Daily Lines of Code Report"
            }}
        }},
        {{
            "type": "divider"
        }},
        {{
            "type": "section",
            "text": {{
                "type": "mrkdwn",
                "text": "*reec L1:* {} {}\n*reec L2:* {} {}\n*levm:* {} {}\n*reec (total):* {} {}"
            }}             
        }}
    ]
}}"#,
        new_report.reec_l1,
        match new_report.reec_l1.cmp(&old_report.reec_l1) {
            std::cmp::Ordering::Greater => format!("(+{reec_l1_diff})"),
            std::cmp::Ordering::Less => format!("(-{reec_l1_diff})"),
            std::cmp::Ordering::Equal => "".to_string(),
        },
        new_report.reec_l2,
        match new_report.reec_l2.cmp(&old_report.reec_l2) {
            std::cmp::Ordering::Greater => format!("(+{reec_l2_diff})"),
            std::cmp::Ordering::Less => format!("(-{reec_l2_diff})"),
            std::cmp::Ordering::Equal => "".to_string(),
        },
        new_report.levm,
        match new_report.levm.cmp(&old_report.levm) {
            std::cmp::Ordering::Greater => format!("(+{levm_diff})"),
            std::cmp::Ordering::Less => format!("(-{levm_diff})"),
            std::cmp::Ordering::Equal => "".to_string(),
        },
        new_report.reec,
        match new_report.reec.cmp(&old_report.reec) {
            std::cmp::Ordering::Greater => format!("(+{reec_diff_total})"),
            std::cmp::Ordering::Less => format!("(-{reec_diff_total})"),
            std::cmp::Ordering::Equal => "".to_string(),
        },
    )
}

pub fn github_step_summary(old_report: LinesOfCodeReport, new_report: LinesOfCodeReport) -> String {
    let reec_l1_diff = new_report.reec_l1.abs_diff(old_report.reec_l1);
    let reec_l2_diff = new_report.reec_l2.abs_diff(old_report.reec_l2);
    let levm_diff = new_report.levm.abs_diff(old_report.levm);
    let reec_diff_total = reec_l1_diff + reec_l2_diff + levm_diff;

    format!(
        r#"```
reec loc summary
====================
reec L1: {} {}
reec L2: {} {}
levm: {} ({})
reec (total): {} {}
```"#,
        new_report.reec_l1,
        if new_report.reec > old_report.reec {
            format!("(+{reec_l1_diff})")
        } else {
            format!("(-{reec_l1_diff})")
        },
        new_report.reec_l2,
        if new_report.reec_l2 > old_report.reec_l2 {
            format!("(+{reec_l2_diff})")
        } else {
            format!("(-{reec_l2_diff})")
        },
        new_report.levm,
        if new_report.levm > old_report.levm {
            format!("(+{levm_diff})")
        } else {
            format!("(-{levm_diff})")
        },
        new_report.reec,
        if new_report.reec > old_report.reec {
            format!("(+{reec_diff_total})")
        } else {
            format!("(-{reec_diff_total})")
        },
    )
}

pub fn shell_summary(new_report: LinesOfCodeReport) -> String {
    format!(
        "{}\n{}\n{} {}\n{} {}\n{} {}\n{} {}",
        "Lines of Code".bold(),
        "=============".bold(),
        "reec L1:".bold(),
        new_report.reec_l1,
        "reec L2:".bold(),
        new_report.reec_l2,
        "levm:".bold(),
        new_report.levm,
        "reec (total):".bold(),
        new_report.reec,
    )
}
