slint::include_modules!();

const LIKES: f64 = 0.30;
const COMMENTS: f64 = 0.55;
const REACH: f64 = 0.05;
const  SHARE: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let likes: f64 = num * LIKES;
        let comments: f64 = num * COMMENTS;
        let reach: f64 = num * REACH;
        let share: f64 = num * SHARE;
        let result = format!("Likes: {:.2}\nComments: {:.2}\nReach: {:.2}\nShare: {:.2}", likes, comments, reach, share);
        ui.set_results(result.into());
    });

    ui.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_100() {
        let result =LIKES + COMMENTS + REACH + SHARE;
        let formatted = f64::trunc(result  * 100.0) / 100.0;
        assert_eq!(formatted, 1.00);
    }
}
