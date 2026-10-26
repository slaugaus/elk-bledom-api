use handlebars::{Handlebars, TemplateError};

pub fn register_templates(hbs: &mut Handlebars) -> Result<(), TemplateError> {
	hbs.register_template_string("layout", include_str!("./ui/layout.hbs"))?;
	hbs.register_template_string("ui", include_str!("./ui/ui.hbs"))?;

	hbs.register_template_string("devices", include_str!("./ui/components/devices.hbs"))?;
	hbs.register_template_string("power", include_str!("./ui/components/power.hbs"))?;
	hbs.register_template_string("color", include_str!("./ui/components/color.hbs"))?;
	hbs.register_template_string(
		"brightness",
		include_str!("./ui/components/brightness.hbs"),
	)?;

	Ok(())
}
