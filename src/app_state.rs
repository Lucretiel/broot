use crate::{
    app_context::AppContext,
    browser_states::BrowserState,
    commands::Command,
    errors::{ProgramError, TreeBuildError},
    external::Launchable,
    screens::Screen,
    task_sync::TaskLifetime,
};
use std::io::Write;

/// Result of applying a command to a state
pub enum AppStateCmdResult {
    Quit,
    Keep,
    Launch(Box<Launchable>),
    DisplayError(String),
    NewState(Box<dyn AppState>, Command),
    PopStateAndReapply, // the state asks the command be executed on a previous state
    PopState,
    RefreshState { clear_cache: bool },
}

impl AppStateCmdResult {
    pub fn verb_not_found(text: &str) -> AppStateCmdResult {
        AppStateCmdResult::DisplayError(format!("verb not found: {:?}", &text))
    }
    pub fn from_optional_state(
        os: Result<Option<BrowserState>, TreeBuildError>,
        cmd: Command,
    ) -> AppStateCmdResult {
        match os {
            Ok(Some(os)) => AppStateCmdResult::NewState(Box::new(os), cmd),
            Ok(None) => AppStateCmdResult::Keep,
            Err(e) => AppStateCmdResult::DisplayError(e.to_string()),
        }
    }
}

impl From<Launchable> for AppStateCmdResult {
    fn from(launchable: Launchable) -> Self {
        AppStateCmdResult::Launch(Box::new(launchable))
    }
}

/// a whole application state, stackable to allow reverting
///  to a previous one
pub trait AppState {
    fn apply(
        &mut self,
        cmd: &mut Command,
        screen: &mut Screen,
        con: &AppContext,
    ) -> Result<AppStateCmdResult, ProgramError>;

    fn can_execute(&self, verb_index: usize, con: &AppContext) -> bool;

    fn refresh(&mut self, screen: &Screen, con: &AppContext) -> Command;

    fn do_pending_task(&mut self, screen: &mut Screen, tl: &TaskLifetime);

    fn has_pending_task(&self) -> bool;

    fn display(
        &mut self,
        w: &mut dyn Write,
        screen: &Screen,
        con: &AppContext,
    ) -> Result<(), ProgramError>;

    fn write_flags(
        &self,
        w: &mut dyn Write,
        screen: &mut Screen,
        con: &AppContext,
    ) -> Result<(), ProgramError>;

    fn write_status(
        &self,
        w: &mut dyn Write,
        cmd: &Command,
        screen: &Screen,
        con: &AppContext,
    ) -> Result<(), ProgramError>;
}
