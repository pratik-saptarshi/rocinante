#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FindingStatus {
    Good,
    Medium,
    Bad,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Finding {
    pub id: &'static str,
    pub text: &'static str,
    pub status: FindingStatus,
}

pub const ACCESSIBILITY_SCORE: u8 = 85;
pub const ACCESSIBILITY_FINDINGS: [Finding; 3] = [
    Finding {
        id: "alt-text",
        text: "Missing image alt text (7 instances)",
        status: FindingStatus::Bad,
    },
    Finding {
        id: "contrast",
        text: "Low color contrast on buttons",
        status: FindingStatus::Medium,
    },
    Finding {
        id: "keyboard",
        text: "Keyboard navigation traps found",
        status: FindingStatus::Medium,
    },
];

pub const SEO_FINDINGS: [Finding; 3] = [
    Finding {
        id: "h1",
        text: "Improve H1 and title tag consistency",
        status: FindingStatus::Medium,
    },
    Finding {
        id: "faq",
        text: "Add FAQ schema",
        status: FindingStatus::Medium,
    },
    Finding {
        id: "qa",
        text: "Optimize for long-tail, question-based queries",
        status: FindingStatus::Medium,
    },
];

pub const SECURITY_FINDINGS: [Finding; 3] = [
    Finding {
        id: "csp",
        text: "Content-Security-Policy: Missing",
        status: FindingStatus::Bad,
    },
    Finding {
        id: "frame",
        text: "X-Frame-Options: SAMEORIGIN",
        status: FindingStatus::Good,
    },
    Finding {
        id: "hsts",
        text: "Strict-Transport-Security: Enabled",
        status: FindingStatus::Good,
    },
];

pub const PERFORMANCE_SCORE: u8 = 65;
pub const PERFORMANCE_FINDINGS: [Finding; 3] = [
    Finding {
        id: "lcp",
        text: "Largest Contentful Paint (LCP): 2.9s",
        status: FindingStatus::Medium,
    },
    Finding {
        id: "cls",
        text: "Cumulative Layout Shift (CLS): 0.05",
        status: FindingStatus::Good,
    },
    Finding {
        id: "tbt",
        text: "Total Blocking Time (TBT): 350ms",
        status: FindingStatus::Medium,
    },
];

pub const ON_PAGE_SEO_SCORE: u8 = 92;
pub const SCHEMA_ENTITY_COUNT: u8 = 7;
pub const ANSWER_ENGINE_OPTIMIZATION_SCORE: u8 = 65;
pub const ANSWER_ENGINE_OPTIMIZATION_GUIDANCE: &str = "Improve structural clarity for citations";
pub const GEOGRAPHIC_SEO_STATUS: &str = "N/A (Set service areas)";
pub const DRUPAL_SECURITY_STATUS: &str = "General Site Security: High";
pub const DRUPAL_RECOMMENDATION: &str = "Install SecKit module for enhanced CSP.";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SeoScope {
    #[default]
    CurrentPage,
    SiteWide,
}

impl SeoScope {
    pub fn label(self) -> &'static str {
        match self {
            Self::CurrentPage => "Current Page",
            Self::SiteWide => "Site-Wide",
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PerformanceDataMode {
    #[default]
    Field,
    Lab,
}

impl PerformanceDataMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Field => "Field Data",
            Self::Lab => "Lab Data",
        }
    }

    pub fn toggled(self) -> Self {
        match self {
            Self::Field => Self::Lab,
            Self::Lab => Self::Field,
        }
    }
}
