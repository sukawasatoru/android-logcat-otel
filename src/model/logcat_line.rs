/*
 * Copyright 2024 sukawasatoru
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::prelude::*;
use regex::Regex;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Debug, Eq, PartialEq)]
pub struct LogcatLine {
    pub timestamp: u128,
    pub uid: String,
    pub pid: u32,
    pub tid: u32,
    pub level: String,
    pub tag: String,
    pub msg: String,
}

impl FromStr for LogcatLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static REG: LazyLock<Regex> = LazyLock::new(create_reg);

        let cap = REG.captures(s).with_context(|| format!("not match: {s}"))?;

        Ok(LogcatLine {
            timestamp: cap[1].parse::<u128>().context("timestamp_sec")? * 1000
                + cap[2].parse::<u128>().context("timestamp_millis")?,
            uid: cap[3].to_owned(),
            pid: cap[4].parse().context("pid")?,
            tid: cap[5].parse().context("tid")?,
            level: cap[6].to_owned(),
            tag: cap[7].to_owned(),
            msg: cap[8].to_owned(),
        })
    }
}

fn create_reg() -> Regex {
    r" +(\d+)\.(\d{3}) +(\w+) +(\d+) +(\d+) ([EWIDV]) (\S+) *: (.*)"
        .parse()
        .expect("create_reg")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logcat_line_parse() {
        let source = r#"
                  0.004  logd   151   151 W auditd  : type=2000 audit(0.0:1): state=initialized audit_enabled=0 res=1
         1722132942.768  logd   151   151 W auditd  : type=1403 audit(0.0:2): auid=4294967295 ses=4294967295 lsm=selinux res=1
         1722132942.768  logd   151   151 W auditd  : type=1404 audit(0.0:3): enforcing=1 old_enforcing=0 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=selinux res=1
         1722132943.523  1000   158   158 W linker  : Warning: failed to find generated linker configuration from "/linkerconfig/ld.config.txt"
         1722132943.527  1000   159   159 W linker  : Warning: failed to find generated linker configuration from "/linkerconfig/ld.config.txt"
"#;

        let mut lines = source.lines();

        lines.next().unwrap();

        let expected = LogcatLine {
            timestamp: 4,
            uid: "logd".into(),
            pid: 151,
            tid: 151,
            level: "W".into(),
            tag: "auditd".into(),
            msg: "type=2000 audit(0.0:1): state=initialized audit_enabled=0 res=1".into(),
        };
        assert_eq!(
            lines.next().unwrap().parse::<LogcatLine>().unwrap(),
            expected,
        );

        let expected = LogcatLine {
            timestamp: 1722132942768,
            uid: "logd".into(),
            pid: 151,
            tid: 151,
            level: "W".into(),
            tag: "auditd".into(),
            msg: "type=1403 audit(0.0:2): auid=4294967295 ses=4294967295 lsm=selinux res=1".into(),
        };
        assert_eq!(
            lines.next().unwrap().parse::<LogcatLine>().unwrap(),
            expected,
        );

        let expected = LogcatLine {
            timestamp: 1722132942768,
            uid: "logd".into(),
            pid: 151,
            tid: 151,
            level: "W".into(),
            tag: "auditd".into(),
            msg: "type=1404 audit(0.0:3): enforcing=1 old_enforcing=0 auid=4294967295 ses=4294967295 enabled=1 old-enabled=1 lsm=selinux res=1".into(),
        };
        assert_eq!(
            lines.next().unwrap().parse::<LogcatLine>().unwrap(),
            expected,
        );

        let expected = LogcatLine {
            timestamp: 1722132943523,
            uid: "1000".into(),
            pid: 158,
            tid: 158,
            level: "W".into(),
            tag: "linker".into(),
            msg: r#"Warning: failed to find generated linker configuration from "/linkerconfig/ld.config.txt""#.into(),
        };
        assert_eq!(
            lines.next().unwrap().parse::<LogcatLine>().unwrap(),
            expected,
        );

        let expected = LogcatLine {
            timestamp: 1722132943527,
            uid: "1000".into(),
            pid: 159,
            tid: 159,
            level: "W".into(),
            tag: "linker".into(),
            msg: r#"Warning: failed to find generated linker configuration from "/linkerconfig/ld.config.txt""#.into(),
        };
        assert_eq!(
            lines.next().unwrap().parse::<LogcatLine>().unwrap(),
            expected,
        );
    }
}
