/// Recursively defined counter for summing up the application of a patch
#[derive(Default, Debug, Clone)]
pub struct PatchResult {
  /// Counters for successful actions
  stats: PatchStat,
  /// A map of errors that occurred during application, keyed on a dot separated target name
  errors: HashMap<String, Vec<ProteanError>>,
}

impl PatchResult {
  pub fn from_patch(patch: Patch) {
    todo!("Analyze a patch for its stats before application")
  }
}

/// A counter for actions taken
#[derive(Default, Debug, Clone)]
pub struct PatchStat {
  target: String,
  action: String,
  count: usize,
  children: HashMap<String, Box<PatchStat>>,
}
