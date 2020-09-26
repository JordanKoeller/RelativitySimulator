use std::ffi::CString;

pub fn show_param(ui: &imgui::Ui, label: &str, text: &str) {
  let label_cstr = CString::new(label).unwrap();
  let text_cstr = CString::new(text).unwrap();
  unsafe {
    let im_label = imgui::ImStr::from_cstr_unchecked(&label_cstr);
    let im_text = imgui::ImStr::from_cstr_unchecked(&text_cstr);
    ui.label_text(im_text, im_label);
  }
}