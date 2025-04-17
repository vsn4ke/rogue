impl Drawable for MainTab {
    fn draw(&self, frame: &mut Frame, rect: Rect, app: &App) {
        const CAMERA_WIDTH: i32 = 60;
        const CAMERA_HEIGHT: i32 = 30;

        let layout_horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(CAMERA_WIDTH as u16),
                Constraint::Length(30),
            ])
            .split(rect);
        let layout_vertical_0 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(CAMERA_HEIGHT as u16), Constraint::Min(2)])
            .split(layout_horizontal[0]);
        let layout_vertical_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(15), Constraint::Min(2)])
            .split(layout_horizontal[1]);

        frame.render_widget(
            camera(&app.world, CAMERA_WIDTH, CAMERA_HEIGHT),
            layout_vertical_0[0],
        );

        frame.render_widget(
            Block::new()
                .borders(Borders::TOP | Borders::LEFT)
                .title(" Top Left Block "),
            layout_vertical_0[0],
        );

        let bottom_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            horizontal_top: symbols::line::NORMAL.horizontal,
            ..symbols::border::PLAIN
        };

        frame.render_widget(logger(&app.world), layout_vertical_0[1]);
        frame.render_widget(
            Block::new()
                .border_set(bottom_left_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                .title(" Bottom Left Block "),
            layout_vertical_0[1],
        );

        let collapsed_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            ..symbols::border::PLAIN
        };

        frame.render_widget(
            Block::new()
                .border_set(collapsed_left_border_set)
                .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                .title(" Top Right Block "),
            layout_vertical_1[0],
        );

        let collapsed_top_and_left_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            bottom_left: symbols::line::NORMAL.horizontal_up,
            ..symbols::border::PLAIN
        };
        frame.render_widget(
            Block::new()
                .border_set(collapsed_top_and_left_border_set)
                .borders(Borders::ALL)
                .title(" Bottom Right Block "),
            layout_vertical_1[1],
        );
    }
}
