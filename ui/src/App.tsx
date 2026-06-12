import { Accessibility, PlayArrow, Public, Search, Security, Settings, Speed } from '@mui/icons-material';
import {
  Box,
  Button,
  Chip,
  Divider,
  List,
  ListItem,
  ListItemText,
  Paper,
  TextField,
  Stack,
  Switch,
  Tab,
  Tabs,
  ToggleButton,
  ToggleButtonGroup,
  Typography
} from '@mui/material';
import { useState } from 'react';
import { readLimits, readPayload } from './dashboard-contract';
import { buildAdminBridgePayload } from './admin-bridge-contract';
import { buildDashboardInsights, type InsightPayload } from './insight-engine';
import { buildQualityPulse, type StakeholderAudience } from './domain/quality-pulse';
import { invokeAdminCommand, type AdminBridgeCommand } from './tauri-admin';

interface Finding {
  id: string;
  text: string;
  status: AuditStatus;
}

type AuditStatus = 'good' | 'medium' | 'bad';

function StatusBadge({ status, label }: { status: AuditStatus; label: string }) {
  const palette = {
    good: 'success',
    medium: 'warning',
    bad: 'error'
  } as const;

  return <Chip color={palette[status]} size="small" label={label} />;
}

function ScoreGauge({ value, subtitle, status }: { value: number; subtitle: string; status: AuditStatus }) {
  return (
    <Box
      sx={{
        display: 'flex',
        alignItems: 'center',
        gap: 2,
        py: 1
      }}
    >
      <Box
        aria-label={`Score ${value} out of 100`}
        sx={{
          width: 72,
          height: 72,
          borderRadius: '50%',
          display: 'grid',
          placeItems: 'center',
          border: '8px solid',
          borderColor: status === 'good' ? '#2e7d32' : status === 'medium' ? '#f57c00' : '#c62828',
          fontWeight: 'bold',
          fontSize: '1.4rem',
          color: '#223',
          backgroundColor: '#fafafa'
        }}
      >
        <Typography component="span" fontWeight="bold">
          {value}
        </Typography>
      </Box>
      <Box>
        <StatusBadge status={status} label={`${subtitle}: ${value}/100`} />
        <Typography variant="body2" color="text.secondary" sx={{ mt: 0.5 }}>
          {status === 'good' ? 'Good' : status === 'medium' ? 'Needs Improvement' : 'Action Required'}
        </Typography>
      </Box>
    </Box>
  );
}

function FindingSection({ title, items }: { title: string; items: Finding[] }) {
  return (
    <Box sx={{ mt: 1.5 }}>
      <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 1 }}>
        {title}
      </Typography>
      <List dense disablePadding>
        {items.map((item) => (
          <ListItem key={item.id} disablePadding>
            <ListItemText
              primary={
                <Stack direction="row" spacing={1} alignItems="center">
                  <StatusBadge
                    status={item.status}
                    label={item.status === 'good' ? 'good' : item.status === 'medium' ? 'medium' : 'bad'}
                  />
                  <Typography variant="body2">{item.text}</Typography>
                </Stack>
              }
            />
          </ListItem>
        ))}
      </List>
    </Box>
  );
}

function MetricItem({
  label,
  value,
  valueTestId
}: {
  label: string;
  value: string;
  valueTestId?: string;
}) {
  return (
    <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 0.5 }}>
      <Typography variant="body2" color="text.secondary">
        {label}
      </Typography>
      <Typography variant="body2" fontWeight={600} data-testid={valueTestId}>
        {value}
      </Typography>
    </Box>
  );
}

