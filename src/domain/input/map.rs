use super::action::{ActiveScopes, InputAction};
use super::trigger::{KeyId, Mods, MouseSpec, Trigger, binding_label, parse_binding};

#[derive(Debug, Clone)]
pub struct InputMap {
    bindings: Vec<(InputAction, Vec<Trigger>)>,
}

fn built_in(action: InputAction) -> Vec<Trigger> {
    parse_binding(action.default_binding()).expect("built-in binding must be valid")
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            bindings: InputAction::ALL
                .into_iter()
                .map(|action| (action, built_in(action)))
                .collect(),
        }
    }
}

impl InputMap {
    pub fn from_bindings(bindings: impl IntoIterator<Item = (InputAction, String)>) -> Self {
        let mut map = Self::default();
        for (action, raw) in bindings {
            let triggers = parse_binding(&raw).unwrap_or_else(|| built_in(action));
            if let Some(slot) = map.bindings.iter_mut().find(|(existing, _)| *existing == action) {
                slot.1 = triggers;
            }
        }
        map
    }

    pub fn triggers(&self, action: InputAction) -> &[Trigger] {
        self.bindings
            .iter()
            .find(|(candidate, _)| *candidate == action)
            .map_or(&[], |(_, triggers)| triggers.as_slice())
    }

    pub fn label(&self, action: InputAction) -> String {
        binding_label(self.triggers(action))
    }

    fn active(&self, scopes: ActiveScopes) -> impl Iterator<Item = &(InputAction, Vec<Trigger>)> {
        self.bindings.iter().filter(move |(action, _)| scopes.contains(action.scope()))
    }

    pub fn lookup_key(&self, id: &KeyId, mods: Mods, scopes: ActiveScopes) -> Option<InputAction> {
        let exact = self.active(scopes).find(|(_, triggers)| {
            triggers.iter().any(|trigger| {
                matches!(trigger, Trigger::Key(spec) if spec.id == *id && spec.mods == mods)
            })
        });
        if let Some((action, _)) = exact {
            return Some(*action);
        }
        if !mods.shift {
            return None;
        }
        let relaxed = Mods { shift: false, ..mods };
        self.active(scopes)
            .find(|(_, triggers)| {
                triggers.iter().any(|trigger| {
                    matches!(trigger, Trigger::Key(spec)
                        if spec.id == *id
                            && spec.mods == relaxed
                            && matches!(spec.id, KeyId::Char(_)))
                })
            })
            .map(|(action, _)| *action)
    }

    pub fn lookup_mouse(&self, spec: MouseSpec, scopes: ActiveScopes) -> Option<InputAction> {
        self.active(scopes)
            .find(|(_, triggers)| triggers.contains(&Trigger::Mouse(spec)))
            .map(|(action, _)| *action)
    }

    pub fn conflicts(&self) -> Vec<(InputAction, InputAction)> {
        let mut conflicts = Vec::new();
        for (index, (left, left_triggers)) in self.bindings.iter().enumerate() {
            for (right, right_triggers) in self.bindings.iter().skip(index + 1) {
                if left.scope().overlaps(right.scope())
                    && left_triggers.iter().any(|trigger| right_triggers.contains(trigger))
                {
                    conflicts.push((*left, *right));
                }
            }
        }
        conflicts
    }

    pub fn trigger_holders(
        &self,
        action: InputAction,
        trigger: &Trigger,
    ) -> impl Iterator<Item = InputAction> {
        self.bindings
            .iter()
            .filter(move |(candidate, triggers)| {
                *candidate != action
                    && candidate.scope().overlaps(action.scope())
                    && triggers.contains(trigger)
            })
            .map(|(candidate, _)| *candidate)
    }
}
