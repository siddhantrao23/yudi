use cursive::Cursive;
use cursive::theme::Palette;
use cursive::theme::PaletteColor::*;
use cursive::theme::Color::*;
use cursive::view::Nameable;
use cursive::view::Resizable;
use cursive::views::Dialog;
use cursive::views::EditView;
use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    let mut theme = siv.current_theme().clone();
    
    let mut p = Palette::default();
    p[Background] = TerminalDefault;
    p[View] = TerminalDefault;
    p[Shadow] = TerminalDefault;
    p[Primary] = TerminalDefault;
    p[Secondary] = TerminalDefault;
    p[Tertiary] = TerminalDefault;
    p[TitlePrimary] = TerminalDefault;
    p[Highlight] = TerminalDefault;
    p[HighlightInactive] = TerminalDefault;
    
    theme.palette = p;
    theme.shadow = false;
    theme.borders = cursive::theme::BorderStyle::None;
    siv.set_theme(theme);
    
    siv.add_layer(Dialog::new()
        .title("Thoughts for today....")
        .padding_lrtb(1, 1, 1, 0)
        .content(
            EditView::new()
                .on_submit(show_popup)
                .with_name("journal")
                .fixed_width(20)
                .fixed_height(20)
        )
        .button("Ok", |s| {
            let entry = s.call_on_name(
                "journal", |view: &mut EditView| {
                    view.get_content()
            })
            .unwrap();
            show_popup(s, &entry);
        })
    );

    siv.add_global_callback('q', |s| s.quit());    

    siv.run();
}

fn show_popup(s: &mut Cursive, msg: &str) {
    if msg.is_empty() {
        s.add_layer(Dialog::info("Please enter journal entry for the day!"));
    } else {
        // persist the data
        s.pop_layer();
        s.add_layer(
            Dialog::around(TextView::new(msg))
                .button("Quit", |s| s.quit())
        );
    }
}