function App() {
  const [audience, setAudience] = useState<StakeholderAudience>('lead');
  const [seoTab, setSeoTab] = useState<'current' | 'site'>('current');
  const [fieldData, setFieldData] = useState(true);
  const [payloadText, setPayloadText] = useState('');
  const [payloadError, setPayloadError] = useState('');
  const [insights, setInsights] = useState(() => buildDashboardInsights());
  const [adminToken, setAdminToken] = useState('alice:admin');
  const [adminResult, setAdminResult] = useState('No admin command executed yet.');

  const accessibilityFindings: Finding[] = [
    { id: 'alt-text', text: 'Missing image alt text (7 instances)', status: 'bad' },
    { id: 'contrast', text: 'Low color contrast on buttons', status: 'medium' },
    { id: 'keyboard', text: 'Keyboard navigation traps found', status: 'medium' }
  ];

  const seoGuidance: Finding[] = [
    { id: 'h1', text: 'Improve H1 and title tag consistency', status: 'medium' },
    { id: 'faq', text: 'Add FAQ schema', status: 'medium' },
    { id: 'qa', text: 'Optimize for long-tail, question-based queries', status: 'medium' }
  ];

  const securityFindings: Finding[] = [
    { id: 'csp', text: 'Content-Security-Policy: Missing', status: 'bad' },
    { id: 'frame', text: 'X-Frame-Options: SAMEORIGIN', status: 'good' },
    { id: 'hsts', text: 'Strict-Transport-Security: Enabled', status: 'good' }
  ];

  const performanceFindings: Finding[] = [
    { id: 'lcp', text: 'Largest Contentful Paint (LCP): 2.9s', status: 'medium' },
    { id: 'cls', text: 'Cumulative Layout Shift (CLS): 0.05', status: 'good' },
    { id: 'tbt', text: 'Total Blocking Time (TBT): 350ms', status: 'medium' }
  ];

  const audienceHighlights: Record<StakeholderAudience, { tone: string; guidance: string }> = {
    lead: {
      tone: 'Team leads: prioritize blocked PR hotspots and coaching cues.',
      guidance: 'Prioritize review-ready commits before opening the next work cycle.'
    },
    manager: {
      tone: 'Managers: monitor cycle time, reviewer load, and handoff stability.',
      guidance: 'Uncover queue pressure to rebalance approval throughput and merge cadence.'
    },
    executive: {
      tone: 'Executives: monitor strategic quality and delivery predictability.',
      guidance: 'Use the opportunity list to stabilize throughput and defect risk over time.'
    },
    security: {
      tone: 'Security: prioritize policy drift and dependency risk signals.',
      guidance: 'Surface release and dependency risks before they enter long-running branches.'
    }
  };

  const { commitRiskCards, bottlenecks, opportunities } = insights;
  const qualityPulse = buildQualityPulse(insights);
  const audienceActions = qualityPulse.recommendations[audience];
  const audienceRoute = qualityPulse.actionRoutes[audience];
  const topOpps = opportunities.slice(0, 2);

  const securitySignals = commitRiskCards.filter((risk) =>
    risk.reasons.some((item) => item === 'Dependency risk' || item === 'Automation failures')
  );

  const criticalBottlenecks = qualityPulse.bottleneckBuckets.critical;
  const highBottlenecks = qualityPulse.bottleneckBuckets.high;

  const applyPayload = () => {
    if (!payloadText.trim()) {
      setInsights(buildDashboardInsights());
      setPayloadError('');
      return;
    }

    try {
      const parsed = JSON.parse(payloadText);
      const parsedRecord = typeof parsed === 'object' && parsed !== null ? (parsed as Record<string, unknown>) : {};
      const nextPayload = readPayload(parsedRecord);
      const nextLimits = readLimits(parsedRecord);
      setInsights(buildDashboardInsights(nextPayload as InsightPayload, nextLimits));
      setPayloadError('');
    } catch {
      setPayloadError('Invalid JSON payload. Paste a valid telemetry payload to refresh the dashboard.');
    }
  };

  const resetPayload = () => {
    setPayloadText('');
    setPayloadError('');
    setInsights(buildDashboardInsights());
  };

  const runAdminBridge = async (command: AdminBridgeCommand) => {
    const result = await invokeAdminCommand(command, buildAdminBridgePayload(command, adminToken));

    setAdminResult(`${result.ok ? 'OK' : 'ERR'} ${result.command}: ${result.message}`);
  };

  return (
    <Box
      sx={{
        minHeight: '100vh',
        background: 'linear-gradient(140deg, #f4f7ff 0%, #f4f4f4 40%, #ffffff 100%)',
        display: 'flex',
        justifyContent: 'flex-end',
        fontFamily: 'Manrope, Segoe UI, sans-serif'
      }}
    >
      <Paper
        square
        elevation={4}
        sx={{
          width: 340,
          minHeight: '100vh',
          borderLeft: '1px solid #e5e7eb',
          borderRadius: 0,
          p: 2.5
        }}
      >
        <Stack direction="row" justifyContent="space-between" alignItems="center" sx={{ mb: 2 }}>
          <Stack direction="row" spacing={1} alignItems="center">
            <Public fontSize="small" color="action" />
            <Typography variant="subtitle1" fontWeight={700}>
              The Web Companion: Optimization Hub
            </Typography>
          </Stack>
          <Settings fontSize="small" color="action" />
        </Stack>

        <ToggleButtonGroup
          size="small"
          value={audience}
          exclusive
          onChange={(_, next) => next && setAudience(next)}
          aria-label="Audience view"
          sx={{ mb: 1.25 }}
          fullWidth
        >
          <ToggleButton value="lead">Team Lead</ToggleButton>
          <ToggleButton value="manager">Manager</ToggleButton>
          <ToggleButton value="executive">Executive</ToggleButton>
          <ToggleButton value="security">Security</ToggleButton>
        </ToggleButtonGroup>

        <Typography variant="body2" color="text.secondary" sx={{ mb: 1.5 }}>
          {audienceHighlights[audience].tone}
        </Typography>

        <Divider sx={{ my: 1 }} />
        <Box sx={{ mb: 1.5 }}>
          <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 0.5 }}>
            Quality Snapshot
          </Typography>
          <MetricItem label="Commit risk cards" value={`${commitRiskCards.length}`} valueTestId="snapshot-risk-count" />
          <MetricItem
            label="High-risk commits"
            value={`${qualityPulse.riskBuckets.high} / ${commitRiskCards.length}`}
            valueTestId="snapshot-high-risk-count"
          />
          <MetricItem
            label="Critical bottlenecks"
            value={`${criticalBottlenecks} critical, ${highBottlenecks} high`}
            valueTestId="snapshot-bottleneck-count"
          />
          <MetricItem
            label="Actionable opportunities"
            value={`${opportunities.length}`}
            valueTestId="snapshot-opportunity-count"
          />
        </Box>

        <Box sx={{ mb: 1.5 }} data-testid="quality-pulse-section">
          <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 0.5 }}>
            Quality Pulse
          </Typography>
          <MetricItem
            label="Pulse score"
            value={`${qualityPulse.overallScore}/100`}
            valueTestId="pulse-score"
          />
          <MetricItem
            label="Security-sensitive commits"
            value={`${qualityPulse.securitySignalCount}`}
            valueTestId="pulse-security-count"
          />
          <MetricItem
            label="Top bottleneck"
            value={qualityPulse.topBottleneckName}
            valueTestId="pulse-top-bottleneck"
          />
          <FindingSection
            title={`Recommended actions (${audience})`}
            items={audienceActions.map((action) => ({
              id: action.id,
              text: action.message,
              status: action.severity
            }))}
          />
          <Paper variant="outlined" sx={{ p: 1.25, mt: 1, borderRadius: 2 }}>
            <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 0.5 }}>
              Action Routing
            </Typography>
            <MetricItem label="Owner" value={audienceRoute.owner} />
            <MetricItem label="Execution Window" value={audienceRoute.window} />
            <List dense disablePadding>
              {audienceRoute.actions.map((action, index) => (
                <ListItem key={`${audience}-route-${index}`} disablePadding>
                  <ListItemText primary={<Typography variant="body2">{action}</Typography>} />
                </ListItem>
              ))}
            </List>
          </Paper>
        </Box>

        <Box sx={{ mb: 1.5 }}>
          <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 0.5 }}>
            Live Insights Payload
          </Typography>
          <TextField
            fullWidth
            multiline
            minRows={4}
            label="Telemetry payload JSON"
            placeholder='{ "commits": [...], "stages": [...], "signals": [...] }'
            value={payloadText}
            onChange={(event) => {
              setPayloadText(event.target.value);
              if (payloadError) {
                setPayloadError('');
              }
            }}
            size="small"
            sx={{ mb: 1 }}
          />
          <Stack direction="row" spacing={1}>
            <Button size="small" variant="contained" onClick={applyPayload}>
              Apply Payload
            </Button>
            <Button size="small" variant="outlined" onClick={resetPayload}>
              Reset to Sample
            </Button>
          </Stack>
          {payloadError && (
            <Typography variant="caption" color="error" role="alert" sx={{ display: 'block', mt: 0.75 }}>
              {payloadError}
            </Typography>
          )}
        </Box>

        <Box sx={{ mb: 1.5 }}>
          <Typography variant="subtitle2" fontWeight={700} sx={{ mb: 0.5 }}>
            Admin Command Bridge
          </Typography>
          <TextField
            fullWidth
            label="Admin token"
            size="small"
            value={adminToken}
            onChange={(event) => setAdminToken(event.target.value)}
            sx={{ mb: 1 }}
          />
          <Stack direction="row" spacing={1} sx={{ mb: 1, flexWrap: 'wrap' }}>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('ingest_event')}>
              Ingest Event
            </Button>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('promote_lifecycle')}>
              Promote Lifecycle
            </Button>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('query_aggregates')}>
              Query Aggregates
            </Button>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('committer_scores')}>
              Committer Scores
            </Button>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('rank_prs')}>
              Rank PRs
            </Button>
            <Button size="small" variant="outlined" onClick={() => void runAdminBridge('update_scoring_weights')}>
              Update Scoring Weights
            </Button>
          </Stack>
          <Typography variant="caption" data-testid="admin-bridge-result">
            {adminResult}
          </Typography>
        </Box>

        <Divider sx={{ my: 1.5 }} />

        {audience === 'lead' && (
          <Box>
            <Typography variant="subtitle2" fontWeight={700}>
              Team Lead Focus
            </Typography>
            <Typography variant="caption" display="block" sx={{ mb: 1 }}>
              {audienceHighlights[audience].guidance}
            </Typography>
            <Typography variant="subtitle2" sx={{ mt: 1, mb: 0.5 }}>
              Top Commit Risks
            </Typography>
            <FindingSection
              title=""
              items={commitRiskCards.slice(0, 3).map((risk) => ({
                id: risk.id,
                text: `${risk.id} score ${risk.score} (${risk.level})`,
                status: risk.level === 'high' ? 'bad' : risk.level === 'medium' ? 'medium' : 'good'
              }))}
            />
          </Box>
        )}

        {audience === 'manager' && (
          <Box>
            <Typography variant="subtitle2" fontWeight={700}>
              Manager Focus
            </Typography>
            <Typography variant="caption" display="block" sx={{ mb: 1 }}>
              {audienceHighlights[audience].guidance}
            </Typography>
            <Typography variant="subtitle2" sx={{ mt: 1, mb: 0.5 }}>
              Bottleneck Radar
            </Typography>
            <List dense disablePadding>
              {bottlenecks.map((item) => (
                <ListItem key={item.name} disablePadding>
                  <ListItemText
                    primary={
                      <Typography variant="body2">
                        {item.name} ({item.status}) impact {item.impact}
                      </Typography>
                    }
                    secondary={item.rationale}
                  />
                </ListItem>
              ))}
            </List>
          </Box>
        )}

        {audience === 'executive' && (
          <Box>
            <Typography variant="subtitle2" fontWeight={700}>
              Executive Focus
            </Typography>
            <Typography variant="caption" display="block" sx={{ mb: 1 }}>
              {audienceHighlights[audience].guidance}
            </Typography>
            <Typography variant="subtitle2" sx={{ mt: 1, mb: 0.5 }}>
              Top Improvement Opportunities
            </Typography>
            <List dense disablePadding>
              {topOpps.map((opp) => (
                <ListItem key={opp.id} disablePadding>
                  <ListItemText
                    primary={
                      <Typography variant="body2">
                        {opp.title} (score {opp.priorityScore})
                      </Typography>
                    }
                  />
                </ListItem>
              ))}
            </List>
          </Box>
        )}

        {audience === 'security' && (
          <Box>
            <Typography variant="subtitle2" fontWeight={700}>
              Security Focus
            </Typography>
            <Typography variant="caption" display="block" sx={{ mb: 1 }}>
              {audienceHighlights[audience].guidance}
            </Typography>
            <Typography variant="subtitle2" sx={{ mt: 1, mb: 0.5 }}>
              Security-Weighted Commit Signals
            </Typography>
            <FindingSection
              title=""
              items={
                securitySignals.length
                  ? securitySignals.map((risk) => ({
                      id: risk.id,
                      text: `${risk.id}: ${risk.reasons.join(', ')}`,
                      status: risk.level === 'high' ? 'bad' : risk.level === 'medium' ? 'medium' : 'good'
                    }))
                  : [
                      {
                        id: 'security-empty',
                        text: 'No critical security signals in sample window',
                        status: 'good'
                      }
                    ]
              }
            />
          </Box>
        )}

        <Divider sx={{ my: 2 }} />
        <Box sx={{ mb: 2 }}>
          <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
            <Accessibility fontSize="small" color="action" />
            <Typography variant="subtitle2" fontWeight={700}>
              WCAG 2.1/2.2 AA Accessibility Audit
            </Typography>
          </Stack>
          <ScoreGauge value={85} subtitle="Overall Score" status="good" />
          <Button
            fullWidth
            variant="contained"
            startIcon={<PlayArrow />}
            aria-label="Run Full Audit"
            sx={{ textTransform: 'none', mt: 1 }}
          >
            Run Full Audit
          </Button>
          <FindingSection title="Findings" items={accessibilityFindings} />
        </Box>

        <Divider sx={{ my: 2 }} />
        <Box>
          <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
            <Search fontSize="small" color="action" />
            <Typography variant="subtitle2" fontWeight={700}>
              SEO, GEO &amp; AEO Performance
            </Typography>
          </Stack>
          <Tabs value={seoTab} onChange={(_, value) => setSeoTab(value)} sx={{ mb: 1.5 }} variant="fullWidth">
            <Tab value="current" label="Current Page" sx={{ minHeight: 36 }} />
            <Tab value="site" label="Site-Wide" sx={{ minHeight: 36 }} />
          </Tabs>
          <MetricItem label="On-Page SEO" value="92/100" />
          <MetricItem label="Schema Markup" value="Found (7 entities)" />
          <MetricItem
            label="Answer Engine Optimization"
            value="65/100 (Improve structural clarity for citations)"
          />
          <MetricItem label="Geographic SEO" value="N/A (Set service areas)" />
          <FindingSection title={`Example guidance (${seoTab})`} items={seoGuidance} />
        </Box>

        <Divider sx={{ my: 2 }} />
        <Box>
          <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
            <Security fontSize="small" color="action" />
            <Typography variant="subtitle2" fontWeight={700}>
              Security &amp; Drupal Review
            </Typography>
          </Stack>
          <StatusBadge status="good" label="General Site Security: High" />
          <Divider sx={{ mt: 1.5 }} />
          <FindingSection title="Drupal-Specific Checks" items={securityFindings} />
          <Typography variant="body2" sx={{ mt: 1 }}>
            Recommendation: Install SecKit module for enhanced CSP.
          </Typography>
        </Box>

        <Divider sx={{ my: 2 }} />
        <Box>
          <Stack direction="row" spacing={1} alignItems="center" sx={{ mb: 1 }}>
            <Speed fontSize="small" color="action" />
            <Typography variant="subtitle2" fontWeight={700}>
              Page Performance Metrics
            </Typography>
          </Stack>
          <ScoreGauge value={65} subtitle="Overall Score" status="medium" />
          <FindingSection title="Top Recommendations" items={performanceFindings} />

          <Box sx={{ display: 'flex', alignItems: 'center', mt: 1.5, gap: 1 }}>
            <Typography variant="body2">Field Data</Typography>
            <Switch checked={fieldData} onChange={() => setFieldData((prev) => !prev)} aria-label="Field or lab data toggle" />
            <Typography variant="body2" fontWeight={700}>
              Lab Data
            </Typography>
          </Box>
        </Box>
      </Paper>
    </Box>
  );
}

export default App;
