use cursive::align::HAlign;
use cursive::theme::{BaseColor, Color, PaletteColor, Style, Theme};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{
    LinearLayout, Panel, ResizedView, SelectView, TextView,
};
use cursive::{Cursive, CursiveExt};
use serde_json::Value;

/// Launch interactive explain UI
pub fn interactive_explain(
    key_paths: Vec<String>,
    layers: Vec<(String, Value)>,
    merged: Value,
) {
    let mut siv = Cursive::default();

    apply_theme(&mut siv);

    // --- KEY LIST ----------------------------------------------------------

    let layers_clone = layers.clone();
    let merged_clone = merged.clone();

    let mut keys_list = SelectView::<String>::new()
        .h_align(HAlign::Left)
        .on_select(move |s, key| {
            let content = build_explain_text(key, &layers_clone, &merged_clone);
            s.call_on_name("details", |view: &mut TextView| {
                view.set_content(content);
            });
        });

    for k in &key_paths {
        keys_list.add_item_str(k);
    }

    // Pre-select first key to populate detail panel
    if let Some(first) = key_paths.first() {
        let content = build_explain_text(first, &layers, &merged);
        keys_list.set_selection(0);

        let details_view = TextView::new(content).with_name("details");

        build_layout(&mut siv, keys_list, details_view);
    } else {
        let details_view = TextView::new("No configuration keys found.")
            .with_name("details");

        build_layout(&mut siv, keys_list, details_view);
    }

    // Global quit
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback('Q', |s| s.quit());

    siv.run();
}

/// Apply a professional dark + feminine accent theme
fn apply_theme(siv: &mut Cursive) {
    let mut theme = Theme::default();

    // Dark base
    theme.palette[PaletteColor::Background] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::View] = Color::Dark(BaseColor::Black);
    theme.palette[PaletteColor::Primary] = Color::Light(BaseColor::White);

    // Muted feminine accents
    theme.palette[PaletteColor::TitlePrimary] =
        Color::Light(BaseColor::Magenta); // pink header accents

    theme.palette[PaletteColor::Highlight] =
        Color::Light(BaseColor::Cyan); // selection highlight

    theme.palette[PaletteColor::HighlightText] =
        Color::Dark(BaseColor::Black);

    theme.palette[PaletteColor::Secondary] =
        Color::Light(BaseColor::Blue);

    theme.palette[PaletteColor::Tertiary] =
        Color::Light(BaseColor::Yellow);

    siv.set_theme(theme);
}

/// Build full layout with header + panels + footer
fn build_layout<V>(
    siv: &mut Cursive,
    keys_list: SelectView<String>,
    details_view: V,
)
where
    V: cursive::View + 'static, {
    let header = TextView::new(" agentic • explain ")
        .style(Style::from(Color::Light(BaseColor::Magenta)));

    let footer = TextView::new(
        " ↑/↓ navigate   •   q quit   •   layered config trace ",
    )
    .style(Style::from(Color::Light(BaseColor::Yellow)));

    let layout = LinearLayout::vertical()
        .child(Panel::new(header))
        .child(
            ResizedView::with_full_width(
                LinearLayout::horizontal()
                    .child(
                        ResizedView::with_fixed_width(
                            42,
                            Panel::new(keys_list)
                                .title("Keys")
                                .title_position(HAlign::Left),
                        ),
                    )
                    .child(
                        ResizedView::with_full_width(
                            Panel::new(details_view)
                                .title("Layer Resolution")
                                .title_position(HAlign::Left),
                        ),
                    ),
            ),
        )
        .child(Panel::new(footer));

    siv.add_fullscreen_layer(layout);
}

/// Build styled explain output for selected key
fn build_explain_text(
    key: &str,
    layers: &[(String, Value)],
    merged: &Value,
) -> StyledString {
    let mut styled = StyledString::new();

    styled.append_styled(
        format!("{key}\n\n"),
        Style::from(Color::Light(BaseColor::Magenta)),
    );

    for (name, layer) in layers {
        if let Some(val) = layer.get(key) {
            styled.append_styled(
                format!("{name}: "),
                Style::from(Color::Light(BaseColor::Blue)),
            );

            styled.append_plain(format!("{val}\n"));
        }
    }

    if let Some(final_val) = merged.get(key) {
        styled.append_styled(
            "\n→ final: ",
            Style::from(Color::Light(BaseColor::Yellow)),
        );
        styled.append_plain(format!("{final_val}\n"));
    }

    styled
}