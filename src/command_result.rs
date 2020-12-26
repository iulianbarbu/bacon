use {
    crate::*,
    anyhow::*,
    std::process::ExitStatus,
};

/// what we get from the execution of a command
pub enum CommandResult {
    /// a trustable report with errors and warnings computed
    Report(Report),
    /// we don't have a proper report
    Failure(Failure),
    /// not yet computed
    None,
}

impl CommandResult {
    pub fn new(lines: Vec<CommandOutputLine>, exit_status: Option<ExitStatus>) -> Result<Self> {
        let error_code = exit_status.and_then(|s| s.code()).filter(|&c| c != 0);
        let report = Report::from_lines(&lines)?;
        if let Some(error_code) = error_code {
            if report.stats.errors == 0 {
                // report shows no error while the command exe reported
                // an error, so the report can't be trusted
                return Ok(Self::Failure(Failure { error_code, lines }));
            }
        }
        // report looks valid
        Ok(Self::Report(report))
    }
    pub fn reverse(&mut self) {
        match self {
            Self::Report(report) => {
                report.reverse();
            }
            Self::Failure(failure) => {
                failure.lines.reverse();
            }
            Self::None => {}
        }
    }
    pub fn lines_len(&self) -> usize {
        match self {
            Self::Report(report) => report.lines.len(),
            Self::Failure(failure) => failure.lines.len(),
            Self::None => 0,
        }
    }
}
