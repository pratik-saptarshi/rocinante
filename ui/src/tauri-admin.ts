export interface AdminBridgeResult {
  ok: boolean;
  command: string;
  payload: string;
  message: string;
}

type CommitIngestionEvent = {
  commit_id: string;
  repo_name: string;
  release: string;
  committer: string;
  telemetry: Array<{
    plugin: string;
    metric_key: string;
    metric_value: number;
    details: string;
  }>;
};

export interface AdminIngestPayload {
  token: string;
  event: CommitIngestionEvent;
}

export interface AdminPromotePayload {
  token: string;
}

export interface AdminQueryPayload {
  token: string;
  name?: string;
  release?: string;
}

export interface AdminCommitterScoresPayload {
  token: string;
  name?: string;
  release?: string;
}

export interface AdminRankPrsPayload {
  token: string;
  prs: Array<{
    pr_id: string;
    repo_name: string;
    author: string;
    release: string;
    file_risk: number;
    author_velocity: number;
    approval_fidelity: number;
  }>;
}

export interface AdminUpdateWeightsPayload {
  token: string;
  weights: {
    version: string;
    complexity_weight: number;
    coverage_weight: number;
    churn_weight: number;
    pipeline_weight: number;
    pr_file_risk_weight: number;
    pr_velocity_weight: number;
    pr_approval_weight: number;
  };
}

export type AdminBridgeCommand =
  | 'ingest_event'
  | 'promote_lifecycle'
  | 'query_aggregates'
  | 'committer_scores'
  | 'rank_prs'
  | 'update_scoring_weights';

type AdminCommandArgs = {
  ingest_event: AdminIngestPayload;
  promote_lifecycle: AdminPromotePayload;
  query_aggregates: AdminQueryPayload;
  committer_scores: AdminCommitterScoresPayload;
  rank_prs: AdminRankPrsPayload;
  update_scoring_weights: AdminUpdateWeightsPayload;
};

type InvokeFn = (command: string, args?: Record<string, unknown>) => Promise<unknown>;

function resolveInvoke(): InvokeFn | null {
  const tauriCore = (globalThis as { __TAURI__?: { core?: { invoke?: InvokeFn } } }).__TAURI__?.core;
  if (tauriCore?.invoke) {
    return tauriCore.invoke;
  }
  return null;
}

export async function invokeAdminCommand(
  command: AdminBridgeCommand,
  args: AdminCommandArgs[typeof command]
): Promise<AdminBridgeResult> {
  const invoke = resolveInvoke();
  if (!invoke) {
    return {
      ok: false,
      command,
      payload: JSON.stringify(args),
      message: 'Tauri runtime not detected. Command bridge is available in desktop runtime only.'
    };
  }

  try {
    const response = await invoke(command, args);
    return {
      ok: true,
      command,
      payload: JSON.stringify(args),
      message: typeof response === 'string' ? response : JSON.stringify(response)
    };
  } catch (error) {
    return {
      ok: false,
      command,
      payload: JSON.stringify(args),
      message: error instanceof Error ? error.message : 'Command failed'
    };
  }
}
