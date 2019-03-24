use cursive::align::HAlign;
use cursive::align::*;
use cursive::direction::*;
use cursive::event::{Event, Key};
use cursive::theme::{Color, Effect};
use cursive::utils::markup::StyledString;
use cursive::view::*;
use cursive::views::*;
use cursive::Cursive;
use std::cmp::Ordering;

use super::model::*;

use std::boxed::Box;

fn config_global_key(siv: &mut Cursive) {
    siv.add_global_callback('q', |s| s.quit());

    siv.add_global_callback('j', |s| {
        s.on_event(Event::from(Key::Down));
    });
    siv.add_global_callback('k', |s| {
        s.on_event(Event::from(Key::Up));
    });
}

fn create_view() -> ViewBox {
    let site = Site::load("/home/baiyan/Projects/Own/baiyan").unwrap();
    let contents = Content::load_all(&site).unwrap();
    let mut contents_views = ListView::new().delimiter();
    contents_views.add_child(
        "",
        LinearLayout::horizontal()
            .child(PaddedView::new(
                (1, 1, 0, 0),
                BoxView::with_fixed_width(5, TextView::new("[+]").effect(Effect::Bold)),
            ))
            .child(PaddedView::new(
                (1, 1, 0, 0),
                BoxView::with_fixed_width(5, TextView::new("ID").effect(Effect::Bold)),
            ))
            .child(PaddedView::new(
                (1, 1, 0, 0),
                BoxView::with_fixed_width(30, TextView::new("TITLE").effect(Effect::Bold)),
            ))
            .child(PaddedView::new(
                (1, 1, 0, 0),
                BoxView::with_fixed_width(30, TextView::new("TYPE").effect(Effect::Bold)),
            ))
            .child(PaddedView::new(
                (1, 1, 0, 0),
                BoxView::with_fixed_width(30, TextView::new("DATETIME").effect(Effect::Bold)),
            )),
    );
    for content in contents {
        contents_views.add_child(
            "",
            LinearLayout::horizontal()
                .child(PaddedView::new(
                    (1, 1, 0, 0),
                    BoxView::with_fixed_width(5, Checkbox::new()),
                ))
                .child(PaddedView::new(
                    (1, 1, 0, 0),
                    BoxView::with_fixed_width(5, TextView::new(format!("{}", content.id))),
                ))
                .child(PaddedView::new(
                    (1, 1, 0, 0),
                    BoxView::with_fixed_width(30, TextView::new(content.title)),
                ))
                .child(PaddedView::new(
                    (1, 1, 0, 0),
                    BoxView::with_fixed_width(30, TextView::new(content.target)),
                ))
                .child(PaddedView::new(
                    (1, 1, 0, 0),
                    BoxView::with_fixed_width(
                        30,
                        TextView::new(content.create_time.format("%b %e, %Y").to_string()),
                    ),
                )),
        );
    }
    let container = LinearLayout::vertical()
        .child(
            Panel::new(
                LinearLayout::horizontal()
                    .child(PaddedView::new(
                        (1, 2, 0, 0),
                        TextView::new(StyledString::styled(
                            site.author.as_ref(),
                            Color::Rgb(200, 0, 0),
                        )),
                    ))
                    .child(PaddedView::new(
                        (1, 1, 0, 0),
                        TextView::new(format!("<{}>", site.subtitle)),
                    )),
            )
            .title(site.title.as_ref()),
        )
        .child(Panel::new(contents_views));
    return ViewBox::boxed(container);
}

pub fn run() {
    let view = create_view();
    let mut siv = Cursive::default();
    config_global_key(&mut siv);
    siv.add_layer(view);
    siv.run();
}
