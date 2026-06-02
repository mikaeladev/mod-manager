use egui::{Atoms, IntoAtoms, Response, Ui, Widget};

pub struct TabButtons<'a, 'v, V> {
  value: &'v mut V,
  buttons: Vec<(Atoms<'a>, V)>,
}

impl<'a, 'v, V> TabButtons<'a, 'v, V> {
  #[inline]
  pub fn new(value: &'v mut V) -> TabButtons<'a, 'v, V> {
    Self {
      value,
      buttons: Vec::new(),
    }
  }

  pub fn add_button(&mut self, atoms: impl IntoAtoms<'a>, value: V) -> &Self {
    self.buttons.push((atoms.into_atoms(), value));
    self
  }
}

impl<V> Widget for TabButtons<'_, '_, V> {
  fn ui(self, ui: &mut Ui) -> Response {
    ui.horizontal(|ui| {
      for (atoms, value) in self.buttons {
        if ui.button(atoms).clicked() {
          *self.value = value;
        }
      }
    })
    .response
  }
}

pub trait TabButtonsUiExt {
  fn tab_buttons<'a, 'v, V, A: IntoAtoms<'a>>(
    &mut self,
    value: &'v mut V,
    buttons: impl Into<Vec<(A, V)>>,
  ) -> Response;
}

impl TabButtonsUiExt for Ui {
  fn tab_buttons<'a, 'v, V, A: IntoAtoms<'a>>(
    &mut self,
    value: &'v mut V,
    buttons: impl Into<Vec<(A, V)>>,
  ) -> Response {
    let mut tab_buttons = TabButtons::new(value);

    for (atoms, value) in buttons.into() {
      tab_buttons.add_button(atoms, value);
    }

    tab_buttons.ui(self)
  }
}
