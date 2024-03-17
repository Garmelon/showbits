use showbits_common::{
    color::{BLACK, WHITE},
    widgets::{Block, Text},
    Node, Tree, WidgetExt,
};
use taffy::{
    style_helpers::{length, percent, repeat},
    AlignContent, AlignItems, Display, FlexDirection,
};
use time::Date;

use super::{Context, Drawer};

impl Drawer {
    pub fn draw_calendar(&mut self, year: i32, month: u8) -> anyhow::Result<()> {
        let mut date = Date::from_calendar_date(year, month.try_into()?, 1)?;

        let mut tree = Tree::<Context>::new(WHITE);

        let mut grid = Node::empty()
            .with_display(Display::Grid)
            .with_grid_template_columns(vec![repeat(7, vec![length(50.0)])])
            .with_grid_auto_rows(vec![length(50.0)])
            .with_gap(length(2.0));

        for weekday in ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"] {
            let text = Text::new()
                .with_metrics(Text::default_metrics().scale(2.0))
                .and_plain(weekday)
                .widget(&mut self.ctx.font_stuff)
                .node()
                .with_justify_self(Some(AlignItems::Center))
                .with_align_self(Some(AlignItems::Center))
                .register(&mut tree)?;

            grid = grid.and_child(text);
        }

        let placeholders = date.weekday().number_days_from_monday();
        for _ in 0..placeholders {
            let empty = Node::empty().register(&mut tree)?;
            grid = grid.and_child(empty);
        }

        loop {
            let day = Text::new()
                .and_plain(date.day().to_string())
                .widget(&mut self.ctx.font_stuff)
                .node()
                .register(&mut tree)?;

            let block = Block::new()
                .with_border(BLACK)
                .node()
                .with_border(length(2.0))
                .with_display(Display::Flex)
                .with_justify_content(Some(AlignContent::Center))
                .with_align_items(Some(AlignItems::Center))
                .and_child(day)
                .register(&mut tree)?;

            grid = grid.and_child(block);

            let next_day = date.next_day().unwrap();
            if date.month() != next_day.month() {
                break;
            }
            date = next_day;
        }

        let title = Text::new()
            .and_plain(format!("Ankreuzkalender {year:04}-{month:02}"))
            .widget(&mut self.ctx.font_stuff)
            .node()
            .register(&mut tree)?;

        let root = Node::empty()
            .with_size_width(percent(1.0))
            .with_padding_bottom(length(Self::FEED))
            .with_display(Display::Flex)
            .with_flex_direction(FlexDirection::Column)
            .with_align_items(Some(AlignItems::Center))
            .and_child(title)
            .and_child(grid.register(&mut tree)?)
            .register(&mut tree)?;

        self.printer.print_tree(&mut tree, &mut self.ctx, root)?;
        Ok(())
    }
}
