import type { InsightLimits } from './insight-engine';

function toPositiveOptional(value: unknown): number | undefined {
  if (typeof value !== 'number' || Number.isNaN(value)) {
    return undefined;
  }
  return Math.max(1, Math.floor(value));
}

export function readLimits(payload: Record<string, unknown>): InsightLimits {
  const limitsSource = payload.limits;
  const candidate = (typeof limitsSource === 'object' && limitsSource !== null ? limitsSource : payload) as Record<
    string,
    unknown
  >;

  return {
    risks: toPositiveOptional(candidate.risks),
    opportunities: toPositiveOptional(candidate.opportunities),
    severityThreshold:
      typeof candidate.severityThreshold === 'number' && Number.isFinite(candidate.severityThreshold)
        ? candidate.severityThreshold
        : undefined,
    latencyP95Ms: toPositiveOptional(candidate.latencyP95Ms)
  };
}

export function readPayload(payload: Record<string, unknown>): Record<string, unknown> {
  const nestedPayload = payload.payload;
  if (typeof nestedPayload === 'object' && nestedPayload !== null) {
    return nestedPayload as Record<string, unknown>;
  }
  return payload;
}
