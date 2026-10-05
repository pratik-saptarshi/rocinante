#[cfg(feature = "native-ui")]
pub mod dashboard;
pub mod dashboard_insights;
pub mod dashboard_reference;
pub mod deep_link;

#[cfg(all(feature = "native-ui", target_os = "macos"))]
mod macos_url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum ShellPage {
    #[default]
    Dashboard,
    Repositories,
    Alerts,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    OpenDashboard,
    OpenRepositories,
    OpenAlerts,
    OpenSettings,
    ChooseRepository,
    Close,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutKey {
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    O,
    Q,
}

pub fn navigation_shortcut(command: bool, key: ShortcutKey) -> Option<NavigationAction> {
    if !command {
        return None;
    }
    match key {
        ShortcutKey::Digit1 => Some(NavigationAction::OpenDashboard),
        ShortcutKey::Digit2 => Some(NavigationAction::OpenRepositories),
        ShortcutKey::Digit3 => Some(NavigationAction::OpenAlerts),
        ShortcutKey::Digit4 => Some(NavigationAction::OpenSettings),
        ShortcutKey::Q => Some(NavigationAction::Close),
        ShortcutKey::O => Some(NavigationAction::ChooseRepository),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellLifecycle {
    Ready,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowCloseBehavior {
    Hide,
    Exit,
}

pub fn window_close_behavior(tray_available: bool, explicit_quit: bool) -> WindowCloseBehavior {
    if tray_available && !explicit_quit {
        WindowCloseBehavior::Hide
    } else {
        WindowCloseBehavior::Exit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowProfile {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

impl Default for WindowProfile {
    fn default() -> Self {
        Self {
            title: "Rocinante Repo Analyzer".to_string(),
            width: 1200,
            height: 800,
            resizable: true,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ShellState {
    page: ShellPage,
    selected_repository: Option<StoredPath>,
    #[serde(skip)]
    lifecycle: Option<ShellLifecycle>,
}

impl ShellState {
    pub fn page(&self) -> ShellPage {
        self.page
    }

    pub fn lifecycle(&self) -> ShellLifecycle {
        self.lifecycle.unwrap_or(ShellLifecycle::Ready)
    }

    pub fn selected_repository(&self) -> Option<PathBuf> {
        self.selected_repository
            .as_ref()
            .map(StoredPath::to_path_buf)
    }

    pub fn select_repository(&mut self, path: Option<PathBuf>) {
        if self.lifecycle() != ShellLifecycle::Closed {
            self.selected_repository = path.as_deref().map(StoredPath::from_path);
        }
    }

    pub fn dispatch(&mut self, action: NavigationAction) {
        if self.lifecycle() == ShellLifecycle::Closed {
            return;
        }

        match action {
            NavigationAction::OpenDashboard => self.page = ShellPage::Dashboard,
            NavigationAction::OpenRepositories => self.page = ShellPage::Repositories,
            NavigationAction::OpenAlerts => self.page = ShellPage::Alerts,
            NavigationAction::OpenSettings => self.page = ShellPage::Settings,
            NavigationAction::ChooseRepository => {}
            NavigationAction::Close => self.lifecycle = Some(ShellLifecycle::Closed),
        }
    }

    pub fn apply_deep_link(&mut self, target: deep_link::DeepLinkTarget) {
        if self.lifecycle() == ShellLifecycle::Closed {
            return;
        }
        match target {
            deep_link::DeepLinkTarget::OpenRepository(path) => {
                self.select_repository(Some(path));
                self.page = ShellPage::Repositories;
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize)]
struct StoredPath {
    #[serde(skip_serializing_if = "Option::is_none")]
    legacy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unix_bytes: Option<Vec<u8>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    windows_wide: Option<Vec<u16>>,
}

impl<'de> serde::Deserialize<'de> for StoredPath {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct StoredPathVisitor;

        impl<'de> serde::de::Visitor<'de> for StoredPathVisitor {
            type Value = StoredPath;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a legacy path string or a platform path map")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(StoredPath {
                    legacy: Some(value.to_owned()),
                    ..StoredPath::default()
                })
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                let mut path = StoredPath::default();
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "legacy" => path.legacy = map.next_value()?,
                        "unix_bytes" => path.unix_bytes = map.next_value()?,
                        "windows_wide" => path.windows_wide = map.next_value()?,
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }
                if path.legacy.is_none() && path.unix_bytes.is_none() && path.windows_wide.is_none()
                {
                    return Err(serde::de::Error::custom("platform path data is missing"));
                }
                Ok(path)
            }
        }

        deserializer.deserialize_any(StoredPathVisitor)
    }
}

impl StoredPath {
    #[cfg(unix)]
    fn from_path(path: &Path) -> Self {
        Self {
            unix_bytes: Some(path.as_os_str().as_bytes().to_vec()),
            ..Self::default()
        }
    }

    #[cfg(windows)]
    fn from_path(path: &Path) -> Self {
        Self {
            windows_wide: Some(path.as_os_str().encode_wide().collect()),
            ..Self::default()
        }
    }

    #[cfg(not(any(unix, windows)))]
    fn from_path(path: &Path) -> Self {
        Self {
            legacy: Some(path.to_string_lossy().into_owned()),
            ..Self::default()
        }
    }

    #[cfg(unix)]
    fn to_path_buf(&self) -> PathBuf {
        if let Some(unix_bytes) = &self.unix_bytes {
            return PathBuf::from(std::ffi::OsString::from_vec(unix_bytes.clone()));
        }
        if let Some(windows_wide) = &self.windows_wide {
            return PathBuf::from(String::from_utf16_lossy(windows_wide));
        }
        self.legacy
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_default()
    }

    #[cfg(windows)]
    fn to_path_buf(&self) -> PathBuf {
        if let Some(windows_wide) = &self.windows_wide {
            return PathBuf::from(std::ffi::OsString::from_wide(windows_wide));
        }
        if let Some(unix_bytes) = &self.unix_bytes {
            return PathBuf::from(String::from_utf8_lossy(unix_bytes).into_owned());
        }
        self.legacy
            .as_deref()
            .map(PathBuf::from)
            .unwrap_or_default()
    }

    #[cfg(not(any(unix, windows)))]
    fn to_path_buf(&self) -> PathBuf {
        if let Some(legacy) = &self.legacy {
            return PathBuf::from(legacy);
        }
        if let Some(unix_bytes) = &self.unix_bytes {
            return PathBuf::from(String::from_utf8_lossy(unix_bytes).into_owned());
        }
        self.windows_wide
            .as_deref()
            .map(String::from_utf16_lossy)
            .map(PathBuf::from)
            .unwrap_or_default()
    }
}

#[cfg(feature = "native-ui")]
mod native_ui {
    use super::{
        dashboard::{format_analysis_summary, DashboardViewModel},
        dashboard_insights::{
            build_dashboard_visuals, build_explainability_traces, build_quality_pulse,
            AppliedInsights, BottleneckStatus, InsightSource, PulseSeverity, RiskLevel,
            StakeholderAudience, VisualTone,
        },
        dashboard_reference::{
            Finding, FindingStatus, PerformanceDataMode, SeoScope, ACCESSIBILITY_FINDINGS,
            ACCESSIBILITY_SCORE, ANSWER_ENGINE_OPTIMIZATION_GUIDANCE,
            ANSWER_ENGINE_OPTIMIZATION_SCORE, DRUPAL_RECOMMENDATION, DRUPAL_SECURITY_STATUS,
            GEOGRAPHIC_SEO_STATUS, ON_PAGE_SEO_SCORE, PERFORMANCE_FINDINGS, PERFORMANCE_SCORE,
            SCHEMA_ENTITY_COUNT, SECURITY_FINDINGS, SEO_FINDINGS,
        },
        deep_link::{parse_deep_link, DeepLinkEvent, DeepLinkInbox, DeepLinkTarget},
        navigation_shortcut, window_close_behavior, NavigationAction, ShellPage, ShellState,
        ShortcutKey, WindowCloseBehavior, WindowProfile,
    };
    use eframe::egui;
    use std::path::{Path, PathBuf};
    use std::sync::mpsc::{self, Receiver, TryRecvError};
    use tray_icon::{
        menu::{Menu, MenuEvent, MenuItem},
        Icon, TrayIcon, TrayIconBuilder,
    };

    type ScanOutcome = Result<
        (
            rocinante_analysis::telemetry::TelemetryImportSummary,
            Vec<rocinante_analysis::types::RepositoryMetric>,
        ),
        String,
    >;
    type MetricsOutcome = Result<Vec<rocinante_analysis::types::RepositoryMetric>, String>;
    type BaselineOutcome = Result<String, String>;
    type AdminOutcome = Result<String, String>;

    fn deep_link_changes_repository(selected: Option<&Path>, linked: &Path) -> bool {
        selected != Some(linked)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TrayMenuAction {
        Show,
        Quit,
    }

    fn tray_menu_action(id: &str) -> Option<TrayMenuAction> {
        match id {
            "show" => Some(TrayMenuAction::Show),
            "quit" => Some(TrayMenuAction::Quit),
            _ => None,
        }
    }

    fn apply_tray_menu_action(action: TrayMenuAction, context: &egui::Context) -> bool {
        match action {
            TrayMenuAction::Show => {
                context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                context.send_viewport_cmd(egui::ViewportCommand::Focus);
                #[cfg(target_os = "macos")]
                {
                    #[cfg(feature = "acceptance-witness")]
                    if let Some(result_path) = option_env!("ROCINANTE_ACCEPTANCE_ACTIVATION_RESULT")
                    {
                        let _ = std::fs::write(result_path, "show_action_started=true\n");
                    }
                    let activation_accepted = super::macos_url::activate_application();
                    #[cfg(feature = "acceptance-witness")]
                    if let Some(result_path) = option_env!("ROCINANTE_ACCEPTANCE_ACTIVATION_RESULT")
                    {
                        let _ = std::fs::write(result_path, format!("show_action_started=true\nrequest_accepted={activation_accepted}\n"));
                    }
                    #[cfg(not(feature = "acceptance-witness"))]
                    let _ = activation_accepted;
                }
                false
            }
            TrayMenuAction::Quit => true,
        }
    }

    #[cfg(feature = "acceptance-witness")]
    #[derive(Debug, Clone, Copy)]
    enum AcceptanceControl {
        Tray(TrayMenuAction),
        Close,
        Minimize,
    }

    fn record_acceptance_state(state: &ShellState, tray_available: bool) {
        #[cfg(feature = "acceptance-witness")]
        if let Some(witness_path) = option_env!("ROCINANTE_ACCEPTANCE_WITNESS") {
            static INSTANCE_ID: std::sync::OnceLock<u128> = std::sync::OnceLock::new();
            let instance_id = INSTANCE_ID.get_or_init(|| {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos()
            });
            let selected = state
                .selected_repository()
                .map(|path| path.to_string_lossy().into_owned())
                .unwrap_or_default();
            let snapshot = format!(
                "pid={}\ninstance={}\npage={:?}\npath={}\ntray_available={}\n",
                std::process::id(),
                instance_id,
                state.page(),
                selected,
                tray_available
            );
            let _ = std::fs::write(witness_path, snapshot);
        }
        #[cfg(not(feature = "acceptance-witness"))]
        let _ = (state, tray_available);
    }

    struct RocinanteApp {
        state: ShellState,
        persistence_path: PathBuf,
        profile: WindowProfile,
        release: String,
        scan_token: String,
        baseline_repository: String,
        baseline_complexity: String,
        baseline_receiver: Option<(String, Receiver<BaselineOutcome>)>,
        baseline_result: Option<String>,
        admin_command: String,
        admin_payload_input: String,
        admin_receiver: Option<(String, Receiver<AdminOutcome>)>,
        admin_result: Option<String>,
        insight_payload_input: String,
        dashboard_insights: AppliedInsights,
        insights_audience: StakeholderAudience,
        seo_scope: SeoScope,
        performance_data_mode: PerformanceDataMode,
        scan_receiver: Option<(PathBuf, String, Receiver<ScanOutcome>)>,
        scan_result: Option<ScanOutcome>,
        metrics_receiver: Option<(PathBuf, String, Receiver<MetricsOutcome>)>,
        metrics_result: Option<MetricsOutcome>,
        tray_icon: Option<TrayIcon>,
        tray_action_receiver: Receiver<TrayMenuAction>,
        initial_visibility_requested: bool,
        #[cfg(feature = "acceptance-witness")]
        acceptance_receiver: Receiver<AcceptanceControl>,
        explicit_quit: bool,
        link_inbox: DeepLinkInbox,
        url_event_receiver: Receiver<String>,
        #[cfg(target_os = "macos")]
        _url_event_handler: super::macos_url::UrlEventHandler,
    }

    #[cfg(target_os = "macos")]
    struct MacosStartup {
        url_event_handler: super::macos_url::UrlEventHandler,
        repaint_context: std::sync::Arc<std::sync::Mutex<Option<egui::Context>>>,
    }

    impl RocinanteApp {
        fn new(
            creation_context: &eframe::CreationContext<'_>,
            link_inbox: DeepLinkInbox,
            persistence_path: PathBuf,
            restored_state: Option<ShellState>,
            initial_links: Vec<String>,
            url_event_receiver: Receiver<String>,
            #[cfg(target_os = "macos")] macos_startup: MacosStartup,
        ) -> Self {
            #[cfg(target_os = "macos")]
            if let Ok(mut repaint_slot) = macos_startup.repaint_context.lock() {
                *repaint_slot = Some(creation_context.egui_ctx.clone());
            }
            let mut state: ShellState = restored_state.unwrap_or_else(|| {
                creation_context
                    .storage
                    .and_then(|storage| {
                        storage
                            .get_string("rocinante_shell_state_json")
                            .and_then(|saved| serde_json::from_str(&saved).ok())
                            .or_else(|| eframe::get_value(storage, "rocinante_shell_state"))
                    })
                    .unwrap_or_default()
            });
            for link in initial_links {
                if let Some(target) = parse_deep_link(&link) {
                    state.apply_deep_link(target);
                }
            }
            #[cfg(feature = "acceptance-witness")]
            notify_acceptance_surface_if_requested();
            let tray_icon = build_tray_icon().ok();
            let (tray_action_sender, tray_action_receiver) = mpsc::channel();
            let repaint = creation_context.egui_ctx.clone();
            MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
                if let Some(action) = tray_menu_action(&event.id.0) {
                    if action == TrayMenuAction::Show {
                        repaint.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        repaint.send_viewport_cmd(egui::ViewportCommand::Focus);
                    }
                    let _ = tray_action_sender.send(action);
                    repaint.request_repaint();
                }
            }));
            #[cfg(feature = "acceptance-witness")]
            let acceptance_receiver =
                start_acceptance_control_watcher(creation_context.egui_ctx.clone());
            record_acceptance_state(&state, tray_icon.is_some());
            Self {
                state,
                persistence_path,
                profile: WindowProfile::default(),
                release: String::new(),
                scan_token: String::new(),
                baseline_repository: String::new(),
                baseline_complexity: String::new(),
                baseline_receiver: None,
                baseline_result: None,
                admin_command: "ingest_event".into(),
                admin_payload_input: sample_admin_payload("ingest_event").into(),
                admin_receiver: None,
                admin_result: None,
                insight_payload_input: String::new(),
                dashboard_insights: AppliedInsights::default(),
                insights_audience: StakeholderAudience::Lead,
                seo_scope: SeoScope::default(),
                performance_data_mode: PerformanceDataMode::default(),
                scan_receiver: None,
                scan_result: None,
                metrics_receiver: None,
                metrics_result: None,
                tray_icon,
                tray_action_receiver,
                initial_visibility_requested: false,
                #[cfg(feature = "acceptance-witness")]
                acceptance_receiver,
                explicit_quit: false,
                link_inbox,
                url_event_receiver,
                #[cfg(target_os = "macos")]
                _url_event_handler: macos_startup.url_event_handler,
            }
        }
    }

    #[cfg(feature = "acceptance-witness")]
    fn start_acceptance_control_watcher(context: egui::Context) -> Receiver<AcceptanceControl> {
        let (sender, receiver) = mpsc::channel();
        let controls = [
            (
                option_env!("ROCINANTE_ACCEPTANCE_QUIT_FILE"),
                AcceptanceControl::Tray(TrayMenuAction::Quit),
            ),
            (
                option_env!("ROCINANTE_ACCEPTANCE_CLOSE_FILE"),
                AcceptanceControl::Close,
            ),
            (
                option_env!("ROCINANTE_ACCEPTANCE_SHOW_FILE"),
                AcceptanceControl::Tray(TrayMenuAction::Show),
            ),
            (
                option_env!("ROCINANTE_ACCEPTANCE_MINIMIZE_FILE"),
                AcceptanceControl::Minimize,
            ),
        ];
        std::thread::spawn(move || loop {
            for (path, control) in controls {
                let Some(path) = path else { continue };
                if std::path::Path::new(path).exists()
                    && std::fs::rename(path, format!("{path}.received")).is_ok()
                {
                    if matches!(control, AcceptanceControl::Tray(TrayMenuAction::Show)) {
                        context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                        context.send_viewport_cmd(egui::ViewportCommand::Focus);
                    }
                    let _ = sender.send(control);
                    context.request_repaint();
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(25));
        });
        receiver
    }

    fn build_tray_icon() -> Result<TrayIcon, Box<dyn std::error::Error>> {
        let menu = Menu::new();
        let show = MenuItem::with_id("show", "Show Rocinante", true, None);
        let quit = MenuItem::with_id("quit", "Quit", true, None);
        menu.append_items(&[&show, &quit])?;

        let mut rgba = Vec::with_capacity(16 * 16 * 4);
        for y in 0..16 {
            for x in 0..16 {
                let mark = x == 3
                    || x == 4
                    || (y == 3 && (4..12).contains(&x))
                    || (x == 11 && (4..8).contains(&y))
                    || (y == 8 && (4..11).contains(&x))
                    || (x >= 8 && y >= 9 && x + y >= 19);
                rgba.extend_from_slice(if mark {
                    &[70, 190, 140, 255]
                } else {
                    &[0, 0, 0, 0]
                });
            }
        }
        let icon = Icon::from_rgba(rgba, 16, 16)?;
        Ok(TrayIconBuilder::new()
            .with_tooltip("Rocinante Repo Analyzer")
            .with_icon(icon)
            .with_menu(Box::new(menu))
            .build()?)
    }

    impl eframe::App for RocinanteApp {
        fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
            let ctx = ui.ctx().clone();
            if !self.initial_visibility_requested {
                self.initial_visibility_requested = true;
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                #[cfg(target_os = "macos")]
                let _ = super::macos_url::activate_application();
            }
            let mut quit_from_tray = false;
            #[cfg(feature = "acceptance-witness")]
            while let Ok(control) = self.acceptance_receiver.try_recv() {
                match control {
                    AcceptanceControl::Tray(action) => {
                        quit_from_tray |= apply_tray_menu_action(action, &ctx)
                    }
                    AcceptanceControl::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                    AcceptanceControl::Minimize => {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true))
                    }
                }
            }
            self.poll_deep_link_inbox(&ctx);
            let close_requested_by_window = ctx.input(|input| input.viewport().close_requested());
            if close_requested_by_window {
                match window_close_behavior(self.tray_icon.is_some(), self.explicit_quit) {
                    WindowCloseBehavior::Hide => {
                        ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                    }
                    WindowCloseBehavior::Exit => {
                        self.state.dispatch(NavigationAction::Close);
                        self.persist_shell_state(frame);
                        #[cfg(target_os = "macos")]
                        {
                            super::macos_url::terminate_application();
                        }
                    }
                }
            }
            while let Ok(action) = self.tray_action_receiver.try_recv() {
                quit_from_tray |= apply_tray_menu_action(action, &ctx);
            }
            if quit_from_tray {
                self.state.dispatch(NavigationAction::Close);
                self.explicit_quit = true;
                self.persist_shell_state(frame);
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if self.tray_icon.is_some() || self.link_inbox.is_primary() {
                ctx.request_repaint_after(std::time::Duration::from_millis(250));
            }
            self.poll_scan(ui.ctx());
            self.poll_metrics(ui.ctx());
            self.poll_baseline(ui.ctx());
            self.poll_admin_command(ui.ctx());
            let shortcut = ui.ctx().input(|input| {
                if !input.modifiers.command {
                    return None;
                }
                [
                    (egui::Key::Num1, ShortcutKey::Digit1),
                    (egui::Key::Num2, ShortcutKey::Digit2),
                    (egui::Key::Num3, ShortcutKey::Digit3),
                    (egui::Key::Num4, ShortcutKey::Digit4),
                    (egui::Key::O, ShortcutKey::O),
                    (egui::Key::Q, ShortcutKey::Q),
                ]
                .into_iter()
                .find_map(|(key, shortcut)| {
                    input
                        .key_pressed(key)
                        .then(|| navigation_shortcut(true, shortcut))
                        .flatten()
                        .map(|action| (shortcut, action))
                })
            });

            let mut close_requested = false;
            if let Some((_, action)) = shortcut {
                if action == NavigationAction::ChooseRepository {
                    self.open_repository_folder();
                } else if action == NavigationAction::Close {
                    self.state.dispatch(action);
                    self.explicit_quit = true;
                    close_requested = true;
                } else {
                    self.state.dispatch(action);
                }
            }

            egui::Panel::top("application_menu").show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(&self.profile.title);
                    ui.separator();
                    let modifier = if cfg!(target_os = "macos") {
                        "⌘"
                    } else {
                        "Ctrl+"
                    };
                    for (label, hint, action) in [
                        (
                            "Dashboard",
                            format!("{modifier}1"),
                            NavigationAction::OpenDashboard,
                        ),
                        (
                            "Repositories",
                            format!("{modifier}2"),
                            NavigationAction::OpenRepositories,
                        ),
                        (
                            "Alerts",
                            format!("{modifier}3"),
                            NavigationAction::OpenAlerts,
                        ),
                        (
                            "Settings",
                            format!("{modifier}4"),
                            NavigationAction::OpenSettings,
                        ),
                    ] {
                        if ui.button(format!("{label} {hint}")).clicked() {
                            self.state.dispatch(action);
                        }
                    }
                    if ui
                        .button(format!("Open Repository…  {modifier}O"))
                        .clicked()
                    {
                        self.open_repository_folder();
                    }
                    if ui.button(format!("Quit  {modifier}Q")).clicked() {
                        self.state.dispatch(NavigationAction::Close);
                        self.explicit_quit = true;
                        close_requested = true;
                    }
                });
            });

            if close_requested {
                self.persist_shell_state(frame);
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }

            egui::CentralPanel::default().show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| match self.state.page() {
                        ShellPage::Dashboard => {
                            ui.heading("Quality dashboard");
                            if let Some(repository) = self.state.selected_repository() {
                                ui.label(format!(
                                    "Repository root: {} · Release: {}",
                                    repository.display(),
                                    if self.release.is_empty() {
                                        "(all releases)"
                                    } else {
                                        &self.release
                                    }
                                ));
                            }
                            if let Some(result) = &self.scan_result {
                                match result {
                                    Ok((summary, metrics)) => {
                                        ui.label(format!(
                                            "Latest scan: {}",
                                            format_analysis_summary(summary)
                                        ));
                                        show_dashboard_overview(ui, metrics);
                                    }
                                    Err(error) => {
                                        ui.colored_label(egui::Color32::RED, error);
                                    }
                                }
                            } else if let Some(result) = &self.metrics_result {
                                match result {
                                    Ok(metrics) => show_dashboard_overview(ui, metrics),
                                    Err(error) => {
                                        ui.colored_label(egui::Color32::RED, error);
                                    }
                                }
                            } else if self.scan_receiver.is_some() {
                                ui.label("Repository analysis is running…");
                            } else if self.metrics_receiver.is_some() {
                                ui.label("Loading stored repository metrics…");
                            } else if self.state.selected_repository().is_some() {
                                ui.label(
                                    "No metric data loaded for this repository and release yet.",
                                );
                                if ui.button("Open repository analysis").clicked() {
                                    self.state.dispatch(NavigationAction::OpenRepositories);
                                }
                            } else {
                                ui.label(
                            "Choose a repository to analyze it or load saved release metrics.",
                        );
                                if ui.button("Choose Repository Folder…").clicked() {
                                    self.open_repository_folder();
                                }
                            }
                            ui.separator();
                            self.show_admin_bridge(ui);
                            ui.separator();
                            self.show_release_baseline(ui);
                            ui.separator();
                            show_companion_insights(
                                ui,
                                &mut self.insight_payload_input,
                                &mut self.dashboard_insights,
                                &mut self.insights_audience,
                            );
                            show_companion_reference_panels(
                                ui,
                                &mut self.seo_scope,
                                &mut self.performance_data_mode,
                            );
                        }
                        ShellPage::Repositories => {
                            ui.heading("Repositories");
                            ui.label(
                                "Select a local repository folder to start a workspace session.",
                            );
                            if ui.button("Choose Repository Folder…").clicked() {
                                self.open_repository_folder();
                            }
                            if let Some(path) = self.state.selected_repository() {
                                ui.separator();
                                ui.label("Selected repository");
                                ui.monospace(path.to_string_lossy());
                                ui.horizontal(|ui| {
                                    ui.label("Release/tag:");
                                    if ui.text_edit_singleline(&mut self.release).changed() {
                                        self.clear_displayed_results();
                                    }
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Admin token:");
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.scan_token)
                                            .password(true)
                                            .hint_text("Paste an admin JWT"),
                                    );
                                });
                                let loading_metrics = self.metrics_receiver.is_some();
                                if ui
                                    .add_enabled(
                                        !loading_metrics && !self.scan_token.trim().is_empty(),
                                        egui::Button::new("Load Stored Metrics"),
                                    )
                                    .clicked()
                                {
                                    self.load_stored_metrics(path.clone(), self.scan_token.clone());
                                }
                                if loading_metrics {
                                    ui.label("Loading stored metrics…");
                                }
                                let scanning = self.scan_receiver.is_some();
                                if ui
                                    .add_enabled(
                                        !scanning && !self.scan_token.trim().is_empty(),
                                        egui::Button::new("Analyze Repository"),
                                    )
                                    .clicked()
                                {
                                    self.start_scan(path, self.scan_token.clone());
                                }
                                if scanning {
                                    ui.label("Analysis is running…");
                                }
                                if let Some(result) = &self.scan_result {
                                    ui.separator();
                                    show_scan_result(ui, result);
                                } else if let Some(result) = &self.metrics_result {
                                    ui.separator();
                                    show_metrics_result(ui, result);
                                }
                            }
                        }
                        ShellPage::Alerts => {
                            ui.heading("Alerts");
                            ui.label("Security and quality alerts will appear here.");
                        }
                        ShellPage::Settings => {
                            ui.heading("Settings");
                            ui.label("Desktop preferences will appear here.");
                        }
                    });
            });
        }

        fn save(&mut self, storage: &mut dyn eframe::Storage) {
            eframe::set_value(storage, "rocinante_shell_state", &self.state);
            if let Ok(saved) = serde_json::to_string(&self.state) {
                storage.set_string("rocinante_shell_state_json", saved.clone());
                if let Err(error) = std::fs::write(&self.persistence_path, saved) {
                    eprintln!("failed to persist shell state: {error}");
                }
            }
        }

        fn persist_egui_memory(&self) -> bool {
            true
        }
    }

    impl RocinanteApp {
        fn persist_shell_state(&mut self, frame: &mut eframe::Frame) {
            if let Some(storage) = frame.storage_mut() {
                eframe::App::save(self, storage);
                storage.flush();
            }
        }

        fn open_repository_folder(&mut self) {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                self.clear_displayed_results();
                self.state.select_repository(Some(path));
                self.state.dispatch(NavigationAction::OpenRepositories);
            }
        }

        fn clear_displayed_results(&mut self) {
            self.scan_result = None;
            self.metrics_result = None;
        }

        fn show_release_baseline(&mut self, ui: &mut egui::Ui) {
            ui.collapsing("Release baseline", |ui| {
                ui.label("Admin access is required. The repository name must match the analytics store key.");
                ui.horizontal(|ui| {
                    ui.label("Admin token:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.scan_token)
                            .password(true)
                            .hint_text("Paste an admin JWT"),
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("Repository name:");
                    ui.text_edit_singleline(&mut self.baseline_repository);
                });
                ui.horizontal(|ui| {
                    ui.label("Baseline complexity:");
                    ui.text_edit_singleline(&mut self.baseline_complexity);
                });

                let busy = self.baseline_receiver.is_some();
                let can_run = !busy
                    && !self.scan_token.trim().is_empty()
                    && !self.baseline_repository.trim().is_empty();
                let mut query = false;
                let mut reseed = false;
                ui.horizontal(|ui| {
                    query = ui
                        .add_enabled(can_run, egui::Button::new("Load baseline"))
                        .clicked();
                    reseed = ui
                        .add_enabled(can_run, egui::Button::new("Reseed baseline"))
                        .clicked();
                });

                if query {
                    self.start_baseline_query();
                } else if reseed {
                    match self.baseline_complexity.trim().parse::<f64>() {
                        Ok(value) if value.is_finite() => self.start_baseline_reseed(value),
                        _ => {
                            self.baseline_result = Some(
                                "ERR reseed_release_baseline: enter a finite numeric baseline complexity.".into(),
                            );
                        }
                    }
                }

                if busy {
                    ui.label("Release baseline operation is running…");
                }
                if let Some(result) = &self.baseline_result {
                    ui.label(result);
                }
            });
        }

        fn show_admin_bridge(&mut self, ui: &mut egui::Ui) {
            ui.collapsing("Admin command bridge", |ui| {
                ui.label("Commands require an admin JWT and use the same services and storage paths as the Tauri host.");
                ui.horizontal(|ui| {
                    ui.label("Admin token:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.scan_token)
                            .password(true)
                            .hint_text("Paste an admin JWT"),
                    );
                });

                let previous_command = self.admin_command.clone();
                egui::ComboBox::from_label("Command")
                    .selected_text(&self.admin_command)
                    .show_ui(ui, |ui| {
                        for command in rocinante_storage::ADMIN_BRIDGE_COMMANDS {
                            if matches!(command, "query_release_baseline" | "reseed_release_baseline") {
                                continue;
                            }
                            ui.selectable_value(
                                &mut self.admin_command,
                                command.to_string(),
                                command,
                            );
                        }
                    });
                if self.admin_command != previous_command {
                    self.admin_payload_input = sample_admin_payload(&self.admin_command).into();
                }
                ui.label("Command payload (JSON; omit the token):");
                ui.add(
                    egui::TextEdit::multiline(&mut self.admin_payload_input)
                        .desired_rows(6)
                        .code_editor(),
                );
                let busy = self.admin_receiver.is_some();
                if ui
                    .add_enabled(
                        !busy && !self.scan_token.trim().is_empty(),
                        egui::Button::new("Run admin command"),
                    )
                    .clicked()
                {
                    self.start_admin_command();
                }
                if busy {
                    ui.label("Admin command is running…");
                }
                if let Some(result) = &self.admin_result {
                    ui.label(result);
                }
            });
        }

        fn start_admin_command(&mut self) {
            let command = self.admin_command.clone();
            let payload = match serde_json::from_str(&self.admin_payload_input) {
                Ok(payload) => payload,
                Err(error) => {
                    self.admin_result =
                        Some(format!("ERR {command}: invalid JSON payload: {error}"));
                    return;
                }
            };
            let token = self.scan_token.clone();
            let (kv_path, columnar_path) = rocinante_storage::default_analytics_store_paths();
            let (weights_path, audit_path) = rocinante_storage::default_scoring_paths();
            let (sender, receiver) = mpsc::channel();
            self.admin_receiver = Some((command.clone(), receiver));
            self.admin_result = None;
            std::thread::spawn(move || {
                let result = rocinante_storage::execute_admin_bridge_command(
                    &command,
                    &token,
                    payload,
                    &kv_path,
                    &columnar_path,
                    &weights_path,
                    &audit_path,
                )
                .map(|value| value.to_string())
                .map_err(|error| error.to_string());
                let _ = sender.send(result);
            });
        }

        fn poll_admin_command(&mut self, context: &egui::Context) {
            let outcome = self
                .admin_receiver
                .as_ref()
                .map(|(command, receiver)| (command.clone(), receiver.try_recv()));
            match outcome {
                Some((command, Ok(Ok(message)))) => {
                    self.admin_result = Some(format!("OK {command}: {message}"));
                    self.admin_receiver = None;
                    context.request_repaint();
                }
                Some((command, Ok(Err(error)))) => {
                    self.admin_result = Some(format!("ERR {command}: {error}"));
                    self.admin_receiver = None;
                    context.request_repaint();
                }
                Some((command, Err(TryRecvError::Disconnected))) => {
                    self.admin_result =
                        Some(format!("ERR {command}: operation stopped unexpectedly."));
                    self.admin_receiver = None;
                    context.request_repaint();
                }
                Some((_, Err(TryRecvError::Empty))) | None => {}
            }
        }

        fn start_baseline_query(&mut self) {
            let token = self.scan_token.clone();
            let repo_name = self.baseline_repository.trim().to_string();
            let (sender, receiver) = mpsc::channel();
            self.baseline_receiver = Some(("query_release_baseline".into(), receiver));
            self.baseline_result = None;
            std::thread::spawn(move || {
                let (kv_path, columnar_path) = rocinante_storage::default_analytics_store_paths();
                let result = rocinante_storage::query_release_baseline(
                    &token,
                    &kv_path,
                    &columnar_path,
                    &repo_name,
                )
                .map(|value| {
                    value
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "no baseline recorded".into())
                })
                .map_err(|error| error.to_string());
                let _ = sender.send(result);
            });
        }

        fn start_baseline_reseed(&mut self, value: f64) {
            let token = self.scan_token.clone();
            let repo_name = self.baseline_repository.trim().to_string();
            let (sender, receiver) = mpsc::channel();
            self.baseline_receiver = Some(("reseed_release_baseline".into(), receiver));
            self.baseline_result = None;
            std::thread::spawn(move || {
                let (kv_path, columnar_path) = rocinante_storage::default_analytics_store_paths();
                let result = rocinante_storage::reseed_release_baseline(
                    &token,
                    &kv_path,
                    &columnar_path,
                    &repo_name,
                    value,
                )
                .map(|value| value.to_string())
                .map_err(|error| error.to_string());
                let _ = sender.send(result);
            });
        }

        fn poll_baseline(&mut self, context: &egui::Context) {
            let outcome = self
                .baseline_receiver
                .as_ref()
                .map(|(command, receiver)| (command.clone(), receiver.try_recv()));
            match outcome {
                Some((command, Ok(Ok(message)))) => {
                    self.baseline_result = Some(format!("OK {command}: {message}"));
                    self.baseline_receiver = None;
                    context.request_repaint();
                }
                Some((command, Ok(Err(error)))) => {
                    self.baseline_result = Some(format!("ERR {command}: {error}"));
                    self.baseline_receiver = None;
                    context.request_repaint();
                }
                Some((command, Err(TryRecvError::Disconnected))) => {
                    self.baseline_result =
                        Some(format!("ERR {command}: operation stopped unexpectedly."));
                    self.baseline_receiver = None;
                    context.request_repaint();
                }
                Some((_, Err(TryRecvError::Empty))) | None => {}
            }
        }

        fn poll_deep_link_inbox(&mut self, context: &egui::Context) {
            while let Ok(link) = self.url_event_receiver.try_recv() {
                if let Some(target) = parse_deep_link(&link) {
                    let DeepLinkTarget::OpenRepository(path) = &target;
                    if deep_link_changes_repository(
                        self.state.selected_repository().as_deref(),
                        path,
                    ) {
                        self.clear_displayed_results();
                    }
                    self.state.apply_deep_link(target);
                    record_acceptance_state(&self.state, self.tray_icon.is_some());
                    context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    context.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    context.send_viewport_cmd(egui::ViewportCommand::Focus);
                }
            }
            let events = self.link_inbox.drain();
            if events.is_empty() {
                return;
            }
            context.send_viewport_cmd(egui::ViewportCommand::Visible(true));
            context.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
            context.send_viewport_cmd(egui::ViewportCommand::Focus);
            if cfg!(target_os = "linux") && std::env::var_os("WAYLAND_DISPLAY").is_some() {
                context.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                    egui::UserAttentionType::Informational,
                ));
            }
            for event in events {
                match event {
                    DeepLinkEvent::Activate => {}
                    DeepLinkEvent::OpenRepository(path) => {
                        if deep_link_changes_repository(
                            self.state.selected_repository().as_deref(),
                            &path,
                        ) {
                            self.clear_displayed_results();
                        }
                        self.state.select_repository(Some(path));
                        self.state.dispatch(NavigationAction::OpenRepositories);
                        record_acceptance_state(&self.state, self.tray_icon.is_some());
                    }
                }
            }
        }

        fn start_scan(&mut self, root: PathBuf, token: String) {
            let release = self.release.clone();
            let (sender, receiver) = mpsc::channel();
            self.scan_receiver = Some((root.clone(), release.clone(), receiver));
            self.clear_displayed_results();
            std::thread::spawn(move || {
                let result = rocinante_analysis::resolve_telemetry_db_path()
                    .map_err(|error| error.to_string())
                    .and_then(|database| {
                        rocinante_analysis::run_scan_with_metrics(
                            &token, &root, &release, &database,
                        )
                        .map_err(|error| error.to_string())
                    });
                let _ = sender.send(result);
            });
        }

        fn load_stored_metrics(&mut self, root: PathBuf, token: String) {
            let release = self.release.clone();
            let (sender, receiver) = mpsc::channel();
            self.metrics_receiver = Some((root.clone(), release.clone(), receiver));
            self.clear_displayed_results();
            std::thread::spawn(move || {
                let result = rocinante_analysis::resolve_telemetry_db_path()
                    .map_err(|error| error.to_string())
                    .and_then(|database| {
                        rocinante_analysis::query_repository_metrics(
                            &token, &root, &release, &database,
                        )
                        .map_err(|error| error.to_string())
                    });
                let _ = sender.send(result);
            });
        }

        fn poll_scan(&mut self, context: &egui::Context) {
            let Some((root, release, receiver)) = &self.scan_receiver else {
                return;
            };
            let query_is_current =
                self.state.selected_repository().as_ref() == Some(root) && self.release == *release;
            match receiver.try_recv() {
                Ok(result) => {
                    if query_is_current {
                        notify_scan_result(&result);
                        self.scan_result = Some(result);
                    }
                    self.scan_receiver = None;
                    context.request_repaint();
                }
                Err(TryRecvError::Disconnected) => {
                    if query_is_current {
                        self.scan_result =
                            Some(Err("Repository analysis stopped unexpectedly.".into()));
                    }
                    self.scan_receiver = None;
                    context.request_repaint();
                }
                Err(TryRecvError::Empty) => {}
            }
        }

        fn poll_metrics(&mut self, context: &egui::Context) {
            let Some((root, release, receiver)) = &self.metrics_receiver else {
                return;
            };
            let query_is_current =
                self.state.selected_repository().as_ref() == Some(root) && self.release == *release;
            match receiver.try_recv() {
                Ok(result) => {
                    if query_is_current {
                        self.metrics_result = Some(result);
                    }
                    self.metrics_receiver = None;
                    context.request_repaint();
                }
                Err(TryRecvError::Disconnected) => {
                    if query_is_current {
                        self.metrics_result =
                            Some(Err("Stored metric query stopped unexpectedly.".into()));
                    }
                    self.metrics_receiver = None;
                    context.request_repaint();
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    }

    fn notify_scan_result(result: &ScanOutcome) {
        let body = match result {
            Ok((summary, _)) => format!(
                "Analysis complete for {} repositories.",
                summary.records_processed
            ),
            Err(_) => "Repository analysis failed. See Rocinante for details.".to_string(),
        };
        send_desktop_notification("Rocinante analysis", body);
    }

    fn sample_admin_payload(command: &str) -> &'static str {
        match command {
            "ingest_event" => {
                r#"{"event":{"commit_id":"ui-bridge-001","repo_name":"sample-repo","release":"v1.0.0","committer":"ui","telemetry":[{"plugin":"ui","metric_key":"bridge_probe","metric_value":1,"details":"admin bridge probe"}]}}"#
            }
            "promote_lifecycle" => "{}",
            "query_aggregates" | "committer_scores" => {
                r#"{"name":"sample-repo","release":"v1.0.0"}"#
            }
            "rank_prs" => {
                r#"{"prs":[{"pr_id":"pr-001","repo_name":"sample-repo","author":"ui","release":"v1.0.0","file_risk":0.4,"author_velocity":0.6,"approval_fidelity":0.9,"files":[{"path":"src/ui-bridge.ts","risk":0.72}],"circuit_breaker_triggered":true}]}"#
            }
            "evaluate_pr_risk" => {
                r#"{"candidate":{"pr_id":"pr-001","repo_name":"sample-repo","author":"ui","release":"v1.0.0","file_risk":0.4,"author_velocity":0.6,"approval_fidelity":0.9,"files":[{"path":"src/ui-bridge.ts","risk":0.72}],"circuit_breaker_triggered":true}}"#
            }
            "query_release_baseline" => r#"{"repoName":"sample-repo"}"#,
            "reseed_release_baseline" => r#"{"repoName":"sample-repo","baselineComplexity":18.5}"#,
            "update_scoring_weights" => {
                r#"{"weights":{"version":"v1","complexity_weight":0.3,"coverage_weight":0.25,"churn_weight":0.2,"pipeline_weight":0.25,"pr_file_risk_weight":0.5,"pr_velocity_weight":0.2,"pr_approval_weight":0.3}}"#
            }
            _ => "{}",
        }
    }

    #[cfg(feature = "acceptance-witness")]
    fn notify_acceptance_surface_if_requested() {
        if option_env!("ROCINANTE_ACCEPTANCE_NOTIFICATION").is_some() {
            send_desktop_notification(
                "Rocinante notification acceptance",
                "The native shell delivered its acceptance notification.".to_string(),
            );
        }
    }

    fn send_desktop_notification(summary: &'static str, body: String) {
        let _ = std::thread::Builder::new()
            .name("rocinante-notification".into())
            .spawn(move || {
                let result = notify_rust::Notification::new()
                    .summary(summary)
                    .body(&body)
                    .show();
                #[cfg(feature = "acceptance-witness")]
                if let Some(result_path) = option_env!("ROCINANTE_ACCEPTANCE_NOTIFICATION_RESULT") {
                    let _ = std::fs::write(
                        result_path,
                        format!("request_succeeded={}\n", result.is_ok()),
                    );
                }
                #[cfg(not(feature = "acceptance-witness"))]
                let _ = result;
            });
    }

    fn show_companion_insights(
        ui: &mut egui::Ui,
        payload_input: &mut String,
        applied: &mut AppliedInsights,
        audience: &mut StakeholderAudience,
    ) {
        ui.collapsing("Optimization insights", |ui| {
            ui.label(match applied.insights.source {
                InsightSource::Sample => "Showing the companion's built-in sample data.",
                InsightSource::Payload => {
                    "Showing the supplied payload; missing or empty sections use sample data."
                }
            });
            ui.label("Paste a commits/stages/signals JSON payload to calculate the companion insight view.");
            ui.add(
                egui::TextEdit::multiline(payload_input)
                    .desired_rows(4)
                    .hint_text("{ \"commits\": [...], \"stages\": [...], \"signals\": [...] }"),
            );
            ui.horizontal(|ui| {
                if ui.button("Apply payload").clicked() {
                    applied.apply_json(payload_input);
                }
                if ui.button("Reset to sample").clicked() {
                    payload_input.clear();
                    applied.reset();
                }
            });
            ui.horizontal_wrapped(|ui| {
                for option in StakeholderAudience::ALL {
                    ui.selectable_value(audience, option, option.label());
                }
            });
            ui.label(audience.tone());
            let pulse = build_quality_pulse(&applied.insights);
            ui.separator();
            ui.heading("Quality snapshot");
            ui.label(format!(
                "Commit risk cards: {} · High-risk commits: {} · Critical bottlenecks: {} critical, {} high · Actionable opportunities: {}",
                applied.insights.commit_risk_cards.len(),
                pulse.risk_buckets.high,
                pulse.bottleneck_buckets.critical,
                pulse.bottleneck_buckets.high,
                applied.insights.opportunities.len()
            ));
            if let Some(error) = &applied.error {
                ui.colored_label(egui::Color32::RED, error);
            }

            ui.heading("Commit risk");
            for risk in &applied.insights.commit_risk_cards {
                let color = match risk.level {
                    RiskLevel::High => egui::Color32::RED,
                    RiskLevel::Medium => egui::Color32::YELLOW,
                    RiskLevel::Good => egui::Color32::GREEN,
                };
                ui.colored_label(
                    color,
                    format!("{} · {} · {}/100", risk.id, risk.level.label(), risk.score),
                );
                if !risk.reasons.is_empty() {
                    ui.label(risk.reasons.join(", "));
                }
            }

            ui.heading("Stage bottlenecks");
            for bottleneck in &applied.insights.bottlenecks {
                let color = match bottleneck.status {
                    BottleneckStatus::Critical | BottleneckStatus::High => egui::Color32::YELLOW,
                    BottleneckStatus::Good => egui::Color32::GREEN,
                };
                ui.colored_label(
                    color,
                    format!("{} · {} · impact {:.0}", bottleneck.name, bottleneck.status.label(), bottleneck.impact),
                );
                ui.label(&bottleneck.rationale);
            }

            ui.heading("Opportunities");
            for opportunity in &applied.insights.opportunities {
                ui.label(format!(
                    "{} · {} · priority {}/100",
                    opportunity.id, opportunity.title, opportunity.priority_score
                ));
            }

            let route = &pulse.action_routes[audience];
            ui.separator();
            ui.heading(format!("Quality pulse · {}", audience.label()));
            ui.label(format!(
                "Score {}/100 · {} security-sensitive commits · top bottleneck: {} · {} opportunities",
                pulse.overall_score,
                pulse.security_signal_count,
                pulse.top_bottleneck_name,
                pulse.opportunity_count
            ));
            ui.label(format!(
                "Risk: {} high, {} medium, {} good · stages: {} critical, {} high, {} good",
                pulse.risk_buckets.high,
                pulse.risk_buckets.medium,
                pulse.risk_buckets.good,
                pulse.bottleneck_buckets.critical,
                pulse.bottleneck_buckets.high,
                pulse.bottleneck_buckets.good
            ));
            ui.strong("Recommendations");
            for recommendation in &pulse.recommendations[audience] {
                let color = match recommendation.severity {
                    PulseSeverity::Bad => egui::Color32::RED,
                    PulseSeverity::Medium => egui::Color32::YELLOW,
                    PulseSeverity::Good => egui::Color32::GREEN,
                };
                ui.colored_label(
                    color,
                    format!("{} · {}", recommendation.severity.label(), recommendation.message),
                );
            }
            ui.strong("Action routing");
            ui.label(format!("Owner: {} · Window: {}", route.owner, route.window));
            for action in &route.actions {
                ui.label(format!("• {action}"));
            }

            ui.separator();
            match audience {
                StakeholderAudience::Lead => {
                    ui.heading("Team Lead Focus");
                    ui.label(audience.guidance());
                    ui.strong("Top Commit Risks");
                    for risk in applied.insights.commit_risk_cards.iter().take(3) {
                        ui.colored_label(
                            match risk.level {
                                RiskLevel::High => egui::Color32::RED,
                                RiskLevel::Medium => egui::Color32::YELLOW,
                                RiskLevel::Good => egui::Color32::GREEN,
                            },
                            format!("{} score {} ({})", risk.id, risk.score, risk.level.label()),
                        );
                    }
                }
                StakeholderAudience::Manager => {
                    ui.heading("Manager Focus");
                    ui.label(audience.guidance());
                    ui.strong("Bottleneck Radar");
                    for item in &applied.insights.bottlenecks {
                        ui.label(format!(
                            "{} ({}) impact {:.0} · {}",
                            item.name,
                            item.status.label(),
                            item.impact,
                            item.rationale
                        ));
                    }
                }
                StakeholderAudience::Executive => {
                    ui.heading("Executive Focus");
                    ui.label(audience.guidance());
                    ui.strong("Top Improvement Opportunities");
                    for item in applied.insights.opportunities.iter().take(2) {
                        ui.label(format!(
                            "{} (score {})",
                            item.title, item.priority_score
                        ));
                    }
                }
                StakeholderAudience::Security => {
                    ui.heading("Security Focus");
                    ui.label(audience.guidance());
                    ui.strong("Security-Weighted Commit Signals");
                    let security_risks = applied
                        .insights
                        .commit_risk_cards
                        .iter()
                        .filter(|risk| {
                            risk.reasons.iter().any(|reason| {
                                reason == "Dependency risk" || reason == "Automation failures"
                            })
                        })
                        .collect::<Vec<_>>();
                    if security_risks.is_empty() {
                        ui.label("No critical security signals in sample window");
                    } else {
                        for risk in security_risks {
                            ui.colored_label(
                                match risk.level {
                                    RiskLevel::High => egui::Color32::RED,
                                    RiskLevel::Medium => egui::Color32::YELLOW,
                                    RiskLevel::Good => egui::Color32::GREEN,
                                },
                                format!("{}: {}", risk.id, risk.reasons.join(", ")),
                            );
                        }
                    }
                }
            }

            let traces = build_explainability_traces(&pulse);
            ui.separator();
            ui.heading("Explainability traces");
            for trace in traces {
                ui.colored_label(
                    tone_color(trace.tone),
                    format!("{} · {}", trace.title, trace.summary),
                );
                ui.label(trace.detail);
            }

            let visuals = build_dashboard_visuals(&applied.insights);
            ui.separator();
            ui.heading("Trend and PR risk");
            ui.label(&visuals.summary);
            for trend in visuals.trend_lines {
                ui.colored_label(
                    tone_color(trend.tone),
                    format!("{} · {}", trend.label, trend.value),
                );
                ui.label(trend.rationale);
            }
            for risk in visuals.pr_risk_rankings {
                ui.colored_label(
                    tone_color(risk.tone),
                    format!("{} · {}", risk.title, risk.rationale),
                );
            }

            ui.separator();
            ui.heading("Job observability");
            ui.label(format!(
                "Observed stages: {}",
                applied.insights.stages.len()
            ));
            for stage in &applied.insights.stages {
                let tone = if stage.queue_depth >= 10.0 || stage.avg_latency_ms >= 2_000.0 {
                    VisualTone::Bad
                } else if stage.queue_depth >= 4.0 || stage.avg_latency_ms >= 1_000.0 {
                    VisualTone::Medium
                } else {
                    VisualTone::Good
                };
                ui.colored_label(
                    tone_color(tone),
                    format!(
                        "{} · queue {} · throughput {} · lag {}ms",
                        stage.name, stage.queue_depth, stage.throughput, stage.avg_latency_ms
                    ),
                );
            }
        });
    }

    fn show_companion_reference_panels(
        ui: &mut egui::Ui,
        seo_scope: &mut SeoScope,
        performance_data_mode: &mut PerformanceDataMode,
    ) {
        ui.group(|ui| {
            ui.heading("Site audits and reference metrics");
            ui.label("Showing static sample audit data; these panels are not repository scan results.");
            ui.heading("WCAG 2.1/2.2 AA Accessibility Audit");
            ui.add(
                egui::ProgressBar::new(ACCESSIBILITY_SCORE as f32 / 100.0)
                    .text(format!("Overall Score {ACCESSIBILITY_SCORE}/100")),
            );
            let _ = ui.button("Run Full Audit");
            show_reference_findings(ui, "Findings", &ACCESSIBILITY_FINDINGS);

            ui.separator();
            ui.heading("SEO, GEO & AEO Performance");
            ui.horizontal(|ui| {
                for scope in [SeoScope::CurrentPage, SeoScope::SiteWide] {
                    ui.selectable_value(seo_scope, scope, scope.label());
                }
            });
            ui.label(format!("On-Page SEO: {ON_PAGE_SEO_SCORE}/100"));
            ui.label(format!("Schema Markup: Found ({SCHEMA_ENTITY_COUNT} entities)"));
            ui.label(format!(
                "Answer Engine Optimization: {ANSWER_ENGINE_OPTIMIZATION_SCORE}/100 ({ANSWER_ENGINE_OPTIMIZATION_GUIDANCE})"
            ));
            ui.label(format!("Geographic SEO: {GEOGRAPHIC_SEO_STATUS}"));
            show_reference_findings(
                ui,
                &format!("Example guidance ({})", seo_scope.label()),
                &SEO_FINDINGS,
            );

            ui.separator();
            ui.heading("Security & Drupal Review");
            ui.colored_label(egui::Color32::GREEN, DRUPAL_SECURITY_STATUS);
            show_reference_findings(ui, "Drupal-Specific Checks", &SECURITY_FINDINGS);
            ui.label(format!("Recommendation: {DRUPAL_RECOMMENDATION}"));

            ui.separator();
            ui.heading("Page Performance Metrics");
            ui.add(
                egui::ProgressBar::new(PERFORMANCE_SCORE as f32 / 100.0)
                    .text(format!("Overall Score {PERFORMANCE_SCORE}/100")),
            );
            show_reference_findings(ui, "Top Recommendations", &PERFORMANCE_FINDINGS);
            ui.horizontal(|ui| {
                ui.selectable_value(
                    performance_data_mode,
                    PerformanceDataMode::Field,
                    "Field Data",
                );
                ui.selectable_value(
                    performance_data_mode,
                    PerformanceDataMode::Lab,
                    "Lab Data",
                );
            });
        });
    }

    fn show_reference_findings(ui: &mut egui::Ui, title: &str, findings: &[Finding]) {
        ui.strong(title);
        for finding in findings {
            let (label, color) = match finding.status {
                FindingStatus::Good => ("good", egui::Color32::GREEN),
                FindingStatus::Medium => ("medium", egui::Color32::YELLOW),
                FindingStatus::Bad => ("bad", egui::Color32::RED),
            };
            ui.colored_label(color, format!("{label} · {}", finding.text));
        }
    }

    fn tone_color(tone: VisualTone) -> egui::Color32 {
        match tone {
            VisualTone::Bad => egui::Color32::RED,
            VisualTone::Medium => egui::Color32::YELLOW,
            VisualTone::Good => egui::Color32::GREEN,
        }
    }

    fn show_scan_result(ui: &mut egui::Ui, result: &ScanOutcome) {
        match result {
            Ok((summary, metrics)) => {
                ui.label(format!(
                    "Analysis complete: {}",
                    format_analysis_summary(summary)
                ));
                show_repository_metrics(ui, metrics);
            }
            Err(error) => {
                ui.colored_label(egui::Color32::RED, error);
            }
        }
    }

    fn show_metrics_result(ui: &mut egui::Ui, result: &MetricsOutcome) {
        match result {
            Ok(metrics) => show_repository_metrics(ui, metrics),
            Err(error) => {
                ui.colored_label(egui::Color32::RED, error);
            }
        }
    }

    fn show_repository_metrics(
        ui: &mut egui::Ui,
        metrics: &[rocinante_analysis::types::RepositoryMetric],
    ) {
        ui.heading("Repository metrics");
        show_metric_table(ui, &DashboardViewModel::from_metrics(metrics));
    }

    fn show_dashboard_overview(
        ui: &mut egui::Ui,
        metrics: &[rocinante_analysis::types::RepositoryMetric],
    ) {
        let view = DashboardViewModel::from_metrics(metrics);
        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label("Repositories");
                ui.heading(view.repository_count.to_string());
            });
            ui.group(|ui| {
                ui.label("Metrics");
                ui.heading(view.metric_count.to_string());
            });
            ui.group(|ui| {
                ui.label("Analyzers");
                ui.heading(view.plugins.len().to_string());
            });
        });
        if !view.plugins.is_empty() {
            ui.heading("Metrics by analyzer");
            egui::Grid::new("dashboard_plugin_summary")
                .striped(true)
                .show(ui, |ui| {
                    ui.strong("Analyzer");
                    ui.strong("Metrics");
                    ui.end_row();
                    for plugin in &view.plugins {
                        ui.label(&plugin.name);
                        ui.label(plugin.metric_count.to_string());
                        ui.end_row();
                    }
                });
        }
        ui.separator();
        show_metric_table(ui, &view);
    }

    fn show_metric_table(ui: &mut egui::Ui, view: &DashboardViewModel) {
        if view.rows.is_empty() {
            ui.label("No metrics are stored for this repository and release.");
            return;
        }
        egui::ScrollArea::vertical()
            .max_height(420.0)
            .show(ui, |ui| {
                egui::Grid::new("repository_metrics")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.strong("Plugin");
                        ui.strong("Repository");
                        ui.strong("Release");
                        ui.strong("Metric");
                        ui.strong("Value");
                        ui.strong("Details");
                        ui.end_row();
                        for metric in &view.rows {
                            ui.label(&metric.plugin);
                            ui.label(&metric.repo_name);
                            ui.label(&metric.release);
                            ui.label(&metric.key);
                            ui.label(format!("{:.3}", metric.value));
                            ui.label(&metric.details);
                            ui.end_row();
                        }
                    });
            });
    }

    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let initial_links = std::env::args()
            .skip(1)
            .filter(|argument| parse_deep_link(argument).is_some())
            .collect::<Vec<_>>();
        let data_dir = if let Some(path) = option_env!("ROCINANTE_ACCEPTANCE_DATA_DIR") {
            PathBuf::from(path)
        } else {
            rocinante_analysis::default_telemetry_db_path()
                .parent()
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "telemetry database path has no parent directory",
                    )
                })?
                .to_path_buf()
        };
        let link_inbox = match DeepLinkInbox::open(&data_dir) {
            Ok(link_inbox) => link_inbox,
            Err(error) => {
                #[cfg(feature = "acceptance-witness")]
                if let Some(witness) = option_env!("ROCINANTE_ACCEPTANCE_FORWARD_WITNESS") {
                    let _ = std::fs::write(
                        witness,
                        format!(
                            "pid={}\nphase=open-inbox\nlinks={initial_links:?}\nerror={error}\n",
                            std::process::id()
                        ),
                    );
                }
                return Err(error.into());
            }
        };
        let persistence_path = data_dir.join("shell-state.json");
        let restored_state = std::fs::read(&persistence_path)
            .ok()
            .and_then(|saved| serde_json::from_slice::<ShellState>(&saved).ok());
        #[cfg(feature = "acceptance-witness")]
        if link_inbox.is_primary() && !initial_links.is_empty() {
            if let Some(witness) = option_env!("ROCINANTE_ACCEPTANCE_FORWARD_WITNESS") {
                let _ = std::fs::write(
                    witness,
                    format!(
                        "pid={}\nrole=primary\nlinks={initial_links:?}\n",
                        std::process::id()
                    ),
                );
            }
        }
        if !link_inbox.is_primary() {
            let forward_result = if initial_links.is_empty() {
                link_inbox.activate()
            } else {
                initial_links
                    .iter()
                    .try_for_each(|link| link_inbox.forward(link))
            };
            #[cfg(feature = "acceptance-witness")]
            if let Some(witness) = option_env!("ROCINANTE_ACCEPTANCE_FORWARD_WITNESS") {
                let snapshot = format!(
                    "pid={}\nlinks={:?}\nresult={:?}\n",
                    std::process::id(),
                    initial_links,
                    forward_result.as_ref().map_err(ToString::to_string)
                );
                let _ = std::fs::write(witness, snapshot);
            }
            forward_result?;
            return Ok(());
        }

        let profile = WindowProfile::default();
        #[cfg(target_os = "macos")]
        let (url_event_sender, url_event_receiver) = mpsc::channel();
        #[cfg(not(target_os = "macos"))]
        let (_url_event_sender, url_event_receiver) = mpsc::channel();
        #[cfg(target_os = "macos")]
        let repaint_context = std::sync::Arc::new(std::sync::Mutex::new(None));
        let icon =
            eframe::icon_data::from_png_bytes(include_bytes!("../packaging/icons/rocinante.png"))
                .expect("bundled application icon must be a valid PNG");
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_title(profile.title)
                .with_inner_size([profile.width as f32, profile.height as f32])
                .with_resizable(profile.resizable)
                .with_icon(icon),
            ..Default::default()
        };
        #[cfg(feature = "acceptance-witness")]
        let options = {
            let mut options = options;
            if let Some(path) = option_env!("ROCINANTE_ACCEPTANCE_DATA_DIR") {
                options.persistence_path = Some(PathBuf::from(path).join("shell-state.ron"));
            }
            options
        };
        #[cfg(target_os = "macos")]
        let url_event_handler =
            super::macos_url::register(url_event_sender, repaint_context.clone())
                .map_err(|error| std::io::Error::other(error.to_string()))?;
        eframe::run_native(
            "Rocinante Repo Analyzer",
            options,
            Box::new(move |creation_context| {
                Ok(Box::new(RocinanteApp::new(
                    creation_context,
                    link_inbox,
                    persistence_path,
                    restored_state,
                    initial_links,
                    url_event_receiver,
                    #[cfg(target_os = "macos")]
                    MacosStartup {
                        url_event_handler,
                        repaint_context,
                    },
                )))
            }),
        )?;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::{
            apply_tray_menu_action, deep_link_changes_repository, tray_menu_action, DeepLinkTarget,
            ShellState, TrayMenuAction,
        };
        use std::path::{Path, PathBuf};

        #[test]
        fn bundled_native_window_icon_is_a_decodable_square_png() {
            let icon = eframe::icon_data::from_png_bytes(include_bytes!(
                "../packaging/icons/rocinante.png"
            ))
            .expect("decode bundled native window icon");

            assert_eq!((icon.width, icon.height), (512, 512));
        }

        #[test]
        fn deep_link_resets_results_only_when_repository_changes() {
            let selected = PathBuf::from("/workspace/one");
            assert!(!deep_link_changes_repository(
                Some(selected.as_path()),
                selected.as_path()
            ));
            assert!(deep_link_changes_repository(
                Some(selected.as_path()),
                Path::new("/workspace/two")
            ));
            assert!(deep_link_changes_repository(None, selected.as_path()));
        }

        #[test]
        fn saved_shell_json_restores_repository_selection() {
            let repository = PathBuf::from("/tmp/rocinante-saved-repository");
            let mut state = ShellState::default();
            state.apply_deep_link(DeepLinkTarget::OpenRepository(repository.clone()));

            let saved = serde_json::to_vec(&state).expect("serialize shell state");
            let restored: ShellState = serde_json::from_slice(&saved).expect("restore shell state");

            assert_eq!(restored.page(), super::ShellPage::Repositories);
            assert_eq!(restored.selected_repository(), Some(repository));
        }

        #[test]
        fn tray_menu_routes_show_quit_and_ignores_unknown_actions() {
            assert_eq!(tray_menu_action("show"), Some(TrayMenuAction::Show));
            assert_eq!(tray_menu_action("quit"), Some(TrayMenuAction::Quit));
            assert_eq!(tray_menu_action("unknown"), None);

            let context = eframe::egui::Context::default();
            assert!(!apply_tray_menu_action(TrayMenuAction::Show, &context));
            assert!(apply_tray_menu_action(TrayMenuAction::Quit, &context));
        }

        #[test]
        fn every_admin_bridge_command_has_a_valid_sample_payload() {
            for command in rocinante_storage::ADMIN_BRIDGE_COMMANDS {
                let payload: serde_json::Value =
                    serde_json::from_str(super::sample_admin_payload(command))
                        .unwrap_or_else(|error| panic!("{command} sample payload: {error}"));
                assert!(payload.is_object(), "{command} sample payload is an object");
            }
        }
    }
}

#[cfg(feature = "native-ui")]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    native_ui::run()
}
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::ffi::{OsStrExt, OsStringExt};
#[cfg(windows)]
use std::os::windows::ffi::{OsStrExt, OsStringExt};
